# cmd/

## Responsibility
Container für die ausführbaren Go-Programme (CLI entry points) des Spielservers. Keine eigenen Quelldateien.

## Design
Ein Unterordner je Binary, jeweils `package main` als dünner Composition Root über `engine/`:

| Ordner | Binary |
|--------|--------|
| [`k3c-server/`](k3c-server/codemap.md) | Spielserver: liefert `dist/` aus, HTTP-API, WebSocket `/ws`, Räume, Spielstände, Berichte |
| [`k3c-load/`](k3c-load/codemap.md) | Lasttest-Client: Bots in Testräumen, misst die Tick-Dauer über `/api/status` |
| [`k3c-tui/`](k3c-tui/codemap.md) | Bubble-Tea-Terminal-UI zur Diagnose eines laufenden Servers (`-once` = Textausgabe) |

## Flow
1. `k3c-server` verdrahtet `engine/net`, `engine/room` und `engine/store` und startet den HTTP-Server.
2. `k3c-load` und `k3c-tui` sprechen den laufenden Server nur über HTTP/WebSocket an.

## Integration
- Gebaut über `Taskfile.yml` (`task serve`, `task start`) nach `bin/`, im Container über das `Dockerfile`.
- Abhängigkeit: `engine/*`, `data/` (via `go:embed`).
