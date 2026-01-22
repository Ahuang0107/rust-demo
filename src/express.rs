use crate::common::UpgradeId::*;
use crate::common::*;
use crate::event::{send_event, Event};
use crate::monitor::ResourceSource;
use crate::resource::Resource;
use i18n_utils::{i18n_str, i18n_string};
use std::time::Duration;

static mut EXPRESS: Express = Express::new();

#[derive(Debug)]
pub struct Express {
    elapsed: Duration,
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
            elapsed: Duration::ZERO,
            runner: vec![],
            capacity: UpgradeValue::multi(),
            speed: UpgradeValue::multi(),
            geet_protocol: UpgradeValue::multi(),
            breaktime: UpgradeValue::once(),
            delivery_rush: UpgradeValue::once(),
            teleportation: UpgradeValue::once(),
        }
    }
    pub fn capacity(&self) -> usize {
        self.capacity.level as usize
    }
    pub fn speed(&self) -> f32 {
        40.0 * (1.0 + self.speed.level as f32 * 0.25)
    }
}

impl Building for Express {
    fn name(&self) -> &'static str {
        i18n_str!(en=>"Express",sc=>"糯米速递站")
    }
    fn handle_upgrade(&mut self, upgrade_id: UpgradeId) {
        match upgrade_id {
            EX_Runner => {
                self.runner.push(());
            }
            EX_Capacity => self.capacity.level_up(),
            EX_Speed => self.speed.level_up(),
            EX_GEET_Protocol => self.geet_protocol.level_up(),
            EX_Breaktime => self.breaktime.level_up(),
            EX_DeliveryRush => self.delivery_rush.level_up(),
            EX_Teleportation => self.teleportation.level_up(),
            _ => {}
        }
    }

    fn update(&mut self, delta: Duration) {
        self.elapsed += delta;
        while self.elapsed > Duration::from_millis(500) {
            self.elapsed -= Duration::from_millis(500);
            send_event(Event::CollectResource(
                ResourceSource::Runner,
                Resource::shards(rand::random_range(1..10)),
            ));
        }
    }

    fn upgrade_options(&self, mut op: impl FnMut(UpgradeOption)) {
        op(UpgradeOption {
            id: EX_Runner,
            name: i18n_string!(
                en=>String::from("Runner"),
                sc=>String::from("跑腿糯米"),
            ),
            description: i18n_string!(
                en=>String::from("Runners are the lifeblood of the gnorpconomy. They ensure that shards get picked up and delivered."),
                sc=>String::from("跑腿糯米就是糯米社会运行下去的根基命脉。它们会确保石块的运送和交付。")
            ),
            value: UpgradeValue::gnorps(self.runner.len()),
            price: Resource::gnorps(1),
        });
        op(UpgradeOption {
            id: EX_Capacity,
            name: String::from("容量"),
            description: String::from(
                "跑腿糯米只能搬运1S，但是这个升级会增加容量\n\n- 增加1S搬运容量",
            ),
            value: self.capacity,
            price: Resource::shards((10.0 + 21.0 * 1.45_f32.powi(self.capacity.level)) as i32),
        });
        op(UpgradeOption {
            id: EX_Speed,
            name: String::from("速度"),
            description: String::from(
                "跑腿糯米只能搬运1S，但是这个升级会增加容量\n\n- 增加1S搬运容量",
            ),
            value: self.speed,
            price: Resource::shards(match self.speed.level {
                0 => 166,
                1 => 166666,
                _ => -1,
            }),
        });
        op(UpgradeOption {
            id: EX_GEET_Protocol,
            name: String::from("G.E.E.T 协议"),
            description: String::from(
                "跑腿糯米只能搬运1S，但是这个升级会增加容量\n\n- 增加1S搬运容量",
            ),
            value: self.geet_protocol,
            price: Resource::shards(match self.geet_protocol.level {
                0 => 5000,
                1 => 16000,
                _ => -1,
            }),
        });
    }
}
