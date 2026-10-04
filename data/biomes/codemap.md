# data/biomes/

## Responsibility

Configuration Data: ein JSON je Biom/Stufe (`forest.json` Tiefe 0, `cave.json` Tiefe 1, `mine.json` Tiefe 2) mit den festen Eckdaten für den Level-Generator.

## Design

- Gleiches Schema je Datei: `id`, `name`, `depth`, `lengthUnits {min,max}`, `chunkWidthUnits`, `hubWidthUnits`, `exitSide`, `portals {count, minDistanceFromHubUnits}`, `chunkWeights` (gewichtete Chunk-Typen, z. B. `tunnel`, `shaft`, `oreVein`), `eventChunks` (Min/Max je Ereignis wie `chest`).
- Längen in Units (1 Unit = 32 px); Werte stehen nur hier, nicht im Code.

## Flow

1. Go: `data.Files` bettet `biomes/*.json` ein; `engine/level` liest sie (`biome.go`) und erzeugt deterministisch Level aus einem Seed.
2. Client: `src/model/biome.ts` importiert die drei Dateien direkt (Vite-JSON-Import) für `BIOMES`.

## Integration

- Konsumenten: `engine/level/biome.go`, `engine/net/level.go`, `src/model/biome.ts`.
- Eingebettet über `data/embed.go` (`//go:embed *.json biomes/*.json`).
