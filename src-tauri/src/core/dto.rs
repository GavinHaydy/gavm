// dto.rs
// Data Transfer Objects returned to frontend

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    pub version: String,
    pub install_status: bool,
    pub use_status: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult {
    pub total: usize,
    pub list: Vec<VersionInfo>,
    pub page_size: usize,
    pub page: usize,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConfigReq {
    #[serde(default)]
    pub auto_activate: Option<bool>,

    #[serde(default)]
    pub download_path: Option<String>,

    #[serde(default)]
    pub versions_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct VersionCache {
    pub updated_at: u64,
    pub versions: Vec<String>,
}
