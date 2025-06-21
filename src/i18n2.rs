use crate::Language;
use std::sync::atomic::{AtomicU8, Ordering};

static CUR_LANG: AtomicU8 = AtomicU8::new(0);

pub fn set_lang(lang: Language) {
    CUR_LANG.store(lang as u8, Ordering::SeqCst);
}

pub fn get_lang() -> Language {
    match CUR_LANG.load(Ordering::SeqCst) {
        x if x == Language::en as u8 => Language::en,
        x if x == Language::sc as u8 => Language::sc,
        x if x == Language::tc as u8 => Language::tc,
        x if x == Language::jp as u8 => Language::jp,
        _ => Language::en, // 默认英语
    }
}
