import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import { ALIAS, DEDUPE, DIST, PUBLIC, REPO } from './paths.ts';

// Frontend des Asset-Studios (B-459): `vite build` schreibt nach tools/k3c-studio/frontend/dist/, das Fenster k3c-studio
// liefert es unter /studio/ aus (B-507, nicht mehr der Spielserver); `base: './'` hält die Adressen relativ.
// Im Dev-Modus (npm run dev, Port 5192) reicht /api an den Server des Studios (Port 8091, task k3c-studio) weiter, nicht an
// den Spielserver (B-507); ohne ihn hilft ?mock=1.
// public/ des Spiels nur beim Dev-Server; im Build bleibt es draußen, die Bilder liefert der Server aus dist/.
// appType 'mpa': unbekannte Pfade beantwortet Vite mit 404 statt index.html.
export default defineConfig(({ command }) => ({
  appType: 'mpa',
  base: './',
  plugins: [react()],
  resolve: { alias: ALIAS, dedupe: DEDUPE },
  publicDir: command === 'serve' ? PUBLIC : false,
  build: { outDir: DIST, emptyOutDir: true },
  server: {
    port: 5192,
    strictPort: true,
    fs: { allow: [REPO] },
    proxy: { '/api': { target: 'http://127.0.0.1:8091', changeOrigin: false } },
  },
}));
