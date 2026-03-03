// commands.rs
// Tauri command layer (bridge between frontend and backend)

use crate::core::dto::PageResult;
use crate::core::manager::LanguageManager;
use crate::core::utils::config::get_base_path;
use crate::utils::config::get_download_path;

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
