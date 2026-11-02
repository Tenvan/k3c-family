# tools/k3c-studio/frontend/src/

## Responsibility
React-Oberfläche des Asset-Studios: Rahmen mit Seitenwahl und die Seiten Assets, Welten Editor, Gebäude-Planer, Monarch und Gegner.

## Design
- `main.tsx`: mountet `App` und lädt die Stile (Radix-Theme, `@webkit/theme.css`, `styles/*.css`).
- `App.tsx`: Rahmen über `AppFrame` aus `@webkit/templates` (Titel, `appearance="dark"`, `fixed`: nur die Seite scrollt) mit Reitern (`PAGES`: assets, welt, planer, monarch, gegner; Seiten-IDs deutsch, Ordner englisch), gemerkte Seite unter `k3c-studio.app.page` (Vorgabe `assets`), Hinweis „Mock“ bei `backend.mock`; nur die offene Seite ist eingehängt (die Seite Assets stoppt beim Verlassen alle Busse). Rechts neben jeder Seite steht `chat/ChatPanel` (`aside`; Knopf `ChatToggle` rechts in der Kopfzeile).
- Seiten: `assetpage/` (Seite Assets, B-677; baut auf `assets/`, `graphics/`, `sound/` und `sources/`), `world/`, `planner/`, `monarch/`, `enemies/` (beide auf `playback/`); `chat/` (Chat rechts, B-488), gemeinsam `stage/` (Phaser-Bühne), `api/` (HTTP-Backend und Mock), `lib/` (`prefs.ts`: bindet `createPrefs` aus `@webkit/lib/prefs` an `k3c-studio.`; `license.ts`: Lizenz-Marke; zugleich Ablage für `useSplit` aus `@webkit/lib/split`), `ui/` (nur `molecules/`, `organisms/`, `templates/`, derzeit leer), `styles/` (`app.css`, `assets.css`, `assetpage.css`, `assetInspector.css`, `assetTools.css`, `szenen.css`, `sources.css`, `world.css`, `planer.css`, `figures.css`).
- Spielcode nur lesend über die Aliase `@game` (`src/`) und `@data` (`data/`); Bedienelemente nur über die Atomic-Design-Ebenen `@webkit/atoms|molecules|organisms|templates` (Radix Themes nie direkt, `tests/atomicDesign.test.ts`), dazu `@webkit/lib/*` und `@webkit/theme.css`; Test `App.test.tsx` (Tests mit Auswahllisten nutzen `tools/webkit/testing/select.ts` und `options.ts`).

## Flow
Start → `App` liest die gemerkte Seite → Seite lädt über `backend` (HTTP oder Mock) → Änderungen gehen als `POST /api/studio/rpc/<Methode>` mit `stand` an das Go-Backend `tools/k3c-studio/backend`.

## Integration
- Backend: `tools/k3c-studio/backend/` (Go-Paket, Routen unter `/api/studio/`); Build und Aliase (`ALIAS`, `DEDUPE`): `../vite.config.ts`, `../paths.ts`; Ausgabe `frontend/dist/`.
