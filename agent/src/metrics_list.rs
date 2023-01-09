use std::env;

use sysinfo::SystemExt;

use crate::redis_info::RedisInfo;
use crate::redis_metrics::RedisMetrics;

pub struct MetricsList {
    sys_info: sysinfo::System,
    redis_info: Option<RedisInfo>,
}

impl MetricsList {
    pub fn new() -> Self {
        let mut redis_info: Option<RedisInfo> = None;
        match env::vars().find(|(key, _)| key == "REDIS_URL") {
            Some((_, url)) => redis_info = Some(RedisInfo::new(url.as_str())),
            None => {}
        }
        Self {
            sys_info: sysinfo::System::new_all(),
            redis_info,
        }
    }
    pub fn flush(&mut self) {
        self.sys_info.refresh_all();
        if let Some(redis_info) = &mut self.redis_info {
            redis_info.flush();
        }
    }
    pub fn get_redis_metrics(&self) -> Option<RedisMetrics> {
        if let Some(redis_info) = &self.redis_info {
            Some(RedisMetrics::metrics(&self.sys_info, &redis_info))
        } else {
            None
        }
    }
}
