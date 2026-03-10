import { List, Progress, Badge, Space, Drawer } from 'antd';
import { useTranslation } from 'react-i18next';

import { useDownload } from '@/hooks/useDownload.ts';
import './index.css';

export const DownloadCenter = ({ onClose, visible }: { onClose: () => void; visible: boolean }) => {
  const { tasks } = useDownload();
  const { t } = useTranslation();

  return (
    <Drawer
      className="download-center-drawer"
      onClose={onClose}
      open={visible}
      title={t('downloader.title')}
      style={{ margin: 24 }}
      closable={false}
    >
      <List
        dataSource={tasks}
        renderItem={item => (
          <List.Item>
            <div style={{ width: '100%' }}>
              <Space style={{ marginBottom: 8 }}>
                <strong>
                  {t('downloader.python_prefix')} {item.version}
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
    </Drawer>
  );
};
