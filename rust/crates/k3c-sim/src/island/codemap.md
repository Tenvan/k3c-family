# rust/crates/k3c-sim/src/island/

## Responsibility

Insel (Port von `engine/sim/island*.go`, Entscheidung 003): ein Level mit mehreren Stufen, die im selben Takt laufen; jeder Spieler ist an eine
Stufe gebunden, das Material gehört der Insel, Gold dem Spieler. Dazu Welt-Dateien, Optionen, Sieg und Niederlage, Spielstand und Inselwechsel.

## Design

- `mod.rs`: `Island`, `step_island` (Takt aller Stufen, danach Aufträge, Einsiedler, Fest, Fracht, Reise, Wechsel, Sieg).
- `create.rs`: `CreateOptions`, `create_island`, `create_island_in`, `create_island_at`. `harbor_start.rs`: Start am Hafen mit gerodeten Bäumen (HB1.5).
- Welt-Datei: `data.rs` (`IslandDef`, `check_island`), `worlds.rs` (`WorldDef`, `load_world`, `parse_world`, `standard_world`), `worlds_decode.rs`
  (strenges Lesen), `keys.rs` (Schlüssel in Datei-Reihenfolge), `sizes.rs` (Chunk-Anzahlen), `nature.rs`, `endboss.rs` (Endboss je Insel), `targets.rs` (Etappen).
- Regeln: `options.rs` (`IslandOptions`, Ziel/Niederlage), `victory.rs` (`check_victory`), `defeat.rs` (`defeat_losses`, `game_over`),
  `storage.rs` (Lager-Maximum), `errand.rs` (Ausrüstung von oben, `ascend`).
- Wechsel: `travel.rs` (Einzelwechsel der Stufe), `switch.rs` (Inselwechsel am Steg).
- Spielstand: `save.rs` (`IslandSave`, `ISLAND_SAVE_VERSION` 12), `save_parse.rs`, `save_load.rs` (`from_island_save`), `save_v3.rs`, `save_v5.rs`,
  `save_v8.rs` (Migration auf Welt und Insel-ID).

## Flow

Welt-Datei lesen und prüfen, `create_island` baut Stufen aus Biom, Seed und Plan, `step_island` rechnet pro Takt `world::step` mit `IslandRef`,
Sieg oder Niederlage beenden die Insel. Speichern schreibt `IslandSave`, Laden prüft und baut sie neu auf.

## Integration

- Abhängigkeiten: `world`, `state`, `unit`, `story`, `hub`, `k3c-level`, `k3c-rng`, `k3c-data`.
- Daten: `data/worlds/<id>.json`, `data/bosses.json`, `data/island-sizes.json`, `data/difficulty.json`.
- Konsumenten: `k3c-room` (Raum, Spielstände), `k3c-game`, `k3c-balance`, `merchant`, `monarch`, `portal`.
