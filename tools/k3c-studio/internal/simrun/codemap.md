# tools/k3c-workbench/internal/simrun/

## Responsibility

Steuert die Testläufe der Workbench (B-348, WZ28.4): ein Register je Workbench für `sim_test` und die Balancing-Seite mit höchstens `MaxPerRoot` (2) gleichzeitigen Läufen je Checkout und Warteschlange, dazu der Offline-Runner und die Verwaltung der Balancing-Berichte unter `reports/`.

## Design

- Register (`registry.go`): `Registry` (`New(log)`), `Run`, `Runner func(ctx, *Run)`; `Exec` (Test-Naht) ist standardmäßig `runEngine`, der Lauf rechnet im Prozess; `Start(root, where, label, spec, runner, queue)`, `Stop`, `CancelAll`, `Get`, `List`; Fortschritt über `SetPhase`, `SetLines`, `SetProgress`, `Finish`; `ReportDir(run)` = `reports/simtest-<id>`. `Info` (`info.go`) ist der Stand eines Laufs für `sim_test` und die RPC `BalanceRuns`.
- Auftrag (`spec.go`): `Spec` mit `Tune`; jedes Feld wird ein Flag von `k3c-game sim balance`; leer heißt Flag weglassen (Ziele aus `data/balance-targets.json`); `Check` prüft Grenzen und Positivlisten, `Title` bildet die Bezeichnung.
- Offline-Runner (`offline.go`, RS2.2): `StartOffline(root, where, spec)` prüft und reiht ein; `Offline(spec)` rechnet `balance --targets …` im Prozess (über die Spiel-EXE `k3c-game sim balance`, `internal/gameexe`, Abbruch über Context) und liest die Events (`Proc`, `Result`) und den Bericht. `pervariant.go` (B-662): „je Variante“ wird n + 1 Läufe mit gleichen Seeds.
- Bericht (`stamp.go`): `Stamp` trägt Label, Auftrag und Lauf-ID nach dem Rechnen in den Bericht (mit kurzen Wiederholungen, falls die Datei gesperrt ist).
- Berichte (`reports.go`, `reportlist.go`, `manage.go`, `baseline.go`): `Reports(root)` → `ReportInfo`; `ReportPath` (nur `*.json` unter `reports/`), `ReadReport`, `ReadTuning`, `ReadInsights` (Typen aus `internal/balance`), `WriteHTML`; `RateReport(s)`, `DeleteReport(s)`; `SaveBaseline` und `RunReport` (B-640).
- Ereignisse (`notify.go`): `Notify`-Rückrufe für Läufe und Berichte, gedrosselt (`notifyEvery`), nie unter der Sperre; die Workbench macht daraus `balance:runs` und `balance:reports`.

## Flow

1. Balancing-Seite → `App.BalanceStart` (`app_balance.go`) → `StartOffline` → `Start(…, Offline(spec), queue)`; MCP `sim_test {action: start}` → `simtest_spec.go` → `Offline`.
2. Runner: `runEngine` (`balance --targets`) → Event-Zeilen → `SetProgress`/`SetLines` → `readResult` → `Stamp` → `Finish`.
3. `Notify` → `app_balance_events.go` → Events an die Oberfläche; Berichte über `Reports`/`ReadReport`/`WriteHTML`.

## Integration

- Konsumenten: `tools/k3c-workbench/app_balance.go`, `app_balance_events.go`, `internal/mcpsrv` (`server.go`, `simtest.go`, `simtest_baseline.go`, `simtest_spec.go`, `simtest_wait.go`).
- Abhängigkeiten: `internal/balance`, `internal/console`, Spiel-EXE (`internal/gameexe`, `k3c-game sim balance`).
