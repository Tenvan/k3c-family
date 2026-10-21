import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import { ALIAS, DEDUPE, DIST, PUBLIC, REPO } from './paths.ts';

// Frontend des Asset-Studios (B-459): `vite build` schreibt nach tools/k3c-studio/frontend/dist/, das Fenster k3c-studio
// liefert es unter /studio/ aus (B-507, nicht mehr der Spielserver); `base: './'` hält die Adressen relativ.
// Im Dev-Modus (npm run dev, Port 5192) reicht /api an das Backend des Studios (`task k3c-studio:dev`: Port 8192) weiter, nicht an
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
    port: Number(process.env.K3C_STUDIO_UI_PORT ?? 5192), // k3c-dev verschiebt ihn je Worktree (services.json › portEnv)
    strictPort: true,
    // Kein CORS: sonst beantwortet Vite Preflights fremder lokaler Seiten und reicht deren Anfragen an das Backend weiter.
    cors: false,
    fs: { allow: [REPO] },
    // Standard: das Backend von `task k3c-studio:dev` (Port 8192); `dev:ui` und `window` setzen K3C_STUDIO_API ausdrücklich.
    proxy: { '/api': { target: process.env.K3C_STUDIO_API ?? 'http://127.0.0.1:8192', changeOrigin: false } },
  },
}));
