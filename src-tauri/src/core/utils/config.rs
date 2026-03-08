use std::{fs, path::Path};
// src/core/utils/config.rs
use serde_json::{json, Value};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

use crate::core::{common::response::ApiResponse, dto::UpdateConfigReq};

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
            "autoActivate": true,
            "downloadPath": base_dir.join("download"),
            "versionsPath": base_dir.join("versions"),
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
pub fn set_config_values(req: UpdateConfigReq) -> ApiResponse<()> {
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
    let req_value = serde_json::to_value(req).unwrap();
    if let Value::Object(map) = req_value {
        for (k, v) in map {
            if !v.is_null() {
                obj.insert(k, v);
            }
        }
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

pub fn get_config_value(key: &str) -> Option<Value> {
    let config_path = shim::get_base_path().join("settings.json");
    let content = fs::read_to_string(config_path).ok()?;

    let json: Value = serde_json::from_str(&content).ok()?;

    json.get(key).cloned()
}

pub fn get_config_bool(key: &str, default: bool) -> bool {
    get_config_value(key)
        .and_then(|v| v.as_bool())
        .unwrap_or(default)
}

pub fn get_dirs(path: &Path) -> Result<Vec<String>, std::io::Error> {
    let dirs = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_dir() {
                entry.file_name().into_string().ok()
            } else {
                None
            }
        })
        .collect();

    Ok(dirs)
}

fn get_language_current_path(language: &str) -> Result<String, String> {
    let current_path = shim::get_base_path().join(language).join("current");

    fs::read_to_string(&current_path)
        .map(|s| s.trim().to_string())
        .map_err(|e| e.to_string())
}

fn get_language_version_path(language: &str, version: &str) -> PathBuf {
    shim::get_base_path().join(language).join(version)
}

fn get_language_download_path(language: &str, version: &str) -> PathBuf {
    shim::get_base_path()
        .join("download")
        .join(format!("{}-{}.zip", language, version))
}

pub fn del_language(language: &str, version: &str) -> Result<(), String> {
    let download_path = get_language_download_path(language, version);
    let version_path = get_language_version_path(language, version);
    let current_version = get_language_current_path(language).unwrap_or_default();

    if current_version == version {
        return Err(format!(
            "Cannot delete the currently active version {}",
            version
        ));
    }

    if !version_path.exists() && !download_path.exists() {
        return Err(format!("Version {} not found", version));
    }

    if version_path.exists() {
        fs::remove_dir_all(&version_path).map_err(|e| e.to_string())?;
    }

    if download_path.exists() {
        fs::remove_file(&download_path).map_err(|e| e.to_string())?;
    }

    Ok(())
}
