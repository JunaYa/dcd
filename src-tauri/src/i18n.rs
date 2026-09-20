use std::sync::OnceLock;

pub const LANGUAGES: [&str; 7] = ["zh", "en", "ja", "ko", "fr", "de", "ar"];
pub fn resolve_language(tag: &str) -> String {
    let base = tag.split(['-', '_']).next().unwrap_or("en").to_lowercase();
    if LANGUAGES.contains(&base.as_str()) { base } else { "en".into() }
}
pub fn system_language() -> String {
    resolve_language(&tauri_plugin_os::locale().unwrap_or_else(|| "en".into()))
}
pub fn text(language: &str, key: &str) -> String {
    static MESSAGES: OnceLock<serde_json::Value> = OnceLock::new();
    let messages = MESSAGES.get_or_init(|| serde_json::from_str(include_str!("../../src/i18n/messages.json")).expect("valid bundled translations"));
    messages[key][language].as_str().or_else(|| messages[key]["en"].as_str()).unwrap_or(key).to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resolves_system_region_tags_and_falls_back() {
        for (tag, expected) in [("zh-Hans-CN", "zh"), ("ja_JP", "ja"), ("FR-ca", "fr"), ("ar-SA", "ar"), ("es-ES", "en")] {
            assert_eq!(resolve_language(tag), expected);
        }
    }
}
