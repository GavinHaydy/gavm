import { createBrowserRouter, Navigate } from 'react-router-dom';

import { BasicLayout } from '@/app/layouts/BasicLayout';
import { PythonManagePage } from '@/features/version-manager/pages/PythonManagePage';
import { Settings } from '@/features/version-manager/pages/Settings';
import { ErrorPage } from '@/pages/error';

export const router = createBrowserRouter([
  {
    path: '/',
    element: <BasicLayout />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <Navigate to="/python" />,
      },
      {
        path: 'python',
        element: <PythonManagePage />,
      },
      {
        path: 'settings',
        element: <Settings />,
      },
    ],
  },
]);
