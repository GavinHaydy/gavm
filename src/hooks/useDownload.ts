import { listen } from '@tauri-apps/api/event';
import { useState, useEffect } from 'react';

export interface DownloadTask {
  language: string;
  version: string;
  percentage: number;
  status: 'downloading' | 'success' | 'error';
}

export const useDownload = () => {
  const [tasks, setTasks] = useState<Record<string, DownloadTask>>({});

  useEffect(() => {
    let lastUpdate = 0;

    // 监听下载进度
    const progressListener = listen<{
      language: string;
      version: string;
      current: number;
      total: number;
      percentage: number;
    }>('download-progress', event => {
      const now = Date.now();
      if (now - lastUpdate < 200) return;
      lastUpdate = now;

      const { language, version, percentage } = event.payload;

      setTasks(prev => ({
        ...prev,
        [version]: {
          language,
          version,
          percentage: Math.floor(percentage),
          status: 'downloading',
        },
      }));
    });

    // 监听下载完成
    const completeListener = listen<{
      language: string;
      version: string;
      path: string;
    }>('download-complete', event => {
      const { language, version } = event.payload;

      setTasks(prev => ({
        ...prev,
        [version]: {
          language,
          version,
          percentage: 100,
          status: 'success',
        },
      }));
    });

    // 监听下载失败
    const errorListener = listen<{
      language: string;
      version: string;
      message: string;
    }>('download-error', event => {
      const { language, version } = event.payload;

      setTasks(prev => ({
        ...prev,
        [version]: {
          language,
          version,
          percentage: prev[version]?.percentage || 0,
          status: 'error',
        },
      }));
    });

    return () => {
      progressListener.then(f => f());
      completeListener.then(f => f());
      errorListener.then(f => f());
    };
  }, []);

  return { tasks: Object.values(tasks) };
};
