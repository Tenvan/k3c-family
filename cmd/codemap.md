# cmd/

## Responsibility
Container für die ausführbaren Go-Programme (CLI entry points). Keine eigenen Quelldateien. Spielserver, Lasttest und
TUI sind entfernt (B-887, Entscheidung B-850).

## Design
Ein Unterordner je Binary, jeweils `package main` als dünner Composition Root über `engine/`:

| Ordner | Binary |
|--------|--------|
| `k3c-sim/` | Engine-CLI: Simulation, Balancing, Level (B-726), für die Workbench-Tools `level_generate`, `sim_run`, `sim_test` |

## Integration
- Gebaut über `Taskfile.yml` (`task k3c-sim:build`, `task buildall`) nach `bin/`.
- Abhängigkeit: `engine/*`, `data/` (via `go:embed`).
