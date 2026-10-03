import '@radix-ui/themes/styles.css';
import './styles/theme.css';
import './styles/app.css';
import './styles/services.css';
import './styles/logs.css';
import './styles/mcp.css';
import './styles/tasks.css';
import './styles/planning.css';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { App } from './App';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
