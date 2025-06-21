fn main() {
    // rust_demo::i18n1::set_lang(rust_demo::Language::en);
    // println!("{:?}", rust_demo::i18n1::get_lang());
    // rust_demo::i18n1::set_lang(rust_demo::Language::sc);
    // println!("{:?}", rust_demo::i18n1::get_lang());
    //
    // rust_demo::i18n2::set_lang(rust_demo::Language::en);
    // println!("{:?}", rust_demo::i18n2::get_lang());
    // rust_demo::i18n2::set_lang(rust_demo::Language::sc);
    // println!("{:?}", rust_demo::i18n2::get_lang());

    // rust_demo::i18n3::set_lang(rust_demo::Language::en);
    // println!("{:?}", rust_demo::i18n4::tr("Revenant"));
    // rust_demo::i18n3::set_lang(rust_demo::Language::ko);
    // println!("{:?}", rust_demo::i18n4::tr("Revenant"));

    // rust_demo::i18n3::set_lang(rust_demo::Language::en);
    // println!("{:?}", rust_demo::i18n5::Text::Revenant.text());
    // rust_demo::i18n3::set_lang(rust_demo::Language::ko);
    // println!("{:?}", rust_demo::i18n5::Text::Revenant.text());

    // return;

    let mut translations = Vec::new();
    let mut translations_keys = Vec::new();
    for line in std::fs::read_to_string("translations_sample.txt")
        .unwrap()
        .lines()
    {
        if !line.is_empty() {
            let units_string = line
                .split('|')
                .map(|unit| unit.trim().replace("\"", ""))
                .collect::<Vec<_>>();
            let units = units_string
                .iter()
                .map(|unit| unit.as_str())
                .collect::<Vec<_>>();
            let unit_key = {
                let s = units[0];
                let unit_key = &s[..10.min(s.len())];
                let unit_key = unit_key
                    .replace(" ", "")
                    .replace("\'", "")
                    .replace("-", "")
                    .replace("+", "")
                    .replace("?", "")
                    .replace("\\", "")
                    .replace("/", "")
                    .replace(".", "")
                    .replace("!", "")
                    .replace("0", "")
                    .replace("1", "")
                    .replace("2", "")
                    .replace("3", "")
                    .replace("4", "")
                    .replace("5", "")
                    .replace("6", "")
                    .replace("7", "")
                    .replace("8", "")
                    .replace("9", "");
                unit_key
            };

            // 打印组成的 enum 的所有值
            // println!("{unit_key},");

            // 打印需要配置的 phf_map!
            // println!("    {unit_key:?} => {:?},", units);

            // 打印需要配置的 i18n!
            // let lang_text_list = ["en", "sc", "tc", "jp", "ko", "es", "fr", "ru"]
            //     .iter()
            //     .enumerate()
            //     .map(|(index, lang)| format!("{lang}=>{:?}", units[index]))
            //     .collect::<Vec<_>>();
            // println!("Text::{unit_key} => i18n!({}),", lang_text_list.join(", "));

            assert_eq!(units.len(), 8);
            translations.push(units_string);
            translations_keys.push(unit_key);
        }
    }
    // println!("Translations: {}", translations.len());
    // println!("{:?}", translations_keys);
}
