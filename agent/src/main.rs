use std::net::SocketAddr;

use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tungstenite::{Error, Message};

use crate::log_config::log_config;
use crate::metrics_list::MetricsList;

mod fatal;
mod log_config;
mod metrics_list;
mod redis_info;
mod redis_metrics;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    log_config();

    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");

        tokio::spawn(accept_connection(peer, stream));
    }

    Ok(())
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> tungstenite::error::Result<()> {
    let mut ws_stream = accept_async(stream).await?;
    println!("{} enter", peer);

    let mut metrics_list = MetricsList::new();

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if let Some(_) = msg.into_text().ok() {
            metrics_list.flush();
            let redis_metrics = metrics_list.get_redis_metrics();
            let response = serde_json::to_string(&redis_metrics).unwrap_or_else(|_| {
                fatal!("unable to serde {:?} to string", redis_metrics);
            });
            ws_stream.send(Message::Text(response)).await?;
        }
    }

    println!("{} exit", peer);

    Ok(())
}
