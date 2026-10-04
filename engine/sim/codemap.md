# engine/sim/

## Responsibility

Domain Model und Simulation-Core des Spiels: deterministischer Tick-Step für eine Stufe (`World`), eine Insel mit mehreren Stufen (`Island`) und die Legacy-`Campaign`, dazu Spielstand-(De)Serialisierung. Reine Logik ohne I/O; Balancing kommt aus `data/` (`go:embed`).

## Design

- **Domain Model** in `types.go`: `World`, `Player`, `Troop`, `Site`, `Enemy`, `Stock`, `Event` (`map[string]any`), `PlayerCommand` als Eingabe.
- **System-Pipeline** pro Tick: `Step` in `world.go` ruft feste Reihenfolge `stepCycle`, `stepSpawns`, `stepPlayers`, `stepCamps`, `stepSites`, `stepTroops`, `stepEnemies`, `stepProjectiles`, `stepTravel`; die Reihenfolge ist Teil des Determinismus-Vertrags (Golden-Files `testdata/golden/sim-*.json`).
- **Aggregate**: `Island` (`island.go`) hält `Stages []*World` mit gemeinsamem `Stock` (`island_storage.go`: Kapazität, `addStockCapped`); `Campaign` (`campaign.go`) ist der v1-Vorgänger mit einer aktiven Stufe.
- **Einzelwechsel** je Spieler: `island_travel.go` (`islandTravel`, `stepIslandTravel`, `moveToStage`); `travel.go` liefert `travelPoints` pro Welt.
- **Spielstand**: `IslandSave` v2 (`island_save.go`, `ParseIslandSave` migriert v1 über `islandFromV1`), `SaveGame` v1 (`save.go`). Gespeichert wird nur, was nicht aus dem Seed folgt.
- **Daten**: `data.go` lädt JSON per `data.Files` (`load[T]`, panic bei Fehler) in Paket-Variablen (`biomes`, `hub`); `island_options.go` hat `IslandOptions`/`Grade` (Schwierigkeit, `SetOptions`, `SetGrade`).
- **Feedback-Events**: `events.go` (`emit`, `hitEvent`, `arrowEvent`, `capEvents`); lesen nur, verbrauchen kein rng.
- Einheiten-KI in `units.go`, `archer.go`; Gegner in `enemies.go`, Wellen in `waves.go`; Dev-Eingriffe in `dev.go` (`DevDropGold`, `DevAddStock`).
- Float-Regel: Produkte vor Addition mit `float64(…)` runden (FMA-Abweichung zu JS).

## Flow

1. Raum erzeugt `CreateIsland(seed, depths, cycleSpeed)` → je Tiefe `CreateWorld(biome, seed, opts)` (`level.Generate` + `placeEntities`).
2. `AddIslandPlayer(isl, stage)` vergibt inselweiten Spielerindex.
3. Pro Tick: `StepIsland(isl, commands, dt)` → `Step` je Stufe (`commands[p.Index]`) → `stepIslandTravel` → jedes Event erhält `stage`.
4. In `Step`: Events leeren, Zeit/Zyklus (`cycleAt`), Wellen via `planWave` (nutzt `rng.Rng`), Zahlung/Bewegung, KI, Kampf (`applyDamage`), Aufräumen, `castleFallen` bei Burg-HP ≤ 0.
5. Speichern: `isl.ToSave(savedAt)` → JSON; Laden: `ParseIslandSave` → `FromIslandSave(s, cycleSpeed)`.

## Integration

- Importiert `k3c/engine/level` (Layout, Biome, Cycle), `k3c/engine/rng`, `k3c/data` (embed).
- Konsumenten: `engine/room` (Spielschleife), `engine/net` (Protokoll v3, Snapshots), `tools/k3c-dev/internal/balance` und `tools/k3c-dev/internal/enginetools` (`sim_run`, Balancing).
- Client spiegelt Typen in `src/model/` (`World`, `GameEvent`).
