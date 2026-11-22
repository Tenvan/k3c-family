# tools/k3c-workbench/

## Responsibility

Wails-Desktop-Entwicklerwerkzeug für K3C: hostet den MCP-Server (Prüfläufe, Logs, Dienste, Tasks, Planung, Balancing/Simulation) und eine Oberfläche mit den Seiten Dienste, Tasks, Planung, Git, MCP, Monitoring und Balancing. Welten, Assets und Hub liegen seit WZ63 im Studio (`tools/k3c-studio`). Das Root-Package `main` verdrahtet die Packages unter `internal/` und stellt sie als Wails-Bindings bzw. HTTP-RPC bereit.

## Design

- Composition Root `main.go`: `run()` wertet `-headless`/`-dev` aus, baut `App` (`newApp`) und startet Wails mit `windowOptions` (gemerkte Größe über `tools/assetkit/windowstate`, Einzelinstanz-Sperre mit `secondInstance`); Frontend per `go:embed all:frontend/dist`; `openServices` lädt `services.json`, übernimmt laufende Dienste und misst sie.
- Wails-Binding-Fassade `App` (`app.go`): Lebenszyklus `startup`/`shutdown`/`beforeClose` (Fenster merken)/`secondInstance`, `Info`, `setMCP`; Ereignisse `mcp:state`, `service:state`, `source:state`, `console:line` (gebündelt in `console_batch.go`, B-895), `mcp:start`, `mcp:call`, `task:state`, `ui:notify`, `planning:changed` (Nutzlast `planning.Changed` mit den IDs der geänderten Dokumente, Wächter `planning.WatchIDs`).
- Bindings nach Seite:
  - `app_services.go` (Dienste, `ServicesReload`), `app_logs.go` (`Sources`, `ConsoleTail`), `app_logview.go` (`LogsQuery`, `LogsErrors`), `app_mcp.go` (`McpOverview`, `McpRestart`, `McpInstructions`, `McpCalls`, `McpUsage`), `app_tasks.go` (Tasks und Läufe ohne Freigabe-Schloss; erfüllt `mcpsrv.TaskHost`).
  - `app_planning.go` (`PlanningData`, `PlanningDocs`, `PlanningDoc`, `PlanningGet`, `PlanningSet`, `PlanningList` und `PlanningItems` (Listen ohne Fließtext, Texte je ID nachladen), `GitHubStatus`, `WorktreeCheck`; Wächter `planningPoll`), `app_ticket.go` (`TicketCreate`, `TicketRecent` über `tools/assetkit/ticket`), `app_git.go` (Seite Git über `internal/gitops` und `internal/github`), `app_claude.go` (`OpenInClaude`: Deep Link `claude://` mit vorbefülltem Prompt).
  - `app_resources.go` (Seite Monitoring, `Resources(windowMin)`).
- Client-Server-Betrieb (`headless.go`): `-headless` startet `App` ohne Fenster (MCP-Port, Oberflächen-API auf `127.0.0.1:K3C_DEV_UI_API_PORT`); `-dev` baut sich per `engine/devrun` als `build/bin/k3c-workbench-devchild-<UI-Port>` und startet bei Go-Änderungen in `.`, `../assetkit`, `../../engine`, `../../data` neu (MCP-Port Vorgabe 5190).
- Oberflächen-API (`uiapi.go`, `uiapi_events.go`): `POST /api/dev/rpc/{Method}` nur für `uiMethods` (gleich `GoApp` in `frontend/src/api/wails.ts`), `GET /api/dev/events` als Server-Sent Events über `eventBus`; Wächter Loopback, Host, `uiSameSite`, Kopf `X-K3C-Dev`. `emitAll` schickt an Wails (nur mit Fenster) und an den `eventBus`.
- Ressourcen-Monitoring (`resmon_start.go`): `internal/resmon` mit Proben alle 10 s, Verlauf 24 h, Profile unter `logs/profile/` (Schwelle `K3C_DEV_HEAP_DUMP_MB`); `Track` je UI-Methode in `uiapi.go`.
- Fenster-Icon zur Laufzeit: `winicon_windows.go` (Windows), `winicon_other.go` (leer).
- Konfiguration: `services.json` (derzeit leere Liste), `wails.json`, `Taskfile.yml` (Namensraum `k3c-workbench`: `dev`, `dev:server`, `dev:ui`, `dev:worktree` (eigene Dev-Instanz im Worktree mit Ports aus dem Pfad, `K3C_DATA_DIR` auf die `data/` des Worktrees), `window`, `frontend`, `test`, `vitest`, `gotest`, `lint`, `build`, `dist`), `go.mod` (Modul `k3c/tools/k3c-workbench`).
- Unterordner: `frontend/` (React-Oberfläche, Wails-Bindings in `frontend/wailsjs`), `internal/` (Packages, siehe `internal/codemap.md`), `build/`, `icon/`.

## Flow

1. `main` → `run` → `newApp(root, port)` → Wails `Run` (bzw. `runHeadless`/`runDev`); `startup` öffnet Log (`applog`), Console-Store und Dienste, startet `resmon` und `mcpsrv.Server` (`setMCP` meldet den Status).
2. Das Frontend ruft `App`-Methoden (Wails-Binding oder `/api/dev/rpc`); Live-Daten kommen per Event (Wails bzw. SSE).
3. Claude Code spricht über `.mcp.json` mit dem MCP-Server auf 127.0.0.1; `mcpsrv` nutzt dieselben Packages wie die Bindings.
4. `shutdown`: Task-Läufe stoppen, Dienste und Worktree-Dienste (`StopWorktreeServices`) und MCP stoppen.

## Integration

- Abhängigkeiten: alle Pakete unter `internal/`; `tools/assetkit` (`windowstate`, `ticket`); `tools/workkit` (`mcpkit`, `applog`, `console`, `proc`, `usage`, `uikit`, `claude`, `winicon`; importiert die Workbench nie); `engine/devrun`; Wails-Runtime; `task`-Katalog aus der Wurzel-`Taskfile.yml` (`taskcat`).
- Konsumenten: Entwickler (Fenster `task k3c-workbench:window`/`task k3c-workbench:build`, Browser `task k3c-workbench` = `dev:server` + `dev:ui`), MCP-Clients (Tools `check_run`, `logs_*`, `svc_*`, `task_*`, `plan_*`, `sim_test` …; Welten, Assets, Hub: MCP-Server `k3c-studio`).
