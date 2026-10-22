# tools/k3c-workbench/

## Responsibility

Wails-Desktop-Entwicklerwerkzeug für K3C: hostet den MCP-Server (Prüfläufe, Logs, Dienste, Planung, Engine-Werkzeuge) und eine Oberfläche dafür. Das Root-Package `main` verdrahtet Dienste, Logs und Frontend-Bindings.

## Design

- Composition Root `main.go`: `run()` baut `console.Store`, `applog.Log`, Services (`openServices`) und `App`; `windowOptions` konfiguriert Wails; Frontend per `go:embed all:frontend/dist`.
- Wails-Binding-Fassade `App` (`app.go`): Lifecycle `startup`/`shutdown`/`beforeClose`/`secondInstance`, `Info`, `MCPState`. Methoden nach Thema: `app_services.go` (Dienste inkl. `ServicesReload`: `services.json` neu lesen, `Controller.Reload`), `app_logs.go` (`Sources`, `ConsoleTail`), `app_logview.go` (`LogsQuery`, `LogsErrors`), `app_mcp.go` (`McpOverview`, `McpRestart`, `McpCalls`, `McpUsage`), `app_planning.go` (`PlanningData`, `PlanningDocs`, `PlanningDoc`, `PlanningGet` = `plan_get` für den Lesemodus, `PlanningSet` = `planning.Set` für Kopf-Felder, `GitHubStatus`, `WorktreeCheck` vergleicht die Planung aller Worktrees mit der Wurzel, nur lesend; Wächter `planningPoll` -> `planning:changed`), `app_tasks.go` (`Tasks`, `TasksReload`, `TaskStart`, `TaskStop`, `TaskRuns`; Freigabe-Schlösser über `taskgate`: `TaskGatesView`, `TaskGateToggle`, `TaskGateSave`, für MCP `TaskList`/`TaskState`), `app_claude.go` (`OpenInClaude`: baut den Deep Link `claude://code/new?q=…&folder=<Repo-Wurzel>` – ab 14.000 Zeichen Prompt oder 32.000 Zeichen Link abgelehnt – und startet ihn über `rundll32`, losgelöst vom Job Object; Claude Desktop füllt nur das Eingabefeld), `app_studio.go` (`StudioOpen`: startet `bin/k3c-studio` losgelöst, ohne EXE Fehler `errStudioNotBuilt`; B-459. Die Assets-Seite lebt seit WZ6 im Asset-Studio, die MCP-Tools `assets_*` nutzen `tools/assetkit` direkt).
- Client-Server-Betrieb ohne Fenster (`headless.go`): `-headless` startet `App` mit einfachem ctx (MCP auf `K3C_DEV_PORT`) und die Oberflächen-API auf `127.0.0.1:K3C_DEV_UI_API_PORT` (Vorgabe 8191, `devrun.Listen`), Ende bei Strg+C/SIGTERM; `-dev` baut `-headless` per `engine/devrun` als `build/bin/k3c-workbench-devchild-<UI-Port>(.exe)` (je Lauf eigene EXE) (normales `go build`, Konsolen-Subsystem für CTRL_BREAK) und startet es bei Go-Änderungen in `.`, `../assetkit`, `../../engine`, `../../data` und `go.mod`/`go.sum` des Hauptmoduls neu (JSON in `data/` löst keinen Neubau aus) (MCP-Port ohne `K3C_DEV_PORT`: 5190, nie der der Workbench; Statistik `mcp-usage-dev.json`). Ohne Wails keine Einzelinstanz-Sperre.
- Oberflächen-API (`uiapi.go`, `uiapi_events.go`): `POST /api/dev/rpc/{Method}` (JSON-Array → `{result}`/`{error}`, nur die Methoden von `GoApp` aus `frontend/src/api/wails.ts`, `uiMethods`; ein Test hält beide gleich), `GET /api/dev/events` als Server-Sent Events; Wächter: nur Loopback, Host nur localhost/Loopback-IP, Origin gleich Host und `Sec-Fetch-Site` same-origin/none (`uiSameSite`, keine andere lokale Seite), POST nur mit `X-K3C-Dev: 1`, kein kodierter Pfad. `emitAll` (Produktion von `emit`) schickt an die Wails-Laufzeit nur im Fenster (`App.window`) und immer an den `eventBus`.
- `window.go`: Fensterposition/-größe persistiert (`windowState`, `loadWindow`, `saveWindow`, `placeWindow`).
- `screen_windows.go`/`screen_other.go`: `onScreen` prüft beim Start, ob die gemerkte Position auf einem angeschlossenen Bildschirm liegt (Windows: Punkt der Titelleiste, 100 px vom linken Rand; sonst immer wahr).
- Ereignisse an die Oberfläche (`app.go`): `mcp:state`, `service:state`, `source:state`, `console:line`, `mcp:start`, `mcp:call`, `task:state`, `ui:notify` (Hinweis eines Agenten über `notify_ui`), `planning:changed`.
- Konfiguration: `services.json` (Vite, Spielserver), `wails.json`, `go.mod` (Modul `k3c/tools/k3c-workbench`).
- Unterordner: `frontend/` (Oberfläche, Wails-Bindings in `frontend/wailsjs`), `internal/` (`mcpsrv`, `planning`, `services`, `proc`, `serverapi`, `applog`, `console`, `logs`, `taskcat`, `taskgate`, `taskrun`, `github`, `usage`, `balance`, `enginetools`, `gamedata`, `botdev`, `botfeed`, `browser`), `build/`. Die Asset-Logik liegt im gemeinsamen Modul `tools/assetkit` (B-459).

## Flow

1. `main` -> `newApp(root, port)` -> Wails `Run` (bzw. `-headless`: `runHeadless`, `-dev`: `runDev`); `startup` öffnet Log, Console-Store und Services und startet `mcpsrv.Server` (`setMCP` meldet den Status).
2. Das Frontend ruft `App`-Methoden (Wails-Binding oder `/api/dev/rpc`); Live-Zeilen kommen per Event (Wails bzw. SSE) aus `console.Store`.
3. Claude Code spricht über `.mcp.json` mit dem MCP-Server auf 127.0.0.1.
4. `beforeClose`/`shutdown`: Dienste und Worktree-Dienste stoppen, MCP stoppen.

## Integration

- Abhängigkeiten: alle Pakete unter `internal/`; Wails-Runtime; `task`-Katalog aus `Taskfile.yml` (`taskcat`).
- Konsumenten: Entwickler (Fenster `task k3c-workbench:window`/`task k3c-workbench:build`, Browser `task k3c-workbench` = `dev:server` + `dev:ui`), MCP-Clients (Tools `check_run`, `logs_*`, `svc_*`, `plan_*`, `sim_run` …).
