mod common;
mod express;
mod slam_club;

pub use crate::express::Express;
pub use crate::slam_club::SlamClub;
use i18n_utils::{set_lang, Language};

fn main() {
    loop {
        SlamClub::single().upgrade_options(|u| {
            println!("{:?}", u);
        });
        Express::single().upgrade_options(|u| {
            println!("{:?}", u);
        });

        set_lang(Language::sc);

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
