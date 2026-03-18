use crate::{core::enums::proxy::EDownload, utils::semver::sort_versions_desc};

use lvm_core::config::get::get_config_bool;
use serde::Deserialize;

#[derive(Deserialize)]
struct GoRelease {
    version: String,
    stable: bool,
}

/// 从 Python 官方 API 获取版本
pub async fn fetch_versions_go() -> Result<Vec<String>, String> {
    let proxy = get_config_bool("proxy", false);
    let url = if proxy {
        format!("{}?mode=json&include=all", EDownload::GoListProxy)
    } else {
        format!("{}?mode=json&include=all", EDownload::Go)
    };

    let releases: Vec<GoRelease> = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let mut versions: Vec<String> = releases
        .into_iter()
        .filter(|r| r.stable)
        .map(|r| r.version.trim_start_matches("go").to_string())
        .collect();

    sort_versions_desc(&mut versions);

    Ok(versions)
}
