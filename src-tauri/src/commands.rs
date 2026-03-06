// commands.rs
// Tauri command layer (bridge between frontend and backend)

use crate::core::dto::PageResult;
use crate::core::manager::LanguageManager;
use crate::core::utils::config::get_base_path;
use crate::utils::config::get_download_path;
use serde_json::Value;
use std::fs;

#[tauri::command]
pub async fn list_versions(
    language: String,
    page: usize,
    page_size: usize,
    key_word: Option<&str>,
) -> Result<PageResult, String> {
    let manager = LanguageManager::new(language)?;
    manager.list_versions(page, page_size, key_word).await
}

#[tauri::command]
pub async fn install(
    app: tauri::AppHandle, // 注入 AppHandle 以读取配置
    window: tauri::Window<tauri::Wry>,
    language: String,
    version: String,
) -> Result<(), String> {
    // 直接从后端配置获取下载目录
    let base_dir = get_base_path(&app);
    let download_dir = get_download_path(&app);
    let manager = LanguageManager::new(language)?;
    manager
        .install(
            window,
            version,
            base_dir.to_string_lossy().to_string(),
            download_dir.to_string_lossy().to_string(),
        )
        .await
}

#[tauri::command]
pub async fn base_path() -> Result<String, String> {
    let base_dir = shim::get_base_path().to_string_lossy().to_string();
    Ok(base_dir)
}

#[tauri::command]
pub fn get_config_value(key: &str) -> Option<Value> {
    let config_path = shim::get_base_path().join("settings.json");
    let content = fs::read_to_string(config_path).ok()?;

    let json: Value = serde_json::from_str(&content).ok()?;

    json.get(key).cloned()
}

#[tauri::command]
pub fn get_config_values(keys: Vec<&str>) -> Value {
    let mut map = serde_json::Map::new();

    for key in keys {
        if let Some(value) = get_config_value(key) {
            map.insert((*key).to_string(), value);
        }
    }

    Value::Object(map)
}