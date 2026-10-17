import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import { ALIAS, PUBLIC, REPO } from './paths.ts';

// Frontend des Asset-Studios (B-459): `wails dev` startet Vite auf diesem Port, `wails build` bettet dist/ ein.
// 5192 liegt neben k3c-dev (5191) und außerhalb der Worktree-Ports der Dienste (5183 + 10·n).
// public/ des Spiels nur beim Dev-Server (Mock im Browser, `wails dev`); im Build bleibt es draußen, sonst landete es
// in dist/ und in der EXE, die es über den Asset-Handler (public.go) liefert.
// appType 'mpa': unbekannte Pfade beantwortet Vite mit 404 statt index.html, damit Wails auf den Handler zurückfällt.
export default defineConfig(({ command }) => ({
  appType: 'mpa',
  plugins: [react()],
  resolve: { alias: ALIAS },
  publicDir: command === 'serve' ? PUBLIC : false,
  server: { port: 5192, strictPort: true, fs: { allow: [REPO] } },
}));
