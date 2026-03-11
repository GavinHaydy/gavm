use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::SeekFrom;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{Emitter, Window, Wry};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Mutex;

#[derive(Clone, Serialize)]
pub struct ProgressPayload {
    pub language: String,
    pub version: String,
    pub current: u64,
    pub total: u64,
    pub percentage: f64,
}

#[derive(Clone, Serialize)]
pub struct ErrorPayload {
    pub language: String,
    pub version: String,
    pub message: String,
}

#[derive(Clone, Serialize)]
pub struct CompletePayload {
    pub language: String,
    pub version: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct ChunkMeta {
    start: u64,
    end: u64,
    finished: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct DownloadMeta {
    chunks: Vec<ChunkMeta>,
}

struct DownloadState {
    downloaded: u64,
}

pub struct Downloader;

impl Downloader {
    pub async fn download_with_progress(
        language: &str,
        window: Window<Wry>,
        version: &str,
        url: &str,
        dest_path: PathBuf,
    ) -> Result<String, String> {
        let client = Client::builder()
            .pool_max_idle_per_host(16)
            .build()
            .map_err(|e| e.to_string())?;

        let head = client.head(url).send().await.map_err(|e| e.to_string())?;

        let total_size = head
            .headers()
            .get("content-length")
            .ok_or("无法获取文件大小")?
            .to_str()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let threads = if total_size > 30_000_000 {
            16
        } else if total_size > 10_000_000 {
            8
        } else if total_size > 2_000_000 {
            4
        } else {
            1
        };

        let chunk_size = total_size / threads;

        let meta_path = dest_path.with_extension("meta");

        let meta: DownloadMeta = if meta_path.exists() {
            let data = tokio::fs::read(&meta_path)
                .await
                .map_err(|e| e.to_string())?;
            serde_json::from_slice(&data).unwrap()
        } else {
            let mut chunks = vec![];

            for i in 0..threads {
                let start = i * chunk_size;

                let end = if i == threads - 1 {
                    total_size - 1
                } else {
                    (i + 1) * chunk_size - 1
                };

                chunks.push(ChunkMeta {
                    start,
                    end,
                    finished: false,
                });
            }

            DownloadMeta { chunks }
        };

        let file = if dest_path.exists() {
            OpenOptions::new()
                .write(true)
                .open(&dest_path)
                .await
                .map_err(|e| e.to_string())?
        } else {
            let f = File::create(&dest_path).await.map_err(|e| e.to_string())?;

            f.set_len(total_size).await.map_err(|e| e.to_string())?;

            f
        };

        drop(file);

        let state = Arc::new(Mutex::new(DownloadState { downloaded: 0 }));

        let error_sent = Arc::new(AtomicBool::new(false));

        let meta_mutex = Arc::new(Mutex::new(meta.clone()));

        let mut tasks = vec![];

        for chunk in meta.chunks.clone() {
            if chunk.finished {
                continue;
            }

            let client = client.clone();
            let url = url.to_string();
            let file_path = dest_path.clone();
            let state = state.clone();
            let window = window.clone();
            let version = version.to_string();
            let error_sent = error_sent.clone();
            let meta_mutex = meta_mutex.clone();
            let meta_path = meta_path.clone();
            let language = language.to_string();

            let task = tokio::spawn(async move {
                let mut retries = 3;

                loop {
                    let resp = client
                        .get(&url)
                        .header("Range", format!("bytes={}-{}", chunk.start, chunk.end))
                        .send()
                        .await;

                    let resp = match resp {
                        Ok(r) => r,
                        Err(e) => {
                            retries -= 1;
                            if retries == 0 {
                                if !error_sent.swap(true, Ordering::SeqCst) {
                                    window
                                        .emit(
                                            "download-error",
                                            ErrorPayload {
                                                language: language.clone(),
                                                version: version.clone(),
                                                message: e.to_string(),
                                            },
                                        )
                                        .ok();
                                }
                                return Err(e.to_string());
                            }
                            continue;
                        }
                    };

                    let mut stream = resp.bytes_stream();

                    let mut file = OpenOptions::new()
                        .write(true)
                        .open(&file_path)
                        .await
                        .map_err(|e| e.to_string())?;

                    file.seek(SeekFrom::Start(chunk.start))
                        .await
                        .map_err(|e| e.to_string())?;

                    while let Some(item) = stream.next().await {
                        let chunk_bytes = item.map_err(|e| e.to_string())?;

                        file.write_all(&chunk_bytes)
                            .await
                            .map_err(|e| e.to_string())?;

                        let mut s = state.lock().await;
                        s.downloaded += chunk_bytes.len() as u64;

                        let percent = (s.downloaded as f64 / total_size as f64) * 100.0;

                        window
                            .emit(
                                "download-progress",
                                ProgressPayload {
                                    language: language.clone(),
                                    version: version.clone(),
                                    current: s.downloaded,
                                    total: total_size,
                                    percentage: percent,
                                },
                            )
                            .ok();
                    }

                    {
                        let mut meta = meta_mutex.lock().await;

                        if let Some(c) = meta.chunks.iter_mut().find(|c| c.start == chunk.start) {
                            c.finished = true;
                        }

                        let data = serde_json::to_vec(&*meta).unwrap();

                        tokio::fs::write(&meta_path, data).await.ok();
                    }

                    break;
                }

                Ok::<(), String>(())
            });

            tasks.push(task);
        }

        for t in tasks {
            if let Err(e) = t.await.unwrap() {
                if !error_sent.swap(true, Ordering::SeqCst) {
                    window
                        .emit(
                            "download-error",
                            ErrorPayload {
                                language: language.to_string(),
                                version: version.to_string(),
                                message: e.clone(),
                            },
                        )
                        .ok();
                }
                return Err(e);
            }
        }

        tokio::fs::remove_file(meta_path).await.ok();

        window
            .emit(
                "download-complete",
                CompletePayload {
                    language: language.to_string(),
                    version: version.to_string(),
                    path: dest_path.to_string_lossy().to_string(),
                },
            )
            .ok();

        Ok(dest_path.to_string_lossy().to_string())
    }
}
