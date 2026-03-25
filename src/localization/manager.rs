use super::bundle::BundleMap;
use super::languages::load_bundles;
use std::collections::HashMap;

pub struct LocalizationManager {
    current_language: String,
    bundles: HashMap<String, BundleMap>,
}

impl LocalizationManager {
    pub fn new(language: &str) -> Self {
        let bundles = load_bundles();
        let current_language = if bundles.contains_key(language) {
            language.to_string()
        } else {
            "ru".to_string()
        };
        Self {
            current_language,
            bundles,
        }
    }

    pub fn get(&self, key: &str) -> String {
        self.bundles
            .get(&self.current_language)
            .and_then(|bundle| bundle.get(key))
            .cloned()
            .or_else(|| {
                self.bundles
                    .get("en")
                    .and_then(|bundle| bundle.get(key))
                    .cloned()
            })
            .unwrap_or_else(|| key.to_string())
    }

    pub fn set_language(&mut self, language: &str) {
        if self.bundles.contains_key(language) {
            self.current_language = language.to_string();
        }
    }

    pub fn current_language(&self) -> &str {
        &self.current_language
    }
}
