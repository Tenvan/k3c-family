# tools/k3c-dev/internal/mcpsrv/

## Responsibility

MCP-Server von k3c-dev (Streamable HTTP, nur 127.0.0.1): registriert den Tool-Katalog, löst pro Aufruf den Worktree-Checkout des Aufrufers auf und führt Prüfläufe, Logabfragen, Dienste, Planung und Engine-Werkzeuge aus. Zählt Aufrufe für die Oberfläche (Stats).

## Design

- Facade `Server` (`server.go`): `Config`, `New`, `Start`/`Stop`/`Restart`, `Stats()`, `Calls()`, `Checks()`; `instructions.md` wird per `go:embed` als MCP-Instructions ausgeliefert.
- Generischer Tool-Helper `add[In]` (`tools.go`) kapselt `mcp.AddTool`, Fehler-/Textergebnis und Annotations (`readOnly()`). Registrierung nach Gruppen: `tools.go` (workbench_status, check_run, console_tail, logs_*, reports_list, report_read, saves_list), `tools_logs.go`, `tools_console.go`, `tools_engine.go`/`tools_gamedata.go`/`tools_replay.go` (level_generate, sim_run, replay_run), `tools_server.go` (server_status, rooms_list, room_snapshot), `tools_services.go` (svc_*), `tools_planning.go` (plan_*, gh_status), `tools_status.go`.
- Middleware `observe.go` (`Server.observe`): misst jeden Tool-Call; `param_hint.go` ergänzt Parameterhinweise bei Fehlern; `stats.go` hält Zähler und Ringpuffer (`ToolStats`, `Call`, `Snapshot`).
- Check-Pipeline: `check_targets.go` (Whitelist `checkTarget`, `findTarget`), `check_run.go` (`checkRuns` mit Einzelflug-Sperre je Ziel, `runProcess`), `check_compact.go` (Einzeiler bei Grün, sonst nur Fehlerzeilen), `check_state.go` (`CheckState` für die UI).
- Workspace-Scoping `workspace.go`: `workspace`, `resolveWorkspace` (Header `X-K3C-Root` bzw. MCP-roots), `FindRoot`, `isWorktreeOf`; `checkout.go`: Argument `checkout` der schreibenden Tools und `check_run` (B-388), gilt vor dem Header, Name über `git worktree list`, sonst absoluter Pfad; `worktree_services.go`: je Worktree eigener `services.Controller` mit Port-Versatz (`freeOffset`).
- `chunked_writer.go`: `forceChunked` HTTP-Middleware (Streaming ohne Content-Length).

## Flow

1. `New(cfg)` baut `mcp.Server`, `register(s)` hängt alle Tools ein, `observe` wird als Middleware gesetzt; `Start()` lauscht auf 127.0.0.1.
2. Tool-Call: `observe` -> `stats.begin` -> `resolveWorkspace` (Checkout aus Header/roots, Argument `checkout` vorrangig) -> Handler -> `stats.end`, Log über `applog`.
3. `check_run`: `findTarget` -> `checkRuns.begin` -> `runProcess` (`proc.Command`, Ausgabe in `console.Store`, Quelle `check:<ziel>`) -> `report` -> `notifyCheck`.
4. `svc_*`/`server_status`: `Server.controller(ctx)` liefert den `services.Controller` des Worktrees, `serverClient` den `serverapi.Client` mit versetztem Port.
5. `plan_*`: Delegation an `planning.Create/Set/Section/Delete/List/Get` mit dem Workspace-Root.

## Integration

- Konsumenten: Wails-`App` (`app.go`, `app_logs.go`, `app_mcp.go`: `McpOverview`, `McpRestart`, `McpCalls`, `McpUsage`) und `main.go`.
- Abhängigkeiten (intern): `console`, `applog`, `proc`, `services`, `serverapi`, `planning`, `logs`, `taskcat`/`taskrun`, `enginetools`, `balance`, `gamedata`, `github`, `usage`.
- Extern: `github.com/modelcontextprotocol/go-sdk/mcp`; Client: Claude Code über `.mcp.json`.
