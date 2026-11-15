# data/

## Responsibility

Configuration Data: einzige Quelle der Spielwerte (Balancing, Welten, Grafik-/Sound-Zuordnung, Werkzeug-Daten) als
JSON für Engine, natives Spiel und Workbench. Werte gehören hierher, nicht in den Code.

## Design

- `embed.go` (Package `data`): `//go:embed *.json biomes/*.json worlds/standard.json`; `Files` (Interface `FS` =
  `fs.ReadFileFS` + `fs.ReadDirFS`) liest aus dem Ordner in `DirEnv` (`K3C_DATA_DIR`), sonst eingebettet (`choose`);
  `LoadedAt` ist der Ladezeitpunkt. Eigene Welten unter `worlds/` bleiben lokal, eingebettet wird nur `standard.json`.
- Spielregeln und Balancing:
  - `buildings.json` (Gebäude mit `hp`, `buildSeconds`, `cost`, Stufen, Spezialfelder je Art), `troops.json`,
    `enemies.json` (Gegner mit Tiefe, Stufe, Kampfwerten, Traits), `waves.json` (Wellentabelle, Mehrspieler-Zuschläge,
    `depthScaling`, `stealGold`), `bosses.json` (`bosses`, `endBosses`), `events.json` (Nacht-Events).
  - `economy.json` (Börse, Zahlziele `payRangeUnits`, Morgengold je Spieler und Tiefe, Sammelbares, Adern, Lager,
    Händler), `monarch.json` (Basiswerte, Reittier, Skill-Linien, `skills`, `presets`), `hub.json` (Hub-Stufen, `sites`,
    `islandSites`, `sitePlacement`, Händler, Startvorrat, Reisen), `difficulty.json` (`grades`), `goals.json`
    (Siegvarianten), `island-sizes.json` (erlaubte Chunk-Anzahlen), `guards.json` (Gegnerlager), `ruins.json`
    (`hpShare`, `kinds`), `ship.json` (Ausbau-Abschnitte des Schiffs), `light.json` (Nachtlicht, Fackeln).
  - `biomes/` (eigene codemap) und `worlds/standard.json` (Standard-Welt mit Inseln, Tiefen, Planung).
- Balancing-Tester: `balance-targets.json` (`margin`, `frozen`, `seedMode`, `seeds`, `targets`), `balance-hints.json`
  (Richtwerte von Kennzahlen und Hebeln).
- Natives Spiel und Darstellung: `paper.json` (Zuordnung Spielobjekt → Papierschnitt-Asset je Gruppe), `sounds.json`
  (Sound-Slots), `native.json` (`stepBudgetMs`, `slowLogEverySeconds`), `stresstests.json` (`presets` des
  Testing-Menüs), `render.json` (Speicherbudget je Plattform, `xbox`).
- Asset-Werkzeuge: `asset-prompts.json` (Prompt-Vorlagen), `assets-recherche.json` (Recherche-Treffer je Slot),
  `assets-vorschlaege.json` (Asset-Vorschläge).

## Flow

1. Prozess-Start: `data.Files` wird einmal gewählt (Platte über `K3C_DATA_DIR` oder Einbettung).
2. `engine/sim/data.go` lädt per `load[T]` in Paket-Variablen; `engine/sim/world_values.go` legt Overrides einer Welt
   darüber; `engine/level` liest Biome, `hub.json`, `economy.json`, `buildings.json`, `ruins.json`, `guards.json`.
3. Das native Spiel liest Darstellungswerte (`paper.json`, `sounds.json`, `light.json`, `native.json`,
   `stresstests.json`) über `data.Files`.
4. Die Workbench schreibt Asset-Daten (`assets-recherche.json`, `asset-prompts.json`) und liest Werte für Doku und
   Welten-Editor.

## Integration

- Go-Konsumenten: `engine/sim`, `engine/level`, `engine/balance`, `engine/enginetools`, `cmd/k3c-sim`;
  `native/k3c-game` (`actors/catalog.go`, `art/manifest.go`, `audio/register.go`, `world/light.go`,
  `devtools/chronicle.go`, `menu/stress_presets.go`, `props/data.go`).
- Workbench: `tools/k3c-workbench/internal/datadoc`, `internal/gamedata`, `internal/world`, `internal/assetsui`,
  `internal/mcpsrv` (`assets_*`), `tools/assetkit` (`research.go`, `suggestions.go`), Frontend
  (`frontend/src/api/typesAssets.ts`).
- Tests: `data/embed_test.go`.
