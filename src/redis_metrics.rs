use sysinfo::SystemExt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RedisMetrics {
    used_memory: u64,
    total_memory: u64,
    total_commands_processed: u64,
    connected_clients: u64,
}

impl RedisMetrics {
    pub fn metrics(con: &mut redis::Connection, sys: &sysinfo::System) -> Self {
        let total_memory = sys.total_memory();
        let used_memory = get_redis_info(
            redis::cmd("info").arg("memory").query(con).unwrap(),
            "used_memory",
        )
        .unwrap();
        let total_commands_processed = get_redis_info(
            redis::cmd("info").arg("stats").query(con).unwrap(),
            "total_commands_processed",
        )
        .unwrap();
        let connected_clients = get_redis_info(
            redis::cmd("info").arg("clients").query(con).unwrap(),
            "connected_clients",
        )
        .unwrap();
        Self {
            total_memory,
            used_memory,
            total_commands_processed,
            connected_clients,
        }
    }
}

fn get_redis_info(query: String, arg: &str) -> Option<u64> {
    let mut result: Option<u64> = None;
    query.lines().for_each(|line| {
        if line.contains((arg.to_string() + ":").as_str()) {
            let temp = line.split(':').last().unwrap();
            result = Some(temp.parse::<u64>().unwrap())
        }
    });
    result
}
