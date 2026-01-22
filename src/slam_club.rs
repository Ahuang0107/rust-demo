use crate::common::UpgradeId::*;
use crate::common::*;
use crate::event::{send_event, Event};
use crate::monitor::ResourceSource;
use crate::resource::Resource;
use i18n_utils::i18n_str;
use std::time::Duration;

static mut SLAM_CLUB: SlamClub = SlamClub::new();

#[derive(Debug)]
pub struct SlamClub {
    elapsed: Duration,
    pub slammers: Vec<()>,
    pub warmup: UpgradeValue,
    pub combo_chance: UpgradeValue,
    pub combo_power: UpgradeValue,
    pub fire_breath: UpgradeValue,
    pub meditation: UpgradeValue,
}

impl SlamClub {
    #[inline]
    pub const fn single() -> &'static SlamClub {
        #[allow(static_mut_refs)]
        unsafe {
            &SLAM_CLUB
        }
    }
    #[inline]
    pub const fn single_mut() -> &'static mut SlamClub {
        #[allow(static_mut_refs)]
        unsafe {
            &mut SLAM_CLUB
        }
    }
    pub const fn new() -> Self {
        Self {
            elapsed: Duration::ZERO,
            slammers: vec![],
            warmup: UpgradeValue::multi(),
            combo_chance: UpgradeValue::multi(),
            combo_power: UpgradeValue::once(),
            fire_breath: UpgradeValue::once(),
            meditation: UpgradeValue::once(),
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
                self.slammers.push(());
            }
            SC_Warmup => self.warmup.level_up(),
            SC_ComboChance => self.combo_chance.level_up(),
            SC_ComboPower => self.combo_power.acquire(),
            SC_FireBreath => self.fire_breath.acquire(),
            SC_Meditation => self.meditation.acquire(),
            _ => {}
        }
    }

    fn update(&mut self, delta: Duration) {
        self.elapsed += delta;
        while self.elapsed > Duration::from_millis(500) {
            self.elapsed -= Duration::from_millis(500);
            send_event(Event::DamageResource(
                ResourceSource::Slammer,
                Resource::shards(rand::random_range(1..10)),
            ));
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
    }
}
