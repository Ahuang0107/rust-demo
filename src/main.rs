use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let buf_reader = BufReader::new(&mut stream);
        #[allow(unused)]
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let result: String = redis::cmd("info").arg("memory").query(&mut con).unwrap();
        let mut used_memory_human = "";
        result.lines().for_each(|line| {
            if line.contains("used_memory_human") {
                used_memory_human = line.split(':').last().unwrap()
            }
        });
        println!("used memory: {}", used_memory_human);

        let response = "HTTP/1.1 200 OK\r\n\r\n".to_string() + used_memory_human;
        stream.write_all(response.as_bytes()).unwrap();
    }
}
