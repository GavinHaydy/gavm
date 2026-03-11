// src/layouts/BasicLayout/index.tsx
import { DownloadOutlined } from '@ant-design/icons';
import { FloatButton, Layout } from 'antd';
import { useState } from 'react';
import { Outlet } from 'react-router-dom';

import { Sider } from './Sider';

import { DownloadCenter } from '@/features/version-manager/components/DownloadCenter';

const { Content, Sider: AntdSider } = Layout;

export const BasicLayout: React.FC = () => {
  const [collapsed, setCollapsed] = useState(false);
  const [visible, setVisible] = useState(false);
  return (
    <Layout style={{ maxHeight: '100vh', overflow: 'hidden' }}>
      <AntdSider width={200} theme="light" collapsed={collapsed} collapsible trigger={null}>
        <Sider collapsed={collapsed} onCollapse={setCollapsed} />
      </AntdSider>

      <Layout>
        <Content style={{ padding: 16 }}>
          <Outlet />
          <FloatButton onClick={() => setVisible(true)} icon={<DownloadOutlined />} />
          <DownloadCenter visible={visible} onClose={() => setVisible(false)} />
        </Content>
      </Layout>
    </Layout>
  );
};
