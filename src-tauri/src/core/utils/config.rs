use std::fs;
// src/core/utils/config.rs
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

use crate::core::common::response::ApiResponse;

/// 获取用户 base_path
pub fn get_base_path(app: &AppHandle) -> PathBuf {
    // 1️⃣ 默认路径（跨平台）
    let default_path = dirs::home_dir().expect("cannot get home dir").join(".lvm"); // 默认 ~/.lvm / C:\Users\xxx\.lvm

    // 2️⃣ config / settings.json 文件
    // Tauri store 会在这个路径生成
    let settings_path = app
        .path()
        .app_data_dir()
        .unwrap_or(default_path.clone())
        .join("settings.json");

    // 3️⃣ 尝试读取 store
    if let Some(store) = app.get_store(settings_path) {
        if let Some(v) = store.get("base_path") {
            if let Some(s) = v.as_str() {
                return PathBuf::from(s); // 用户自定义路径优先
            }
        }
    }

    // 4️⃣ fallback 到默认路径
    default_path
}
pub fn get_download_path(app: &AppHandle) -> PathBuf {
    let base = get_base_path(app);
    let download_dir = base.join("download");

    // 自动创建下载目录
    if !download_dir.exists() {
        let _ = std::fs::create_dir_all(&download_dir);
    }

    download_dir
}

pub fn init_settings() -> PathBuf {
    let base_dir = shim::get_base_path();
    let settings_path = base_dir.join("settings.json");
    if !settings_path.exists() {
        let config = json!({
            "auto_activate": true,
            "download_path": base_dir.join("download"),
            "versions_path": base_dir.join("versions"),
        });

        fs::write(
            &settings_path,
            serde_json::to_string_pretty(&config).unwrap(),
        )
        .expect("Failed to create settings file");
    }
    settings_path
}

// 修改配置
pub fn set_config_values(new_values: HashMap<String, Value>) -> ApiResponse<()> {
    let settings_path = init_settings();

    // 1. 读取原来的配置
    let content = match fs::read_to_string(&settings_path) {
        Ok(c) => c,
        Err(_) => return ApiResponse::error("Failed to read settings file"),
    };

    let mut settings: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return ApiResponse::error("Invalid settings format"),
    };

    // 确保是对象
    if !settings.is_object() {
        return ApiResponse::error("Settings is not a JSON object");
    }

    let obj = settings.as_object_mut().unwrap();

    // 2. 更新
    for (k, v) in new_values {
        obj.insert(k, v);
    }

    // 3. 写回文件
    if fs::write(
        &settings_path,
        serde_json::to_string_pretty(&settings).unwrap(),
    )
    .is_err()
    {
        return ApiResponse::error("Failed to write settings file");
    }

    ApiResponse::success_with_msg()
}
