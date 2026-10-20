# data/

## Responsibility

Configuration Data: einzige Quelle für Balancing-Werte (JSON) für den TS-Client (Import) und den Go-Server (`go:embed`); Werte gehören hierher, nicht in den Code.

## Design

- `embed.go`: Package `data` mit `Files embed.FS` über `*.json` und `biomes/*.json`.
- `buildings.json`: Schlüssel je Gebäude (`castle`, `wall`, `tower`, `gate`, `workshop`, `storage`, `farm`, `barracks`, `stairsUp`, `stairsDown`, `tavern`, `healer`, `smithy`, `armory`) mit `hp`, `buildSeconds`, `cost`, `unlockDepth`; je nach Gebäude `levels` (Ausbaustufen), `spell` (Turm), `offers`/`craftSeconds` (Werkstatt), `plantation` (Farm), `troopLimit` (Kaserne), `vagrants` (Taverne).
- `troops.json`: `vagrant`, `peasant` (mit `professions`: `miner`, `builder`, `craftsman`), `archer`, `warrior`, `eliteArcher`, `eliteWarrior`; Kampfwerte, `upgradeFrom`, `cost`.
- `enemies.json`: je Gegner `depth`, `tier` (`standard|elite`), Kampfwerte, `attacksPerSecond`, `gold`, `traits`; Trait-Parameter `kiteDistance`, `aoe`, `swarmSize`, `phases`. `waves.json`: Tabelle ab `fromWave` (`standard`, `elite`), `spawnSpreadSeconds`, `perExtraPlayer`, `depthScaling` (nur Welten ohne Insel, Inseln nutzen ihre Welt-Datei), `stealGold` (Angriffsrate steht je Gegner).
- `economy.json`: `purse`, Reichweiten und Intervalle, `chestGold`, `enemyResourceDrop`, `gatherables` (Baum, Fels, Kupfererz), `veins` (`stoneVein`, `copperVein`, `ironVein`, `crystalVein` mit `maxWorkers`), `storage`, `recruitCamp`, `merchant`.
- `monarch.json`: `base`, `sprintMultiplier`, `revive`, `mount`, `attack`, `tierPoints`, `skillPointSources`, `lines` (`tank`, `mage`, `healer`, `thief`) und `skills` je Skill.
- `hub.json`: `levels` (Burgausbau), `sites` und `islandSites` (Bauplätze), `wallLines`, `merchant`, `islandStartStock`, Radien, `startTroops`, `travel`. `difficulty.json`: `grades` (`dev`, `easy`, `normal`, `hard`, `ultra`) mit `waveSize`, `enemyHp`, `enemyDamage`, `defaultDefeat`, bei `easy` zusätzlich `protectedNights`.
- `bosses.json`: `bosses` (Minibosse je Stufe: `kind`, `depth`, `wave`, `base`, `hpFactor`, `damageFactor`, `ability` `summon|aoe|flameTrail|shardThrow`, `reward`) und `endBosses` (`lair`, `phases`). `events.json`: Nacht-Events `fullMoon`, `bloodMoon`, `merchantRaid` (nur Inseln). `worlds/standard.json`: Standard-Welt (Welt-Datei nach `docs/rules/welten.md`: `id`, `name`, `notes`, `islands` mit `depths`, `depthScaling`, `plannedChunks`, `chunkCounts`); als einzige Welt eingebettet, eigene Welten unter `worlds/` bleiben lokal (`.gitignore`). `goals.json`: Startwerte der Siegvarianten (`gold`, `days`).
- `assets.json`: Slot-Register (`slots` mit `art` grafik|sound, `kategorie`, `wahl`), geschrieben von k3c-dev und vom Asset-Studio. `assets-recherche.json`: Recherche-Treffer je Slot (`treffer`: Link, Quelle, Urheber, Lizenz, `lizenzStatus`), geschrieben von k3c-dev (`assets_hit_add`, `assets_research`).
- `island-sizes.json`: erlaubte Chunk-Anzahlen einer Stufe (`sizes`: Name, `chunkCount`, alle Biome), gesetzt je Insel und Tiefe in der Welt-Datei › `chunkCounts`; gelesen von `engine/sim/island_sizes.go`.
- `sprites.json` (Sheets, Skalierung, Reittiere) und `balance-targets.json` (`margin`, `seeds`, `targets` für Balancing-Läufe). `biomes/`: siehe eigene codemap.

## Flow

1. Go: `engine/sim/data.go` (`load[...]`, u. a. `goals.json`), `boss_data.go`, `island_worlds.go`, `events.go`, `engine/level/biome.go`, `engine/net/level.go` lesen per `data.Files`.
2. TS: `src/model/data.ts` importiert die JSON-Dateien direkt (Vite), `src/scenes/render/sprites.ts` importiert `sprites.json`.
3. `tools/atlas` liest `sprites.json`; `tools/assetkit/register.go` (genutzt von `tools/k3c-dev/internal/mcpsrv/tools_assets.go` und `tools/k3c-studio/backend`) liest und schreibt `assets.json`; `tools/k3c-dev/internal/balance` liest `balance-targets.json` und die Spielwerte.

## Integration

- Konsumenten: `engine/sim`, `engine/level`, `engine/net`, `tools/k3c-dev/internal/balance` (`bots.go`, `replay.go`, `targets.go`), `tools/atlas`, `tools/assetkit`, `tools/k3c-studio`, `src/model/data.ts`, `src/model/biome.ts`, `src/scenes/render/sprites.ts`.
- Tests: `data/embed_test.go`, Vitest-Tests wie `tests/sprites.test.ts`.
