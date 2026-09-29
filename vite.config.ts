import { defineConfig } from 'vite';
import { readdirSync } from 'node:fs';
import { resolve } from 'node:path';
// Reines JS-Modul, gemeinsam mit dem Heimnetz-Server genutzt
import { handleReport } from './server/reports.mjs';
import { handleSave } from './server/saves.mjs';

export default defineConfig({
  // Relative Pfade, damit der Build von jedem Heimnetz-Server/Unterordner aus läuft.
  base: './',
  server: { host: true, port: 5173 },
  preview: { host: true, port: 4173 },
  build: {
    target: 'es2022',
    chunkSizeWarningLimit: 2000,
    rollupOptions: {
      // Jede *.html im Projektordner ist eine eigene Seite (Landingpage, Spiel, Testseiten).
      input: Object.fromEntries(
        readdirSync(__dirname)
          .filter((f) => f.endsWith('.html'))
          .map((f) => [f.replace(/\.html$/, ''), resolve(__dirname, f)]),
      ),
    },
  },
  plugins: [
    {
      // Testberichte und Spielstände auch im Dev-Server (POST /api/report -> reports/, /api/save -> saves/)
      name: 'k3c-api',
      configureServer(server) {
        server.middlewares.use((req, res, next) => {
          handleReport(req, res)
            .then(async (handled: boolean) => handled || (await handleSave(req, res)) || next())
            .catch(next);
        });
      },
    },
  ],
});
