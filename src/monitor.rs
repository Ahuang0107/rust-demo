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
    pub last_1min_damage: EachShards,
    pub last_5sec_damage: EachShards,
    pub last_1min_collect: EachShards,
    pub last_5sec_collect: EachShards,
}

impl Monitor {
    pub const fn new() -> Self {
        Self {
            window_size: Duration::from_secs(60),
            record_frequency: Duration::from_secs(1),
            elapsed: Duration::ZERO,
            damage_history: VecDeque::new(),
            collection_history: VecDeque::new(),
            last_1min_damage: EachShards::new(),
            last_5sec_damage: EachShards::new(),
            last_1min_collect: EachShards::new(),
            last_5sec_collect: EachShards::new(),
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
            let record_len = self.record_len() + 1;
            while self.damage_history.len() > record_len {
                self.damage_history.pop_front();
            }
            while self.collection_history.len() > record_len {
                self.collection_history.pop_front();
            }
            (self.last_1min_damage, self.last_5sec_damage) = self.statistics(&self.damage_history);
            (self.last_1min_collect, self.last_5sec_collect) =
                self.statistics(&self.collection_history);
        }
    }
    fn statistics(&self, history: &VecDeque<EachShards>) -> (EachShards, EachShards) {
        let mut last_1min_statistics = EachShards::new();
        let mut last_5sec_statistics = EachShards::new();
        // 永远只从最近第二次记录的数据开始往前统计，因为最近一次的数据，可能才刚开始统计，为 0
        // NOTE 注意，一分钟的窗口时间，是比较长的，是没办法及时地反应当前变化的
        //  如果在原本汇总值稳定的情况下，先上调参数，看到汇总值上升一点后，马上恢复参数，此时汇总值是不会下降的，这个变化要等到1分钟之后才会反应在汇总值上
        //  所以最好分别提供 5s 和 1min 两个维度的数据
        let mut sec_index = 0;
        for history in history.iter().rev().skip(1) {
            for item in history.iter() {
                last_1min_statistics += *item;
            }
            if sec_index < 5 {
                for item in history.iter() {
                    last_5sec_statistics += *item;
                }
            }
            sec_index += 1;
        }
        (last_1min_statistics, last_5sec_statistics)
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
