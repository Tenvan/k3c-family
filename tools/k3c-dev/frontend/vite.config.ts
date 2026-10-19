import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import { ALIAS, DEDUPE, WEBKIT } from './paths.ts';

// Frontend von k3c-dev: `npx vite` läuft ohne Wails gegen den Mock (src/api/mock.ts), `wails build` bettet dist/ ein.
export default defineConfig({
  plugins: [react()],
  resolve: { alias: ALIAS, dedupe: DEDUPE },
  server: { port: 5191, strictPort: true, fs: { allow: ['.', WEBKIT] } },
});
