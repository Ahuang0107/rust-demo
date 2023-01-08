use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::ops::Add;

use sysinfo::{System, SystemExt};

use crate::redis_metrics::RedisMetrics;

mod redis_metrics;
mod util;

// 主要目的是获得压测期间所有服务器和具体中间件的指标变化
fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut sys = System::new_all();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let buf_reader = BufReader::new(&mut stream);
        #[allow(unused)]
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        sys.refresh_all();
        let redis_metrics = RedisMetrics::metrics(&mut con, &sys);

        let mut response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
        response = response.add(serde_json::to_string(&redis_metrics).unwrap().as_str());
        stream.write_all(response.as_bytes()).unwrap();
    }
}
