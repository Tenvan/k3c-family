# tools/workkit/applog/

## Responsibility

Eigenes Log von k3c-workbench: strukturiertes JSON nach `logs/k3c-workbench.jsonl` im Format, das `internal/logs` liest, plus Spiegelung derselben Einträge in den Konsolenpuffer.

## Design

- `Log` (`applog.go`): `Open(logsDir, store)` liefert den `slog.Logger`; `Close`; `Discard()` für Tests.
- Fan-out-Handler `fanout` (`slog.Handler`): schreibt gleichzeitig in die Datei (JSON) und farbig über `conlog.New` und `console.LineWriter` in `console.Store` (Quelle `Source = "k3c-workbench"`, Strom `log`).
- Konfiguration: `LevelEnv` (`K3C_DEV_LOG_LEVEL`), `ParseLevel`, `Levels()` (getrennte Level für Datei und Spiegel); Rotation `Rotate`/`MaxFileSize` (10 MiB) beim Öffnen.

## Flow

1. `main.go` ruft `Open`: `Rotate` prüft die Größe, Datei öffnen, `fanout` bauen.
2. Komponenten loggen per `slog` (Emoji-Präfix laut Projektregel).
3. `Handle` verteilt jeden Record an Datei-Handler und Konsolen-Handler.

## Integration

- Konsumenten: `main.go`, `app.go`, `internal/mcpsrv/server.go` und `tools_logs.go`, `internal/services/controller.go` und `outlog.go`.
- Abhängigkeiten: `internal/console`, `engine/conlog`. Lesende Gegenseite: `internal/logs` und MCP-Tools `logs_services`, `logs_query`, `logs_errors`, `logs_since`, `logs_context`, `logs_stats`.
