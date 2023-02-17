use std::fs::File;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use std::sync::{Arc, Mutex};

use chrono::Local;
use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
use tokio_tungstenite::tungstenite::{connect, Message, WebSocket};

type PeerMap = Arc<Mutex<Vec<MonitorInfo>>>;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MonitorConfig {
    interval: u64,
    targets: Vec<TargetInfo>,
    duration: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct TargetInfo {
    url: String,
    name: String,
}

impl MonitorConfig {
    pub async fn from_yaml(path: &str) -> anyhow::Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let config = serde_yaml::from_str::<Self>(&content)?;
        Ok(config)
    }
}

struct MonitorInfo {
    name: String,
    ws_stream: WebSocket<MaybeTlsStream<TcpStream>>,
    file: File,
    file_path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = MonitorConfig::from_yaml("./config.yaml").await?;

    let peers: PeerMap = Arc::new(Mutex::new(vec![]));

    std::fs::create_dir_all("./temp").unwrap();
    for target_info in config.targets {
        if let Some((ws_stream, _)) = connect(&target_info.url).ok() {
            let now = Local::now();
            let filename = format!(
                "./temp/[Metrics][{}][{}].csv",
                target_info.name,
                now.format("%Y-%m-%dT%H-%M-%S")
            );
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(filename.clone())
                .unwrap();
            writeln!(&mut file, "timestamp,cpu_usage,memory_usage").unwrap();
            peers.lock().unwrap().push(MonitorInfo {
                name: target_info.name,
                ws_stream,
                file,
                file_path: filename,
            });
        }
    }

    for info in peers.lock().unwrap().iter_mut() {
        info.ws_stream.write_message(Message::Text(String::new()))?;
    }

    let start = Local::now();
    while Local::now().signed_duration_since(start).num_seconds()
        < (config.duration.unwrap() as i64)
    {
        for info in peers.lock().unwrap().iter_mut() {
            if let Some(msg) = info.ws_stream.read_message().ok() {
                match msg {
                    Message::Text(content) => {
                        let timestamp = Local::now().timestamp();
                        println!("[{}] {}", info.name, content);
                        writeln!(&mut info.file, "{},{}", timestamp, content).unwrap();
                        info.ws_stream.write_message(Message::Text(String::new()))?;
                    }
                    _ => {}
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(config.interval));
    }

    for info in peers.lock().unwrap().iter_mut() {
        let filename = Path::new(&info.file_path).file_name().unwrap();
        visualizer::visualization(filename.to_str().unwrap().to_string(), &info.file_path)?;
    }

    Ok(())
}
