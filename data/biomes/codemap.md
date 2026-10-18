# data/biomes/

## Responsibility

Configuration Data: ein JSON je Biom/Stufe (`forest.json` Tiefe 0, `cave.json` 1, `mine.json` 2, `ironhold.json` 3, `crystal.json` 4) mit den festen Eckdaten für den Level-Generator und die Simulation.

## Design

- Gemeinsames Schema: `id`, `name`, `depth`, `lengthUnits {min,max}`, `chunkWidthUnits`, `hubWidthUnits`, `exitSide`, `portals {count, minDistanceFromHubUnits}`, `chunkWeights` (gewichtete Chunk-Typen), `eventChunks` (`chest`, `recruitCamp` je Min/Max), `skillPoints`, `resourcesPerChunk`, `primaryResource`, `veins {count, kind}`, optional `lair`, `enemies {portal, night}`, `cycle` (`dayNight` bei `forest`, sonst `aggressionPool` mit `percentPerMinute/Kill/Gather`), `palette {sky, far, near, ground}`.
- Chunk-Typen je Biom: `cave` = `tunnel`, `cavern`, `crystals`, `chasm`; `mine` = `shaft`, `rails`, `oreVein`; `ironhold` = `gallery`, `forge`, `lava`; `crystal` = `grotto`, `geode`.
- `ironhold.json` hat zusätzlich `lava {damagePerSecond, widthUnits}` (Lava-Chunks, Schaden in `engine/sim/lava.go`); `crystal.json` und `ironhold.json` haben eigene Portal-Pools (2 Standard + 1 Elite, Nacht leer), geprüft von `checkPool` in `engine/sim/data.go`.
- `crystal.json` setzt `lair: true` (Bau des Endbosses, `engine/level/level.go` platziert die Entität `lair`, `lairX`); `cave.json` und `crystal.json` führen `veins` mit `stoneVein` bzw. `crystalVein` (Definition in `data/economy.json`).
- Längen in Units (1 Unit = 32 px); Werte stehen nur hier, nicht im Code.

## Flow

1. Go: `data.Files` bettet `biomes/*.json` ein; `engine/level/biome.go` (`LoadBiome(id)`) liest sie und erzeugt deterministisch Level aus einem Seed.
2. Client: `src/model/biome.ts` lädt per `import.meta.glob('../../data/biomes/*.json')` alle fünf Biome, sortiert nach `depth`, in `BIOMES` (`biomeForDepth`).

## Integration

- Konsumenten: `engine/level/biome.go`, `engine/net/level.go`, `engine/sim/data.go`, `engine/sim/lava.go`, `src/model/biome.ts`.
- Eingebettet über `data/embed.go` (`//go:embed *.json biomes/*.json`).
