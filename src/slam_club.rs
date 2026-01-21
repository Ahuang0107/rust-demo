use crate::common::*;
use i18n_utils::i18n_string;

static mut SLAM_CLUB: SlamClub = SlamClub::new();

#[derive(Debug)]
pub struct SlamClub {
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
            slammers: vec![],
            warmup: UpgradeValue::multi(),
            combo_chance: UpgradeValue::multi(),
            combo_power: UpgradeValue::once(),
            fire_breath: UpgradeValue::once(),
            meditation: UpgradeValue::once(),
        }
    }
    pub fn upgrade_options(&self, mut op: impl FnMut(UpgradeOption)) {
        op(UpgradeOption {
            id: UpgradeId::SC_Slammer,
            name: i18n_string!(
                en=>String::from("Slammer"),
                sc=>String::from("铁头糯米"),
            ),
            description: i18n_string!(
                en=>String::from("Buy!\
                Slammers use their head - they understand that without pain, there can be no gain."),
                sc=>String::from("购买！\
                铁头糯米喜欢用脑袋，它们明白没有痛苦就没有收获。")
            ),
            value: UpgradeValue::gnorps(self.slammers.len()),
            price: Resource::gnorps(1),
        });
    }
}
