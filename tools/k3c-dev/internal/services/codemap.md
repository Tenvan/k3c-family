# tools/k3c-dev/internal/services/

## Responsibility

Supervisor für Entwicklungs-Dienste (Vite-Dev-Server, Go-Spielserver) aus `services.json`: Start/Stop/Restart, Health-Check, Crash-Restart, Neustart bei Dateiänderung, Übernahme bereits laufender Prozesse und Ressourcen-Sampling.

## Design

- Konfiguration `config.go`: `Service` (command, cwd, port, health, portEnv, autoRestart, `Watch`), `Load`, `validate`, `Shift` (Port-Versatz je Worktree), `HealthURL`.
- Zustandsautomat `controller.go`: `State`, `Status`, `Controller` mit `unit`/`run`; Seams per Dependency Injection: `Process`, `Starter`, `Checker`, `Options` (testbar ohne echte Prozesse).
- `lifecycle.go`: `start`, `awaitHealthy`, `stop`, `killRun`, `watch`, `crashed`, `allowRestart` (Restart-Limit).
- `runtime.go`: Produktiv-Implementierung `ProcStarter` (über `proc`), `HealthCheck` (HTTP). `system.go`: `PortListener`, `NewSampler` (gopsutil; `Metrics`).
- `adopt.go`: `Adopt` übernimmt Prozesse auf belegten Ports, `Monitor`/`sampleAll`, `LogLevels` (`LevelCounts`). `filewatch.go`: Polling-Snapshot (`fileSig`) und `restartForChange`.
- `outlog.go`: `outSink` leitet Prozessausgabe in die Konsole (`console.Store`) und ins JSONL-Log (`applog`), erkennt Level (`outLevel`, ANSI-Strip).

## Flow

1. `Load` + `New(list, opts)`; `StartAll` startet alle Units.
2. `Start`: `start` -> `Starter` (`proc`) -> `outSink` verdrahtet stdout/stderr -> `awaitHealthy` (`Checker`) -> State running.
3. `watch` wartet auf Exit -> `crashed` -> `allowRestart` -> erneuter `start`.
4. `filesLoop`: Snapshot-Diff -> `settle` -> `restartForChange`.
5. Beim App-Start: `Adopt` erkennt einen Prozess am Port (`PortListener`), `watchAdopted` beobachtet ihn.

## Integration

- Konsumenten: MCP-Tools `svc_status`, `svc_start`, `svc_stop`, `svc_restart` (`internal/mcpsrv/tools_services.go`, `worktree_services.go`); Wails-Bindings `Services`, `ServiceStart`, `ServiceStop`, `ServiceRestart`, `ServicesStartAll`, `ServicesStopAll`, `ServiceLogLevels` (`app_services.go`); `main.go` (`openServices`).
- Abhängigkeiten: `internal/proc`, `internal/console`, `internal/applog`, gopsutil (process/net); Konfig `tools/k3c-dev/services.json`.
