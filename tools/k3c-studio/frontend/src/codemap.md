# tools/k3c-studio/frontend/src/

## Responsibility
React-Oberfläche des Asset-Studios: Rahmen mit Seitenwahl und die Seiten Grafik, Sound, Welten Editor, Gebäude-Planer, Monarch und Gegner.

## Design
- `main.tsx`: mountet `App` und lädt die Stile (Radix-Theme, `@webkit/theme.css`, `styles/*.css`).
- `App.tsx`: Rahmen mit Kopfzeile und Tabs (`PAGES`: grafik, sound, welt, planer, monarch, gegner; Seiten-IDs deutsch, Ordner englisch), gemerkte Seite unter `k3c-studio.app.page`, Hinweis „Mock“ bei `backend.mock`; nur die offene Seite ist eingehängt (Sound stoppt beim Verlassen). Rechts neben jeder Seite steht `chat/ChatPanel` (Knopf `ChatToggle` in der Kopfzeile).
- Seiten: `graphics/`, `sound/` (beide auf `assets/`), `world/`, `planner/`, `monarch/`, `enemies/` (beide auf `playback/`); `chat/` (Chat rechts, B-488), gemeinsam `stage/` (Phaser-Bühne), `api/` (HTTP-Backend und Mock), `lib/` (`prefs.ts`: bindet `createPrefs` aus `@webkit/prefs` an `k3c-studio.`), `ui/` (`Splitter.tsx`), `styles/` (`app.css`, `assets.css`, `szenen.css`, `world.css`, `planer.css`).
- Spielcode nur lesend über die Aliase `@game` (`src/`) und `@data` (`data/`); geteilte UI-Teile über `@webkit` (`tools/webkit/`: `errors.ts`, `useDebounced.ts`, `parts.tsx`, `theme.css`, `prefs.ts`); Test `App.test.tsx`.

## Flow
Start → `App` liest die gemerkte Seite → Seite lädt über `backend` (HTTP oder Mock) → Änderungen gehen als `POST /api/studio/rpc/<Methode>` mit `stand` an das Go-Backend `tools/k3c-studio/backend`.

## Integration
- Backend: `tools/k3c-studio/backend/` (Go-Paket, Routen unter `/api/studio/`); Build und Aliase (`ALIAS`, `DEDUPE`): `../vite.config.ts`, `../paths.ts`; Ausgabe `frontend/dist/`.
