# tools/k3c-studio/

## Responsibility
Eigenständiges Fenster (Wails, `main.go`, `window.go`): startet das Backend `backend/` und die Oberfläche `frontend/dist` auf Loopback (Port 8091, `K3C_STUDIO_PORT`), zeigt sie im Fenster und merkt Größe und Position wie k3c-dev (`%AppData%\k3c\k3c-studio.json`). Bauen: `task k3c-studio:build` → `bin/k3c-studio.exe`. Ohne Fenster: `-headless` (Backend und Engine auf `K3C_STUDIO_PORT`, `dev.go`); `-dev` baut es bei jeder Go-Änderung als Kind neu (`build/bin/k3c-studio-devchild-<Port>.exe`, `engine/devrun`, `task k3c-studio:dev` mit Vite 5192 im Browser, Backend 8192; das Fenster unter `wails dev`: `task k3c-studio:window`).

Frontend des Asset-Studios (B-459): React-Oberfläche in `frontend/`, Build nach `frontend/dist/` (`task k3c-studio:frontend`), nur vom Fenster unter `/studio/` ausgeliefert; der Spielserver hat keine Studio-Seite und keine Kachel mehr (Entscheidung 004, B-507). Das **Backend** (RPC, Register, Planung) liegt in [`backend/`](backend/codemap.md) (Paket `backend`, bis TS1.1 `engine/studio`).

## Design
- `frontend/`: Vite/React/Radix; Details in [`frontend/codemap.md`](frontend/codemap.md). Spiel-Code nur lesend über die Aliase `@game` (`src/`) und `@data` (`data/`), festgelegt in `frontend/paths.ts` für `vite.config.ts` (publicDir = `public/` nur im Dev-Modus, `server.fs.allow` auf die Repo-Wurzel) und `vitest.config.ts`. Gemeinsame UI-Teile (Fehlertext, Entprellen, Bausteine, Theme, Prefs-Fabrik) liegen in [`tools/webkit/`](../webkit/) (Alias `@webkit/...`, geteilt mit k3c-dev).
- Seiten und Bausteine unter `frontend/src/` (Detail je Ordner in `frontend/src/*/codemap.md`):
  - `App.tsx`: Rahmen mit Navigation Grafik/Sound/Welten Editor/Gebäude-Planer/Monarch/Gegner (gemerkt unter `k3c-studio.app.page`), Mock-Hinweis.
  - `api/`: Typen mit `stand`, Aufrufe an `/api/studio/` (`http.ts`), Mock im Test und mit `?mock=1`.
  - `assets/`: gemeinsame Seitenstruktur für Grafik und Sound (Slots, Katalog, Recherche, `useAssets` mit Neuladen bei `assets:changed`).
  - `graphics/`: Seite Grafik (Slots, Szenen, Packs; `GraphicsPage`).
  - `sound/`: Seite Sound (Slots, Szenen, Hörprobe).
  - `world/`: Welten Editor mit Chunk-Streifen, Live-Vorschau, Planung geplanter Chunks, Inselgrößen, Seed-Chunk umwandeln, „Im Spiel öffnen“.
  - `planner/`: Gebäude-Planer (Matrix der Hub-Stufen, Bearbeiten und Speichern; `PlannerPage`).
  - `stage/`: wiederverwendbare Phaser-Bühne mit `LivePreview`; Phaser kommt aus der Wurzel-`node_modules`.
  - `playback/`, `monarch/`, `enemies/`: Abspiel-Bausteine für Figuren und die Seiten Monarch und Gegner (`EnemiesPage`, nur lesend).
  - `lib/`, `ui/`, `styles/`: `lib/prefs.ts` bindet die Prefs-Fabrik aus `@webkit/prefs` an das Präfix `k3c-studio.`, `ui/Splitter.tsx` und die eigenen Styles (ohne `theme.css`, das kommt aus `@webkit/theme.css`).

## Flow
1. `task k3c-studio:frontend` baut nach `tools/k3c-studio/frontend/dist/`; das Fenster (`main.go`, `server.go`) liefert es auf Loopback unter `/studio/` aus.
2. Die Seite ruft `/api/studio/rpc/<Methode>` auf (`src/api/http.ts`), lädt bei `assets:changed` (SSE) neu und speichert mit dem `stand` aus dem Laden.
3. Ohne Server (`npm run dev` mit `?mock=1`, Vitest) antwortet der Mock.

## Integration
- Backend: `backend/` (eingehängt von `server.go`; der Spielserver bindet es seit TS1.1 nicht mehr ein); Asset-Logik in `tools/assetkit`; die MCP-Tools `assets_*` bleiben in k3c-dev (dessen Knopf „Asset-Studio öffnen“ öffnet `/studio/index.html` im Browser).
- Tasks: `task k3c-studio:frontend` (Build), `task check:studio` (Studio-Vitest, Go-Tests und Lint von assetkit und diesem Modul samt Backend). `task k3c-studio:dev` (Vite 5192) leitet `/api` an das Backend des Studios (8192, `K3C_STUDIO_API`). `server.go` liefert neben der Studio-API die Spiel-API der eingebundenen Engine (`/api/…`, `/ws`, B-507/AC-02), Vorschau-Spiele laufen ohne Spielserver. Das Vitest der Wurzel nimmt `tools/k3c-studio/**` aus.
- Daten (über das Backend): `data/assets.json`, `data/assets-recherche.json`, `public/**`.
