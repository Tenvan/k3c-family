# tools/k3c-workbench/internal/

## Responsibility
Interne Go-Packages von k3c-workbench (Service Layer hinter Wails-Bindings, Oberflächen-API und MCP-Server). Keine eigenen Quelldateien.

## Design
| Ordner | Rolle |
|--------|-------|
| [`mcpsrv/`](mcpsrv/codemap.md) | MCP-Server (Streamable HTTP, 127.0.0.1): Tool-Katalog (Entwicklung, Planung, Prüfläufe, Dienste, Logs), Worktree-Scoping, Check-Pipeline |
| [`planning/`](planning/codemap.md) | Tickets, Sprints, Sessions, Projekte und Fahrplan unter `docs/`, transaktional mit Vorlagenpflicht; nächste Session, Wellen, Queue, schlanke Listen mit Texten je ID, Änderungs-IDs |
| [`services/`](services/codemap.md) | Supervisor für Dev-Dienste aus `services.json` mit Reload im laufenden Betrieb |
| [`proc/`](proc/codemap.md) | Kindprozesse mit Prozessbaum-Kill (Job Object) |
| [`taskrun/`](taskrun/codemap.md) / [`taskcat/`](taskcat/codemap.md) | Go-Task-Läufe und Task-Katalog |
| [`console/`](console/codemap.md) / [`applog/`](applog/codemap.md) | Konsolenpuffer je Quelle, eigenes Log `logs/k3c-workbench.jsonl` |
| [`logs/`](logs/codemap.md) | Leser für `logs/*.jsonl` mit Cursor und Digest |
| [`resmon/`](resmon/codemap.md) | Ressourcen-Monitoring: Proben, Verlauf, Trends, Profile, Messung je Vorgang |
| (entfernt, B-924) | Die Workbench rechnet nicht mit der Engine (RU40.4: Läufe über `k3c-game sim`) |
| [`gitops/`](gitops/codemap.md) | Seite Git: Status, Branches mit Squash-Erkennung, Fetch/Pull/Push/Stash, Löschen gemergter Branches |
| [`github/`](github/codemap.md) | `gh`-CLI-Client für PR- und CI-Stand sowie PR-Aktionen |
| [`usage/`](usage/codemap.md) | Statistik der MCP-Aufrufe |

Balancing (`simrun/`, `balance/`, `datadoc/`, `gamedata/`) liegt seit WZ63.5 in `tools/k3c-studio/internal/`.

## Flow
1. `tools/k3c-workbench/main.go` baut die Packages zusammen (`newApp`, `openServices`).
2. Die Wails-App (`app*.go`) und `mcpsrv` rufen dieselben Packages auf; Frontend (Wails-Binding oder `/api/dev/rpc`) und MCP-Clients bekommen dieselben Daten.

## Integration
- Konsument: `tools/k3c-workbench` (Wails-Host, Headless-Betrieb).
- Abhängigkeiten: `tools/workkit` (`applog`, `console`, `proc`, `usage`, `mcpkit`), kein Engine-Kern (depguard, B-726); `tools/assetkit`; externe CLIs `task`, `gh`, `git`, `go`, `bun`, `bin/k3c-sim`.
