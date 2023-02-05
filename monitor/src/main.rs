use std::fs::File;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use chrono::Local;
use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
use tokio_tungstenite::tungstenite::{connect, Message, WebSocket};

type PeerMap = Arc<Mutex<Vec<MonitorInfo>>>;

struct MonitorInfo {
    ws_stream: WebSocket<MaybeTlsStream<TcpStream>>,
    file: File,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MonitorConfig {
    interval: u64,
    targets: Vec<TargetInfo>,
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = MonitorConfig::from_yaml("./config.yaml").await?;

    let peers: PeerMap = Arc::new(Mutex::new(vec![]));

    for target_info in config.targets {
        if let Some((ws_stream, _)) = connect(&target_info.url).ok() {
            let now = Local::now();
            let filename = format!(
                "metrics-{}-{}.csv",
                target_info.name,
                now.format("%Y-%m-%d-%H-%M-%S")
            );
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(filename)
                .unwrap();
            writeln!(&mut file, "timestamp,cpu_usage,memory_usage").unwrap();
            peers.lock().unwrap().push(MonitorInfo { ws_stream, file });
        }
    }

    for info in peers.lock().unwrap().iter_mut() {
        info.ws_stream.write_message(Message::Text(String::new()))?;
    }

    loop {
        for info in peers.lock().unwrap().iter_mut() {
            if let Some(msg) = info.ws_stream.read_message().ok() {
                match msg {
                    Message::Text(content) => {
                        println!("{}", content);
                        writeln!(&mut info.file, "{},{}", Local::now().timestamp(), content)
                            .unwrap();
                        info.ws_stream.write_message(Message::Text(String::new()))?;
                    }
                    _ => {}
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(config.interval));
    }
}
