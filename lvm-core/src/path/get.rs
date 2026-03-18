use crate::config::get::get_config_value;
use crate::enums::path::EPath;
use std::path::PathBuf;

/// 获取 base_path，支持用户自定义
pub fn get_base_path() -> PathBuf {
    dirs::home_dir().unwrap().join(".lvm")
}

pub fn get_config_path(key: &str) -> PathBuf {
    let value = get_config_value(key).unwrap_or_else(|| panic!("config key '{}' not found", key));

    let path_str = value
        .as_str()
        .unwrap_or_else(|| panic!("config key '{}' is not a string", key));

    PathBuf::from(path_str)
}

pub fn get_download_path() -> PathBuf {
    let download_dir = EPath::Download.path();

    // 自动创建下载目录
    if !download_dir.exists() {
        let _ = std::fs::create_dir_all(&download_dir);
    }

    download_dir
}

pub fn current_path(language: &str) -> PathBuf {
    EPath::Version.path().join(language).join("current")
}

pub fn get_language_version_path(language: &str, version: &str) -> PathBuf {
    get_config_path(EPath::Version.config_key().unwrap())
        .join(language)
        .join(version)
}

pub fn get_language_download_path(language: &str, version: &str) -> PathBuf {
    get_config_path(EPath::Download.config_key().unwrap())
        .join(format!("{}-{}.zip", language, version))
}
