use crate::path::get::{get_base_path, get_config_path};
use std::fmt;
use std::path::{Path, PathBuf};

pub enum EPath {
    CACHE,
    Download,
    Version,
    Settings,
    Shims,
}

impl EPath {
    pub fn as_str(&self) -> &'static str {
        match self {
            EPath::CACHE => "cache",
            EPath::Download => "download",
            EPath::Version => "versions",
            EPath::Settings => "settings.json",
            EPath::Shims => "shims",
        }
    }

    pub fn is_configurable(&self) -> bool {
        matches!(self, EPath::Download | EPath::Version)
    }

    pub fn config_key(&self) -> Option<&'static str> {
        match self {
            EPath::Download => Some("downloadPath"),
            EPath::Version => Some("versionsPath"),
            _ => None,
        }
    }

    pub fn path(&self) -> PathBuf {
        if self.is_configurable() {
            return get_config_path(self.config_key().unwrap());
        }

        // 固定路径
        get_base_path().join(self)
    }
}

impl fmt::Display for EPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl AsRef<Path> for EPath {
    fn as_ref(&self) -> &Path {
        Path::new(self.as_str())
    }
}
