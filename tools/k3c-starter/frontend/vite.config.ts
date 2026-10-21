import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import { ALIAS, DEDUPE, WEBKIT } from './paths.ts';

// Frontend des Starters: `npx vite` spricht über /api/starter mit dem Starter im Modus -headless (src/api/http.ts, Port
// K3C_STARTER_API_PORT, Standard 8194); `?mock=1` zeigt erfundene Daten (src/api/mock.ts), `wails build` bettet dist/ ein.
const API = process.env.K3C_STARTER_API ?? 'http://127.0.0.1:8194';

export default defineConfig({
  plugins: [react()],
  resolve: { alias: ALIAS, dedupe: DEDUPE },
  server: { port: 5194, strictPort: true, cors: false, fs: { allow: ['.', WEBKIT] }, proxy: { '/api': API } },
});
