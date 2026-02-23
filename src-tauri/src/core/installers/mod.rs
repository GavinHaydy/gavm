use regex::Regex;
use reqwest;
use async_trait::async_trait;

use crate::core::utils::semver::sort_versions_desc;
use crate::core::language::LanguageInstaller;

pub struct PythonInstaller;

impl PythonInstaller {
    pub fn new() -> Self {
        Self
    }

    fn versions_dir() -> std::path::PathBuf {
        std::env::current_dir()
            .unwrap()
            .join("pyvm")
            .join("versions")
    }

    fn current_path() -> std::path::PathBuf {
        std::env::current_dir()
            .unwrap()
            .join("pyvm")
            .join("current_version")
    }
}

#[async_trait]
impl LanguageInstaller for PythonInstaller {

    async fn list_versions(&self) -> Result<Vec<String>, String> {
        let url = "https://www.python.org/ftp/python/";

        let html = reqwest::get(url)
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;

        let re = Regex::new(r#"href="(\d+\.\d+\.\d+)/""#)
            .map_err(|e| e.to_string())?;

        let mut versions = Vec::new();

        for cap in re.captures_iter(&html) {
            versions.push(cap[1].to_string());
        }

        sort_versions_desc(&mut versions);

        Ok(versions)
    }

    async fn list_installed(&self) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let dir = Self::versions_dir();

        if dir.exists() {
            for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                if entry.path().is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        result.push(name.to_string());
                    }
                }
            }
        }

        Ok(result)
    }

    async fn current(&self) -> Result<Option<String>, String> {
        let path = Self::current_path();

        if path.exists() {
            let v = std::fs::read_to_string(path)
                .map_err(|e| e.to_string())?;
            return Ok(Some(v));
        }

        Ok(None)
    }
}