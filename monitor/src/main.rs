use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let case_url = Url::parse("ws://127.0.0.1:7878/ws")?;

    let (mut ws_stream, _) = connect_async(case_url).await?;
    ws_stream.send(Message::Ping(Vec::new())).await?;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            println!("{:?}", msg);
            ws_stream.send(Message::Text(String::new())).await?;
        }
    }

    Ok(())
}
