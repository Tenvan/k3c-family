import { defineConfig } from 'vite';
import { resolve } from 'node:path';
// Reines JS-Modul, gemeinsam mit dem Heimnetz-Server genutzt
import { handleReport } from './server/reports.mjs';

export default defineConfig({
  // Relative Pfade, damit der Build von jedem Heimnetz-Server/Unterordner aus läuft.
  base: './',
  server: { host: true, port: 5173 },
  preview: { host: true, port: 4173 },
  build: {
    target: 'es2022',
    chunkSizeWarningLimit: 2000,
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
        gamepadTest: resolve(__dirname, 'gamepad-test.html'),
      },
    },
  },
  plugins: [
    {
      // Testberichte auch im Dev-Server annehmen (POST /api/report -> reports/*.json)
      name: 'k3c-reports',
      configureServer(server) {
        server.middlewares.use((req, res, next) => {
          handleReport(req, res).then((handled: boolean) => handled || next(), next);
        });
      },
    },
  ],
});
