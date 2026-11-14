# data/biomes/

## Responsibility

Configuration Data: ein JSON je Biom/Stufe (`forest.json` Tiefe 0, `cave.json` 1, `mine.json` 2, `ironhold.json` 3,
`crystal.json` 4) mit den festen Eckdaten für Level-Generator, Simulation und Darstellung.

## Design

- Gemeinsames Schema: `id`, `name`, `depth`, `backdropVariants` (Varianten der Hintergrund-Kacheln), `lengthUnits
  {min,max}`, `chunkWidthUnits`, `hubWidthUnits`, `exitSide`, `portals {count, minDistanceFromHubUnits}`,
  `chunkWeights`, `eventChunks`, `skillPoints`, `resourcesPerChunk`, `traits` (Natur-Eigenschaften mit `name`, `min`,
  `max`, `default`, `scales`, B-891), `primaryResource`, `enemies {portal, night}`, `cycle`, `palette`.
- Chunk-Typen: `forest` = `forest`, `clearing`, `rocks`, `stream`; `cave` = `tunnel`, `cavern`, `crystals`, `chasm`;
  `mine` = `shaft`, `rails`, `oreVein`; `ironhold` = `gallery`, `forge`, `lava`; `crystal` = `grotto`, `geode`.
- Zyklus: `forest` hat `dayNight`, die übrigen `aggressionPool`.
- Biom-spezifisch: `weights` (Gewichts-Gruppen, z. B. Baumarten) in `forest.json` und `crystal.json`; `veins
  {count, kind}` unter Tage (`stoneVein`, `copperVein`, `ironVein`, `crystalVein`, Definition in `data/economy.json`);
  `lava {damagePerSecond, widthUnits}` in `ironhold.json`; `lair` (Bau des Endbosses) in `crystal.json`;
  `rootsPerNode` in `cave.json` (Darstellung).
- Längen in Units (1 Unit = 32 px); Werte stehen nur hier, nicht im Code.

## Flow

1. `data.Files` bettet `biomes/*.json` ein (`data/embed.go`).
2. `engine/level/biome.go` (`LoadBiome(id)`) liest ein Biom; `engine/sim/data.go` (`loadBiomes`, `checkPool`) lädt alle
   und prüft die Gegner-Pools.
3. Das native Spiel liest Darstellungswerte direkt: `palette` (`world/backdrop.go`), `rootsPerNode` und Tiefen
   (`props/data.go`), Level-IDs (`game/assets.go`); `backdropVariants` kommt über `level.Layout.Backdrops`.

## Integration

- Go-Konsumenten: `engine/level/biome.go`, `engine/sim/data.go`, `engine/sim/world_lava.go`,
  `engine/enginetools/level.go`, `engine/balance/vary.go`.
- Natives Spiel: `native/k3c-game/props/data.go`, `native/k3c-game/world/backdrop.go`, `native/k3c-game/game/assets.go`.
- Workbench: `tools/k3c-workbench/internal/datadoc`, `internal/gamedata`, `internal/world`.
