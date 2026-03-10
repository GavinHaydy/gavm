import type { TableProps } from 'antd';
import { Table, Input, Button } from 'antd';
import React from 'react';
import { useTranslation } from 'react-i18next';

import { CommandEnum, InstallStatusEnum } from '@/core/constants/enum';
import { VersionItem, VersionResult } from '@/core/types/common';

import './index.css';

interface VersionTableProps {
  data: VersionResult;
  loading?: boolean;
  onSearch?: (value: string) => void;
  handlePageChange: (page: number, pageSize: number) => void;
  handleVersionAction?: (
    command: CommandEnum | InstallStatusEnum,
    record: VersionItem,
  ) => Promise<void>;
}

export const VersionTable: React.FC<VersionTableProps> = ({
  data,
  loading,
  onSearch,
  handleVersionAction,
  handlePageChange,
}) => {
  const { t } = useTranslation();

  const onInstallToggle = async (record: VersionItem) => {
    const command = record.installStatus
      ? InstallStatusEnum.UNINSTALLED
      : InstallStatusEnum.INSTALLED;
    await handleVersionAction?.(command, record);
  };

  const onUseToggle = async (record: VersionItem) => {
    const command = record.useStatus ? CommandEnum.UNUSE_VERSION : CommandEnum.USE_VERSION;
    await handleVersionAction?.(command, record);
  };

  const columns: TableProps<VersionItem>['columns'] = [
    {
      title: t('table.version'),
      dataIndex: 'version',
    },
    {
      title: t('table.install_status'),
      dataIndex: 'installStatus',
      render: (_, record) => (
        <Button
          className="table-button"
          type="primary"
          danger={record.installStatus}
          onClick={() => onInstallToggle?.(record)}
        >
          {record.installStatus ? t('table.uninstall') : t('table.install')}
        </Button>
      ),
    },
    {
      title: t('table.use_status'),
      dataIndex: 'useStatus',
      render: (_, record) => (
        <Button
          type={record.useStatus ? 'primary' : 'default'}
          onClick={() => onUseToggle?.(record)}
        >
          {record.useStatus ? t('table.used') : t('table.use')}
        </Button>
      ),
    },
  ];

  return (
    <>
      <div style={{ marginBottom: 12, marginTop: 12, textAlign: 'center' }}>
        <Input.Search
          placeholder={t('search.placeholder')}
          enterButton={t('search.button')}
          onSearch={onSearch}
          style={{
            marginBottom: 12,
            width: '100%',
            maxWidth: 400,
          }}
        />
      </div>

      <Table
        size="small"
        dataSource={data.list}
        columns={columns}
        rowKey={record => record.version}
        loading={loading}
        scroll={{ x: 'max-content', y: 'calc(100vh - 200px)' }}
        pagination={{
          total: data.total,
          current: data.page + 1,
          pageSize: data.pageSize,
          showSizeChanger: true,
          pageSizeOptions: ['10', '20', '50'],
        }}
        onChange={pagination => {
          handlePageChange((pagination.current || 1) - 1, pagination.pageSize || 10);
        }}
      />
    </>
  );
};
