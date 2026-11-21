# rust/crates/k3c-sim/src/world/

## Responsibility

Eine Stufe (Port von `engine/sim/world*.go`): Startzustand aus Biom und Seed, der Tick mit verbindlicher Teilschritt-Reihenfolge, Tag/Nacht-Zyklus,
Wald, Adern, Lava, Schiff, Stufenwechsel, dazu der Wertesatz je Welt (Basis aus `data/` plus Overrides).

## Design

- `mod.rs`: `Options`, `create_world`, `add_player`, `vals`, `hub_of`, `level_sites`, `castle_fallen`. `step.rs`: `step` (Reihenfolge Zyklus, Spawns,
  Spieler, Kronen, Passive, Lager, Plätze, Kampf, Truppen, Portale, Endboss, Gegner oder `step_flee`, Lava, Geschosse, Flut, Reise).
- Spielwelt: `cycle.rs` (`step_cycle`, `cycle_at`), `forest.rs` und `stands.rs` (Wald, Waldstücke, Rodung), `vein.rs`, `lava.rs`, `lines.rs`
  (Mauer- und Turmplätze), `ship.rs` (Schiff am Hafen), `travel.rs`, `state_mirror.rs`, `preview.rs` (Vorschau des Betrachters), `skins.rs`.
- Wertesatz: `values.rs` (`resolve_world_values`, `world_chain`, `MAX_WORLD_CHAIN`), `values_from.rs`, `values_merge.rs` (`merge_file`),
  `values_locked.rs` (nicht übersteuerbare Felder), `values_shape*.rs` (Formtabellen), `values_room.rs` (`default_values`, `values_of`), `base.rs`
  (eingebettete Daten), `hub_sites.rs` (`check_world_plans`), `links.rs`.

## Flow

Wertesatz einmal beim Anlegen oder Laden auflösen (Kette `extends` bis 4, Overrides mergen, prüfen), `create_world` baut die Stufe, `step` rechnet je
Takt. Die Insel ruft `step` für jede Stufe mit `IslandRef`.

## Integration

- Abhängigkeiten: `state`, `island`, `unit`, `economy`, `combat`, `enemies`, `portal`, `weather`, `festival`, `k3c-level`, `k3c-rng`, `k3c-data`.
- Daten: `data/*.json` (Basis), `overrides` in `data/worlds/<id>.json`.
- Konsumenten: `island`, `campaign`, `k3c-room`, `k3c-balance`, Workbench (Wertesatz- und Welten-Prüfung).
