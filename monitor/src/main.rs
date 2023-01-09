#[tokio::main]
async fn main() {
    let client: reqwest::Client = reqwest::Client::new();
    loop {
        let res = client.get("http://127.0.0.1:7878").send().await.unwrap();
        if res.status().is_success() {
            let body = res.text().await.unwrap();
            println!("{}", body);
        } else {
            panic!("get request return error {}", res.text().await.unwrap())
        }

        std::thread::sleep(std::time::Duration::from_secs(5))
    }
}
