use std::path::Path;
use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> io::Result<()> {
    let file_path = Path::new("logs/server.log");
    update_file_realtime(file_path).await?;
    Ok(())
}

async fn update_file_realtime(file_path: &Path) -> io::Result<()> {
    // 打开文件
    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();

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
                println!("New Line: {:?}", String::from_utf8(buffer_group[i].clone()));
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
