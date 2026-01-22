use crate::common::UpgradeId;
use crate::monitor::ResourceSource;
use crate::resource::Resource;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub enum Event {
    DamageResource(ResourceSource, Resource),
    CollectResource(ResourceSource, Resource),
    Upgrade(UpgradeId),
}

thread_local! {
    // 全局事件队列
    static EVENT_QUEUE: RefCell<VecDeque<Event>> = RefCell::new(VecDeque::new());
    // 递归事件队列
    static EVENT_QUEUE_RECURSION: RefCell<VecDeque<Event>> = RefCell::new(VecDeque::new());
}

/// 表示是否正在处理事件中
static PROCESSING: AtomicBool = AtomicBool::new(false);

#[inline]
fn processing() -> bool {
    PROCESSING.load(Ordering::Relaxed)
}

#[inline]
fn set_processing(v: bool) {
    PROCESSING.store(v, Ordering::Relaxed)
}

/// 发送事件到事件队列
#[allow(unused)]
pub fn send_event(event: Event) {
    if processing() {
        // 当正在处理事件时，新加入的事件，需要插入到 EVENT_QUEUE_RECURSION
        EVENT_QUEUE_RECURSION.with(|queue| {
            queue.borrow_mut().push_back(event);
        });
    } else {
        // 否则，插入到 EVENT_QUEUE 的末尾
        EVENT_QUEUE.with(|queue| {
            queue.borrow_mut().push_back(event);
        });
    }
}

/// 尝试获取事件队列中的下一个事件
///
/// 每一帧开始时处理上一帧积累的所有事件
///
/// ```rust,ignore
/// while let Some(event) = try_take_event() {
///     log::info!("handle event: {:?}", event);
/// }
/// ```
pub fn try_take_event() -> Option<Event> {
    if !processing() {
        set_processing(true);
    }
    let event = EVENT_QUEUE_RECURSION.with(|queue| {
        let mut queue_ref = queue.borrow_mut();
        queue_ref.pop_front()
    });
    if event.is_some() {
        return event;
    }

    let event = EVENT_QUEUE.with(|queue| {
        let mut queue_ref = queue.borrow_mut();
        queue_ref.pop_front()
    });
    if event.is_none() {
        set_processing(false);
    }

    event
}
