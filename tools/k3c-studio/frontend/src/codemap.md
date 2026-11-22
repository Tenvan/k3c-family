# tools/k3c-studio/frontend/src/

## Responsibility

React-Single-Page-App von k3c-studio (WZ63.3): Shell mit den drei Bereichen **Welten** (Welten-Editor), **Assets** (Review, Katalog, Styleguide, Sound & Quellen) und **Hub** (Gebäude-Planer), Theme-Umschaltung und Anbindung an das Backend über die `api`-Schicht.

## Design

- `main.tsx` (Einstieg, Stile aus `styles/`: `assets`, `world`, `planer`), `App.tsx` (Rahmen über `AppFrame` aus `@webkit/templates`), `Header.tsx` (Bereichsumschalter, MCP-Badge, Mock-Badge, Dark-Schalter).
- `api/`: Backend-Schicht (`wails.ts`, `http.ts`, `mock.ts`, `mockWorld.ts`, `mockAssets.ts`, `mockHub.ts`, Verträge `types*.ts`, `hubFixture.json`); Methodenliste gleich `uiMethods` in `uiapi.go`.
- Seiten: `world/` (Welten-Editor, B-879), `assets/` (Review, Katalog, Styleguide, Sound & Quellen; eigene Codemaps in den Unterordnern), `planner/` (Gebäude-Planer, B-923); `lib/` (`TreeBar`, `useTreeOpen`, Prefs).
- Gemeinsames in `tools/webkit` (`@webkit/…`); Radix nur darüber.

## Flow

1. `main` → `App` → Bereich → Seite; Seiten rufen nur `api/` (Wails-Binding im Fenster, RPC im Browser, Mock ohne Backend).

## Integration

- Backend: Bindings in `tools/k3c-studio/app_*.go`; Tests mit Vitest (`task k3c-studio:vitest`), Typecheck `task k3c-studio:typecheck`.
