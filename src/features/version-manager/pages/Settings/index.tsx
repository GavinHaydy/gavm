import { Button, Card, Form, Input, message } from 'antd';
import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';

import { safeInvoke } from '@/api/tauri.ts';
import { CommandEnum } from '@/core/constants/enum.ts';
import { keysToCamel } from '@/shared/utils/common';

export const Settings = () => {
  const [basePath, setBasePath] = useState('');
  const [downloadPath, setDownloadPath] = useState('');
  const [versionsPath, setVersionsPath] = useState('');
  const { t } = useTranslation();

  useEffect(() => {
    const init = async () => {
      try {
        const defaultPath = await safeInvoke<string>(CommandEnum.BASE_PATH);
        const configData = await safeInvoke<Record<string, boolean | string>>(
          CommandEnum.GET_CONFIG_VALUES,
          {
            keys: [CommandEnum.VERSIONS_PATH, CommandEnum.DOWNLOAD_PATH, CommandEnum.AUTO_ACTIVATE],
          },
        );

        const { versionsPath, downloadPath } = keysToCamel(configData);

        setDownloadPath(downloadPath as string);
        setVersionsPath(versionsPath as string);
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
      <Form layout="vertical" onFinish={handleSave}>
        {basePath && (
          <Form.Item label={t('settings.base_path')}>
            <Input
              value={basePath}
              onChange={e => setBasePath(e.target.value)}
              placeholder={t('settings.placeholder', { path: basePath })}
            />
          </Form.Item>
        )}
        {downloadPath && (
          <Form.Item label={t('settings.download_path')}>
            <Input
              value={downloadPath}
              onChange={e => setDownloadPath(e.target.value)}
              placeholder={t('settings.placeholder', { path: downloadPath })}
            />
          </Form.Item>
        )}
        {versionsPath && (
          <Form.Item label={t('settings.versions_path')}>
            <Input
              value={versionsPath}
              onChange={e => setVersionsPath(e.target.value)}
              placeholder={t('settings.placeholder', { path: versionsPath })}
            />
          </Form.Item>
        )}
        <Button onClick={handleSave}>{t('settings.save')}</Button>
      </Form>
    </Card>
  );
};
