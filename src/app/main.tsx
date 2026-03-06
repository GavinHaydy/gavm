import React from 'react';
import ReactDOM from 'react-dom/client';
import { Provider } from 'react-redux';
import { RouterProvider } from 'react-router-dom';

import { router } from '@/app/routes';
import { initializeI18n } from '@/features/i18n';
import { ThemeProvider } from '@/features/theme/ThemeProvider';
import { setMode } from '@/features/theme/themeSlice';
import { loadTheme } from '@/shared/utils/tauriStore';
import { store } from '@/store';

import '../features/i18n/index';
import './main.css';
async function bootstrap() {
  const savedTheme = await loadTheme();

  await initializeI18n();

  if (savedTheme === 'dark' || savedTheme === 'light') {
    store.dispatch(setMode(savedTheme));
  }

  ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
      <Provider store={store}>
        <ThemeProvider>
          <RouterProvider router={router} />
        </ThemeProvider>
      </Provider>
    </React.StrictMode>,
  );
}

void bootstrap();
