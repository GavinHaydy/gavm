use crate::enums::path::EPath;
use crate::path::get::current_path;
use serde_json::Value;
use std::fs;

pub fn get_config_value(key: &str) -> Option<Value> {
    let config_path = EPath::Settings.path();
    let content = fs::read_to_string(config_path).ok()?;

    let json: Value = serde_json::from_str(&content).ok()?;
    json.get(key).cloned()
}

pub fn get_config_bool(key: &str, default: bool) -> bool {
    get_config_value(key)
        .and_then(|v| v.as_bool())
        .unwrap_or(default)
}

pub fn get_language_current_version(language: &str) -> Result<String, String> {
    let current_path = current_path(language);

    fs::read_to_string(&current_path)
        .map(|s| s.trim().to_string())
        .map_err(|e| e.to_string())
}
