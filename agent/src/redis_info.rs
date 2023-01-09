pub struct RedisInfo {
    connection: redis::Connection,
    raw_memory: String,
    raw_stats: String,
    raw_clients: String,
}

impl RedisInfo {
    pub fn new() -> Self {
        let connection = redis::Client::open("redis://127.0.0.1/")
            .unwrap()
            .get_connection()
            .unwrap();
        Self {
            connection,
            raw_memory: "".to_string(),
            raw_stats: "".to_string(),
            raw_clients: "".to_string(),
        }
    }
    pub fn flush(&mut self) {
        self.raw_memory = redis::cmd("info")
            .arg("memory")
            .query(&mut self.connection)
            .unwrap();
        self.raw_stats = redis::cmd("info")
            .arg("stats")
            .query(&mut self.connection)
            .unwrap();
        self.raw_clients = redis::cmd("info")
            .arg("clients")
            .query(&mut self.connection)
            .unwrap();
    }
    pub fn get_metrics_arg(&self, arg: RedisInfoType) -> Option<u64> {
        match arg {
            RedisInfoType::UsedMemory => get_redis_info(&self.raw_memory, arg.metrics_arg()),
            RedisInfoType::TotalCommandsProcessed => {
                get_redis_info(&self.raw_stats, arg.metrics_arg())
            }
            RedisInfoType::ConnectedClients => get_redis_info(&self.raw_clients, arg.metrics_arg()),
        }
    }
}

pub enum RedisInfoType {
    UsedMemory,
    TotalCommandsProcessed,
    ConnectedClients,
}

impl RedisInfoType {
    pub fn metrics_arg(&self) -> &str {
        match self {
            Self::UsedMemory => "used_memory",
            Self::TotalCommandsProcessed => "total_commands_processed",
            Self::ConnectedClients => "connected_clients",
        }
    }
}

fn get_redis_info(query: &String, arg: &str) -> Option<u64> {
    let mut result: Option<u64> = None;
    query.lines().for_each(|line| {
        if line.contains((arg.to_string() + ":").as_str()) {
            let temp = line.split(':').last().unwrap();
            result = Some(temp.parse::<u64>().unwrap())
        }
    });
    result
}
