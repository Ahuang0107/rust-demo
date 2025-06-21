pub mod i18n1;
pub mod i18n2;
pub mod i18n3;
pub mod i18n4;
pub mod i18n5;

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Language {
    en,
    sc,
    tc,
    jp,
    ko,
    es,
    fr,
    ru,
}
