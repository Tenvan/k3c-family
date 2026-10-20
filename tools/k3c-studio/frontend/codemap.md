# tools/k3c-studio/frontend/

## Responsibility
Vite/React-Frontend des Asset-Studios im eigenen Fenster `tools/k3c-studio` (keine Seite des Spielservers, Entscheidung 004): Build nach `tools/k3c-studio/frontend/dist/` (`task studio:frontend`), ausgeliefert vom Fenster unter `/studio/`; im Dev-Modus lädt Vite (Port 5192).

## Design
- `package.json`: React 19, Radix Themes, Vite; Skripte `dev` (`vite`) und `build` (`tsc --noEmit && vite build`).
- `index.html`: Einstieg, lädt `src/main.tsx`.
- `paths.ts`: `REPO`, `ALIAS` (`@game` → `src/`, `@data` → `data/`, `@webkit` → `tools/webkit/`), `DEDUPE` (`react`, `react-dom`, `@radix-ui/themes`: auch `tools/webkit/` bezieht sie aus diesem `node_modules`), `PUBLIC` (`public/` des Spiels) und `DIST` (`frontend/dist`), gemeinsam für `vite.config.ts` und `vitest.config.ts`; `tsconfig.json` mit denselben `paths` (`include` auch `../../webkit`).
- `vite.config.ts`: `resolve.alias` = `ALIAS`, `resolve.dedupe` = `DEDUPE`, `base: './'` (relative Adressen), `appType: 'mpa'` (unbekannte Pfade 404 statt `index.html`), `build.outDir` = `DIST` mit `emptyOutDir`, `publicDir` = `public/` nur im Dev-Server (im Build liefert der Server die Bilder aus `dist/`), `server`: Port 5192 (`strictPort`), `fs.allow` auf die Repo-Wurzel, Proxy `/api` → `http://127.0.0.1:8080` (Spielserver).
- `vitest.config.ts`: Tests `src/**/*.test.{ts,tsx}` mit `ALIAS` und `DEDUPE`; das Vitest der Wurzel nimmt `tools/k3c-studio` aus.
- `src/`: siehe dort; gemeinsame UI-Teile aus `../../webkit/` (`errors.ts`, `useDebounced.ts`, `parts.tsx`, `theme.css`, `prefs.ts`) kommen über `@webkit/...`.

## Flow
Produktiv: `task studio:frontend` baut nach `frontend/dist/`, das Fenster `k3c-studio` liefert es unter `/studio/` aus (nur Loopback, Port 8091). Entwicklung: `npm run dev` im Ordner startet Vite (5192), `/api` geht an das Studio (8091); ohne Server läuft mit `?mock=1` der Mock.

## Integration
- Backend: Go-Paket `tools/k3c-studio/backend/` (`rpc.go`, `app*.go`), eingehängt durch `tools/k3c-studio/server.go`; Asset-Logik in `tools/assetkit`.
- Tasks: `task studio:frontend`, `task check:studio` (Vitest mit `vitest.config.ts`).
