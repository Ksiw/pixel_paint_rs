use std::collections::HashMap;

pub type BundleMap = HashMap<String, String>;

pub fn parse_bundle(text: &str) -> BundleMap {
    serde_json::from_str(text).unwrap_or_default()
}
