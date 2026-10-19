# tools/k3c-dev/

## Responsibility

Wails-Desktop-Entwicklerwerkzeug für K3C: hostet den MCP-Server (Prüfläufe, Logs, Dienste, Planung, Engine-Werkzeuge) und eine Oberfläche dafür. Das Root-Package `main` verdrahtet Dienste, Logs und Frontend-Bindings.

## Design

- Composition Root `main.go`: `run()` baut `console.Store`, `applog.Log`, Services (`openServices`) und `App`; `windowOptions` konfiguriert Wails; Frontend per `go:embed all:frontend/dist`.
- Wails-Binding-Fassade `App` (`app.go`): Lifecycle `startup`/`shutdown`/`beforeClose`/`secondInstance`, `Info`, `MCPState`. Methoden nach Thema: `app_services.go` (Dienste inkl. `ServicesReload`: `services.json` neu lesen, `Controller.Reload`), `app_logs.go` (`Sources`, `ConsoleTail`), `app_logview.go` (`LogsQuery`, `LogsErrors`), `app_mcp.go` (`McpOverview`, `McpRestart`, `McpCalls`, `McpUsage`), `app_planning.go` (`PlanningData`, `PlanningDocs`, `PlanningDoc`, `PlanningGet` = `plan_get` für den Lesemodus, `PlanningSet` = `planning.Set` für Kopf-Felder, `GitHubStatus`, `WorktreeCheck` vergleicht die Planung aller Worktrees mit der Wurzel, nur lesend; Wächter `planningPoll` -> `planning:changed`), `app_tasks.go` (`Tasks`, `TasksReload`, `TaskStart`, `TaskStop`, `TaskRuns`; Freigabe-Schlösser über `taskgate`: `TaskGatesView`, `TaskGateToggle`, `TaskGateSave`, für MCP `TaskList`/`TaskState`), `app_claude.go` (`OpenInClaude`: baut den Deep Link `claude://code/new?q=…&folder=<Repo-Wurzel>` – ab 14.000 Zeichen Prompt oder 32.000 Zeichen Link abgelehnt – und startet ihn über `rundll32`, losgelöst vom Job Object; Claude Desktop füllt nur das Eingabefeld), `app_studio.go` (`StudioOpen`: öffnet `http://localhost:<Port des Spielservers aus services.json>/studio/index.html` im Browser, ohne Spielserver-Eintrag Fehler; B-459. Die Assets-Seite lebt seit WZ6 im Asset-Studio, die MCP-Tools `assets_*` nutzen `tools/assetkit` direkt).
- `window.go`: Fensterposition/-größe persistiert (`windowState`, `loadWindow`, `saveWindow`, `placeWindow`).
- `screen_windows.go`/`screen_other.go`: `onScreen` prüft beim Start, ob die gemerkte Position auf einem angeschlossenen Bildschirm liegt (Windows: Punkt der Titelleiste, 100 px vom linken Rand; sonst immer wahr).
- Ereignisse an die Oberfläche (`app.go`): `mcp:state`, `service:state`, `source:state`, `console:line`, `mcp:start`, `mcp:call`, `task:state`, `ui:notify` (Hinweis eines Agenten über `notify_ui`), `planning:changed`.
- Konfiguration: `services.json` (Vite, Spielserver), `wails.json`, `go.mod` (Modul `k3c/tools/k3c-dev`).
- Unterordner: `frontend/` (Oberfläche, Wails-Bindings in `frontend/wailsjs`), `internal/` (`mcpsrv`, `planning`, `services`, `proc`, `serverapi`, `applog`, `console`, `logs`, `taskcat`, `taskgate`, `taskrun`, `github`, `usage`, `balance`, `enginetools`, `gamedata`, `botdev`, `botfeed`, `browser`), `build/`. Die Asset-Logik liegt im gemeinsamen Modul `tools/assetkit` (B-459).

## Flow

1. `main` -> `newApp(root, port)` -> Wails `Run`; `startup` öffnet Log, Console-Store und Services und startet `mcpsrv.Server` (`setMCP` meldet den Status).
2. Das Frontend ruft `App`-Methoden (Wails-Binding); Live-Zeilen kommen per Event aus `console.Store`.
3. Claude Code spricht über `.mcp.json` mit dem MCP-Server auf 127.0.0.1.
4. `beforeClose`/`shutdown`: Dienste und Worktree-Dienste stoppen, MCP stoppen.

## Integration

- Abhängigkeiten: alle Pakete unter `internal/`; Wails-Runtime; `task`-Katalog aus `Taskfile.yml` (`taskcat`).
- Konsumenten: Entwickler (Fenster, `task k3c-dev`, `task k3c-dev:build`), MCP-Clients (Tools `check_run`, `logs_*`, `svc_*`, `plan_*`, `sim_run` …).
