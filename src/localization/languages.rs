use super::bundle::{BundleMap, parse_bundle};
use std::collections::HashMap;

pub const SUPPORTED_LANGUAGES: [(&str, &str); 7] = [
    ("ru", "lang_ru"),
    ("en", "lang_en"),
    ("de", "lang_de"),
    ("es", "lang_es"),
    ("fr", "lang_fr"),
    ("ja", "lang_ja"),
    ("zh", "lang_zh"),
];

pub fn load_bundles() -> HashMap<String, BundleMap> {
    let mut bundles = HashMap::new();
    bundles.insert(
        "ru".to_string(),
        parse_bundle(include_str!("../../locales/ru.json")),
    );
    bundles.insert(
        "en".to_string(),
        parse_bundle(include_str!("../../locales/en.json")),
    );
    bundles.insert(
        "de".to_string(),
        parse_bundle(include_str!("../../locales/de.json")),
    );
    bundles.insert(
        "es".to_string(),
        parse_bundle(include_str!("../../locales/es.json")),
    );
    bundles.insert(
        "fr".to_string(),
        parse_bundle(include_str!("../../locales/fr.json")),
    );
    bundles.insert(
        "ja".to_string(),
        parse_bundle(include_str!("../../locales/ja.json")),
    );
    bundles.insert(
        "zh".to_string(),
        parse_bundle(include_str!("../../locales/zh.json")),
    );
    bundles
}
