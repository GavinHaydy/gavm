// language/mod.rs
// Trait definition for all language installers

use async_trait::async_trait;
use tauri::Wry;

pub mod python;

#[async_trait]
pub trait LanguageInstaller {
    async fn list_versions(&self) -> Result<Vec<String>, String>;
    async fn list_installed(&self) -> Result<Vec<String>, String>;
    async fn current(&self) -> Result<Option<String>, String>;
    async fn install(
        &self,
        window: tauri::Window<Wry>,
        version: &str,
        save_path: &str,
    ) -> Result<(), String>;
    fn get_download_url(&self, version: &str) -> Result<String, String>;
    #[allow(dead_code)]
    async fn download(&self, version: &str) -> Result<String, String>;
}
