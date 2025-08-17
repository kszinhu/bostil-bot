use rust_i18n::Backend;
use std::collections::HashMap;

pub struct DiscordI18n {
    trs: HashMap<String, HashMap<String, String>>,
}

impl DiscordI18n {
    pub fn new() -> Self {
        // get from files in "app/public/locales"
        let files = std::fs::read_dir("app/public/locales").unwrap();
        let trs = serde_yaml::from_str::<HashMap<String, HashMap<String, String>>>(
            &files
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        if e.path().extension()?.to_str()? == "yaml" {
                            std::fs::read_to_string(e.path()).ok()
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<String>>()
                .join("\n"),
        )
        .unwrap();

        DiscordI18n { trs }
    }
}

impl Backend for DiscordI18n {
    fn available_locales(&self) -> Vec<&str> {
        return self.trs.keys().map(|k| k.as_str()).collect();
    }

    fn translate(&self, locale: &str, key: &str) -> Option<&str> {
        return self.trs.get(locale)?.get(key).map(|k| k.as_str());
    }
}
