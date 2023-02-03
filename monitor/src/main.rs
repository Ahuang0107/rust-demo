use std::env;

use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let (_, url) = env::vars().find(|(key, _)| key == "URL").unwrap();

    monitor_agent(&url).await?;

    Ok(())
}

async fn monitor_agent(url: &str) -> anyhow::Result<()> {
    let (mut ws_stream, _) = connect_async(url).await?;
    ws_stream.send(Message::Text(String::new())).await?;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        match msg {
            Message::Text(content) => {
                println!("{}", content);
                std::thread::sleep(std::time::Duration::from_secs(5));
                ws_stream.send(Message::Text(String::new())).await?;
            }
            _ => {}
        }
    }
    Ok(())
}
