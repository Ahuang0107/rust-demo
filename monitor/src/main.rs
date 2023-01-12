#[tokio::main]
async fn main() {
    let client: reqwest::Client = reqwest::Client::new();
    loop {
        let res = client.get("http://127.0.0.1:7878").send().await.unwrap();
        if res.status().is_success() {
            let body = res.text().await.unwrap();
            let redis_metrics = serde_json::from_str::<RedisMetrics>(body.as_str()).unwrap();
            println!("{:?}", redis_metrics);
        } else {
            panic!("get request return error {}", res.text().await.unwrap())
        }

        std::thread::sleep(std::time::Duration::from_secs(5))
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RedisMetrics {
    used_memory: u64,
    total_memory: u64,
    total_commands_processed: u64,
    connected_clients: u64,
}
