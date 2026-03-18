use std::fmt;

pub enum EDownload {
    Python,
    PythonProxy,
    Go,
    GoListProxy,
    GoDownLoadProxy,
}

impl fmt::Display for EDownload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            EDownload::Python => "https://www.python.org/ftp/python/",
            EDownload::PythonProxy => "https://mirrors.huaweicloud.com/python/",
            EDownload::Go => "https://go.dev/dl/",
            EDownload::GoListProxy => "https://golang.google.cn/dl/",
            EDownload::GoDownLoadProxy => "https://dl.google.com/go/",
        };
        write!(f, "{}", s)
    }
}
