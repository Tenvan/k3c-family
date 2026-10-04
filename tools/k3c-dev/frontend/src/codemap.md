# tools/k3c-dev/frontend/src/

## Responsibility

React-Single-Page-App von k3c-dev: Shell mit vier Reitern (Dienste, Tasks, Planung, MCP), Theme-Umschaltung und Anbindung an das Backend über die `api`-Schicht.

## Design

- `main.tsx`: Einstieg; lädt `@radix-ui/themes/styles.css` und `styles/*.css` (`theme`, `app`, `services`, `logs`, `mcp`, `tasks`, `planning`), rendert `App` in `StrictMode`.
- `App.tsx`: Wurzel mit Radix `Theme` (dark/light, gemerkt, Vorgabe aus `prefers-color-scheme`) und `Tabs.Root`; Seite je Reiter gemerkt über `lib/prefs`; hält den MCP-Zustand (`mcp:state`, `backend.info()`).
- `Header.tsx`: Reiterleiste `PAGES = dienste|tasks|planung|mcp`, MCP-Badge, `Mock`-Badge, Dark-Switch.
- Unterordner (je mit eigener codemap):
  - `api/` – Backend-Vertrag, Wails-Adapter und Mock.
  - `lib/` – Formatierung, Prefs, Hooks.
  - `ui/` – gemeinsame Bausteine (`StatusBadge`, `ActionButton`, `MarkdownView`).
  - `services/`, `tasks/`, `planning/`, `mcp/` – je eine Seite; `logs/` – Logs-Bereich der Dienste-Seite, Konsole auch von `tasks/` genutzt.
  - `styles/` – CSS je Seite (keine Logik).
- Muster: Feature-Ordner mit pure Logik-Dateien (Vitest) getrennt von Komponenten und Data-Hooks.

## Flow

1. `main.tsx` -> `App`; `backend` (aus `api/index.ts`) wird beim Import gewählt: Wails oder Mock.
2. `App` abonniert `mcp:state`, lädt `backend.info()` und reicht Zustand an `Header`.
3. Reiterwechsel (`Tabs.onValueChange`) speichert die Seite; die jeweilige Seite lädt ihre Daten selbst über `backend` und Events.

## Integration

- Abhängigkeiten: `react`, `react-dom`, `@radix-ui/themes`; Backend ausschließlich über `api/`.
- Laufzeit: Wails-Fenster (Go-App `tools/k3c-dev/app.go`) oder Browser mit Mock (`npx vite`).
- Tests: Vitest des Hauptprojekts (siehe `tsconfig.json` des Frontends).
