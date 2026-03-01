use futures_util::StreamExt;
use serde::Serialize;
use std::path::PathBuf;
use tauri::Window;
use tauri::{Emitter, Wry};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Clone, Serialize)]
pub struct ProgressPayload {
    pub version: String,
    pub current: u64,
    pub total: u64,
    pub percentage: f64,
}

pub struct Downloader;

impl Downloader {
    pub async fn download_with_progress(
        window: Window<Wry>,
        version: &str,
        url: &str,
        dest_path: PathBuf,
    ) -> Result<String, String> {
        let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
        let total_size = response.content_length().ok_or("无法获取文件大小")?;

        let mut file = File::create(&dest_path).await.map_err(|e| e.to_string())?;
        let mut stream = response.bytes_stream();
        let mut downloaded: u64 = 0;
        let mut last_emit_percent = -1.0;

        while let Some(item) = stream.next().await {
            let chunk = item.map_err(|e| e.to_string())?;
            file.write_all(&chunk).await.map_err(|e| e.to_string())?;
            downloaded += chunk.len() as u64;

            let percentage = (downloaded as f64 / total_size as f64) * 100.0;

            // 优化：进度每变化 1% 才发送一次事件，减轻前端渲染压力
            if (percentage - last_emit_percent).abs() >= 1.0 || percentage >= 100.0 {
                window
                    .emit(
                        "download-progress",
                        ProgressPayload {
                            version: version.to_string(),
                            current: downloaded,
                            total: total_size,
                            percentage,
                        },
                    )
                    .unwrap();
                last_emit_percent = percentage;
            }
        }
        Ok(dest_path.to_string_lossy().to_string())
    }
}
