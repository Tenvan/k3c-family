import { defineConfig } from 'vite';

export default defineConfig({
  // Relative Pfade, damit der Build von jedem Heimnetz-Server/Unterordner aus läuft.
  base: './',
  server: { host: true, port: 5173 },
  preview: { host: true, port: 4173 },
  build: { target: 'es2022', chunkSizeWarningLimit: 2000 },
});
