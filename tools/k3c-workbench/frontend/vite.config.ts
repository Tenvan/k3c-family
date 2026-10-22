import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import { ALIAS, DEDUPE, WEBKIT } from './paths.ts';
import { waitForBackend } from '../../webkit/lib/devProxy.ts';

const API = process.env.K3C_DEV_API ?? 'http://127.0.0.1:8191';

// Frontend von k3c-workbench: `task k3c-workbench:dev` startet Vite gegen `k3c-workbench -dev` (Proxy /api auf die Oberflächen-API,
// K3C_DEV_API, Vorgabe 127.0.0.1:8191), `?mock=1` zeigt den Mock (src/api/mock.ts), `wails build` bettet dist/ ein.
// changeOrigin bleibt aus: Host und Origin (localhost:5191) bestehen die Prüfungen von k3c-workbench (uiapi.go). cors aus: keine
// fremde Seite darf Antworten des Dev-Servers lesen.
export default defineConfig({
  plugins: [react(), waitForBackend(API)], // /api wartet, bis das Backend bereit ist (auch beim Neustart)
  resolve: { alias: ALIAS, dedupe: DEDUPE },
  server: {
    port: 5191,
    strictPort: true,
    cors: false,
    fs: { allow: ['.', WEBKIT] },
    proxy: { '/api': { target: API, changeOrigin: false } },
  },
});
