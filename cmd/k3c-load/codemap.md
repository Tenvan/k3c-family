# cmd/k3c-load/

## Responsibility

CLI entry point (Load-Test-Client): legt n Test-Räume mit je m Bots gegen einen laufenden `k3c-server` an, misst Tick-Dauer, Phase und CPU über `/api/status` und schreibt einen Bericht mit Verdict (B-175, Ziel aus B-042).

## Design

- `main.go`: `config` (Flags, Fallback `K3C_STATUS_TOKEN`), `parse`, `run(ctx, args, getenv, stdout, stderr)` testbar ohne Prozess; `roomRun`/`tickPoint` je Raum; `meldung`/`failf` als Fehlertyp für Menschen.
- `bots.go`: `bot` = Gerät mit Spieler (Slot 0), spricht das WebSocket-Protokoll (`hello`, `create`, `join`, `input`, `leave`) mit eigenen Nachrichtentypen, weil die Typen in `engine/net` nicht exportiert sind; `script` erzeugt deterministische Eingaben aus einem Seed (`engine/rng`).
- `api.go`: `api` = HTTP-Client für `GET /api/status` (Bearer-Token, nie im Fehlertext), `waitDisconnected`.
- `poll.go`: `poller` nimmt periodisch Proben (`-interval`), erkennt das Ende der ersten Nacht (`-duration night`).
- `report.go`: `sample`/`row`/`report`, `evaluate` (erreicht/knapp/verfehlt/ohneDaten gegen `-target-p99`), `exitCode`, Ausgabe `<out>.json` + `<out>.md`.

## Flow

1. `main` -> `run` -> `parse`; `load` prüft Server und Token via `api.status`.
2. `startRooms`: je Raum erster Bot `create`, übrige `join` per Code; Räume heißen `test-load-<tag>-…` (der Server räumt sie auf).
3. Je Bot läuft `play()`: auf jeden `snap`/`delta` folgt die nächste Eingabe aus `script.next`.
4. `poller.run` sammelt Proben bis Dauer, Nacht oder Signal (Strg+C).
5. Alle Bots `stop()`, `waitDisconnected`, `buildReport` -> `write`; Exit-Code 0/1/2 aus `exitCode`.

## Integration

- Abhängigkeiten: `engine/net` (Protokollversion), `engine/rng`; WebSocket `/ws` und `/api/status` von `cmd/k3c-server`; `docs/protocol.md`.
- Gebaut und gestartet über Taskfile-Task `load` (`bin/k3c-load`, Aufruf `task load -- -url … -token …`).
- Berichte landen in `reports/load-<Zeit>.{json,md}`.
