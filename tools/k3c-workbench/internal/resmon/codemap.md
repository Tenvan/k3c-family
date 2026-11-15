# tools/k3c-workbench/internal/resmon/

## Responsibility

Misst den Ressourcenverbrauch von k3c-workbench über Stunden (Go-Heap, Prozess- und Kindprozess-Speicher, WebView2-Renderer, Goroutinen, Handles, Puffergrößen), hält einen begrenzten Verlauf, loggt in Abständen, legt bei Schwellen Profile ab und misst Dauer und Allokation je Vorgang.

## Design

- `Monitor` (`resmon.go`): `New(Config)` mit Vorgaben für Nullwerte (`Interval`, `History`, `LogEvery`, `Dir`, `HeapDump` = `DefaultHeapDump` 1 GiB, `KeepDumps`, `WebViewWarn` = `DefaultWebViewWarn` 500 MiB, `Log`, `Now`); `Run(ctx)` nimmt Proben im Takt, `Sample()` misst einmal, `History`, `Latest`; `Gauge(name, unit, fn)` registriert Messwerte fremder Puffer.
- `Sample`: `HeapLive`, `Total`, `Private`, `RSS`, `Children`/`ChildrenRSS`, `WebView` (größter `msedgewebview2.exe` unter den Kindern, B-895), `Goroutines`, `Handles`, `GCCycles`, `Gauges`.
- Prozessdaten per Plattform-Datei: `proc_windows.go` (Snapshot, Nachfahren über die Elternkette, `memory` je Handle), `proc_other.go` (leer); `ProcInfo` in `proc.go`.
- Vorgänge (`ops.go`): `Track(name)` liefert eine Stopp-Funktion (`defer m.Track("tool:check_run")()`), Allokation prozessweit aus `runtime/metrics`; höchstens `maxOps` Namen, danach `(weitere)`; `Ops()` absteigend nach Bytes. `nil`-Monitor misst nichts.
- Schwellen (`dump.go`): `checkThresholds` schreibt `heap-*.pprof` und `goroutines-*.txt` nach `Config.Dir` (je weitere 50 % erneut), behält `KeepDumps`; WebView-Warnung mit 🤒; `Dumps()` listet sie.
- Auswertung: `series.go` (`Gauges`, `Top`, `Trends` je Stunde, `Series(window, maxPoints)` verdichtet mit Maximum je Fenster, `Interval`, `Started`), `report.go` (`Report()` für `workbench_resources`, Trend als Ausgleichsgerade `trendOf`, Log-Zeile `summary`).

## Flow

1. `resmon_start.go` (`startResources`): `New` mit Takt 10 s, Verlauf 24 h, Log alle 5 min (`ns=res`), Profile unter `logs/profile/`, Schwelle aus `K3C_DEV_HEAP_DUMP_MB`; `Gauge` für die Log-Dateien; der MCP-Server registriert in `internal/mcpsrv/resources.go` Konsole, Prüfläufe, Testläufe und MCP-Sessions; `go Run(ctx)`.
2. Je Probe: `Sample` → Verlauf → `checkThresholds` → jede n-te Probe eine Log-Zeile.
3. Abfragen: Seite Monitoring über `App.Resources(windowMin)` (`app_resources.go` → `Series`, `Gauges`, `Trends`, `Ops`), MCP `workbench_resources` → `Report`; MCP-Middleware und UI-RPC rufen `Track`.

## Integration

- Konsumenten: `tools/k3c-workbench/resmon_start.go`, `app.go`, `app_resources.go`, `uiapi.go` (`Track` je UI-Methode), `internal/mcpsrv/server.go` (`Config.Resources`) und `observe.go` (`Track` je Tool).
- Abhängigkeiten: `golang.org/x/sys/windows` (Windows), Standardbibliothek (`runtime/metrics`, `runtime/pprof`).
