use crate::common::UpgradeId::*;
use crate::common::*;
use crate::event::{send_event, Event};
use crate::monitor::ShardsSource;
use crate::resource::Resource;
use crate::singleton;
use crate::talent::{Talent, Talents};
use i18n_utils::i18n_str;
use std::ops::RangeInclusive;
use std::time::Duration;

singleton!(SlamClub = SlamClub::new());

#[derive(Debug)]
pub struct SlamClub {
    elapsed: Duration,
    pub slammers: Vec<Slammer>,
    pub upgrades: Upgrades,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Upgrades {
    pub warmup: UpgradeValue,
    pub combo_chance: UpgradeValue,
    pub combo_power: UpgradeValue,
    pub fire_breath: UpgradeValue,
    pub meditation: UpgradeValue,
}

#[derive(Debug)]
pub struct Slammer {
    elapsed: Duration,
    upgrades: Upgrades,
    on_fire_breath: bool,
    on_motivation: bool,
}

impl Slammer {
    pub fn new(upgrades: Upgrades) -> Slammer {
        Self {
            elapsed: Duration::ZERO,
            upgrades,
            on_fire_breath: false,
            on_motivation: false,
        }
    }
    pub fn damage_range(&self) -> RangeInclusive<usize> {
        let mut base = 0..=5;
        if self.upgrades.combo_power.acquired {
            base = (*base.start() + 2)..=(*base.end());
        }
        if self.upgrades.meditation.acquired {
            base = (*base.start() + 5)..=(*base.end() + 5);
        }
        base
    }
    pub fn damage(&self) -> u32 {
        let mut base = 1.0 + self.upgrades.warmup.level as f32;
        if self.on_fire_breath {
            base *= 30.0;
        }
        if self.on_motivation {
            base *= 2.0;
        }
        base as u32
    }
    pub fn update(&mut self, delta: Duration) {
        self.elapsed += delta;
        while self.elapsed > Duration::from_millis(500) {
            self.elapsed -= Duration::from_millis(500);
            send_event(Event::DamageResource(ShardsSource::Slammer, self.damage()));
        }
    }
}

impl SlamClub {
    pub const fn new() -> Self {
        Self {
            elapsed: Duration::ZERO,
            slammers: vec![],
            upgrades: Upgrades {
                warmup: UpgradeValue::multi(),
                combo_chance: UpgradeValue::multi(),
                combo_power: UpgradeValue::once(),
                fire_breath: UpgradeValue::once(),
                meditation: UpgradeValue::once(),
            },
        }
    }
}

impl Building for SlamClub {
    fn name(&self) -> &'static str {
        i18n_str!(en=>"Slam Club",sc=>"搏击俱乐部")
    }
    fn handle_upgrade(&mut self, upgrade_id: UpgradeId) {
        match upgrade_id {
            SC_Slammer => {
                self.slammers.push(Slammer::new(self.upgrades));
                return;
            }
            SC_Warmup => self.upgrades.warmup.level_up(),
            SC_ComboChance => self.upgrades.combo_chance.level_up(),
            SC_ComboPower => self.upgrades.combo_power.acquire(),
            SC_FireBreath => self.upgrades.fire_breath.acquire(),
            SC_Meditation => self.upgrades.meditation.acquire(),
            _ => {
                return;
            }
        }
        for slammer in self.slammers.iter_mut() {
            slammer.upgrades = self.upgrades;
        }
    }

    fn update(&mut self, delta: Duration) {
        let on_motivation = Talents::single().if_toggle(Talent::Motivation);
        for slammer in self.slammers.iter_mut() {
            slammer.on_motivation = on_motivation;
            slammer.update(delta);
        }
    }

    fn upgrade_options(&self, mut op: impl FnMut(UpgradeOption)) {
        op(UpgradeOption {
            id: SC_Slammer,
            name: String::from("铁头糯米"),
            description: String::from("铁头糯米喜欢用脑袋，它们明白没有痛苦就没有收获。"),
            value: UpgradeValue::gnorps(self.slammers.len()),
            price: Resource::gnorps(1),
        });
        op(UpgradeOption {
            id: SC_Warmup,
            name: String::from("热身"),
            description: String::from(
                "铁头糯米通过热身跳跃来为即将到来的痛苦做准备。\n\n- 每次跳跃增加 1S 伤害",
            ),
            value: self.upgrades.warmup,
            price: Resource::shards(match self.upgrades.warmup.level {
                0 => 56,
                1 => 164,
                3 => 720,
                4 => 2_400,
                5 => 64_000,
                6 => 143_000,
                7 => 584_000,
                8 => 2_400_000,
                _ => 0,
            }),
        });
        op(UpgradeOption {
            id: SC_ComboChance,
            name: String::from("连招几率"),
            description: String::from(
                "多亏了糯米推进实验室那边的家伙，铁头糯米现在准备了小型强脉冲喷漆背包，让它们能打出连招！\n没错，它们可以在连招上继续打连招。\n\n- 连招几率增加20%",
            ),
            value: self.upgrades.combo_chance,
            price: Resource::shards(match self.upgrades.combo_chance.level {
                0 => 56,
                1 => 164,
                3 => 720,
                4 => 2_400,
                _ => 0,
            }),
        });
        op(UpgradeOption {
            id: SC_Meditation,
            name: String::from("冥想"),
            description: String::from(
                "铁头糯米现在会在撞击前冥想，让石块飞得更远\n\n- 花费4秒来预热准备\n+5石块溅射范围，从0-5提升到5-10\n可以关闭",
            ),
            value: self.upgrades.meditation,
            price: Resource::shards(12),
        });
    }
}
