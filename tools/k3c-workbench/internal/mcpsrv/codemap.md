# tools/k3c-workbench/internal/mcpsrv/

## Responsibility

MCP-Server von k3c-workbench (Streamable HTTP, nur 127.0.0.1): registriert den Tool-Katalog, löst pro Aufruf den Worktree-Checkout des Aufrufers auf und führt Prüfläufe, Logabfragen, Dienste, Planung und Sim-Werkzeuge aus. Welten, Assets, Hub und `level_generate` liegen seit WZ63 im MCP-Server `k3c-studio` (`tools/k3c-studio/internal/mcpsrv`); die Tool-Namen beider Server sind disjunkt (`tests/mcpToolsDisjoint.test.ts`). Das Gerüst (Server, Stats) kommt aus `tools/workkit/mcpkit` (`kit.go`). Zählt Aufrufe für die Oberfläche (Stats).

## Design

- Facade `Server` (`server.go`): `Config`, `New`, `Start`/`Stop`/`Restart`, `Stats()`, `Calls()`, `Checks()`; `instructions.md` wird per `go:embed` als MCP-Instructions ausgeliefert.
- Generischer Tool-Helper `add[In]` (`tools.go`) kapselt `mcp.AddTool`, Fehler-/Textergebnis und Annotations (`readOnly()`). Registrierung nach Gruppen: `tools.go` (workbench_status, check_run, console_tail, logs_*, reports_list, report_read, saves_list), `tools_logs.go`, `tools_console.go`, `tools_engine.go`/`tools_gamedata.go`/`tools_replay.go` (sim_run, replay_run; rechnen im Prozess über `simcli.Call` mit Timeout), `tools_services.go` (svc_*), `tools_planning.go` (plan_list, plan_get, plan_next, plan_create, plan_set, plan_section, plan_sections, plan_delete, gh_status), `tools_planning_waves.go` (plan_waves, plan_queue, plan_check), `tools_planning_index.go` (plan_index), `tools_planning_digest.go` (plan_digest), `tools_status.go`, `resources.go` (workbench_resources, `registerGauges` meldet Prüfläufe, Konsole, Testläufe, Sessions an `resmon`), `tools_logstats.go` (logs_stats, logs_context), `tools_notify.go` (`Notice`, notify_ui), `tools_ports.go` (svc_health, svc_start_all, svc_stop_all, ports_status, get_urls), `tools_tasks.go` (task_list, task_start, task_stop, task_status, task_output über das Interface `TaskHost`; jeder Task im Katalog ist startbar).
- Middleware `observe.go` (`Server.observe`): misst jeden Tool-Call; `param_hint.go` ergänzt Parameterhinweise bei Fehlern; `stats.go` hält Zähler und Ringpuffer (`ToolStats`, `Call`, `Snapshot`).
- Check-Pipeline: `check_targets.go` (Whitelist `checkTarget`, `findTarget`), `check_run.go` (`checkRuns` mit Einzelflug-Sperre je Ziel, `runProcess`), `check_compact.go` (Einzeiler bei Grün, sonst nur Fehlerzeilen), `check_state.go` (`CheckState` für die UI), `check_job.go` (`checkJob`: wartet `check_run` höchstens `checkWait`, danach läuft der Lauf im Hintergrund weiter und `check_run {run: <ID>}` liefert das Ergebnis, B-372). Dazu `check_wait.go` (B-520: mit `progressToken` Fortschritt alle `progressEvery` und Warten bis zum Ende), `check_steps.go` (B-739: Schritt-Protokoll aus den Markerzeilen von `task check:diff`, Sperre für schwere Läufe), `check_taskversion.go` (`minTaskVersion` von Go-Task für Testmuster an Task-Ziele, B-866).
- Testläufe `sim_test` (B-348): `simtest.go` (Aktionen start/status/stop/list/wait über das Register `simrun.Registry` in `Server.sims`), `simtest_spec.go` (`simTestIn` → geprüfter `simSpec` → `sims.Offline`), `simtest_wait.go` (B-644: Warten wie `check_run`), `simtest_baseline.go` (Aktion `baseline`: auflisten oder Lauf unter `reports/baseline/<name>/` sichern), `simtest_lock.go` (B-788: `sim_test start` und `sim_run` lehnen ab, solange die Workbench nicht mit `K3C_SIM_ENTSPERRT=1` läuft). Offline-Runner in `internal/simrun`.
- Workspace-Scoping `workspace.go`: `workspace`, `resolveWorkspace` (Header `X-K3C-Root` bzw. MCP-roots), `FindRoot`, `isWorktreeOf`; `checkout.go`: Argument `checkout` der schreibenden Tools sowie `check_run` (B-388), gilt vor dem Header, Name über `git worktree list`, sonst absoluter Pfad; `ownCheckoutErr` (B-924, B-925) lehnt `sim_test start`, `sim_run` und `replay_run` ab, wenn der Checkout nicht der der laufenden Workbench ist (Engine rechnet im Prozess; Hinweis auf `task k3c-workbench:dev:worktree`); `worktree_services.go`: je Worktree eigener `services.Controller` mit Port-Versatz (`freeOffset`).
- `chunked_writer.go`: `forceChunked` HTTP-Middleware (Streaming ohne Content-Length).

## Flow

1. `New(cfg)` baut `mcp.Server`, `register(s)` hängt alle Tools ein, `observe` wird als Middleware gesetzt; `Start()` lauscht auf 127.0.0.1.
2. Tool-Call: `observe` -> `stats.begin` -> `resolveWorkspace` (Checkout aus Header/roots, Argument `checkout` vorrangig) -> Handler -> `stats.end`, Log über `applog`.
3. `check_run`: `findTarget` -> `checkRuns.begin` -> `runProcess` (`proc.Command`, Ausgabe in `console.Store`, Quelle `check:<ziel>`) -> `report` -> `notifyCheck`.
4. `svc_*`: `Server.controller(ctx)` liefert den `services.Controller` des Worktrees.
5. `plan_*`: Delegation an `planning.Create/Set/Section(s)/Delete/List/Get/Next/Waves/Queue/Check/RebuildIndex/Digest` mit dem Workspace-Root.
6. `sim_test start`: Sperre (`simtest_lock.go`) → `simSpec` prüfen → `simRunner` (`sims.Offline`) → `sims.Start` im Hintergrund; `status`/`wait` zeigen Phase und Kennzahlen.
7. `task_*`: `Server.taskHost` -> `TaskHost` der Wails-App; `task_start` startet jeden Task aus `task_list`, es gibt kein Freigabe-Schloss mehr.

## Integration

- Konsumenten: Wails-`App` (`app.go`, `app_logs.go`, `app_mcp.go`: `McpOverview`, `McpRestart`, `McpCalls`, `McpUsage`) und `main.go`; `TaskHost` wird von `app_tasks.go` erfüllt.
- Abhängigkeiten (intern): `console`, `applog`, `proc`, `services`, `planning`, `logs`, `taskcat`/`taskrun`, `engine/simcli` (im Prozess: `sim_run`, `replay_run`), `simrun` (`sim_test`), `balance` (Leseteil), `resmon`, `gamedata`, `github`, `usage`; kein Engine-Kern (depguard, B-726).
- Extern: `k3c/tools/workkit` (`mcpkit`, `applog`, `console`, `proc`, `usage`), `github.com/modelcontextprotocol/go-sdk/mcp`, `github.com/google/jsonschema-go`; Client: Claude Code über `.mcp.json`.
