use std::fmt;

pub enum EDownload {
    Python,
    PythonProxy,
    Go,
    GoListProxy,
    GoDownLoadProxy,
    Node,
    NodeProxy,
    NodeDownloadProxy,
}

impl fmt::Display for EDownload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            EDownload::Python => "https://www.python.org/ftp/python/",
            EDownload::PythonProxy => "https://mirrors.huaweicloud.com/python/",
            EDownload::Go => "https://go.dev/dl/",
            EDownload::GoListProxy => "https://golang.google.cn/dl/",
            EDownload::GoDownLoadProxy => "https://dl.google.com/go/",
            EDownload::Node => "https://nodejs.org/dist/",
            EDownload::NodeProxy => "https://node.org.cn/dist/",
            EDownload::NodeDownloadProxy => "https://mirrors.huaweicloud.com/nodejs/",
        };
        write!(f, "{}", s)
    }
}
