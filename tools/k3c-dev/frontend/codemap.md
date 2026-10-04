# tools/k3c-dev/frontend/

## Responsibility

Build- und Toolchain-Konfiguration der k3c-dev-Oberfläche (React + TypeScript + Vite). Das gebaute `dist/` wird von Wails in die Desktop-Anwendung eingebettet; ohne Wails läuft dasselbe Frontend gegen die Mock-API im Browser.

## Design

- `package.json`: private ESM-Anwendung `k3c-dev-frontend`; Abhängigkeiten `react`/`react-dom` und `@radix-ui/themes` (feste Versionen), Dev: `vite`, `@vitejs/plugin-react`, `typescript`. Skripte `dev` (`vite`) und `build` (`tsc --noEmit && vite build`); `package.json.md5` ist ein Install-Stempel.
- `vite.config.ts`: Plugin `react()`, Dev-Server Port 5181 (`strictPort`).
- `tsconfig.json`: `strict`, `jsx: react-jsx`, `noUnusedLocals/Parameters`, Bundler-Resolution, `include: src, vite.config.ts`; Tests laufen im Vitest des Hauptprojekts.
- `index.html`: Einstieg mit `#root` und `src/main.tsx`.
- Generiert, nicht von Hand: `wailsjs/` (Wails-Bindings), `dist/`; Quellcode in `src/`.

## Flow

1. Entwicklung: `npx vite` (Port 5181) -> `src/main.tsx` -> `api/index.ts` erkennt fehlende Wails-Laufzeit -> Mock-Backend.
2. Build: `npm run build` (Typecheck + `vite build`) erzeugt `dist/`; `wails build`/`wails dev` bettet es ein bzw. startet den Dev-Server.
3. Im Wails-Fenster stellt die Runtime `window.go.main.App` bereit; `api/wails.ts` bindet daran.

## Integration

- Übergeordnet: Wails-Projekt `tools/k3c-dev/` (Go-Backend `app.go`, `internal/*`).
- Aufruf über Taskfile-Ziele des Repos (`task k3c-dev`, `task check:dev`).
- Quellstruktur: siehe `src/codemap.md`.
