use crate::core::dto::VersionCache;
use crate::core::{common::response::ApiResponse, dto::UpdateConfigReq};
use lvm_core::config::get::get_language_current_version;
use lvm_core::enums::path::EPath;
use lvm_core::path::get::{get_language_download_path, get_language_version_path};
use serde_json::{json, Value};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs as tokio_fs;

pub const CACHE_TTL: u64 = 60 * 60 * 24;

pub fn default_settings() -> Result<Value, String> {
    let settings_path = EPath::Settings.path();
    let config = json!({
        "autoActivate": true,
        "downloadPath": EPath::Download.path().to_string_lossy(),
        "versionsPath": EPath::Version.path().to_string_lossy(),
        "proxy": false,
    });
    fs::write(
        &settings_path,
        serde_json::to_string_pretty(&config).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(config)
}

// 修改配置
pub fn set_config_values(req: UpdateConfigReq) -> ApiResponse<()> {
    let settings_path = EPath::Settings.path();

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

pub fn del_language(language: &str, version: &str) -> Result<(), String> {
    let download_path = get_language_download_path(language, version);
    let version_path = get_language_version_path(language, version);
    let current_version = get_language_current_version(language).unwrap_or_default();

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

pub async fn versions_list<F, Fut>(language: &str, fetch_fn: F) -> Result<Vec<String>, String>
where
    F: FnOnce() -> Fut, // F -> Future
    Fut: std::future::Future<Output = Result<Vec<String>, String>>,
{
    // let cache_path = dirs::home_dir()
    //     .ok_or("无法获取 home 目录")?
    //     .join(".lvm")
    //     .join("cache")
    //     .join(format!("{}.json", language));

    let cache_path = EPath::CACHE.path().join(format!("{}.json", language));

    // 如果缓存存在
    if cache_path.exists() {
        let data = tokio_fs::read(&cache_path)
            .await
            .map_err(|e| e.to_string())?;

        let cache: VersionCache = serde_json::from_slice(&data).map_err(|e| e.to_string())?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if now - cache.updated_at < CACHE_TTL {
            return Ok(cache.versions);
        }
    }

    let versions = fetch_fn().await?;

    let cache = VersionCache {
        updated_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        versions: versions.clone(),
    };

    if let Some(p) = cache_path.parent() {
        fs::create_dir_all(p).ok();
    }

    let data = serde_json::to_vec(&cache).unwrap();

    tokio_fs::write(cache_path, data).await.ok();

    Ok(versions)
}
