# data/

## Responsibility

Configuration Data: einzige Quelle der Spielwerte (Balancing, Welten, Grafik-/Sound-Zuordnung, Werkzeug-Daten) als
JSON für natives Spiel, Simulation und Studio. Werte gehören hierher, nicht in den Code.

## Design

- Laden: `rust/crates/k3c-data/src/source.rs` liest aus dem Ordner in `K3C_DATA_DIR`, sonst `data/` neben der EXE, sonst
  die eingebetteten Dateien. Eigene Welten unter `worlds/` bleiben lokal, eingebettet wird nur `standard.json`.
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
- Welt-Schalter (`worlds` ordnet einer Welt ihre Ausprägung zu, Welten ohne Eintrag wie `standard` haben die Regel nicht, alle Zahlen vorläufig):
  - `cargo.json` (Fracht auf dem Schiff: Börse, Kiste, Plätze, drei Ladeplätze `bays`, Abschnitt `hold`), `coop.json` (Koop-Aktion Doppel-Truhe: `everyNth`, `goldFactor`),
    `festival.json` (friedliche Insel und Fest nach dem Endboss, `fleeSpeedFactor`, `exitMarginUnits`), `hermits.json` (Einsiedler mit Hütte und Hub-Platz `hubSite`),
    `mounts.json` (Welt-Reittiere mit `sprite`-Schlüssel aus `paper.json`, `mountGate`), `weather.json` (Wetter je Welt, z. B. `taifun`, `flut`).
- Darstellung und Grenzen: `look.json` (Look-Profile `k2c`/`go` des Rust-Spiels, Wahl über `K3C_LOOK`), `sizes.json` (Größenstufen als Vielfaches der Monarchen-Höhe, nur Profil k2c),
  `story.json` (Obergrenzen der Geschichte in Welt-Dateien: `maxLineChars`, `maxLexicon`).
- Balancing-Tester: `balance-targets.json` (`margin`, `frozen`, `seedMode`, `seeds`, `targets`), `balance-hints.json`
  (Richtwerte von Kennzahlen und Hebeln).
- Natives Spiel und Darstellung: `paper.json` (Zuordnung Spielobjekt → Papierschnitt-Asset je Gruppe), `sounds.json`
  (Sound-Slots), `native.json` (`stepBudgetMs`, `slowLogEverySeconds`), `stresstests.json` (`presets` des
  Testing-Menüs), `render.json` (Speicherbudget je Plattform, `xbox`).
- Asset-Werkzeuge: `asset-prompts.json` (Prompt-Vorlagen), `assets-recherche.json` (Recherche-Treffer je Slot),
  `assets-vorschlaege.json` (Asset-Vorschläge).

## Flow

1. Prozess-Start: Die Quelle wird gewählt (`K3C_DATA_DIR`, `data/` neben der EXE oder Einbettung).
2. `k3c-data` lädt die Dateien typisiert; Simulation, Level-Generator und Raum rechnen damit (Overrides einer Welt legt
   `rust/crates/k3c-sim/src/world` darüber).
3. Das native Spiel liest Darstellungswerte (`paper.json`, `sounds.json`, `light.json`, `native.json`,
   `stresstests.json`).
4. Das Studio schreibt Asset-Daten (`assets-recherche.json`, `asset-prompts.json`) und liest Werte für Doku und
   Welten-Editor.

## Integration

- Spiel: `rust/crates/k3c-data`, `k3c-sim`, `k3c-level`, `k3c-room`, `k3c-game`.
- Studio: `rust/tools/crates/k3c-studio` (`datadoc`, `world`, `assetsui`, `mcpsrv` mit `assets_*`),
  `rust/tools/crates/k3c-assetkit` (Recherche, Vorschläge), Frontend (`tools/k3c-studio/frontend/src/api/typesAssets.ts`).
- Tests: `rust/crates/k3c-data` (`cargo test`).
