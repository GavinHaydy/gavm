use crate::core::caches::node_cache::fetch_versions_node;
use crate::core::common::error::io_err;
use crate::core::enums::proxy::EDownload;
use crate::core::installers::extract::{untar_file, unzip_file};
use crate::core::language::LanguageInstaller;
use crate::core::utils::config::{del_language, versions_list};
use async_trait::async_trait;
use lvm_core::config::get::{get_config_bool, get_language_current_version};
use lvm_core::enums::path::EPath;
use lvm_core::files::get::get_dirs;
use lvm_core::path::get::current_path;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use tauri::Wry;

pub struct NodeInstaller;

impl NodeInstaller {
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

    fn name(&self) -> &'static str {
        "node"
    }

    fn get_base_dir(&self) -> PathBuf {
        let path = EPath::Version.path().join(self.name());
        if !path.exists() {
            fs::create_dir_all(&path).expect("create dirs err");
        }
        path
    }
}
#[async_trait]
impl LanguageInstaller for NodeInstaller {
    async fn list_versions(&self) -> Result<Vec<String>, String> {
        let versions = versions_list("node", fetch_versions_node).await?;

        Ok(versions)
    }
    async fn list_installed(&self) -> Result<Vec<String>, String> {
        let dir = self.get_base_dir();
        get_dirs(&dir).map_err(|e| e.to_string())
    }
    async fn current(&self) -> Result<Option<String>, String> {
        let node_current_path = self.get_base_dir().join("current");

        match fs::read_to_string(node_current_path) {
            Ok(v) => Ok(Some(v.trim().to_string())),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.to_string()),
        }
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

        let dest_path = PathBuf::from(save_path).join(format!("node-{}.{}", version, extension));

        match crate::core::installers::downloader::Downloader::download_with_progress(
            "Node",
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

    async fn activate(&self, version: &str) -> Result<(), String> {
        let current_file = current_path(self.name());

        fs::write(current_file, version).map_err(|e| e.to_string())?;

        Ok(())
    }
    async fn deactivate(&self, version: &str) -> Result<(), String> {
        let current_version = get_language_current_version("node").unwrap_or_default();

        let current_file = current_path(self.name());

        if current_version != version {
            return Err(format!("The currently active version is not {}", version));
        }

        fs::write(current_file, "").map_err(|e| e.to_string())?;

        Ok(())
    }
    async fn uninstall(&self, version: &str) -> Result<(), String> {
        del_language("node", version)?;

        Ok(())
    }
    fn get_download_url(&self, version: &str) -> Result<String, String> {
        // let proxy = get_config_bool("proxy", false);
        let platform = self.get_platform();
        let arch = self.get_arch();

        let node_arch = match arch.as_str() {
            "x86_64" => "x64",
            "arm64" => "arm64",
            _ => return Err(format!("Unsupported architecture: {}", arch)),
        };

        let node_platform = match platform.as_str() {
            "windows" => "win",
            "macos" => "darwin",
            "linux" => "linux",
            _ => return Err(format!("Unsupported platform: {}", platform)),
        };

        // 确定文件扩展名
        let extension = match node_platform {
            "windows" => "zip",
            _ => "tar.gz",
        };

        // 构建下载 URL
        let url = format!(
            "{domain}dist/v{v}/node-v{v}-{platform}-{arch}.{e}",
            domain = EDownload::Node,
            v = version,
            platform = node_platform,
            arch = node_arch,
            e = extension,
        );

        Ok(url)
    }
}
