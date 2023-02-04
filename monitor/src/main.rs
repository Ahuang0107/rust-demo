use std::env;
use std::io::Write;

use chrono::Local;
use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let (_, url) = env::vars().find(|(key, _)| key == "URL").unwrap();
    let (_, interval) = env::vars().find(|(key, _)| key == "INTERVAL").unwrap();

    monitor_agent(&url, interval.parse::<u64>().unwrap()).await?;

    Ok(())
}

async fn monitor_agent(url: &str, interval: u64) -> anyhow::Result<()> {
    let (mut ws_stream, _) = connect_async(url).await?;
    ws_stream.send(Message::Text(String::new())).await?;

    let now = Local::now();
    let filename = format!("metrics-{}.csv", now.format("%Y-%m-%d-%H-%M-%S"));
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();
    writeln!(&mut file, "timestamp,cpu_usage,memory_usage").unwrap();

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        match msg {
            Message::Text(content) => {
                println!("{}", content);
                writeln!(&mut file, "{},{}", Local::now().timestamp(), content).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(interval));
                ws_stream.send(Message::Text(String::new())).await?;
            }
            _ => {}
        }
    }
    Ok(())
}
