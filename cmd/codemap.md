# cmd/

## Responsibility
Container für die ausführbaren Go-Programme (CLI entry points). Keine eigenen Quelldateien. Spielserver, Lasttest und
TUI sind entfernt (B-887, Entscheidung B-850).

## Design
Ein Unterordner je Binary, jeweils `package main` als dünner Composition Root über `engine/`:

| Ordner | Binary |
|--------|--------|
| [`k3c-sim/`](k3c-sim/codemap.md) | Engine-CLI (B-726): Unterbefehle `run`, `level`, `balance`, `world`; JSON-Zeilen auf stdout, Kindprozess der Workbench |

## Flow
Die Workbench ruft dieselbe Logik über `engine/simcli` im Prozess auf (B-924) und liest dessen
JSON-Zeilen; `cmd/` selbst hat keinen Ablauf.

## Integration
- Gebaut über `cmd/k3c-sim/Taskfile.yml` (`task k3c-sim:build`) bzw. `task buildall` nach `bin/`.
- Abhängigkeit: `engine/*`, `data/` (via `go:embed`).
