// commands.rs
// Tauri command layer (bridge between frontend and backend)

use crate::core::common::response::ApiResponse;
use crate::core::dto::{PageResult, UpdateConfigReq};
use crate::core::manager::LanguageManager;
use crate::core::utils::config::{get_base_path, set_config_values};
use crate::utils::config::{get_config_value, get_download_path};
use serde_json::Value;

#[tauri::command]
pub async fn list_versions(
    language: String,
    page: usize,
    page_size: usize,
    key_word: Option<String>,
) -> ApiResponse<PageResult> {
    let manager = match LanguageManager::new(language) {
        Ok(m) => m,
        Err(e) => return ApiResponse::error(&e),
    };

    match manager.list_versions(page, page_size, key_word).await {
        Ok(data) => ApiResponse::success_with_data(data),
        Err(e) => ApiResponse::error(&e),
    }
}

#[tauri::command]
pub async fn install(
    app: tauri::AppHandle, // 注入 AppHandle 以读取配置
    window: tauri::Window<tauri::Wry>,
    language: String,
    version: String,
) -> ApiResponse<()> {
    // 直接从后端配置获取下载目录
    let base_dir = get_base_path(&app);
    let download_dir = get_download_path(&app);
    let manager = match LanguageManager::new(language) {
        Ok(m) => m,
        Err(e) => return ApiResponse::error(&e.to_string()),
    };

    match manager
        .install(
            window,
            version,
            base_dir.to_string_lossy().to_string(),
            download_dir.to_string_lossy().to_string(),
        )
        .await
    {
        Ok(_) => ApiResponse::success_with_msg(),
        Err(e) => ApiResponse::error(&e),
    }
}

#[tauri::command]
pub async fn use_version(language: String, version: String) -> ApiResponse<()> {
    let manager = match LanguageManager::new(language) {
        Ok(m) => m,
        Err(e) => return ApiResponse::error(&e.to_string()),
    };

    match manager.use_version(&version).await {
        Ok(_) => ApiResponse::success_with_msg(),
        Err(e) => ApiResponse::error(&e),
    }
}

#[tauri::command]
pub async fn uninstall(language: String, version: String) -> ApiResponse<()> {
    let manager = match LanguageManager::new(language) {
        Ok(m) => m,
        Err(e) => return ApiResponse::error(&e.to_string()),
    };

    match manager.uninstall(&version).await {
        Ok(_) => ApiResponse::success_with_msg(),
        Err(e) => ApiResponse::error(&e),
    }
}

#[tauri::command]
pub async fn base_path() -> Result<String, String> {
    let base_dir = shim::get_base_path().to_string_lossy().to_string();
    Ok(base_dir)
}

#[tauri::command]
pub fn get_config_values(keys: Vec<&str>) -> ApiResponse<Value> {
    let mut map = serde_json::Map::new();

    for key in keys {
        if let Some(value) = get_config_value(key) {
            map.insert((*key).to_string(), value);
        }
    }

    ApiResponse::success_with_data(Value::Object(map))
}

#[tauri::command]
pub fn update_configs(req: UpdateConfigReq) -> ApiResponse<()> {
    set_config_values(req)
}
