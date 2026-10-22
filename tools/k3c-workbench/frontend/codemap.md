# tools/k3c-workbench/frontend/

## Responsibility

Build- und Toolchain-Konfiguration der k3c-workbench-Oberfläche (React + TypeScript + Vite). Das gebaute `dist/` wird von Wails in die Desktop-Anwendung eingebettet; ohne Wails läuft dasselbe Frontend im Browser gegen `k3c-workbench -headless`/`-dev` (HTTP, `src/api/http.ts`) oder mit `?mock=1` gegen die Mock-API.

## Design

- `package.json`: private ESM-Anwendung `k3c-workbench-frontend`; Abhängigkeiten `react`/`react-dom` und `@radix-ui/themes` (feste Versionen), Dev: `vite`, `@vitejs/plugin-react`, `typescript`. Skripte `dev` (`vite`) und `build` (`tsc --noEmit && vite build`); `package.json.md5` ist ein Install-Stempel.
- `vite.config.ts`: Plugin `react()`, Dev-Server Port 5191 (`strictPort`), Proxy `/api` auf `K3C_DEV_API` (Vorgabe `http://127.0.0.1:8191`, `changeOrigin: false` für Host- und Origin-Prüfung), `cors: false`.
- `tsconfig.json`: `strict`, `jsx: react-jsx`, `noUnusedLocals/Parameters`, Bundler-Resolution, `include: src, ../../webkit`, `paths` für `@webkit/*` und für `react`/`@radix-ui/themes` aus dem eigenen `node_modules`. `paths.ts` (`ALIAS` mit `@webkit`, `DEDUPE`, `WEBKIT`) teilen `vite.config.ts` (Alias, Dedupe, `server.fs.allow`) und `vitest.config.ts` (eigenes Vitest, `task check:dev`; das Vitest der Wurzel nimmt das Frontend aus).
- `index.html`: Einstieg mit `#root` und `src/main.tsx`.
- Generiert, nicht von Hand: `wailsjs/` (Wails-Bindings), `dist/`; Quellcode in `src/`.

## Flow

1. Entwicklung: `task k3c-workbench:dev` -> Vite (Port 5191) -> `src/main.tsx` -> `api/index.ts` erkennt fehlende Wails-Laufzeit -> HTTP-Backend über den Proxy (`?mock=1`: Mock).
2. Build: `npm run build` (Typecheck + `vite build`) erzeugt `dist/`; `wails build` bettet es ein (auch `go:embed` im `-headless`), `wails dev` (`task k3c-workbench:window`) startet den Dev-Server.
3. Im Wails-Fenster stellt die Runtime `window.go.main.App` bereit; `api/wails.ts` bindet daran.

## Integration

- Übergeordnet: Wails-Projekt `tools/k3c-workbench/` (Go-Backend `app.go`, `internal/*`).
- Aufruf über Taskfile-Ziele des Repos (`task k3c-workbench`, `task check:dev`).
- Quellstruktur: siehe `src/codemap.md`.
