import { configDefaults, defineConfig } from 'vitest/config';
import { execFileSync } from 'node:child_process';
import { readdirSync } from 'node:fs';
import { isAbsolute, relative, resolve } from 'node:path';

/** Version des Clients: K3C_VERSION (Dockerfile, ohne .git), sonst `git describe`, sonst `dev`. */
function clientVersion(): string {
  if (process.env.K3C_VERSION) return process.env.K3C_VERSION;
  try {
    return execFileSync('git', ['describe', '--tags', '--always', '--dirty'], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).trim();
  } catch {
    return 'dev';
  }
}

const serverPort = process.env.K3C_HTTP_PORT ?? '8080';

// Nur das eigene .claude/ (Worktrees) ignorieren: Ein Glob auf .claude träfe jede Datei, wenn diese Wurzel selbst darunter liegt (B-275).
const claudeDir = resolve(__dirname, '.claude');
function inClaudeDir(file: string): boolean {
  const rel = relative(claudeDir, file);
  return !rel.startsWith('..') && !isAbsolute(rel);
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
  // k3c-dev setzt beide Ports je Worktree (K3C_VITE_PORT, K3C_HTTP_PORT), damit Worktrees nebeneinander laufen.
  server: {
    host: true,
    port: Number(process.env.K3C_VITE_PORT ?? 5173),
    proxy: { '/api': `http://localhost:${serverPort}`, '/ws': { target: `ws://localhost:${serverPort}`, ws: true } },
    // Worktrees im eigenen .claude/ (zehntausende Dateien) und laufend geschriebene Ausgaben nicht beobachten,
    // sonst blockiert der Watcher unter Windows die Event-Loop und Anfragen hängen sekundenlang.
    watch: { ignored: [inClaudeDir, '**/.omc/**', '**/.work/**', '**/logs/**', '**/reports/**', '**/saves/**', '**/dist/**', '**/_site/**', '**/bin/**'] },
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
