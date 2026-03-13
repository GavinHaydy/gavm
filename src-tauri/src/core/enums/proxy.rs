use std::fmt;

pub enum EDownload {
    Python,
    PythonProxy,
}

impl fmt::Display for EDownload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            EDownload::Python => "https://www.python.org/ftp/python/",
            EDownload::PythonProxy => "https://mirrors.huaweicloud.com/python/",
        };
        write!(f, "{}", s)
    }
}
