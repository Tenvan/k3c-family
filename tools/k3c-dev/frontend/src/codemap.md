# tools/k3c-dev/frontend/src/

## Responsibility

React-Single-Page-App von k3c-dev: Shell mit vier Reitern (Dienste & Logs, Tasks, Planung, MCP), Verweis „Asset-Studio öffnen“ (B-459), Theme-Umschaltung, Agenten-Hinweisen und Anbindung an das Backend über die `api`-Schicht.

## Design

- `main.tsx`: Einstieg; lädt `@radix-ui/themes/styles.css`, `@webkit/theme.css` und `styles/*.css` (`app`, `services`, `logs`, `mcp`, `tasks`, `planning`, `planning-reader`), rendert `App` in `StrictMode`.
- `App.tsx`: Wurzel mit Radix `Theme` (dark/light, gemerkt, Vorgabe aus `prefers-color-scheme`) und `Tabs.Root`; Seite je Reiter gemerkt über `lib/prefs`; hält den MCP-Zustand (`mcp:state`, `backend.info()`); rendert unterhalb der Seiten `ui/Notices`.
- `Header.tsx`: Reiterleiste `PAGES = dienste|tasks|planung|mcp`, `StudioLink`, MCP-Badge, `Mock`-Badge, Dark-Switch.
- `StudioLink.tsx`: Knopf „Asset-Studio öffnen“ -> `backend.studioOpen()` (startet die EXE `bin/k3c-studio`); zeigt den Hinweis aus Go (z. B. „bin/k3c-studio fehlt“) als Badge, bis der nächste Versuch gelingt.
- Unterordner (je mit eigener codemap):
  - `api/` – Backend-Vertrag (`types.ts`), Wails-Adapter und Mock.
  - `lib/` – Formatierung, Prefs (Präfix `k3c-dev.`), `useNow`; Gemeinsames in `tools/webkit` (`@webkit/…`).
  - `ui/` – `MarkdownView`, `Notices`; die Radix-Bausteine (`StatusBadge`, `ActionButton` …) aus `@webkit/parts`.
  - `services/`, `tasks/`, `planning/`, `mcp/` – je eine Seite; `logs/` – Logs-Bereich der Dienste-Seite, Konsole auch von `tasks/` genutzt.
  - `styles/` – CSS je Seite (keine Logik).
- Muster: Feature-Ordner mit pure Logik-Dateien (Vitest) getrennt von Komponenten und Data-Hooks.

## Flow

1. `main.tsx` -> `App`; `backend` (aus `api/index.ts`) wird beim Import gewählt: Wails oder Mock.
2. `App` abonniert `mcp:state`, lädt `backend.info()` und reicht Zustand an `Header`.
3. Reiterwechsel (`Tabs.onValueChange`) speichert die Seite; die jeweilige Seite lädt ihre Daten selbst über `backend` und Events.
4. Ein Agent-Hinweis (`ui:notify`) erscheint unabhängig vom Reiter als Toast.

## Integration

- Abhängigkeiten: `react`, `react-dom`, `@radix-ui/themes`; Backend ausschließlich über `api/`.
- Laufzeit: Wails-Fenster (Go-App `tools/k3c-dev/app.go`, Bindings u. a. `app_studio.go`) oder Browser mit Mock (`npx vite`).
- Tests: eigenes Vitest (`../vitest.config.ts`, `task check:dev`), z. B. `StudioLink.test.tsx`.
