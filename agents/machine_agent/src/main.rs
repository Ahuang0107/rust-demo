use std::net::SocketAddr;

use futures_util::{SinkExt, StreamExt};
use sysinfo::{CpuExt, NetworkExt, NetworksExt, SystemExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tungstenite::{Error, Message};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 服务端监听127.0.0.1，则客户端只能通过127.0.0.1连接，不能通过局域网ip或者外网ip连接
    // 监听地址设置为0.0.0.0，这样本机、内网、外网都可以连接
    let listener = TcpListener::bind("0.0.0.0:7878").await?;
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

    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_all();

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        match msg {
            Message::Text(_) => {
                sys.refresh_cpu();
                sys.refresh_memory();
                sys.refresh_networks();
                let cu = sys.global_cpu_info().cpu_usage();
                let mu = ((sys.used_memory() as f64) / (sys.total_memory() as f64)) as f32 * 100.0;
                let mut received_total = 0_u64;
                let mut transmitted_total = 0_u64;
                for (_, data) in sys.networks().iter() {
                    received_total += data.received();
                    transmitted_total += data.transmitted();
                }
                println!(
                    "cpu usage: {:.2}%, mem usage: {:.2}%, data received: {} B, data sent: {} B",
                    cu, mu, received_total, transmitted_total
                );
                ws_stream
                    .send(Message::Text(String::from(format!(
                        "{:.2},{:.2},{},{}",
                        cu, mu, received_total, transmitted_total
                    ))))
                    .await?;
            }
            _ => {}
        }
    }

    println!("{} exit", peer);

    Ok(())
}
