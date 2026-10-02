import { configDefaults, defineConfig } from 'vitest/config';
import { readdirSync } from 'node:fs';
import { resolve } from 'node:path';

export default defineConfig({
  // Relative Pfade, damit der Build von jedem Heimnetz-Server/Unterordner aus läuft.
  base: './',
  // /api und /ws gehören dem Go-Server (task start, Port 8080); läuft er nicht, meldet Vite den Proxy-Fehler im Terminal.
  server: {
    host: true,
    port: 5173,
    proxy: { '/api': 'http://localhost:8080', '/ws': { target: 'ws://localhost:8080', ws: true } },
  },
  preview: { host: true, port: 4173 },
  // .claude/worktrees enthält komplette Checkouts anderer Branches; deren Tests gehören nicht zu diesem Lauf.
  test: { exclude: [...configDefaults.exclude, '.claude/**'] },
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
});
