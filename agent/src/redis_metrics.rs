use sysinfo::SystemExt;

use crate::redis_info::{RedisInfo, RedisInfoType};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RedisMetrics {
    used_memory: u64,
    total_memory: u64,
    total_commands_processed: u64,
    connected_clients: u64,
}

impl RedisMetrics {
    pub fn metrics(sys_info: &sysinfo::System, redis_info: &RedisInfo) -> Self {
        Self {
            total_memory: sys_info.total_memory(),
            used_memory: redis_info
                .get_metrics_arg(RedisInfoType::UsedMemory)
                .unwrap(),
            total_commands_processed: redis_info
                .get_metrics_arg(RedisInfoType::TotalCommandsProcessed)
                .unwrap(),
            connected_clients: redis_info
                .get_metrics_arg(RedisInfoType::ConnectedClients)
                .unwrap(),
        }
    }
}
