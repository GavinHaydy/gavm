use crate::enums::path::EPath;
use crate::path::get::get_base_path;

use serde_json::json;
use std::{fs, path::PathBuf};

// init settings.json
pub fn ensure_settings() -> Result<PathBuf, String> {
    let base_dir = get_base_path();
    let settings_path = EPath::Settings.path();
    if !settings_path.exists() {
        let config = json!({
            "autoActivate": true,
            "downloadPath": base_dir.join(EPath::Download).to_string_lossy(),
            "versionsPath": base_dir.join(EPath::Version).to_string_lossy(),
            "proxy": false,
        });

        let content = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;

        fs::write(&settings_path, content).map_err(|e| e.to_string())?;
    }
    Ok(settings_path)
}
