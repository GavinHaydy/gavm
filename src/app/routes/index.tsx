// src/router/index.tsx
import { createBrowserRouter, Navigate, type RouteObject } from 'react-router-dom';

import { BasicLayout } from '@/app/layouts/BasicLayout';
import { GoManagePage } from '@/features/version-manager/pages/GoManagePage';
import { PythonManagePage } from '@/features/version-manager/pages/PythonManagePage';
import { Settings } from '@/features/version-manager/pages/Settings';
import { ErrorPage } from '@/pages/error';

interface RouteMeta {
  label?: string;
  icon?: string;
  hideInMenu?: boolean;
}

export type AppRouteObject = RouteObject & {
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
        path: 'java',
        element: <div>java</div>,
        meta: {
          label: 'nav.java',
          icon: 'icon-java',
        },
      },
      {
        path: 'js',
        element: <div>js</div>,
        meta: {
          label: 'nav.js',
          icon: 'icon-JavaScript',
        },
      },
      {
        path: 'go',
        element: <GoManagePage />,
        meta: {
          label: 'nav.go',
          icon: 'icon-golang',
        },
      },
      {
        path: 'rust',
        element: <div>rust</div>,
        meta: {
          label: 'nav.rust',
          icon: 'icon-rust',
        },
      },
      {
        path: 'v',
        element: <div>v</div>,
        meta: {
          label: 'nav.v',
          icon: 'icon-vlang',
        },
      },
      {
        path: 'zig',
        element: <div>zig</div>,
        meta: {
          label: 'nav.zig',
          icon: 'icon-zig',
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
    ],
  },
];

export const router = createBrowserRouter(routes as RouteObject[]);
