# tools/k3c-dev/internal/applog/

## Responsibility

Eigenes Log von k3c-dev: strukturiertes JSON nach `logs/k3c-dev.jsonl` (gleiches Format wie der Spielserver, damit `logs_*` es lesen) plus Spiegelung in die Konsole.

## Design

- `Log` (`applog.go`): `Open(logsDir, store)` liefert den `slog.Logger`; `Close`; `Discard()` für Tests.
- Fan-out-Handler `fanout` (`slog.Handler`): schreibt gleichzeitig in die Datei (JSON) und in `console.Store` (Quelle `Source = "k3c-dev"`).
- Konfiguration: `LevelEnv` (`K3C_DEV_LOG_LEVEL`), `ParseLevel`, `Levels()` (getrennte Level für Datei und Spiegel); Rotation `Rotate`/`MaxFileSize` (10 MiB).

## Flow

1. `main.go` ruft `Open`: `Rotate` prüft die Größe, Datei öffnen, `fanout` bauen.
2. Komponenten loggen per `slog` (Emoji-Präfix laut Projektregel).
3. `Handle` verteilt jeden Record an Datei-Handler und Konsolen-Handler.

## Integration

- Konsumenten: `main.go`, `app.go`, `internal/mcpsrv/server.go` und `tools_logs.go`, `internal/services/controller.go` und `outlog.go`.
- Abhängigkeiten: `internal/console`. Lesende Gegenseite: `internal/logs` und MCP-Tools `logs_sources`, `logs_query`, `logs_errors`, `logs_since`.
