import { Button, Card, Input, message } from 'antd';
import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';

import { safeInvoke } from '@/api/tauri.ts';
import { CommandEnum } from '@/core/constants/enum.ts';

export const Settings = () => {
  const [basePath, setBasePath] = useState('');
  const [downloadPath, setDownloadPath] = useState('');
  const [versionsPath, setVersionsPath] = useState('');
  const { t } = useTranslation();

  useEffect(() => {
    const init = async () => {
      try {
        const defaultPath = await safeInvoke<string>(CommandEnum.BASE_PATH);
        const downPath = await safeInvoke<string>(CommandEnum.GET_CONFIG_VALUE, {
          key: CommandEnum.DOWNLOAD_PATH,
        });
        const versionPath = await safeInvoke<string>(CommandEnum.GET_CONFIG_VALUE, {
          key: CommandEnum.VERSIONS_PATH,
        });
        const testV = await safeInvoke<string>(CommandEnum.GET_CONFIG_VALUES, {
          keys: [CommandEnum.VERSIONS_PATH, CommandEnum.DOWNLOAD_PATH, CommandEnum.AUTO_ACTIVATE],
        });
        console.log(testV);
        
        setDownloadPath(downPath);
        setVersionsPath(versionPath);
        setBasePath(defaultPath);
      } catch (e) {
        console.error(e);
      }
    };
    void init();
  }, []);

  const handleSave = async () => {
    // await store.set('base_path', basePath);
    // await store.save(); // 持久化到硬盘
    message.success(t('settings.success'));
  };

  return (
    <Card title={t('settings.title')}>
      {basePath && (
        <Input
          value={basePath}
          onChange={e => setBasePath(e.target.value)}
          placeholder={t('settings.placeholder', { path: basePath })}
        />
      )}
      {downloadPath && (
        <Input
          value={downloadPath}
          onChange={e => setDownloadPath(e.target.value)}
          placeholder={t('settings.placeholder', { path: downloadPath })}
        />
      )}
      {versionsPath && (
        <Input
          value={versionsPath}
          onChange={e => setVersionsPath(e.target.value)}
          placeholder={t('settings.placeholder', { path: versionsPath })}
        />
      )}
      <Button onClick={handleSave}>{t('settings.save')}</Button>
    </Card>
  );
};
