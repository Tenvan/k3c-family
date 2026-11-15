# engine/level/

## Responsibility

Domain Service: prozeduraler Level-Generator samt Validator. Erzeugt aus Biom, Seed und optionaler Planung ein
deterministisches `Layout` (Chunks, Entities, Bauplätze, Ruinen, Wachen, Hintergrund-Varianten, Natur-Werte).

## Design

- `level.go`: `Generate(b, seed)` = `GenerateWith(b, seed, Options{})`. Interner Builder `gen` mit Schritten `frame`
  (Chunk-Zahl, Hub, Ausgang) → `reserve` (geplante Chunks) → `placePortals` → `placeEvents` (Camps sicher vor Bau und
  Portal-Gegnern, `safeCampSlots`) → `entities` (Burg, Portale, Ressourcen, Skill-Punkte, Bau `lair` über `lairX`) →
  `veins`. Typen `Chunk`, `Entity`, `Layout` (JSON wie früher `LevelLayout`; Laufzeitfelder `Sites`, `FreeGround`,
  `Ruins`, `Guards`, `Nature` ohne JSON, `Backdrops` mit). `HarborKind` ist die Chunk-Art des Hafens.
- `planned_gen.go`: `Options` (`Planned`, `ChunkCount` = Inselgröße, `Nature` = fest gesetzte Natur-Werte; Nullwert
  wie `Generate`); `slotIndex`, `reserve`, `plannedEntities`, `chunkStream` (Chunk-Seed `<biom>:chunk:<chunk-seed>`),
  `clearOfRuin`.
- `biome.go`: `Biome` (Felder aus `data/biomes/*.json`), `Cycle` (`dayNight` / `aggressionPool`, `Check` ohne stillen
  Standardwert), `Range`, `LoadBiome(id)`; `Ordered[V]`/`Entry[V]` bewahren die JSON-Schlüsselreihenfolge.
- Geplante Chunks (`docs/rules/geplante-chunks.md`): `planned.go` (`PlannedChunk` streng gelesen, `PlannedObject`,
  `ValidatePlan`/`ValidatePlanFor` gegen kürzeste Länge bzw. `ChunkCount`), `planned_totals.go` (Biom-Summen und freie
  Plätze), `ruins.go` (`PlannedBuilding`, `Ruin`, `RuinRules` aus `data/ruins.json`, `RuinHP`, `ruinsOf`),
  `guards.go` (Gegnerlager: `PlannedEnemy`, `Guard`, `GuardRules` aus `data/guards.json`, `guardsOf`).
- `sites.go` (B-652): `WallSite`, `SitePlacement` (`data/hub.json › sitePlacement`), `placeSites` verteilt Mauer- und
  Turm-Plätze je Zone aus eigenem Strom, `placeNurseries` (Strom `<biom>:<seed>:nursery`), `FreeGround` und
  `Layout.OnFreeGround`. `lines.go` liest Hub-/Insel-Plätze (`hubSite`) und den Zahlziel-Abstand aus `hub.json` und
  `economy.json`.
- `nature.go`/`nature_check.go` (B-891): `Trait`, `WeightGroup`, `NatureValues` (`SetTrait`, `SetWeight`, `Empty`),
  `MergeNature`, `NeutralNature`, `ResolveNature` (Würfe aus `<biom>:<seed>:nature`), `assignVariants`,
  `Biome.CheckNatureValues`.
- `backdrop.go`: `BackdropTileUnits`, `backdrops` wählt je Hintergrund-Kachel eine Variante aus Strom
  `<biom>:<seed>:backdrop`.
- `validate.go`: `Validate`/`ValidateWith` (leere Liste = gültig): Länge, Chunks, Portale außerhalb der Linien, Bau,
  Entities, Adern.
- Determinismus: Reihenfolge der `rng`-Aufrufe ist Vertrag (Golden `testdata/golden/level-*.json`); Neues würfelt aus
  eigenen Strömen; `float64(…)`-Rundung gegen FMA.

## Flow

1. `LoadBiome(id)` → `Biome`.
2. `GenerateWith`: `ValidatePlanFor` → `ResolveNature` → `rng.New(seed)` → `frame` → `reserve` → Portale, Ereignisse,
   Entities, Adern → `placeSites` → `Layout` mit `ruinsOf`, `guardsOf`, `backdrops`.
3. Optional `ValidateWith(layout, biome, opts)` prüft Spielbarkeit.

## Integration

- Abhängigkeiten: `k3c/engine/rng`, `k3c/data` (`biomes/*.json`, `hub.json`, `economy.json`, `buildings.json`,
  `ruins.json`, `guards.json`, `island-sizes.json`).
- Konsumenten: `engine/sim` (`CreateWorld`, Biome, Cycle, Ruinen, Wachen, Bauplätze), `engine/room` (Vorschau),
  `engine/enginetools`, `cmd/k3c-sim` (`level`, `world`), `native/k3c-game/local` (`level.Layout`).
