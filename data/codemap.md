# data/

## Responsibility

Configuration Data: einzige Quelle für Balancing-Werte (JSON) für den TS-Client (Import) und den Go-Server (`go:embed`); Werte gehören hierher, nicht in den Code.

## Design

- `embed.go`: Package `data` mit `Files embed.FS` über `*.json` und `biomes/*.json`.
- Dateien: `buildings.json`, `troops.json`, `enemies.json`, `waves.json` (Tabelle ab `fromWave`), `economy.json` (Börse, Reichweiten, Drops), `monarch.json` (Basiswerte, Level, Presets), `hub.json` (Bauplätze, Inselstart, Starttruppen), `difficulty.json` (Faktoren je Grad), `sprites.json` (Sheets, Skalierung, Reittiere), `balance-targets.json` (Zielkorridore und Seeds für Balancing-Läufe); `biomes/` siehe eigene codemap.

## Flow

1. Go: `engine/sim/data.go`, `engine/level/biome.go`, `engine/net/level.go` lesen per `data.Files`.
2. TS: `src/model/data.ts` importiert die JSON-Dateien direkt (Vite), `src/scenes/sprites.ts` importiert `sprites.json`.
3. `tools/atlas` liest `sprites.json`; `tools/k3c-dev/internal/balance` liest `balance-targets.json` und die Spielwerte.

## Integration

- Konsumenten: `engine/sim`, `engine/level`, `engine/net`, `tools/k3c-dev/internal/balance` (`bots.go`, `replay.go`, `targets.go`), `tools/atlas`, `src/model/data.ts`, `src/model/biome.ts`, `src/scenes/sprites.ts`.
- Tests: `data/embed_test.go`, Vitest-Tests wie `tests/sprites.test.ts`.
