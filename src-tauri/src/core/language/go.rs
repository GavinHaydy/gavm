use crate::core::caches::go_cache::fetch_versions_go;
use crate::core::common::error::io_err;
use crate::core::enums::proxy::EDownload;
use crate::core::installers::extract::{untar_file, unzip_file};
use crate::core::language::LanguageInstaller;
use crate::core::utils::config::versions_list;
use async_trait::async_trait;
use lvm_core::config::get::get_config_bool;
use lvm_core::enums::path::EPath;
use lvm_core::path::get::current_path;
use std::fs;
use std::path::PathBuf;
use tauri::Wry;

pub struct GoInstaller;

impl GoInstaller {
    pub fn new() -> Self {
        Self
    }

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
}
#[async_trait]
impl LanguageInstaller for GoInstaller {
    fn name(&self) -> &str {
        "go"
    }

    async fn list_versions(&self) -> Result<Vec<String>, String> {
        versions_list("go", fetch_versions_go).await
    }

    async fn install(
        &self,
        window: tauri::Window<Wry>,
        version: &str,
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
        let extract_path = EPath::Version.path().join(self.name()).join(version);

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
        let current = current_path(self.name());
        let auto_activate = get_config_bool("autoActivate", false);

        if !current.exists() || auto_activate {
            let _ = fs::write(current, version).map_err(io_err);
        }

        Ok(())
    }

    fn get_download_url(&self, version: &str) -> Result<String, String> {
        let proxy = get_config_bool("proxy", false);
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
            "{domain}go{v}.{platform}-{arch}.{e}",
            domain = if !proxy {
                EDownload::Go
            } else {
                EDownload::GoDownLoadProxy
            },
            v = version,
            platform = go_platform,
            arch = go_arch,
            e = extension,
        );

        Ok(url)
    }
}
