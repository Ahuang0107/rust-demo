use crate::resource::Resource;
use std::time::Duration;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UpgradeId {
    // ------ Slam Club ------
    SC_Slammer,
    SC_Warmup,
    SC_ComboChance,
    SC_ComboPower,
    SC_FireBreath,
    SC_Meditation,
    // ------ Express ------
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
    /// -1 表示这个升级项表示的是某种能力，只需要升级一次
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
    pub fn acquire(&mut self) {
        self.acquired = true;
    }
    pub fn level_up(&mut self) {
        self.level += 1;
    }
}

#[derive(Debug)]
pub struct UpgradeOption {
    pub id: UpgradeId,
    pub name: String,
    pub description: String,
    pub value: UpgradeValue,
    /// 当 price 为 0 时，并且 value.level 不是 0 时，就表示已经升级到最大值了
    pub price: Resource,
}

pub trait Building {
    fn name(&self) -> &'static str;
    fn handle_upgrade(&mut self, upgrade_id: UpgradeId);
    fn update(&mut self, delta: Duration);
    fn upgrade_options(&self, op: impl FnMut(UpgradeOption));
}

pub fn scale_number(value: f32) -> (f32, char) {
    let abs_value = value.abs();

    match abs_value {
        _ if abs_value >= 1e18 => (value / 1e18, 'E'),
        _ if abs_value >= 1e15 => (value / 1e15, 'P'),
        _ if abs_value >= 1e12 => (value / 1e12, 'T'),
        _ if abs_value >= 1e9 => (value / 1e9, 'B'),
        _ if abs_value >= 1e6 => (value / 1e6, 'M'),
        _ if abs_value >= 1e3 => (value / 1e3, 'K'),
        _ => (value, ' '),
    }
}

/// TODO 这个方案还是有一个问题的，那就是四舍五入后，会导致显示的，和实际需要的值对不上
pub fn format_scale_number(value: f32) -> String {
    let (num, unit) = scale_number(value);
    if num.fract() == 0.0 {
        // 整数，如 1.0
        format!("{:.0}", num)
    } else {
        // 尝试保留 2 位小数
        let formatted = format!("{:.2}", num);

        // 去除末尾的零
        let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');

        if unit == ' ' {
            format!("{trimmed}")
        } else {
            format!("{trimmed}{unit}")
        }
    }
}
