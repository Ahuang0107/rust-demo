use crate::fatal;

pub struct RedisInfo {
    connection: redis::Connection,
    raw_memory: String,
    raw_stats: String,
    raw_clients: String,
}

impl RedisInfo {
    pub fn new(redis_url: &str) -> Self {
        log::info!("try to connect to {}", redis_url);
        let connection = redis::Client::open(redis_url)
            .unwrap_or_else(|_| {
                fatal!("unable to open redis client");
            })
            .get_connection()
            .unwrap_or_else(|_| {
                fatal!("unable to get redis connection");
            });
        log::info!("connected");
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
            .unwrap_or_else(|_| {
                fatal!("unable to query memory");
            });
        self.raw_stats = redis::cmd("info")
            .arg("stats")
            .query(&mut self.connection)
            .unwrap_or_else(|_| {
                fatal!("unable to query stats");
            });
        self.raw_clients = redis::cmd("info")
            .arg("clients")
            .query(&mut self.connection)
            .unwrap_or_else(|_| {
                fatal!("unable to query clients");
            });
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
            let temp = line.split(':').last().unwrap_or_else(|| {
                fatal!("unable to get split {} with ':'", line);
            });
            result = Some(temp.parse::<u64>().unwrap_or_else(|_| {
                fatal!("unable to parse {} to u64", temp);
            }))
        }
    });
    result
}
