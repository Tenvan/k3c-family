# WR1.1 · Spielserver, engine/net, k3c-load und k3c-tui entfernen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Lane:** –
- **Modell:** –
- **Umgebung:** ?
- **Branch:** wr1/1-server-entfernen
- **Abhängig von:** –
- **Tickets:** B-887
- **Kriterien:** AC-01

## Ziel

Im Repo gibt es keinen Spielserver, kein WebSocket-Protokoll, keinen Lasttest, keine TUI und keinen Windows-Starter mehr; Tasks, CI, Docker und Workbench kommen ohne sie aus (B-887).

## Kontext

Entscheidung B-850 (Ralf, 2026-11-14): kein Online-Spiel, das Spiel ist nativ (Entscheidung 008). `engine/room` und `engine/store` nutzt das native Spiel (`native/k3c-game`) weiter, sie bleiben. `engine/watch` nutzt nur der Server, es entfällt. `engine/devrun` und `engine/conlog` nutzt die Workbench weiter. In der Workbench hängen am Server: MCP-Tools `server_status`, `rooms_list`, `room_snapshot` (`internal/mcpsrv/tools_server.go`), entfernte Server-Logs (`remote*.go`, B-771), `sim_test` mit `mode: online` samt Clients (`simtest_online.go`, `simtest_clients*.go`) und die Pakete `serverapi`, `botdev`, `botfeed`, `browser`, dazu die Dienste in `services.json`. Offene Frage B-887 entschieden mit Default: `tools/k3c-starter` entfällt.

## Erlaubte Dateien

- `cmd/k3c-server/**`
- `cmd/k3c-load/**`
- `cmd/k3c-tui/**`
- `cmd/codemap.md`
- `engine/net/**`
- `engine/watch/**`
- `engine/Taskfile.yml`
- `engine/codemap.md`
- `engine/*/codemap.md`
- `engine/conlog/**`
- `engine/devrun/**`
- `engine/internal/rules/**`
- `tools/k3c-starter/**`
- `tools/k3c-workbench/**`
- `tools/checkdiff/**`
- `tools/webkit/**`
- `tools/codemap.md`
- `Taskfile.yml`
- `Dockerfile`
- `compose.yaml`
- `.github/workflows/**`
- `.gitignore`
- `.oxlintrc.json`
- `.claude/launch.json`
- `tests/**`
- `docs/protocol.md`
- `CLAUDE.md`
- `README.md`
- `requirements.md`
- `codemap.md`

## Nicht-Ziele

Browser-Client `src/` und Phaser (WR1.2, B-848), `site/` (WR1.3). `engine/sim`, `engine/level`, `engine/room`, `engine/store` und das native Spiel bleiben. Die Dienste-Infrastruktur der Workbench bleibt (leere `services.json`), ihr Rückbau wäre ein eigenes Ticket.

## Schritte

1. Importe prüfen (`engine/room`, `engine/store`, `engine/watch`, `engine/net`).
2. `cmd/k3c-server`, `cmd/k3c-load`, `cmd/k3c-tui`, `engine/net`, `engine/watch`, `tools/k3c-starter` löschen.
3. Workbench: Server-Tools, entfernte Server-Logs, `sim_test` online/clients, Pakete `serverapi`, `botdev`, `botfeed`, `browser`, Dienst Spielserver, Log-Quelle k3c-server entfernen; Frontend nachziehen.
4. `tools/checkdiff`, Taskfiles (`buildall`, `start`, `serve`, `check:starter`, Includes), CI, `release.yml`, Docker/Compose bereinigen.
5. `docs/protocol.md` entfernen, CLAUDE.md, README, codemaps nachziehen.
6. `check_run task:check:diff` grün, Commit.

## Fertig, wenn

- [ ] AC-01: Die Ordner oben existieren nicht mehr, `task --list` nennt `k3c-server`, `k3c-load`, `k3c-tui`, `k3c-starter`, `start`, `serve` nicht.
- [ ] AC-03: Die Workbench hat keine Server-Dienste und keine Tools `server_status`, `rooms_list`, `room_snapshot` mehr (go test der Workbench grün).
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: `cmd/k3c-server`, `cmd/k3c-load`, `cmd/k3c-tui`, `engine/net`, `engine/watch`, `tools/k3c-starter`, `Dockerfile`, `compose.yaml`, `.dockerignore` entfernt; `task --list` nennt weder `k3c-server`, `k3c-load`, `k3c-tui`, `k3c-starter` noch `start`/`serve`. CI ohne Docker- und Starter-Job, Release ohne Server-Binaries und Docker-Image.
- B-887/AC-03 geprüft (go test): Workbench ohne `server_status`, `rooms_list`, `room_snapshot`, ohne entfernte Server-Logs (B-771) und ohne `sim_test` online/Clients; Pakete `serverapi`, `botdev`, `botfeed`, `browser` entfernt; `services.json` leer.
- `engine/room` und `engine/store` bleiben (natives Spiel). `docs/protocol.md` bleibt als Referenz für Zustand und Ereignisse (Kopfnotiz), weil `engine/room` und viele Regeln darauf verweisen.
- `go mod tidy`: websocket, bubbletea, lipgloss, fsnotify fallen weg.
- Nebenbei (für grünes `task lint`): zwei Komplexitäts-Fehler im Workbench-Frontend (`SizeCompare`, `levelModel`) ohne Verhaltensänderung aufgeteilt; Planungs-Format von B-887/B-888 und Sprint-README WR1 nachgezogen.
- `task check:diff` grün.
