import { configDefaults, defineConfig } from 'vitest/config';
import { execFileSync } from 'node:child_process';
import { readdirSync } from 'node:fs';
import { resolve } from 'node:path';

/** Version des Clients: K3C_VERSION (Dockerfile, ohne .git), sonst `git describe`, sonst `dev`. */
function clientVersion(): string {
  if (process.env.K3C_VERSION) return process.env.K3C_VERSION;
  try {
    return execFileSync('git', ['describe', '--tags', '--always', '--dirty'], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).trim();
  } catch {
    return 'dev';
  }
}

export default defineConfig({
  // Relative Pfade, damit der Build von jedem Heimnetz-Server/Unterordner aus läuft.
  base: './',
  // Build-Infos für src/core/version.ts (Landingpage, Lobby, Debug-Overlay)
  define: {
    __APP_VERSION__: JSON.stringify(clientVersion()),
    __BUILD_TIME__: JSON.stringify(new Date().toISOString()),
  },
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
