# tools/k3c-dev/

## Responsibility

Wails-Desktop-Entwicklerwerkzeug für K3C: hostet den MCP-Server (Prüfläufe, Logs, Dienste, Planung, Engine-Werkzeuge) und eine Oberfläche dafür. Das Root-Package `main` verdrahtet Dienste, Logs und Frontend-Bindings.

## Design

- Composition Root `main.go`: `run()` baut `console.Store`, `applog.Log`, Services (`openServices`) und `App`; `windowOptions` konfiguriert Wails; Frontend per `go:embed all:frontend/dist`.
- Wails-Binding-Fassade `App` (`app.go`): Lifecycle `startup`/`shutdown`/`beforeClose`/`secondInstance`, `Info`, `MCPState`. Methoden nach Thema: `app_services.go` (Dienste), `app_logs.go` (`Sources`, `ConsoleTail`), `app_logview.go` (`LogsQuery`, `LogsErrors`), `app_mcp.go` (`McpOverview`, `McpRestart`, `McpCalls`, `McpUsage`), `app_planning.go` (`PlanningData`, `GitHubStatus`), `app_tasks.go` (`Tasks`, `TaskStart`, `TaskStop`, `TaskRuns`).
- `window.go`: Fensterposition/-größe persistiert (`windowState`, `loadWindow`, `saveWindow`).
- Konfiguration: `services.json` (Vite, Spielserver), `wails.json`, `go.mod` (Modul `k3c/tools/k3c-dev`).
- Unterordner: `frontend/` (Oberfläche, Wails-Bindings in `frontend/wailsjs`), `internal/` (`mcpsrv`, `planning`, `services`, `proc`, `serverapi`, `applog`, `console`, `logs`, `taskcat`, `taskrun`, `github`, `usage`, `balance`, `enginetools`, `gamedata`), `build/`.

## Flow

1. `main` -> `newApp(root, port)` -> Wails `Run`; `startup` öffnet Log, Console-Store und Services und startet `mcpsrv.Server` (`setMCP` meldet den Status).
2. Das Frontend ruft `App`-Methoden (Wails-Binding); Live-Zeilen kommen per Event aus `console.Store`.
3. Claude Code spricht über `.mcp.json` mit dem MCP-Server auf 127.0.0.1.
4. `beforeClose`/`shutdown`: Dienste und Worktree-Dienste stoppen, MCP stoppen.

## Integration

- Abhängigkeiten: alle Pakete unter `internal/`; Wails-Runtime; `task`-Katalog aus `Taskfile.yml` (`taskcat`).
- Konsumenten: Entwickler (Fenster, `task k3c-dev`, `task k3c-dev:build`), MCP-Clients (Tools `check_run`, `logs_*`, `svc_*`, `plan_*`, `sim_run` …).
