import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import { ALIAS, DIST, PUBLIC, REPO } from './paths.ts';

// Frontend des Asset-Studios (B-459), eine Seite des Go-Servers: `vite build` schreibt nach dist/studio/ (der Server
// liefert dist/ aus, die Landingpage verlinkt studio.html), `base: './'` hält die Adressen relativ.
// Im Dev-Modus (npm run dev, Port 5192) reicht /api an den Spielserver (Port 8080) weiter; ohne ihn hilft ?mock=1.
// public/ des Spiels nur beim Dev-Server; im Build bleibt es draußen, die Bilder liefert der Server aus dist/.
// appType 'mpa': unbekannte Pfade beantwortet Vite mit 404 statt index.html.
export default defineConfig(({ command }) => ({
  appType: 'mpa',
  base: './',
  plugins: [react()],
  resolve: { alias: ALIAS },
  publicDir: command === 'serve' ? PUBLIC : false,
  build: { outDir: DIST, emptyOutDir: true },
  server: {
    port: 5192,
    strictPort: true,
    fs: { allow: [REPO] },
    proxy: { '/api': { target: 'http://127.0.0.1:8080', changeOrigin: false } },
  },
}));
