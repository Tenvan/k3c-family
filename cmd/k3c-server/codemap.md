# cmd/k3c-server/

## Responsibility

CLI entry point und Composition Root des Go-Servers: liefert `dist/` aus, stellt HTTP-/WebSocket-API, Spielstände und Berichte bereit und hostet die Räume (Online-Modus) samt Messreihen (`/api/metrics`) und Dev-Seite (`/api/dev`).

## Design

- `main.go`: `config` aus der Umgebung (`K3C_HTTP_PORT`, `K3C_HTTPS_PORT`, `K3C_DIST`, `K3C_SAVES_DIR`, `K3C_REPORTS_DIR`, `K3C_CERTS_DIR`, `K3C_STATUS_TOKEN`, `K3C_DEV`; Dev-Mode an, solange nicht `0`); `run` verdrahtet `store.Saves`, `store.Reports`, `room.Manager` (`Log`, `Sessions` = Reports für die Spielmetrik, `Dev`) und `k3cnet.NewHandler` (Dependency Wiring); `-health` dient als Docker-HEALTHCHECK (`checkHealth`).
- Version und Buildzeit per `-ldflags` (`version`, `built`, Fallback `buildTime` = Änderungszeit der EXE).
- Kein Asset-Studio mehr (B-507, TS1.1): das Backend liegt in `tools/k3c-studio/backend`; der Server importiert nichts aus `tools/` (Test `engine/internal/rules/imports_test.go`).
- Datei-Watch und Selbst-Neustart (B-507, Entscheidung 004): `supervise.go` startet den Server als Kindprozess (`K3C_CHILD=1`, `K3C_DATA_DIR` gesetzt, Daten von der Platte) und startet ihn neu, wenn er mit Exit-Code 75 endet; die PID des Supervisors bleibt. Absturz mit Daten von der Platte binnen 10 s: einmal eingebettet. `restart.go` beobachtet `data/` (`K3C_DATA_DIR`) und `public/` (`K3C_PUBLIC_DIR`) mit `engine/watch` (Takt 1 s, 2 s Ruhe), prüft den neuen Stand per `k3c-server -check-data`, schickt allen Geräten `restarting` („Server startet neu“) und endet nach 3 s mit 75; ungültiger Stand: ❌ im Log, alter Stand läuft weiter.
- `logging.go`: `newLogger`/`newNamedLogger` mit `fanout` (zwei `slog.Handler`: farbige Konsole via `engine/conlog` + JSON-Datei), `rotating` (eine Vorgänger-Generation, `maxClientLogBytes`), `consoleLevel` (`K3C_LOG_LEVEL`), `logDir` (`K3C_LOG_DIR` oder vorhandener `logs/`).

## Flow

1. `main` parst `-health` oder baut den Logger (`k3c-server.jsonl`) und ruft `run`.
2. `run` prüft `dist/index.html` und räumt Test-Spielstände (`room.TestPrefix`, älter als 24 h) mit `saves.Purge` auf.
3. Client-Logger (`k3c-client.jsonl`) und `k3cnet.NewHandler` entstehen; `rooms.Run(ctx)` startet die Raum-Goroutines (30 Hz).
4. HTTP-Server auf `:8080`, optional HTTPS auf `:8443`, wenn `certs/key.pem` und `cert.pem` existieren.
5. Bei SIGINT/SIGTERM: `rooms.Close()` (speichert, sendet `room_closed`), `waitConns` (2 s), `shutdown` (5 s).

## Integration

- Abhängigkeiten: `engine/net`, `engine/room`, `engine/store`, `engine/conlog`; `dist/` aus `task build`.
- Konsumenten: Taskfile-Tasks `start`/`serve` (`bin/k3c-server`), `Dockerfile` (Image, HEALTHCHECK `-health`), `.github/workflows/ci.yml` (Smoke-Test), `cmd/k3c-tui` und `cmd/k3c-load` (über `/api/status`); k3c-workbench liest `logs/k3c-server.jsonl` und `k3c-client.jsonl`.
