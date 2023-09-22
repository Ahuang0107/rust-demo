use futures_util::{SinkExt, StreamExt};
use log::*;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    env_logger::init();

    let (messages_tx, messages_rx) = mpsc::channel::<Message>();

    let (ws_stream, _) = connect_async("ws://127.0.0.1:9002").await.unwrap();
    let (mut write, mut read) = ws_stream.split();
    tokio::spawn(async move {
        while let Some(message) = read.next().await {
            let message = message.unwrap();
            match messages_tx.send(message) {
                Ok(()) => {}
                Err(err) => {
                    error!("send incoming channel message failed: {err}");
                }
            }
        }
    });
    let call_registry = Arc::new(Mutex::new(vec![]));
    {
        let call_registry = call_registry.clone();
        std::thread::spawn(move || {
            while let Ok(msg) = messages_rx.recv() {
                let mut call_registry = call_registry.lock().unwrap();
                call_registry.push(msg.to_text().unwrap().to_string())
            }
        });
    }
    loop {
        write
            .send(Message::Text("random message".into()))
            .await
            .unwrap();
        let call_registry = call_registry.lock().unwrap();
        info!("call registry: {call_registry:?}");
        info!("sleep for 10s");
        sleep(Duration::from_secs(10));
    }
}
