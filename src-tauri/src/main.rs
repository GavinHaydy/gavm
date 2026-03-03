// main.rs
// Entry point of Tauri application

use std::path::PathBuf;

pub fn install_shim() -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs::home_dir().unwrap();
    let shim_dir = home.join(".lvm").join("shims");

    std::fs::create_dir_all(&shim_dir)?;

    let shim_binary = PathBuf::from("target/release/shim.exe");

    let languages = ["python", "node", "go", "rustc"];

    for lang in languages {
        let target = if cfg!(windows) {
            shim_dir.join(format!("{}.exe", lang))
        } else {
            shim_dir.join(lang)
        };

        std::fs::copy(&shim_binary, target)?;
    }

    Ok(())
}
fn main() {
    lvm_lib::run()
}
