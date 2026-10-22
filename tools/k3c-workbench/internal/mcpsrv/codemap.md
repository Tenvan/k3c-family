# tools/k3c-workbench/internal/mcpsrv/

## Responsibility

MCP-Server von k3c-workbench (Streamable HTTP, nur 127.0.0.1): registriert den Tool-Katalog, löst pro Aufruf den Worktree-Checkout des Aufrufers auf und führt Prüfläufe, Logabfragen, Dienste, Planung und Engine-Werkzeuge aus. Zählt Aufrufe für die Oberfläche (Stats).

## Design

- Facade `Server` (`server.go`): `Config`, `New`, `Start`/`Stop`/`Restart`, `Stats()`, `Calls()`, `Checks()`; `instructions.md` wird per `go:embed` als MCP-Instructions ausgeliefert.
- Generischer Tool-Helper `add[In]` (`tools.go`) kapselt `mcp.AddTool`, Fehler-/Textergebnis und Annotations (`readOnly()`). Registrierung nach Gruppen: `tools.go` (workbench_status, check_run, console_tail, logs_*, reports_list, report_read, saves_list), `tools_logs.go`, `tools_console.go`, `tools_engine.go`/`tools_gamedata.go`/`tools_replay.go` (level_generate, sim_run, replay_run), `tools_server.go` (server_status, rooms_list, room_snapshot), `tools_services.go` (svc_*), `tools_planning.go` (plan_*, gh_status), `tools_assets.go` (assets_list, assets_get, assets_set, assets_catalog), `tools_assets_research.go` (assets_research, assets_hits, assets_hit_add, assets_hit_drop, assets_import), `tools_status.go`, `tools_logstats.go` (logs_stats, logs_context), `tools_notify.go` (`Notice`, notify_ui), `tools_ports.go` (svc_health, svc_start_all, svc_stop_all, ports_status, get_urls), `tools_tasks.go` (task_list, task_start, task_stop, task_status, task_output über das Interface `TaskHost`; Freigabe über `taskgate.State`).
- Middleware `observe.go` (`Server.observe`): misst jeden Tool-Call; `param_hint.go` ergänzt Parameterhinweise bei Fehlern; `stats.go` hält Zähler und Ringpuffer (`ToolStats`, `Call`, `Snapshot`).
- Check-Pipeline: `check_targets.go` (Whitelist `checkTarget`, `findTarget`), `check_run.go` (`checkRuns` mit Einzelflug-Sperre je Ziel, `runProcess`), `check_compact.go` (Einzeiler bei Grün, sonst nur Fehlerzeilen), `check_state.go` (`CheckState` für die UI), `check_job.go` (`checkJob`: wartet `check_run` höchstens `checkWait`, danach läuft der Lauf im Hintergrund weiter und `check_run {run: <ID>}` liefert das Ergebnis, B-372).
- Testläufe `sim_test` (B-348): `simtest.go` (`simRuns`-Register je Checkout, Aktionen start/status/stop/list), `simtest_spec.go` (`simTestIn` -> geprüfter `simSpec`, Wahl des Runners), `simtest_offline.go` (baut `bin/k3c-balance` im Checkout und führt `--targets` aus), `simtest_online.go` (Räume mit `botdev`-Geräten gegen den Spielserver, Messung `onlineRun`), `simtest_clients.go` (Modus mit Clients: je Client Raum, Beobachter-Gerät, `botfeed`-Feed, Browser über `browser`), `simtest_clientlog.go` (Kennzahlen der Clients aus dem Log `k3c-client`). Berichte in `reports/simtest-<id>/`.
- Workspace-Scoping `workspace.go`: `workspace`, `resolveWorkspace` (Header `X-K3C-Root` bzw. MCP-roots), `FindRoot`, `isWorktreeOf`; `checkout.go`: Argument `checkout` der schreibenden Tools und `check_run` (B-388), gilt vor dem Header, Name über `git worktree list`, sonst absoluter Pfad; `worktree_services.go`: je Worktree eigener `services.Controller` mit Port-Versatz (`freeOffset`).
- `Server` hält zusätzlich `feeds` (`botfeed.Hub`, Route `/bot/`) und die Nähte `findBrowser`/`launch`.
- `chunked_writer.go`: `forceChunked` HTTP-Middleware (Streaming ohne Content-Length).

## Flow

1. `New(cfg)` baut `mcp.Server`, `register(s)` hängt alle Tools ein, `observe` wird als Middleware gesetzt; `Start()` lauscht auf 127.0.0.1.
2. Tool-Call: `observe` -> `stats.begin` -> `resolveWorkspace` (Checkout aus Header/roots, Argument `checkout` vorrangig) -> Handler -> `stats.end`, Log über `applog`.
3. `check_run`: `findTarget` -> `checkRuns.begin` -> `runProcess` (`proc.Command`, Ausgabe in `console.Store`, Quelle `check:<ziel>`) -> `report` -> `notifyCheck`.
4. `svc_*`/`server_status`: `Server.controller(ctx)` liefert den `services.Controller` des Worktrees, `serverClient` den `serverapi.Client` mit versetztem Port.
5. `plan_*`: Delegation an `planning.Create/Set/Section/Delete/List/Get` mit dem Workspace-Root.
6. `sim_test start`: `simSpec` prüfen -> `simRunner` (offline, online, mit Clients) -> Lauf im Hintergrund, `status` zeigt Phase und Kennzahlen, `finish` schreibt den Bericht.
7. `task_*`: `Server.taskHost` -> `TaskHost` der Wails-App; `task_start` nur, wenn `TaskState(name)` freigegeben ist.

## Integration

- Konsumenten: Wails-`App` (`app.go`, `app_logs.go`, `app_mcp.go`: `McpOverview`, `McpRestart`, `McpCalls`, `McpUsage`) und `main.go`; `TaskHost` wird von `app_tasks.go` erfüllt.
- Abhängigkeiten (intern): `console`, `applog`, `proc`, `services`, `serverapi`, `planning`, `logs`, `taskcat`/`taskrun`, `enginetools`, `balance`, `botdev`, `botfeed`, `browser`, `taskgate`, `gamedata`, `github`, `usage`; `engine/net`, `engine/room`, `engine/sim`, `engine/store`.
- Extern: `k3c/tools/assetkit` (Ressourcen-Manager), `github.com/modelcontextprotocol/go-sdk/mcp`; Client: Claude Code über `.mcp.json`.
