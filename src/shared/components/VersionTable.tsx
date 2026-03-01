import type { TableProps } from 'antd';
import { Table, Input, Button } from 'antd';
import React from 'react';
import { useTranslation } from 'react-i18next';

const { Search } = Input;

export interface VersionItem {
  version: string;
  install_status: boolean;
  use_status: boolean;
}
export interface VersionResult {
  total: number;
  list: VersionItem[];
}

interface VersionTableProps {
  data: VersionResult;
  loading?: boolean;

  onSearch?: (value: string) => void;
  onInstallToggle?: (record: VersionItem) => void;
  onUseToggle?: (record: VersionItem) => void;
}

export const VersionTable: React.FC<VersionTableProps> = ({
  data,
  loading,
  onSearch,
  onInstallToggle,
  onUseToggle,
}) => {
  const { t } = useTranslation();

  const columns: TableProps<VersionItem>['columns'] = [
    {
      title: t('table.version'),
      dataIndex: 'version',
    },
    {
      title: t('table.installStatus'),
      dataIndex: 'install_status',
      render: (_, record) => (
        <Button
          type="primary"
          danger={record.install_status}
          onClick={() => onInstallToggle?.(record)}
        >
          {record.install_status ? t('table.uninstall') : t('table.install')}
        </Button>
      ),
    },
    {
      title: t('table.useStatus'),
      dataIndex: 'use_status',
      render: (_, record) => (
        <Button
          type={record.use_status ? 'primary' : 'default'}
          onClick={() => onUseToggle?.(record)}
        >
          {record.use_status ? t('table.used') : t('table.use')}
        </Button>
      ),
    },
  ];

  return (
    <>
      <Search
        placeholder={t('search.placeholder')}
        enterButton={t('search.button')}
        onSearch={onSearch}
        style={{ marginBottom: 12 }}
      />

      <Table
        size="small"
        dataSource={data.list}
        columns={columns}
        rowKey={record => record.version}
        loading={loading}
        pagination={{
          total: data.total,
          current: 1,
          pageSize: 10,
        }}
      />
    </>
  );
};
