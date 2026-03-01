import { List, Progress, Card, Badge, Space } from 'antd';
import { useTranslation } from 'react-i18next';

import { useDownload } from '@/hooks/useDownload.ts';

export const DownloadCenter = () => {
  const { tasks } = useDownload();
  const { t } = useTranslation();

  return (
    <Card title={t('downloader.title')} style={{ margin: 24 }}>
      <List
        dataSource={tasks}
        renderItem={item => (
          <List.Item>
            <div style={{ width: '100%' }}>
              <Space style={{ marginBottom: 8 }}>
                <strong>
                  {t('downloader.pythonPrefix')} {item.version}
                </strong>
                {item.status === 'success' ? (
                  <Badge status="success" text={t('downloader.completed')} />
                ) : (
                  <Badge status="processing" text={t('downloader.downloading')} />
                )}
              </Space>
              <Progress
                percent={item.percentage}
                status={item.status === 'success' ? 'normal' : 'active'}
              />
            </div>
          </List.Item>
        )}
      />
    </Card>
  );
};
