# data/

## Responsibility

Configuration Data: einzige Quelle für Balancing-Werte (JSON) für den TS-Client (Import) und den Go-Server (`go:embed`); Werte gehören hierher, nicht in den Code.

## Design

- `embed.go`: Package `data` mit `Files embed.FS` über `*.json` und `biomes/*.json`.
- `buildings.json`: Schlüssel je Gebäude (`castle`, `wall`, `tower`, `gate`, `workshop`, `storage`, `farm`, `barracks`, `stairsUp`, `stairsDown`, `tavern`, `healer`, `smithy`, `armory`) mit `hp`, `buildSeconds`, `cost`, `unlockDepth`; je nach Gebäude `levels` (Ausbaustufen), `spell` (Turm), `offers`/`craftSeconds` (Werkstatt), `plantation` (Farm), `troopLimit` (Kaserne), `vagrants` (Taverne).
- `troops.json`: `vagrant`, `peasant` (mit `professions`: `miner`, `builder`, `craftsman`), `archer`, `warrior`, `eliteArcher`, `eliteWarrior`; Kampfwerte, `upgradeFrom`, `cost`.
- `enemies.json`: je Gegner `depth`, `tier` (`standard|elite`), Kampfwerte, `attacksPerSecond`, `gold`, `traits`; Trait-Parameter `kiteDistance`, `aoe`, `swarmSize`, `phases`. `waves.json`: Tabelle ab `fromWave` (`standard`, `elite`), `spawnSpreadSeconds`, `perExtraPlayer`, `depthScaling`, `stealGold` (Angriffsrate steht je Gegner, nicht mehr hier).
- `economy.json`: `purse`, Reichweiten und Intervalle, `chestGold`, `enemyResourceDrop`, `gatherables` (Baum, Fels, Kupfererz), `veins` (`stoneVein`, `copperVein`, `ironVein`, `crystalVein` mit `maxWorkers`), `storage`, `recruitCamp`, `merchant`.
- `monarch.json`: `base`, `sprintMultiplier`, `revive`, `mount`, `attack`, `tierPoints`, `skillPointSources`, `lines` (`tank`, `mage`, `healer`, `thief`) und `skills` je Skill.
- `hub.json`: `levels` (Burgausbau), `sites` und `islandSites` (Bauplätze), `wallLines`, `merchant`, `islandStartStock`, Radien, `startTroops`, `travel`. `difficulty.json`: `grades` (`dev`, `easy`, `normal`, `hard`, `ultra`) mit `waveSize`, `enemyHp`, `enemyDamage`, `defaultDefeat`, bei `easy` zusätzlich `protectedNights`.
- `sprites.json` (Sheets, Skalierung, Reittiere) und `balance-targets.json` (`margin`, `seeds`, `targets` für Balancing-Läufe). `biomes/`: siehe eigene codemap.

## Flow

1. Go: `engine/sim/data.go`, `engine/level/biome.go`, `engine/net/level.go` lesen per `data.Files`.
2. TS: `src/model/data.ts` importiert die JSON-Dateien direkt (Vite), `src/scenes/sprites.ts` importiert `sprites.json`.
3. `tools/atlas` liest `sprites.json`; `tools/k3c-dev/internal/balance` liest `balance-targets.json` und die Spielwerte.

## Integration

- Konsumenten: `engine/sim`, `engine/level`, `engine/net`, `tools/k3c-dev/internal/balance` (`bots.go`, `replay.go`, `targets.go`), `tools/atlas`, `src/model/data.ts`, `src/model/biome.ts`, `src/scenes/sprites.ts`.
- Tests: `data/embed_test.go`, Vitest-Tests wie `tests/sprites.test.ts`.
