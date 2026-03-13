use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::io;
use std::path::PathBuf;
use tar::Archive;
use zip::ZipArchive;

#[allow(dead_code)]
pub fn unzip_file(zip_path: &PathBuf, dest_path: &PathBuf) -> Result<(), String> {
    let file = File::open(zip_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    fs::create_dir_all(dest_path).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = dest_path.join(file.mangled_name());
        // let outpath = Path::new(dest_path).join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| {
                eprintln!("21==error: {:?}", e);
                e.to_string()
            })?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).map_err(|e| {
                        eprintln!("25==error: {:?}", e);
                        e.to_string()
                    })?;
                }
            }
            let mut outfile = File::create(&outpath).map_err(|e| {
                eprintln!("28==error: {:?}", e);
                e.to_string()
            })?;
            io::copy(&mut file, &mut outfile).map_err(|e| {
                eprintln!("29==error: {:?}", e);
                e.to_string()
            })?;
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn untar_file(tar_path: &PathBuf, dest_path: &PathBuf) -> Result<(), String> {
    let file = File::open(tar_path).map_err(|e| e.to_string())?;
    let gz_decoder = GzDecoder::new(file);
    let mut archive = Archive::new(gz_decoder);

    fs::create_dir_all(dest_path).map_err(|e| e.to_string())?;

    archive
        .unpack(dest_path)
        .map_err(|e| format!("Failed to extract tar.gz: {}", e))?;

    Ok(())
}
