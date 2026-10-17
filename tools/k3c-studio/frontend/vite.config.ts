import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

// Frontend des Asset-Studios (B-459): `wails dev` startet Vite auf diesem Port, `wails build` bettet dist/ ein.
// 5192 liegt neben k3c-dev (5191) und außerhalb der Worktree-Ports der Dienste (5183 + 10·n).
// appType 'mpa': unbekannte Pfade (Bilder und Sounds aus public/) beantwortet Vite mit 404 statt index.html, damit
// Wails unter `wails dev` auf publicHandler zurückfällt (ungeprüft bis zum ersten Fensterlauf, WZ6.3).
export default defineConfig({
  appType: 'mpa',
  plugins: [react()],
  server: { port: 5192, strictPort: true },
});
