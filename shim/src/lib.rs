pub mod path_inject;

use lvm_core::enums::path::EPath;
#[cfg(target_os = "windows")]
use path_inject::inject_path;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use path_inject::inject_path_unix;
use std::fs;

/// 安装 shim（复制 + PATH 注入）
pub fn install_shims() -> Result<(), Box<dyn std::error::Error>> {
    let shim_dir = EPath::Shims.path();

    fs::create_dir_all(&shim_dir)?;
    println!("install_shims: shim_dir = {:?}", shim_dir);

    // 自动判断 debug / release
    let workspace_root = std::env::current_dir()?.parent().unwrap().to_path_buf();

    #[allow(unused_mut)]
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
