pub mod path_inject;

use std::fs;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use path_inject::inject_path;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use path_inject::inject_path_unix;

/// 获取 base_path，支持用户自定义
pub fn get_base_path() -> PathBuf {
    let default_path = dirs::home_dir().unwrap().join(".lvm");
    let config_path = default_path.join("config.json");

    if config_path.exists()
        && let Ok(s) = fs::read_to_string(&config_path)
        && let Ok(json) = serde_json::from_str::<serde_json::Value>(&s)
        && let Some(base) = json.get("base_path").and_then(|v| v.as_str())
    {
        return PathBuf::from(base);
    }

    default_path
}

/// 安装 shim（复制 + PATH 注入）
pub fn install_shims() -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = get_base_path();
    let shim_dir = base_dir.join("shims");

    fs::create_dir_all(&shim_dir)?;
    println!("install_shims: shim_dir = {:?}", shim_dir);

    // 自动判断 debug / release
    let workspace_root = std::env::current_dir()?.parent().unwrap().to_path_buf();

    let mut binary = workspace_root.join(if cfg!(debug_assertions) {
        "target/debug/shim"
    } else {
        "target/release/shim"
    });

    #[cfg(target_os = "windows")]
    {
        binary.set_extension("exe");
    }

    if !binary.exists() {
        panic!("shim binary not found: {:?}", binary);
    }

    println!("install_shims: copying from {:?}", binary);

    let languages = ["python", "go"];

    for lang in &languages {
        let target = if cfg!(windows) {
            shim_dir.join(format!("{}.exe", lang))
        } else {
            shim_dir.join(lang)
        };
        fs::copy(&binary, &target)?;
        println!("Copied shim for {} to {:?}", lang, target);
    }

    // 注入 PATH
    #[cfg(target_os = "windows")]
    inject_path(&shim_dir)?;
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    inject_path_unix(&shim_dir)?;

    Ok(())
}
