# tools/k3c-studio/frontend/src/

## Responsibility
React-Oberfläche des Asset-Studios: Rahmen mit Seitenwahl und die Seiten Grafik, Sound, Welten Editor, Gebäude-Planer, Monarch und Gegner.

## Design
- `main.tsx` mountet `App`; `App.tsx`: Kopfzeile mit Tabs (`PAGES`: grafik, sound, welt, planer, monarch, gegner), gemerkte Seite unter `k3c-studio.app.page`, Mock-Hinweis ohne Wails; nur die offene Seite ist eingehängt (Sound stoppt beim Verlassen).
- Seiten: `grafik/`, `sound/` (auf `assets/`), `world/`, `planer/`, `monarch/`, `gegner/` (auf `abspiel/`); gemeinsam `stage/` (Phaser-Bühne), `api/` (Backend und Mock), `lib/`, `ui/` (`parts.tsx`, `Splitter.tsx`), `styles/` (`theme.css`, `app.css`, `assets.css`, `szenen.css`, `world.css`, `planer.css`).
- Spielcode nur lesend über die Aliase `@game` (`src/`) und `@data` (`data/`); Test `App.test.tsx`.

## Flow
Start → `App` liest die gemerkte Seite → Seite lädt über `backend` (Wails oder Mock) → Änderungen gehen als Binding-Aufruf mit `stand` ans Go-Backend.

## Integration
- Backend: `tools/k3c-studio/app*.go`; Build und Aliase: `../vite.config.ts`, `../paths.ts`.
