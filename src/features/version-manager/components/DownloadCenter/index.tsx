import { List, Progress, Badge, Space, Drawer } from 'antd';
import { useTranslation } from 'react-i18next';

import { DownloadStatusEnum } from '@/core/constants/enum';
import { useDownload } from '@/hooks/useDownload.ts';
import './index.css';

const statusMap: Record<DownloadStatusEnum, 'exception' | 'active' | 'success'> = {
  [DownloadStatusEnum.ERROR]: 'exception',
  [DownloadStatusEnum.DOWNLOADING]: 'active',
  [DownloadStatusEnum.SUCCESS]: 'success',
};

export const DownloadCenter = ({ onClose, visible }: { onClose: () => void; visible: boolean }) => {
  const { tasks } = useDownload();
  const { t } = useTranslation();

  const renderStatus = (status: DownloadStatusEnum) => {
    switch (status) {
      case DownloadStatusEnum.ERROR:
        return <Badge status="error" text={t('downloader.failed')} />;
      case DownloadStatusEnum.DOWNLOADING:
        return <Badge status="processing" text={t('downloader.downloading')} />;
      case DownloadStatusEnum.SUCCESS:
        return <Badge status="success" text={t('downloader.completed')} />;
      default:
        return null;
    }
  };

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
        locale={{ emptyText: t('downloader.empty') }}
        renderItem={item => (
          <List.Item>
            <div style={{ width: '100%' }}>
              <Space style={{ marginBottom: 8 }}>
                <strong>
                  {item.language} {item.version}
                </strong>

                {renderStatus(item.status)}
              </Space>

              <Progress percent={item.percentage} status={statusMap[item.status]} />
            </div>
          </List.Item>
        )}
      />
    </Drawer>
  );
};
