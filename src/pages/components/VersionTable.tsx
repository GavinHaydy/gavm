import React from 'react';
import type { TableProps } from 'antd';
import { Table, Input, Button } from 'antd';

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
  const columns: TableProps<VersionItem>['columns'] = [
    {
      title: 'Version',
      dataIndex: 'version',
    },
    {
      title: 'Install Status',
      dataIndex: 'install_status',
      render: (_, record) => (
        <Button
          type="primary"
          danger={record.install_status}
          onClick={() => onInstallToggle?.(record)}
        >
          {record.install_status ? 'Uninstall' : 'Install'}
        </Button>
      ),
    },
    {
      title: 'Use Status',
      dataIndex: 'use_status',
      render: (_, record) => (
        <Button
          type={record.use_status ? 'primary' : 'default'}
          onClick={() => onUseToggle?.(record)}
        >
          {record.use_status ? 'Used' : 'Use'}
        </Button>
      ),
    },
  ];

  return (
    <>
      <Search
        placeholder="Input search text"
        enterButton="Search"
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
