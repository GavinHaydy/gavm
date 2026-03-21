// src/router/index.tsx
import { createBrowserRouter, Navigate, type RouteObject } from 'react-router-dom';

import { BasicLayout } from '@/app/layouts/BasicLayout';
import { LanguageEnum } from '@/core/constants/enum';
import { LanguageManagePage, Settings } from '@/features/version-manager/pages';
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
        element: <LanguageManagePage language={LanguageEnum.PYTHON} />,
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
        meta: {
          label: 'nav.js',
          icon: 'icon-JavaScript',
        },
        children: [
          {
            path: 'node',
            element: <LanguageManagePage language={LanguageEnum.NODE} />,
            meta: {
              label: 'nav.node',
              icon: 'icon-JavaScript',
            },
          },
          {
            path: 'deno',
            element: <div>deno</div>,
            meta: {
              label: 'nav.deno',
              icon: 'icon-JavaScript',
            },
          },
        ],
      },
      {
        path: 'go',
        element: <LanguageManagePage language={LanguageEnum.GO} />,
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
