#[cfg(target_os = "windows")]
pub fn inject_path(shim_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    use std::ffi::CString;
    use winapi::um::winuser::{
        HWND_BROADCAST, SMTO_ABORTIFHUNG, SendMessageTimeoutA, WM_SETTINGCHANGE,
    };
    use winreg::RegKey;
    use winreg::enums::*;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;
    let current_path: String = env.get_value("Path").unwrap_or_default();

    let shim_str = shim_dir.to_str().unwrap();
    if !current_path
        .to_lowercase()
        .contains(&shim_str.to_lowercase())
    {
        let new_path = format!("{};{}", shim_str, current_path);
        env.set_value("Path", &new_path)?;
        println!("Injected shim path into user PATH: {}", shim_str);

        let param = CString::new("Environment").unwrap();
        unsafe {
            SendMessageTimeoutA(
                HWND_BROADCAST,
                WM_SETTINGCHANGE,
                0,
                param.as_ptr() as isize,
                SMTO_ABORTIFHUNG,
                5000,
                std::ptr::null_mut(),
            );
        }
    }

    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn inject_path_unix(shim_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let home = dirs::home_dir().unwrap();
    let shell_rc = home.join(".bashrc"); // 或 .zshrc 根据实际 shell
    let shim_str = shim_dir.to_str().unwrap();

    let content = std::fs::read_to_string(&shell_rc).unwrap_or_default();
    if !content.contains(shim_str) {
        let mut file = OpenOptions::new().append(true).open(&shell_rc)?;
        writeln!(file, "\nexport PATH=\"{}:$PATH\"", shim_str)?;
        println!("Injected shim path into shell rc: {}", shim_str);
    }

    Ok(())
}
