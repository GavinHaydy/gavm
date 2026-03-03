use std::env;
use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let exe_name = exe_path.file_stem().unwrap().to_string_lossy().into_owned();

    let base_dir = shim::get_base_path();
    let current_file = base_dir.join(&exe_name).join("current");
    let version = fs::read_to_string(current_file)?.trim().to_string();

    let mut real_exe = base_dir.join(&exe_name).join(&version).join(&exe_name);
    #[cfg(target_os = "windows")]
    real_exe.set_extension("exe");

    let args: Vec<String> = env::args().skip(1).collect();
    let status = Command::new(real_exe).args(args).status()?;

    std::process::exit(status.code().unwrap_or(1));
}
