import { RouterProvider } from 'react-router-dom';

import { router } from '@/app/routes';
import '../features/i18n/index';

function App() {
  return <RouterProvider router={router} />;
}

export default App;
