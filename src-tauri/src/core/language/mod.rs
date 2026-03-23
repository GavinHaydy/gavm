use crate::core::utils::config::del_language;
use async_trait::async_trait;
use lvm_core::config::get::get_language_current_version;
use lvm_core::enums::path::EPath;
use lvm_core::files::get::get_dirs;
use lvm_core::path::get::current_path;
use std::fs;
use std::io::ErrorKind;
use tauri::Wry;

pub mod go;
pub mod node;
pub mod python;

#[async_trait]
pub trait LanguageInstaller {
    fn name(&self) -> &str;
    async fn list_versions(&self) -> Result<Vec<String>, String>;
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
    ) -> Result<(), String>;

    fn get_base_dir(&self) -> std::path::PathBuf {
        let path = EPath::Version.path().join(self.name());
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("create dirs err");
        }
        path
    }
    async fn activate(&self, version: &str) -> Result<(), String> {
        let current_file = current_path(self.name());
        std::fs::write(current_file, version).map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn deactivate(&self, version: &str) -> Result<(), String> {
        let current_version = get_language_current_version(self.name()).unwrap_or_default();

        let current_file = current_path(self.name());

        if current_version != version {
            return Err(format!("The currently active version is not {}", version));
        }

        fs::write(current_file, "").map_err(|e| e.to_string())?;

        Ok(())
    }
    async fn uninstall(&self, version: &str) -> Result<(), String> {
        del_language(self.name(), version)?;

        Ok(())
    }
    fn get_download_url(&self, version: &str) -> Result<String, String>;
}
