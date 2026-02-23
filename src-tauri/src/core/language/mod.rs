// language/mod.rs
// Trait definition for all language installers

use async_trait::async_trait;

pub mod python;

#[async_trait]
pub trait LanguageInstaller {
    async fn list_versions(&self) -> Result<Vec<String>, String>;
    async fn list_installed(&self) -> Result<Vec<String>, String>;
    async fn current(&self) -> Result<Option<String>, String>;
}