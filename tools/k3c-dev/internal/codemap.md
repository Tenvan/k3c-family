# tools/k3c-dev/internal/

## Responsibility
Interne Go-Packages von k3c-dev (Service Layer hinter Wails-Bindings und MCP-Server). Keine eigenen Quelldateien.

## Design
| Ordner | Rolle |
|--------|-------|
| [`mcpsrv/`](mcpsrv/codemap.md) | MCP-Server (Streamable HTTP, 127.0.0.1): Tool-Katalog, Worktree-Scoping, Check-Pipeline |
| [`planning/`](planning/codemap.md) | Tickets, Sprints, Sessions und Roadmap unter `docs/`, transaktional mit Vorlagenpflicht |
| [`services/`](services/codemap.md) | Supervisor für Dev-Dienste aus `services.json` |
| [`proc/`](proc/codemap.md) | Kindprozesse mit Prozessbaum-Kill (Job Object) |
| [`taskrun/`](taskrun/codemap.md) / [`taskcat/`](taskcat/codemap.md) | Go-Task-Läufe und Task-Katalog |
| [`console/`](console/codemap.md) / [`applog/`](applog/codemap.md) | Konsolenpuffer je Quelle, eigenes Log `logs/k3c-dev.jsonl` |
| [`logs/`](logs/codemap.md) | Leser für `logs/*.jsonl` mit Cursor und Digest |
| [`serverapi/`](serverapi/codemap.md) | Read-only-Client für `/api/status` des Spielservers |
| [`enginetools/`](enginetools/codemap.md) | In-process-Facade auf `engine/level` und `engine/sim` |
| [`balance/`](balance/codemap.md) (+ [`cmd/`](balance/cmd/codemap.md)) | Balancing-Matrix, Zielkorridor, Baseline, Replays |
| [`gamedata/`](gamedata/codemap.md) | Zusammenfassungen von `reports/` und `saves/` |
| [`github/`](github/codemap.md) | `gh`-CLI-Client für PR- und CI-Stand |
| [`usage/`](usage/codemap.md) | Statistik der MCP-Aufrufe |

## Flow
1. `tools/k3c-dev/main.go` baut die Packages zusammen.
2. Die Wails-App (`app*.go`) und `mcpsrv` rufen dieselben Packages auf; das Frontend und MCP-Clients bekommen also dieselben Daten.

## Integration
- Konsument: `tools/k3c-dev` (Wails-Host).
- Abhängigkeiten: `engine/*` (enginetools, balance), externe CLIs `task`, `gh`, `go`.
