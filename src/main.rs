use std::error::Error;
use std::path::Path;

use regex::Regex;
use serde_json::json;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/* %d{yyyy-MM-dd HH:mm:ss.SSS} [%thread] %-5level %logger{50} - %msg%n */

const URL: &'static str = "http://localhost:5080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_path =
        Path::new(r"D:\project\business-project\smart hub 2.0\smart-hub\logs\smart-hub.log");

    let re = Regex::new(r"^(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2})\.(\d{3})").unwrap();
    // 打开文件
    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();

    let mut lines = vec![];

    let client = reqwest::Client::new();

    let login_req = client
        .post(URL.to_string() + "/auth/login")
        .json(&json!({
            "name": "root@example.com",
            "password": "Complexpass#123"
        }))
        .basic_auth("root@example.com", Some("Complexpass#123"))
        .build()?;
    let login_res = client.execute(login_req).await?;
    println!("{:?}", login_res.text().await);

    loop {
        // 读取文件中的新内容
        let bytes_read = file.read_to_end(&mut buffer).await?;

        // 没有新内容则等待一段时间后再次轮询
        if bytes_read == 0 {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            continue;
        }

        let buffer_group = split_buffer(&buffer, 10);
        if buffer_group.len() > 1 {
            for i in 0..buffer_group.len() - 1 {
                let line = String::from_utf8(buffer_group[i].clone()).unwrap();

                if let Some(captures) = re.captures(&line) {
                    let year = captures.get(1).unwrap().as_str();
                    let month = captures.get(2).unwrap().as_str();
                    let day = captures.get(3).unwrap().as_str();
                    let hour = captures.get(4).unwrap().as_str();
                    let minute = captures.get(5).unwrap().as_str();
                    let second = captures.get(6).unwrap().as_str();
                    let millisecond = captures.get(7).unwrap().as_str();

                    if let Some(last_line) = lines.last() {
                        let timestamp =
                            format!("{year}-{month}-{day} {hour}:{minute}:{second}.{millisecond}");
                        let req = client
                            .post(URL.to_string() + "/api/smart-hub/normal/_json")
                            .basic_auth("root@example.com", Some("Complexpass#123"))
                            .json(&json!([{
                                "timestamp": timestamp,
                                "msg": last_line
                            }]))
                            .build()?;
                        let res = client.execute(req).await?;
                    }
                    lines.push(line);
                    continue;
                }

                if let Some(last_line) = lines.last_mut() {
                    last_line.push_str("\n");
                    last_line.push_str(&line);
                }
            }
            buffer = buffer_group[buffer_group.len() - 1].clone();
        }
    }
}

pub fn split_buffer(data: &Vec<u8>, delimiter: u8) -> Vec<Vec<u8>> {
    let mut parts: Vec<Vec<u8>> = Vec::new();
    let mut current_part: Vec<u8> = Vec::new();

    for byte in data {
        if *byte == delimiter {
            // 如果遇到分隔符，将当前部分添加到结果列表中，并重新初始化当前部分
            if !current_part.is_empty() {
                parts.push(current_part);
                current_part = Vec::new();
            }
        } else {
            // 否则，将字节添加到当前部分
            current_part.push(*byte);
        }
    }

    // 将最后一个部分添加到结果列表中（如果不为空）
    if !current_part.is_empty() {
        parts.push(current_part);
    }
    parts
}
