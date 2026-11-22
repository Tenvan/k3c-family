# tools/k3c-workbench/frontend/src/

## Responsibility

React-Single-Page-App von k3c-workbench: Shell mit einer Reiterleiste (Dienste & Logs, Tasks, Planung, Git, MCP, Monitoring; Welten, Assets und Gebäude-Planer sind seit WZ63.3, Balancing seit WZ63.6 im Studio `tools/k3c-studio`) –, Theme-Umschaltung, Ticket-Knopf, Agenten-Hinweisen und Anbindung an das Backend über die `api`-Schicht.

## Design

- `main.tsx`: Einstieg; lädt `@radix-ui/themes/styles.css`, `@webkit/theme.css` und die Stile aus `styles/` (`app`, `services`, `logs`, `mcp`, `mcpCharts`, `monitoring`, `tasks`, `planning`, `planning-reader`, `git`, `balancing`, `balancing-editor`), rendert `App` in `StrictMode`.
- `App.tsx`: Wurzel über `AppFrame` (`@webkit/templates`) in einem `TicketProvider` (`@webkit/lib/ticketContext`, `backend.ticketCreate`/`ticketRecent`, Vorgabe Domäne DEV, Projekt WZG); Titel „K3C Workbench“, Reiter aus `useNavigation`, Farbmodus dark/light (gemerkt `app.mode`, Vorgabe aus `prefers-color-scheme`); hält den MCP-Zustand (`mcp:state`, `backend.info()` samt Version); `ui/organisms/Notices` als `overlay`. Dienste, Tasks, Git, Monitoring und Balancing bleiben eingehängt (`forceMount`, `active`-Prop), Planung, MCP, Welten und Planer bauen sich bei jedem Wechsel neu auf.
- `Header.tsx`: `PAGES`/`LABELS` (sieben Seiten); `useNavigation` merkt `app.page` und liefert die Reiter; `useNavigateRequests` (`@webkit/lib/navigate`) nimmt Seitenwechsel auf Wunsch einer Seite an (z. B. Welten-Editor -> Katalog, nur Studio-Seiten) und schaltet dazu den Bereich um; `HeaderRight` (Bereichsumschalter `ChipSwitch`, MCP-Badge, `Mock`-Badge, Dark-Schalter).
- Unterordner (je mit eigener codemap):
  - `api/` – Backend-Vertrag (`types*.ts`), Wails-, HTTP-Adapter und Mock.
  - `lib/` – Formatierung, Prefs (Präfix `k3c-workbench.`), Markdown-Parser, `liveReload`/`useLiveReload`, `useNow`; Gemeinsames in `tools/webkit` (`@webkit/…`).
  - `ui/organisms/` (`Notices`; `MarkdownView` und `Confirm` liegen seit WZ63.6 in `@webkit`); Radix und die Atome nur über `@webkit/atoms|molecules|organisms|templates`.
  - Seiten Development: `services/`, `logs/` (Logs-Bereich der Dienste-Seite, Konsole auch von `tasks/` und `mcp/` genutzt), `tasks/`, `planning/`, `git/`, `mcp/`, `monitoring/`.
  - Die früheren Studio-Seiten (`assets/`, `world/`, `planner/`, seit WZ63.6 auch `balancing/`) liegen in `tools/k3c-studio/frontend/src`.
  - `styles/` – CSS je Seite (keine Logik).
- Muster: Feature-Ordner mit reiner Logik in eigenen Dateien (Vitest) getrennt von Komponenten und Data-Hooks; Live-Daten über Ereignisse statt Abfrage-Takt, gedrosselt über `lib/liveReload.ts`.

## Flow

1. `main.tsx` -> `App`; `backend` (aus `api/index.ts`) wird beim Import gewählt: Wails, HTTP (`k3c-workbench -dev`) oder Mock (Test, `?mock=1`).
2. `App` abonniert `mcp:state`, lädt `backend.info()` und reicht Zustand an `HeaderRight`.
3. Bereich- oder Reiterwechsel (`HeaderRight.onArea`, `AppFrame` › `tabs.onChange`) speichert die Wahl; die jeweilige Seite lädt ihre Daten selbst über `backend` und Events und meldet sich per `useTicketPage` beim Ticket-Kontext.
4. Ein Agent-Hinweis (`ui:notify`) erscheint unabhängig vom Reiter als Toast.

## Integration

- Abhängigkeiten: `react`, `react-dom`, `tools/webkit` (Radix Themes nur darüber, `main.tsx` lädt `@radix-ui/themes/styles.css`); Backend ausschließlich über `api/`.
- Laufzeit: Wails-Fenster (Go-App `tools/k3c-workbench/app.go` mit `app_*.go`) oder Browser über `task k3c-workbench:dev` (HTTP zu `k3c-workbench -dev`), mit `?mock=1` gegen den Mock.
- Tests: eigenes Vitest (`../vitest.config.ts`, `task check:dev`), z. B. `App.test.tsx`.
