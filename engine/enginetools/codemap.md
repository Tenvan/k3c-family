# engine/enginetools/

## Responsibility

Application Service: rechnet Level und kurze Simulationen in-process mit `engine/level` und `engine/sim` und gibt sie
als deterministischen Text zurück (gleiche Eingabe, gleicher Text). Spricht mit keinem laufenden Spiel.

## Design

- `level.go`: `Level(biomeID, seed)` erzeugt ein Level und beschreibt es (Kopfzeile, eine Zeile je Abschnitt, Objekte
  nach Art über `entityLines`, Warnungen von `level.Validate`); `Biomes` listet die erlaubten Biome, `loadBiome` prüft
  die ID.
- `run.go`: `Run(biomeID, seed, ticks, inputs)` rechnet bis `MaxTicks` Ticks im Server-Takt `TickHz` (30) mit einem
  oder mehreren Monarchen. `Segment` hält die Eingabe eines Monarchen von `FromTick` bis `ToTick`; höchstens
  `MaxSegments`. Helfer `playerCount`, `commandsAt` (`[]sim.PlayerCommand` je Tick), `summary`/`counts` (Ereignisse
  als `tally`, erster Burgfall).

## Flow

1. Aufrufer übergibt Biom, Seed und ggf. Eingaben.
2. `Level`: `level.LoadBiome` → `level.Generate` → `level.Validate` → Text.
3. `Run`: `sim.CreateWorld` → je Tick `sim.Step(w, commandsAt(…), 1.0/TickHz)` → Ereignisse zählen → `summary`-Text.

## Integration

- Abhängigkeiten: `k3c/engine/level`, `k3c/engine/sim`.
- Konsumenten: `engine/simcli/input.go` (Unterbefehle `run` und `level`, MCP `sim_run`/`level_generate` der Workbench),
  `engine/balance` (nutzt `TickHz` in `run.go` und `replay.go`).
