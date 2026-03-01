import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { LazyStore } from '@tauri-apps/plugin-store';
import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';

const store = new LazyStore('.settings.dat');

export interface DownloadTask {
  version: string;
  percentage: number;
  status: 'downloading' | 'success' | 'error';
}

export const useDownload = () => {
  const [tasks, setTasks] = useState<Record<string, DownloadTask>>({});
  const { t } = useTranslation();
  useEffect(() => {
    const unlistenPromise = listen<{ version: string; percentage: number }>(
      'download-progress',
      event => {
        const { version, percentage } = event.payload;
        setTasks(prev => ({
          ...prev,
          [version]: {
            version,
            percentage: Math.floor(percentage),
            status: percentage >= 100 ? 'success' : 'downloading',
          },
        }));
      },
    );

    return () => {
      unlistenPromise.then(f => f());
    };
  }, []);

  const startDownload = async (language: string, version: string) => {
    // 1. 获取记住的路径，没有则用默认
    const savePath = (await store.get<string>('download_path')) || 'D:\\lvm\\download';

    try {
      // 2. 调用后端
      await invoke('download_version', { language, version, savePath });
    } catch (err) {
      console.error(t('downloader.failed'), err);
    }
  };

  return { tasks: Object.values(tasks), startDownload };
};
