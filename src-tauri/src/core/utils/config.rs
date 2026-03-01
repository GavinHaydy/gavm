// src/core/utils/config.rs
use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

pub fn get_base_path(app: &AppHandle) -> PathBuf {
    let mut path_str = String::from("d:\\lvm");

    // 尝试获取已经加载的 store 或创建一个新的
    // 注意：这里的 ".settings.json" 路径需要转换成 PathBuf
    let settings_path = PathBuf::from(".settings.json");

    // 使用 get_store 获取引用
    if let Some(store) = app.get_store(settings_path) {
        // store 默认被内部互斥锁保护，我们需要锁定它来读取
        if let Some(v) = store.get("base_path") {
            if let Some(s) = v.as_str() {
                path_str = s.to_string();
            }
        }
    }

    PathBuf::from(path_str)
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
