#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UpgradeId {
    SC_Slammer,
    SC_Warmup,
    SC_ComboChance,
    SC_ComboPower,
    SC_FireBreath,
    SC_Meditation,
    EX_Runner,
    EX_Capacity,
    EX_Speed,
    EX_GEET_Protocol,
    EX_Breaktime,
    EX_DeliveryRush,
    EX_Teleportation,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct UpgradeValue {
    /// 表示是否已解锁，默认未解锁
    pub unlocked: bool,
    pub acquired: bool,
    pub level: i32,
}

impl UpgradeValue {
    pub const fn once() -> Self {
        Self {
            unlocked: true,
            acquired: false,
            level: -1,
        }
    }
    pub const fn multi() -> Self {
        Self {
            unlocked: true,
            acquired: false,
            level: 0,
        }
    }
    pub const fn gnorps(v: usize) -> Self {
        Self {
            unlocked: true,
            acquired: false,
            level: v as i32,
        }
    }
}

#[derive(Debug)]
pub struct UpgradeOption {
    pub id: UpgradeId,
    pub name: String,
    pub description: String,
    pub value: UpgradeValue,
    pub price: Resource,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Resource {
    pub shards: usize,
    pub gnorps: usize,
    pub zybelliums: usize,
}

impl Resource {
    pub const fn new() -> Self {
        Self {
            shards: 0,
            gnorps: 0,
            zybelliums: 0,
        }
    }
    pub const fn shards(v: usize) -> Self {
        Self {
            shards: v,
            gnorps: 0,
            zybelliums: 0,
        }
    }
    pub const fn gnorps(v: usize) -> Self {
        Self {
            shards: 0,
            gnorps: v,
            zybelliums: 0,
        }
    }
    pub const fn zybelliums(v: usize) -> Self {
        Self {
            shards: 0,
            gnorps: 0,
            zybelliums: v,
        }
    }
}

impl std::ops::AddAssign for Resource {
    fn add_assign(&mut self, rhs: Self) {
        self.shards += rhs.shards;
        self.gnorps += rhs.gnorps;
        self.zybelliums += rhs.zybelliums;
    }
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}|{}|{}", self.shards, self.gnorps, self.zybelliums)
    }
}
