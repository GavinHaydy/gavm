use lvm_core::enums::path::EPath;
use lvm_core::path::get::current_path;
use std::env;
use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let exe_name = exe_path.file_stem().unwrap().to_string_lossy().into_owned();

    let current_file = current_path(&exe_name);
    let version = fs::read_to_string(current_file)?.trim().to_string();

    // let mut real_exe = EPath::Version.path().join(&exe_name).join(&version).join(&exe_name);
    let real_exe = if exe_name.starts_with("go") {
        EPath::Version
            .path()
            .join(&exe_name)
            .join(&version)
            .join(&exe_name)
            .join("bin")
            .join(&exe_name)
    } else {
        EPath::Version
            .path()
            .join(&exe_name)
            .join(&version)
            .join(&exe_name)
    };
    #[cfg(target_os = "windows")]
    real_exe.set_extension("exe");

    let args: Vec<String> = env::args().skip(1).collect();
    let status = Command::new(real_exe).args(args).status()?;

    std::process::exit(status.code().unwrap_or(1));
}
