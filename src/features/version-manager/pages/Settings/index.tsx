import { LazyStore } from '@tauri-apps/plugin-store';
import { Button, Card, Input, message } from 'antd';
import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';

const store = new LazyStore('.settings.json');

export const Settings = () => {
  const [basePath, setBasePath] = useState('');
  const { t } = useTranslation();

  // 初始化读取
  useEffect(() => {
    void store.get<{ value: string }>('base_path').then(val => {
      setBasePath(val?.value || 'd:\\lvm');
    });
  }, []);

  const handleSave = async () => {
    await store.set('base_path', basePath);
    await store.save(); // 持久化到硬盘
    message.success(t('settings.success'));
  };

  return (
    <Card title={t('settings.title')}>
      <Input
        value={basePath}
        onChange={e => setBasePath(e.target.value)}
        placeholder={t('settings.placeholder')}
      />
      <Button onClick={handleSave}>{t('settings.save')}</Button>
    </Card>
  );
};
