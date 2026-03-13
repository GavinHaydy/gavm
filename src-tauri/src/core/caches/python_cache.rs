use crate::utils::semver::sort_versions_desc;

use serde::Deserialize;

#[derive(Deserialize)]
struct Release {
    name: String,
    pre_release: bool,
}

/// 从 Python 官方 API 获取版本
pub async fn fetch_versions_python() -> Result<Vec<String>, String> {
    let url = "https://www.python.org/api/v2/downloads/release/?is_published=true";

    let releases: Vec<Release> = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let mut versions = Vec::new();

    for r in releases {
        if let Some(v) = r.name.strip_prefix("Python ") {
            if v.starts_with("3.") && !r.pre_release {
                versions.push(v.to_string());
            }
        }
    }

    sort_versions_desc(&mut versions);

    Ok(versions)
}
