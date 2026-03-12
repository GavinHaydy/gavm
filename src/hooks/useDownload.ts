/* eslint-disable @typescript-eslint/no-floating-promises */
import { listen } from '@tauri-apps/api/event';
import { useState, useEffect } from 'react';

import { DownloadStatusEnum } from '@/core/constants/enum';

export interface DownloadTask {
  language: string;
  version: string;
  percentage: number;
  status: DownloadStatusEnum;
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
          status: DownloadStatusEnum.DOWNLOADING,
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
          status: DownloadStatusEnum.SUCCESS,
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
          status: DownloadStatusEnum.ERROR,
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
