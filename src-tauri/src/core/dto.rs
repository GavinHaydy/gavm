// dto.rs
// Data Transfer Objects returned to frontend

use serde::Serialize;

#[derive(Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub install_status: bool,
    pub use_status: bool,
}

#[derive(Serialize)]
pub struct PageResult {
    pub total: usize,
    pub list: Vec<VersionInfo>,
}
