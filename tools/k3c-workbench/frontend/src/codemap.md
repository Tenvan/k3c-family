# tools/k3c-workbench/frontend/src/

## Responsibility

React-Single-Page-App von k3c-workbench: Shell mit vier Reitern (Dienste & Logs, Tasks, Planung, MCP), Verweis „Asset-Studio öffnen“ (B-459), Theme-Umschaltung, Agenten-Hinweisen und Anbindung an das Backend über die `api`-Schicht.

## Design

- `main.tsx`: Einstieg; lädt `@radix-ui/themes/styles.css`, `@webkit/theme.css` und `styles/*.css` (`app`, `services`, `logs`, `mcp`, `tasks`, `planning`, `planning-reader`), rendert `App` in `StrictMode`.
- `App.tsx`: Wurzel über `AppFrame` (`@webkit/templates`): Titel „K3C Workbench“, Reiter aus `PAGES`/`LABELS` als `Tabs.Content`, Farbmodus dark/light (gemerkt `app.mode`, Vorgabe aus `prefers-color-scheme`), Seite gemerkt (`app.page`) über `lib/prefs`; hält den MCP-Zustand (`mcp:state`, `backend.info()`); `ui/organisms/Notices` als `overlay`.
- `Header.tsx`: `PAGES = dienste|tasks|planung|mcp`, `LABELS` und `HeaderRight` (rechte Leiste von AppFrame: `StudioLink`, MCP-Badge, `Mock`-Badge, Dark-Schalter `SwitchField`).
- `StudioLink.tsx`: Knopf „Asset-Studio öffnen“ -> `backend.studioOpen()` (startet die EXE `bin/k3c-studio`); zeigt den Hinweis aus Go (z. B. „bin/k3c-studio fehlt“) als Badge, bis der nächste Versuch gelingt.
- Unterordner (je mit eigener codemap):
  - `api/` – Backend-Vertrag (`types.ts`), Wails-, HTTP-Adapter und Mock.
  - `lib/` – Formatierung, Prefs (Präfix `k3c-workbench.`, `prefsStore` für `useSplit`), Markdown-Parser, `useNow`; Gemeinsames in `tools/webkit` (`@webkit/…`).
  - `ui/` – `molecules/MarkdownView`, `organisms/Notices`; Radix und die Atome (`StatusBadge`, `ActionButton`, `Splitter` …) nur über `@webkit/atoms|molecules|organisms|templates`.
  - `services/`, `tasks/`, `planning/`, `mcp/` – je eine Seite; `logs/` – Logs-Bereich der Dienste-Seite, Konsole auch von `tasks/` genutzt.
  - `styles/` – CSS je Seite (keine Logik).
- Muster: Feature-Ordner mit pure Logik-Dateien (Vitest) getrennt von Komponenten und Data-Hooks.

## Flow

1. `main.tsx` -> `App`; `backend` (aus `api/index.ts`) wird beim Import gewählt: Wails, HTTP (`k3c-workbench -dev`) oder Mock (Test, `?mock=1`).
2. `App` abonniert `mcp:state`, lädt `backend.info()` und reicht Zustand an `HeaderRight`.
3. Reiterwechsel (`AppFrame` › `tabs.onChange`) speichert die Seite; die jeweilige Seite lädt ihre Daten selbst über `backend` und Events.
4. Ein Agent-Hinweis (`ui:notify`) erscheint unabhängig vom Reiter als Toast.

## Integration

- Abhängigkeiten: `react`, `react-dom`, `tools/webkit` (Radix Themes nur darüber, `main.tsx` lädt `@radix-ui/themes/styles.css`); Backend ausschließlich über `api/`.
- Laufzeit: Wails-Fenster (Go-App `tools/k3c-workbench/app.go`, Bindings u. a. `app_studio.go`) oder Browser über `task k3c-workbench:dev` (HTTP zu `k3c-workbench -dev`), mit `?mock=1` gegen den Mock.
- Tests: eigenes Vitest (`../vitest.config.ts`, `task check:dev`), z. B. `StudioLink.test.tsx`.
