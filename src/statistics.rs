use crate::monitor::{Monitor, ShardsSource};
use crate::resource::Resource;
use std::time::Duration;

static mut STATISTICS: Statistics = Statistics::new();

#[derive(Debug)]
pub struct Statistics {
    pub resource: Resource,
    pub monitor: Monitor,
}

impl Statistics {
    #[inline]
    pub const fn single() -> &'static Statistics {
        #[allow(static_mut_refs)]
        unsafe {
            &STATISTICS
        }
    }
    #[inline]
    pub const fn single_mut() -> &'static mut Statistics {
        #[allow(static_mut_refs)]
        unsafe {
            &mut STATISTICS
        }
    }
    pub const fn new() -> Self {
        Self {
            resource: Resource::new(),
            monitor: Monitor::new(),
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.monitor.update(delta);
    }
    pub fn damage(&mut self, source_key: ShardsSource, resource_value: u32) {
        self.monitor.damage(source_key, resource_value);
    }
    pub fn collect(&mut self, source_key: ShardsSource, resource_value: u32) {
        self.resource.shards += resource_value as u64;
        self.monitor.collect(source_key, resource_value);
    }
}
