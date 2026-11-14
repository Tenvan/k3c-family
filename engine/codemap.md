# engine/

## Responsibility
Go-Spiel-Engine (ADR 001): Simulation, Level-Generierung, Raumverwaltung, Netzwerk und Persistenz. Der Browser ist reiner Client, alle Spielregeln liegen hier. Keine eigenen Quelldateien.

## Design
Schichten von innen nach außen:

| Ordner | Rolle |
|--------|-------|
| [`rng/`](rng/codemap.md) | deterministischer mulberry32-Generator, einzige Zufallsquelle |
| [`sim/`](sim/codemap.md) | Domain Model und deterministischer Tick-Core (World, Island, Campaign, Saves), ohne I/O |
| [`level/`](level/codemap.md) | deterministischer Level-Generator mit Validator und Biome-Loader |
| [`room/`](room/codemap.md) | Room/Manager: Geräte, Slots, Takt, Fristen, Speichern, Crash-Supervision über Peer/Store-Ports |
| [`store/`](store/codemap.md) | File-Repository für Spielstände (`saves/`) und Berichte (`reports/`) |
| [`conlog/`](conlog/codemap.md) | farbiger `slog.Handler` mit Emoji-Topics |
| `devrun/` | Dev-Supervisor der Tasks `<app>:dev`: baut die App als Kind (`go build`), startet sie bei Go-Änderungen neu, tauscht die EXE über `.new`, Job-Objekt gegen verwaiste Kinder, `Listen` mit Wiederholung bei belegtem Port |

## Flow
1. Das native Spiel (`native/k3c-game`) legt über `room.Manager` einen Raum an (Spielserver und `net` sind entfernt, B-887).
2. Der Raum taktet `sim` (Seed aus `rng`, Level aus `level`); das Spiel liest den Zustand im selben Prozess.
3. Spielstände laufen über die Store-Ports von `room` nach `store`.

## Integration
- Konsumenten: `native/k3c-game`, `cmd/k3c-sim`; in k3c-workbench `internal/balance`, `internal/mcpsrv`, `internal/studioview`, `internal/applog` (`conlog`).
- Daten: `data/` per `go:embed`.
- `internal/` (nur Tests und Test-Hilfen): `golden` (Golden-Dateien unter `testdata/golden/`), `rules` (Repo-Regeln für Go: Verschachtelungstiefe `nesting_test.go`, `map_range_test.go` für `sim`/`level`).
