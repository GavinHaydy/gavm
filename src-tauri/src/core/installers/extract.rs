use flate2::read::GzDecoder;
use std::{
    collections::HashSet,
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
};
use tar::Archive;
use zip::ZipArchive;

#[allow(dead_code)]
pub fn unzip_file(zip_path: &Path, dest_path: &Path) -> Result<(), String> {
    // 临时目录
    let tmp_dir = dest_path.with_extension("tmp");
    fs::create_dir_all(&tmp_dir).map_err(|e| e.to_string())?;

    let file = File::open(zip_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader).map_err(|e| e.to_string())?;

    let mut created_dirs = HashSet::new();
    let mut buffer = [0u8; 128 * 1024]; // 128KB buffer

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = tmp_dir.join(file.enclosed_name().unwrap());

        if file.name().ends_with('/') {
            if created_dirs.insert(outpath.clone()) {
                fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
            }
        } else {
            if let Some(parent) = outpath.parent() {
                if created_dirs.insert(parent.to_path_buf()) {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
            }

            let mut outfile = File::create(&outpath).map_err(|e| e.to_string())?;
            loop {
                let n = file.read(&mut buffer).map_err(|e| e.to_string())?;
                if n == 0 {
                    break;
                }
                outfile.write_all(&buffer[..n]).map_err(|e| e.to_string())?;
            }
        }
    }

    // 最后一步：一次性 rename 临时目录到目标目录
    if dest_path.exists() {
        fs::remove_dir_all(dest_path).map_err(|e| e.to_string())?;
    }
    fs::rename(&tmp_dir, dest_path).map_err(|e| e.to_string())?;

    Ok(())
}

#[allow(dead_code)]
pub fn untar_file(tar_path: &PathBuf, dest_path: &PathBuf) -> Result<(), String> {
    let file = File::open(tar_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let gz = GzDecoder::new(reader);
    let mut archive = Archive::new(gz);

    fs::create_dir_all(dest_path).map_err(|e| e.to_string())?;

    archive.unpack(dest_path).map_err(|e| e.to_string())?;

    Ok(())
}
