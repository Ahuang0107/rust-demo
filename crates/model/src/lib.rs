mod models;

pub use models::*;

pub trait Output {
    fn output(&self) -> String;
    fn to_xml(&self) -> String {
        const METADATA: &'static str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#;
        METADATA.to_string() + &self.output()
    }
}
