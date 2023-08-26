use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(false)
            .build()
            .expect("Could not find chrome-executable"),
    )?;
    let tab = browser.new_tab()?;
    tab.navigate_to("https://wikipedia.org")?
        .wait_for_element("input#searchInput")?;
    tab.type_str("Elvis Aaron Presley")?;
    tab.wait_for_element("button[type='submit']")?.click()?;
    Ok(())
}
