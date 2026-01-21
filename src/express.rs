use crate::common::*;
use i18n_utils::i18n_string;

static mut EXPRESS: Express = Express::new();

#[derive(Debug)]
pub struct Express {
    pub runner: Vec<()>,
    pub capacity: UpgradeValue,
    pub speed: UpgradeValue,
    pub geet_protocol: UpgradeValue,
    pub breaktime: UpgradeValue,
    pub delivery_rush: UpgradeValue,
    pub teleportation: UpgradeValue,
}

impl Express {
    #[inline]
    pub const fn single() -> &'static Express {
        #[allow(static_mut_refs)]
        unsafe {
            &EXPRESS
        }
    }
    #[inline]
    pub const fn single_mut() -> &'static mut Express {
        #[allow(static_mut_refs)]
        unsafe {
            &mut EXPRESS
        }
    }
    pub const fn new() -> Self {
        Self {
            runner: vec![],
            capacity: UpgradeValue::multi(),
            speed: UpgradeValue::multi(),
            geet_protocol: UpgradeValue::multi(),
            breaktime: UpgradeValue::once(),
            delivery_rush: UpgradeValue::once(),
            teleportation: UpgradeValue::once(),
        }
    }
    pub fn upgrade_options(&self, mut op: impl FnMut(UpgradeOption)) {
        op(UpgradeOption {
            id: UpgradeId::EX_Runner,
            name: i18n_string!(
                en=>String::from("Runner"),
                sc=>String::from("跑腿糯米"),
            ),
            description: i18n_string!(
                en=>String::from("Buy!\
                Runners are the lifeblood of the gnorpconomy. They ensure that shards get picked up and delivered."),
                sc=>String::from("购买！\
                跑腿糯米就是糯米社会运行下去的根基命脉。它们会确保石块的运送和交付。")
            ),
            value: UpgradeValue::gnorps(self.runner.len()),
            price: Resource::gnorps(1),
        });
    }
}
