# engine/

## Responsibility
Go-Spiel-Engine (Entscheidung 001): Simulation, Level-Generierung, Raumverwaltung, Persistenz, Balancing und
Dev-Infrastruktur. Alle Spielregeln liegen hier; das native Spiel zeichnet nur. Eigene Datei nur `Taskfile.yml`.

## Design
Schichten von innen nach außen:

| Ordner | Rolle |
|--------|-------|
| [`rng/`](rng/codemap.md) | deterministischer Generator, einzige Zufallsquelle |
| [`level/`](level/codemap.md) | deterministischer Level-Generator mit Validator, Biomen, Planung, Natur-Werten |
| [`sim/`](sim/codemap.md) | Domain Model und Tick-Core (World, Island, Welten mit Wertesatz, Spielstände), ohne I/O |
| [`room/`](room/codemap.md) | Raum/Manager: Geräte, Slots, Beobachter, Pause, Takt, Speichern, Dev-Aktionen, Vorschau |
| [`store/`](store/codemap.md) | File-Repository für Spielstände (`saves/`) und Berichte (`reports/`) |
| [`enginetools/`](enginetools/codemap.md) | Level und kurze Simulation als deterministischer Text |
| [`balance/`](balance/codemap.md) | Balancing-Tester: Bots, Kennzahlen, Zielkorridore, Berichte, Tuning |
| [`conlog/`](conlog/codemap.md) | farbiger `slog.Handler` mit Emoji-Topics |
| [`devrun/`](devrun/codemap.md) | Dev-Supervisor der Tasks `<app>:dev` (Neubau und Neustart des Kindes) |

`Taskfile.yml` ist der Namespace `go`: `test`, `lint`, `race` (ohne `engine/balance`), `cover`, `golden`
(Golden-Dateien neu schreiben), `golden-setup` (Git-Diff der gepackten Golden-Dateien).

## Flow
1. Das native Spiel (`native/k3c-game/local`) legt über `room.Manager` einen Raum an und taktet ihn selbst
   (`Room.SafeTick`).
2. Der Raum rechnet `sim.StepIsland` (Level aus `level`, Zufall aus `rng`); das Spiel liest den Zustand im selben Prozess.
3. Spielstände laufen über die Store-Ports von `room` nach `store`.
4. Werkzeug-Pfad: `cmd/k3c-sim` → `enginetools` bzw. `balance` → `sim`/`level`.

## Integration
- Konsumenten: `native/k3c-game`, `cmd/k3c-sim`; in k3c-workbench `internal/studioview` (`room`, `store`),
  `internal/world` (`store`), `internal/applog` und `headless.go` (`conlog`, `devrun`).
- Daten: `data/` per `go:embed` bzw. `K3C_DATA_DIR`.
- `internal/` (nur Tests und Test-Hilfen): `golden` (Golden-Dateien unter `testdata/golden/`), `rules` (Repo-Regeln
  für Go, z. B. Importe und Verschachtelungstiefe).
