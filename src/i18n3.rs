use crate::Language;
use std::sync::atomic::{AtomicU8, Ordering};

static CUR_LANG: AtomicU8 = AtomicU8::new(0);

pub fn set_lang(lang: Language) {
    CUR_LANG.store(lang as u8, Ordering::Relaxed);
}

#[inline]
pub fn get_lang() -> Language {
    unsafe { std::mem::transmute(CUR_LANG.load(Ordering::Relaxed)) }
}
