# B-887 · k3c-server, engine/net, k3c-load und k3c-tui sind entfernt, Tasks, CI, Docker und Workbench kommen ohne sie aus

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WR1
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14 („WR1 auch abarbeiten“)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Entscheidung B-850 (🧑 2026-11-14): Ohne Online-Spiel entfallen Spielserver und alles, was nur ihm dient. Das Spiel ist nativ (Go + Ebitengine, Entscheidung 008).

## Ziel

Im Repo gibt es keinen Spielserver, kein WebSocket-Protokoll, keinen Lasttest und keine TUI mehr; alle übrigen Tasks und die CI laufen grün.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `cmd/k3c-server`, `cmd/k3c-load`, `cmd/k3c-tui`, `engine/net` entfernen; `engine/room`, `engine/store`, `engine/watch` nur, wenn nichts anderes sie nutzt.
- Taskfiles (`k3c-server`, `k3c-load`, `k3c-tui`, `serve`, `start`), `buildall`, CI-Server-Smoke-Test, Docker/Compose, `tools/k3c-starter` (startet nur den Server) bereinigen.
- Workbench: Dienste, MCP-Tools (`server_status`, `rooms_list`, `room_snapshot`, `svc_*` für den Server) und Seiten ohne Server.
- `docs/protocol.md`, CLAUDE.md und codemap nachziehen.

## Nicht-Ziele

Browser-Client `src/` und Phaser (B-848). Simulation `engine/sim` und Level-Generator bleiben.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne. `engine/room` und `engine/store` nutzt das native Spiel weiter.

- **Bereich:** `cmd/k3c-server/**`, `cmd/k3c-load/**`, `cmd/k3c-tui/**`, `engine/net/**`, `engine/watch/**`, `tools/k3c-starter/**`, `tools/k3c-workbench/**`, `tools/checkdiff/**`, `Taskfile.yml`, `.github/workflows/**`, `Dockerfile`, `compose.yaml`, `docs/protocol.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** deps, taskfile, exe

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Die Ordner und Tasks oben existieren nicht mehr; `task --list` nennt sie nicht.
- **AC-02** `task check:all` und die CI sind grün.
- **AC-03** Die Workbench startet und zeigt keine Server-Dienste mehr.

## Offene Fragen

keine (`tools/k3c-starter` entfällt mit dem Server, Default übernommen).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
