// python.rs
// Python installer implementation

use crate::caches::python_cache::*;
use crate::core::common::error::io_err;
use crate::core::enums::proxy::EDownload;
use crate::core::installers::extract::unzip_file;
use crate::core::language::LanguageInstaller;
use crate::core::utils::config::{
    del_language, get_config_bool, get_dirs, get_language_current_path, versions_list,
};
use async_trait::async_trait;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use tauri::Wry;

pub struct PythonInstaller;

impl PythonInstaller {
    pub fn new() -> Self {
        Self
    }

    // Get current platform identifier
    #[allow(dead_code)]
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

    // Get architecture identifier
    #[allow(dead_code)]
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

    // Get base directory
    fn get_base_dir(&self) -> PathBuf {
        shim::get_base_path().join("python")
    }
}

#[async_trait]
impl LanguageInstaller for PythonInstaller {
    async fn list_versions(&self) -> Result<Vec<String>, String> {
        let versions = versions_list("python", fetch_versions_python).await?;

        Ok(versions)
    }

    async fn list_installed(&self) -> Result<Vec<String>, String> {
        let dir = self.get_base_dir();
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
        // 1. 获取 URL
        let url = self.get_download_url(version)?;
        println!("url {}", url);

        // 2. 确定本地路径
        let dest_path = PathBuf::from(save_path).join(format!("python-{}.zip", version));

        // 3. 调用通用下载器（流式下载 + 进度回传）
        match crate::core::installers::downloader::Downloader::download_with_progress(
            "Python",
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

        // 4. 下载完成后，继续执行解压逻辑...
        // self.extract(&dest_path, ...).await?;
        let extract_path = PathBuf::from(base_dir).join("python").join(version);
        println!("extract_path {:?}", extract_path);
        unzip_file(&dest_path, &extract_path).expect("TODO: unzip Error");

        // 创建或修改current 根据配置来
        let current = PathBuf::from(base_dir).join("python").join("current");
        let auto_activite = get_config_bool("autoActivate", false);

        // 不存在或开启自动切换
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
        let current_version = get_language_current_path("python").unwrap_or_default();

        let current_file = self.get_base_dir().join("current");

        if current_version != version {
            return Err(format!("The currently active version is not {}", version));
        }

        fs::write(current_file, "").map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn uninstall(&self, version: &str) -> Result<(), String> {
        del_language("python", version)?;

        Ok(())
    }

    fn get_download_url(&self, version: &str) -> Result<String, String> {
        let platform = self.get_platform();
        let arch = self.get_arch();
        let proxy = get_config_bool("proxy", false);

        let domain = if proxy {
            EDownload::PythonProxy
        } else {
            EDownload::Python
        };

        let url = match platform.as_str() {
            "windows" => {
                let arch_suffix = if arch == "x86_64" { "amd64" } else { "win32" };
                format!(
                    "{d}{v}/python-{v}-embed-{arch}.zip",
                    d = domain,
                    v = version,
                    arch = arch_suffix
                )
            }
            "macos" => format!("{d}{v}/python-{v}-macosx11.0.pkg", d = domain, v = version),
            "linux" => format!("{d}{v}/Python-{v}.tgz", d = domain, v = version),
            _ => return Err("Unsupported platform".into()),
        };
        Ok(url)
    }
}
