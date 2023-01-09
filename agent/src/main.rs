use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::ops::Add;

use dotenv::dotenv;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use sysinfo::{System, SystemExt};

use crate::redis_info::RedisInfo;
use crate::redis_metrics::RedisMetrics;

mod redis_info;
mod redis_metrics;

#[macro_export]
macro_rules! fatal {
    () => {
        log::error!("config panic");
        panic!()
    };
    ($msg:expr) => {
        log::error!($msg);
        panic!()
    };
    ($msg:expr,) => {
        log::error!($msg);
        panic!()
    };
    ($fmt:expr, $($arg:tt)+) => {
        let msg = format!($fmt, $($arg)+);
        log::error!("{}", &msg);
        panic!()
    };
}

// 主要目的是获得压测期间所有服务器和具体中间件的指标变化
fn main() {
    dotenv().ok();

    let stdout = ConsoleAppender::builder().build();
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("log/agent-output.log")
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Debug),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    let (_, redis_url) = env::vars()
        .find(|(key, _)| key == "REDIS_URL")
        .unwrap_or_else(|| {
            fatal!("unable to read REDIS_URL");
        });

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut sys_info = System::new_all();
    let mut redis_info = RedisInfo::new(redis_url.as_str());

    for stream in listener.incoming() {
        let mut stream = stream.unwrap_or_else(|_| {
            fatal!("unable to get stream");
        });

        let buf_reader = BufReader::new(&mut stream);
        #[allow(unused)]
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| {
                result.unwrap_or_else(|_| {
                    fatal!("unable to get stream lines");
                })
            })
            .take_while(|line| !line.is_empty())
            .collect();

        sys_info.refresh_all();
        redis_info.flush();

        let redis_metrics = RedisMetrics::metrics(&sys_info, &redis_info);

        let mut response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
        response = response.add(
            serde_json::to_string(&redis_metrics)
                .unwrap_or_else(|_| {
                    fatal!("unable to serde {:?} to string", redis_metrics);
                })
                .as_str(),
        );
        stream.write_all(response.as_bytes()).unwrap_or_else(|_| {
            fatal!("unable to write response");
        });
    }
}
