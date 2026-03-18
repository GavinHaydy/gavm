use std::fs;
use std::path::Path;

pub fn get_dirs(path: &Path) -> Result<Vec<String>, std::io::Error> {
    let dirs = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_dir() {
                entry.file_name().into_string().ok()
            } else {
                None
            }
        })
        .collect();

    Ok(dirs)
}
