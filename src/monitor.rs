use std::collections::VecDeque;
use std::time::Duration;

#[derive(Debug)]
pub struct Monitor {
    /// 统计过去多长时间
    window_size: Duration,
    /// 每个多久统计一次
    record_frequency: Duration,
    elapsed: Duration,
    damage_history: VecDeque<EachShards>,
    collection_history: VecDeque<EachShards>,
    pub last_damage: EachShards,
    pub last_collect: EachShards,
}

impl Monitor {
    pub const fn new() -> Self {
        Self {
            window_size: Duration::from_secs(60),
            record_frequency: Duration::from_millis(1000),
            elapsed: Duration::ZERO,
            damage_history: VecDeque::new(),
            collection_history: VecDeque::new(),
            last_damage: EachShards::new(),
            last_collect: EachShards::new(),
        }
    }
    fn record_len(&self) -> usize {
        (self.window_size.as_millis() / self.record_frequency.as_millis()) as usize
    }
    pub fn update(&mut self, delta: Duration) {
        self.elapsed += delta;
        let mut changed = false;
        while self.elapsed >= self.record_frequency {
            self.elapsed -= self.record_frequency;
            self.damage_history.push_back(EachShards::new());
            self.collection_history.push_back(EachShards::new());
            changed = true;
        }
        if changed {
            let record_len = self.record_len() + 2;
            while self.damage_history.len() > record_len {
                self.damage_history.pop_front();
            }
            while self.collection_history.len() > record_len {
                self.collection_history.pop_front();
            }
            self.last_damage = self.statistics(&self.damage_history);
            self.last_collect = self.statistics(&self.collection_history);
        }
    }
    fn statistics(&self, history: &VecDeque<EachShards>) -> EachShards {
        let mut statistics = EachShards::new();
        let record_len = self.record_len();
        let mut index = 0;
        // 永远只从最近第二次记录的数据开始往前统计，因为最近一次的数据，可能才刚开始统计，为 0
        for history in history.iter().rev().skip(1) {
            if index >= record_len {
                break;
            }
            for item in history.iter() {
                statistics += *item;
            }
            index += 1;
        }
        statistics
    }
    pub fn damage(&mut self, source_key: ShardsSource, resource_value: u32) {
        let length = self.damage_history.len();
        if length > 0 {
            let cur_history = &mut self.damage_history[length - 1];
            *cur_history += (source_key, resource_value);
        }
    }
    pub fn collect(&mut self, source_key: ShardsSource, resource_value: u32) {
        let length = self.collection_history.len();
        if length > 0 {
            let cur_history = &mut self.collection_history[length - 1];
            *cur_history += (source_key, resource_value);
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub enum ShardsSource {
    Click,
    Runner,
    Slammer,
}

/// 表示各个不同来源的数据
#[derive(Debug)]
pub struct EachShards(Vec<(ShardsSource, u32)>);

impl EachShards {
    pub const fn new() -> Self {
        Self(vec![])
    }
}

impl std::ops::Deref for EachShards {
    type Target = Vec<(ShardsSource, u32)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for EachShards {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::AddAssign<(ShardsSource, u32)> for EachShards {
    fn add_assign(&mut self, rhs: (ShardsSource, u32)) {
        for (source, value) in self.iter_mut() {
            if *source == rhs.0 {
                *value += rhs.1;
                return;
            }
        }
        self.push(rhs);
    }
}
