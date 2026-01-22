use crate::common::format_scale_number;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Resource {
    pub shards: u64,
    pub gnorps: u16,
    pub zybelliums: u8,
}

impl Resource {
    pub const fn new() -> Self {
        Self {
            shards: 0,
            gnorps: 0,
            zybelliums: 0,
        }
    }
    pub const fn shards(v: u64) -> Self {
        Self {
            shards: v,
            gnorps: 0,
            zybelliums: 0,
        }
    }
    pub const fn gnorps(v: u16) -> Self {
        Self {
            shards: 0,
            gnorps: v,
            zybelliums: 0,
        }
    }
    pub const fn zybelliums(v: u8) -> Self {
        Self {
            shards: 0,
            gnorps: 0,
            zybelliums: v,
        }
    }
    pub fn if_empty(&self) -> bool {
        self.shards == 0 && self.gnorps == 0 && self.zybelliums == 0
    }
    pub fn ui_string(&self) -> String {
        [
            if self.shards > 0 {
                format!("{} S", format_scale_number(self.shards as f32))
            } else {
                String::new()
            },
            if self.gnorps > 0 {
                format!("{} G", self.gnorps)
            } else {
                String::new()
            },
            if self.zybelliums > 0 {
                format!("{} Z", self.zybelliums)
            } else {
                String::new()
            },
        ]
        .join(" ")
    }
}

impl std::ops::AddAssign for Resource {
    fn add_assign(&mut self, rhs: Self) {
        self.shards += rhs.shards;
        self.gnorps += rhs.gnorps;
        self.zybelliums += rhs.zybelliums;
    }
}
