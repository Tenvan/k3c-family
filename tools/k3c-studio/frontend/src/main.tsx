import '@radix-ui/themes/styles.css';
import './styles/theme.css';
import './styles/app.css';
import './styles/assets.css';
import './styles/szenen.css';
import './styles/world.css';
import './styles/planer.css';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { App } from './App';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
