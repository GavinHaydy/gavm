// src/router/index.tsx
import { createBrowserRouter, Navigate, type RouteObject } from 'react-router-dom';
import { BasicLayout } from '@/app/layouts/BasicLayout';
import { PythonManagePage } from '@/features/version-manager/pages/PythonManagePage';
import { GoManagePage } from '@/features/version-manager/pages/Go';
import { Settings } from '@/features/version-manager/pages/Settings';
import { ErrorPage } from '@/pages/error';

// 菜单元数据类型
interface RouteMeta {
  label?: string; // i18n key
  icon?: string; // IconFont type 或 Antd 图标名
  hideInMenu?: boolean; // 是否隐藏
}

// 使用类型交叉扩展 RouteObject（而不是 extends）
type AppRouteObject = RouteObject & {
  meta?: RouteMeta;
  children?: AppRouteObject[];
};

export const routes: AppRouteObject[] = [
  {
    path: '/',
    element: <BasicLayout />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <Navigate to="/python" />,
        meta: { hideInMenu: true },
      },
      {
        path: 'python',
        element: <PythonManagePage />,
        meta: {
          label: 'nav.python',
          icon: 'icon-python',
        },
      },
      {
        path: 'settings',
        element: <Settings />,
        meta: {
          label: 'nav.settings',
          icon: 'SettingOutlined',
        },
      },
      // 2. 在这里添加 Go 路由
      {
        path: 'go',
        element: <GoManagePage />,
        meta: {
          label: 'nav.go',
          icon: 'icon-golang', // 对应 IconFont 的 type
        },
      },
    ],
  },
];

export const router = createBrowserRouter(routes as RouteObject[]);
