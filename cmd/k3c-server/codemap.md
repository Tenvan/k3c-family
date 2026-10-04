# cmd/k3c-server/

## Responsibility

CLI entry point und Composition Root des Go-Servers: liefert `dist/` aus, stellt HTTP-/WebSocket-API, Spielstände und Berichte bereit und hostet die Räume (Online-Modus).

## Design

- `main.go`: `config` aus der Umgebung (`K3C_HTTP_PORT`, `K3C_HTTPS_PORT`, `K3C_DIST`, `K3C_SAVES_DIR`, `K3C_REPORTS_DIR`, `K3C_CERTS_DIR`, `K3C_STATUS_TOKEN`, `K3C_DEV`); `run` verdrahtet `store.Saves`, `room.Manager` und `k3cnet.NewHandler` (Dependency Wiring); `-health` dient als Docker-HEALTHCHECK (`checkHealth`).
- Version und Buildzeit per `-ldflags` (`version`, `built`, Fallback `buildTime` = Änderungszeit der EXE).
- `logging.go`: `newLogger`/`newNamedLogger` mit `fanout` (zwei `slog.Handler`: farbige Konsole via `engine/conlog` + JSON-Datei), `rotating` (eine Vorgänger-Generation, `maxClientLogBytes`), `consoleLevel` (`K3C_LOG_LEVEL`), `logDir` (`K3C_LOG_DIR` oder vorhandener `logs/`).

## Flow

1. `main` parst `-health` oder baut den Logger (`k3c-server.jsonl`) und ruft `run`.
2. `run` prüft `dist/index.html` und räumt Test-Spielstände (`room.TestPrefix`, älter als 24 h) mit `saves.Purge` auf.
3. Client-Logger (`k3c-client.jsonl`) und `k3cnet.NewHandler` entstehen; `rooms.Run(ctx)` startet die Raum-Goroutines (30 Hz).
4. HTTP-Server auf `:8080`, optional HTTPS auf `:8443`, wenn `certs/key.pem` und `cert.pem` existieren.
5. Bei SIGINT/SIGTERM: `rooms.Close()` (speichert, sendet `room_closed`), `waitConns` (2 s), `shutdown` (5 s).

## Integration

- Abhängigkeiten: `engine/net`, `engine/room`, `engine/store`, `engine/conlog`; `dist/` aus `task build`.
- Konsumenten: Taskfile-Tasks `start`/`serve` (`bin/k3c-server`), `Dockerfile` (Image, HEALTHCHECK `-health`), `.github/workflows/ci.yml` (Smoke-Test), `cmd/k3c-tui` und `cmd/k3c-load` (über `/api/status`); k3c-dev liest `logs/k3c-server.jsonl` und `k3c-client.jsonl`.
