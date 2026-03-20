use crate::utils::semver::sort_versions_desc;

use crate::core::enums::proxy::EDownload;
use lvm_core::config::get::get_config_bool;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
struct NodeRelease {
    version: String,
    lts: Option<BoolOrString>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(untagged)]
enum BoolOrString {
    Bool(bool),
    Str(String),
}

#[allow(dead_code)]
pub async fn fetch_versions_node() -> Result<Vec<String>, String> {
    let proxy = get_config_bool("proxy", false);
    let url = if proxy {
        format!("{}?mode=json&include=all", EDownload::NodeProxy)
    } else {
        format!("{}?mode=json&include=all", EDownload::Node)
    };

    let releases: Vec<NodeRelease> = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let mut versions = Vec::new();

    for r in releases {
        if let Some(BoolOrString::Str(_)) = r.lts {
            let version = r.version.trim_start_matches('v').to_string();
            versions.push(version);
        }
    }

    sort_versions_desc(&mut versions);

    Ok(versions)
}
