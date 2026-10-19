# tools/k3c-studio/frontend/

## Responsibility
Vite/React-Frontend des Asset-Studios, eine Seite des Go-Servers: Build nach `dist/studio/` (`task studio:frontend`), ausgeliefert unter `/studio/`; im Dev-Modus lädt Vite (Port 5192).

## Design
- `package.json`: React 19, Radix Themes, Vite; Skripte `dev` (`vite`) und `build` (`tsc --noEmit && vite build`).
- `index.html`: Einstieg, lädt `src/main.tsx`.
- `paths.ts`: `REPO`, `ALIAS` (`@game` → `src/`, `@data` → `data/`, `@webkit` → `tools/webkit/`), `DEDUPE` (`react`, `react-dom`, `@radix-ui/themes`: auch `tools/webkit/` bezieht sie aus diesem `node_modules`), `PUBLIC` (`public/` des Spiels) und `DIST` (`dist/studio`), gemeinsam für `vite.config.ts` und `vitest.config.ts`; `tsconfig.json` mit denselben `paths` (`include` auch `../../webkit`).
- `vite.config.ts`: `resolve.alias` = `ALIAS`, `resolve.dedupe` = `DEDUPE`, `base: './'` (relative Adressen), `appType: 'mpa'` (unbekannte Pfade 404 statt `index.html`), `build.outDir` = `DIST` mit `emptyOutDir`, `publicDir` = `public/` nur im Dev-Server (im Build liefert der Server die Bilder aus `dist/`), `server`: Port 5192 (`strictPort`), `fs.allow` auf die Repo-Wurzel, Proxy `/api` → `http://127.0.0.1:8080` (Spielserver).
- `vitest.config.ts`: Tests `src/**/*.test.{ts,tsx}` mit `ALIAS` und `DEDUPE`; das Vitest der Wurzel nimmt `tools/k3c-studio` aus.
- `src/`: siehe dort; gemeinsame UI-Teile aus `../../webkit/` (`errors.ts`, `useDebounced.ts`, `parts.tsx`, `theme.css`, `prefs.ts`) kommen über `@webkit/...`.

## Flow
Produktiv: `task studio:frontend` baut nach `dist/studio/`, der Go-Server liefert es unter `/studio/` aus (nur Dev-Mode, nur Loopback). Entwicklung: `npm run dev` im Ordner startet Vite (5192), `/api` geht an den Spielserver (8080); ohne Server läuft mit `?mock=1` der Mock.

## Integration
- Backend: Go-Paket `engine/studio/` (`rpc.go`, `app*.go`), eingehängt durch `cmd/k3c-server/studio.go`; Asset-Logik in `tools/assetkit`. Landingpage-Kachel „Welten bauen“: `studio.html`.
- Tasks: `task studio:frontend`, `task check:studio` (Vitest mit `vitest.config.ts`).
