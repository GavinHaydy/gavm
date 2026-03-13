// python.rs
// Python installer implementation

use crate::core::caches::go_cache::fetch_versions_go;
use crate::core::common::error::io_err;
use crate::core::installers::extract::{untar_file, unzip_file};
use crate::core::language::LanguageInstaller;
use crate::core::utils::config::{
    del_language, get_config_bool, get_dirs, get_language_current_path, versions_list,
};
use async_trait::async_trait;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use tauri::Wry;

pub struct GoInstaller;

impl GoInstaller {
    pub fn new() -> Self {
        Self
    }

    // fn get_base_url(&self) -> &'static str {
    //     "https://go.dev/dl/"
    // }

    fn get_platform(&self) -> String {
        #[cfg(target_os = "windows")]
        {
            "windows".to_string()
        }
        #[cfg(target_os = "macos")]
        {
            "macos".to_string()
        }
        #[cfg(target_os = "linux")]
        {
            "linux".to_string()
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            "unknown".to_string()
        }
    }

    fn get_arch(&self) -> String {
        #[cfg(target_arch = "x86_64")]
        {
            "x86_64".to_string()
        }
        #[cfg(target_arch = "aarch64")]
        {
            "arm64".to_string()
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            "unknown".to_string()
        }
    }

    fn get_base_dir(&self) -> PathBuf {
        shim::get_base_path().join("go")
    }
}
#[async_trait]
impl LanguageInstaller for GoInstaller {
    async fn list_versions(&self) -> Result<Vec<String>, String> {
        let versions = versions_list("go", fetch_versions_go).await?;

        Ok(versions)
    }
    async fn list_installed(&self) -> Result<Vec<String>, String> {
        let dir = self.get_base_dir();

        if !dir.exists() {
            return Ok(vec![]);
        }

        get_dirs(&dir).map_err(|e| e.to_string())
    }
    async fn current(&self) -> Result<Option<String>, String> {
        let path = self.get_base_dir().join("current");

        match std::fs::read_to_string(path) {
            Ok(v) => Ok(Some(v.trim().to_string())),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn install(
        &self,
        window: tauri::Window<Wry>,
        version: &str,
        base_dir: &str,
        save_path: &str,
    ) -> Result<(), String> {
        let url = self.get_download_url(version)?;

        let extension = if url.ends_with(".zip") {
            "zip"
        } else if url.ends_with(".tar.gz") {
            "tar.gz"
        } else {
            return Err("Unsupported file format".into());
        };

        let dest_path = PathBuf::from(save_path).join(format!("go-{}.{}", version, extension));

        match crate::core::installers::downloader::Downloader::download_with_progress(
            "Go",
            window,
            version,
            &url,
            dest_path.clone(),
        )
        .await
        {
            Ok(v) => v,
            Err(e) => {
                let _ = self.uninstall(version).await;
                return Err(e);
            }
        };

        // 5. 根据文件格式选择解压方式
        let extract_path = PathBuf::from(base_dir).join("go").join(version);
        println!("extract_path {:?}", extract_path);

        match extension {
            "zip" => {
                unzip_file(&dest_path, &extract_path).expect("TODO: unzip Error");
            }
            "tar.gz" => {
                untar_file(&dest_path, &extract_path).expect("TODO: untar Error");
            }
            _ => {
                return Err("Unsupported file format".into());
            }
        }

        // 6. 设置当前版本
        let current = PathBuf::from(base_dir).join("go").join("current");
        let auto_activite = get_config_bool("autoActivate", false);

        if !current.exists() || auto_activite {
            let _ = fs::write(current, version).map_err(io_err);
        }

        Ok(())
    }

    async fn activate(&self, version: &str) -> Result<(), String> {
        let current_file = self.get_base_dir().join("current");

        fs::write(current_file, version).map_err(|e| e.to_string())?;

        Ok(())
    }
    async fn deactivate(&self, version: &str) -> Result<(), String> {
        let current_version = get_language_current_path("go").unwrap_or_default();

        let current_file = self.get_base_dir().join("current");

        if current_version != version {
            return Err(format!("The currently active version is not {}", version));
        }

        fs::write(current_file, "").map_err(|e| e.to_string())?;

        Ok(())
    }
    async fn uninstall(&self, version: &str) -> Result<(), String> {
        del_language("go", version)?;

        Ok(())
    }
    fn get_download_url(&self, version: &str) -> Result<String, String> {
        let platform = self.get_platform();
        let arch = self.get_arch();

        let go_arch = match arch.as_str() {
            "x86_64" => "amd64",
            "arm64" => "arm64",
            _ => return Err(format!("Unsupported architecture: {}", arch)),
        };

        let go_platform = match platform.as_str() {
            "windows" => "windows",
            "macos" => "darwin",
            "linux" => "linux",
            _ => return Err(format!("Unsupported platform: {}", platform)),
        };

        // 确定文件扩展名
        let extension = match go_platform {
            "windows" => "zip",
            _ => "tar.gz",
        };

        // 构建下载 URL
        let url = format!(
            "https://go.dev/dl/go{}.{}-{}.{}",
            version, go_platform, go_arch, extension
        );

        Ok(url)
    }
}
