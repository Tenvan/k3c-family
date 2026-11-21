# rust/crates/k3c-sim/src/dev/

## Responsibility

Dev-Aktionen der Insel (Port von `engine/sim/dev*.go`, B-178, B-603, B-766): nur für den Dev-Mode des Raums, keine Spielregel.
Alle Go-`Dev*(isl *Island, …)` sind hier freie Funktionen `dev_*(isl: &mut Island, …)`.

## Design

- `mod.rs`: `DevFlags`, `pure` (reiner Stressmodus), `on_stage` (rechnet eine Stufe wie `step_island` mit `IslandRef` und
  getauschtem Vorrat), `dev_drop_gold`, `dev_add_stock`, `dev_start_wave`, `dev_castle_fall`, `dev_set_phase`,
  `dev_next_phase`, `dev_raise_max_stock`, `dev_fill_stock`, `dev_set_pure`, `dev_set_cheap_build`, `dev_add_shield`.
- `build.rs`: `dev_build_all`, `dev_build_site`, `dev_ready_to_sail` (Schiff fertig, Endboss besiegt).
- `combat.rs`: `dev_spawn_boss`, `dev_boss_phase`, `dev_start_event` (Vollmond, Blutmond, Händler-Überfall).
- `spawn.rs`: `dev_spawn_units`, `dev_spawn_at_castle`, `dev_spawn_x`, `DEV_MAX_SPAWN` 200; gleichmäßig verteilt, kein rng.

## Flow

Der Raum ruft die Funktionen aus seinen Dev-Befehlen auf. Sie nutzen die Abläufe des Spiels (`spawn_boss`, `endboss_phase`,
Nachtbeginn, Händler-Überfall) und damit deren rng-Strom, sonst keinen. `difficulty` ist die Grad-Tabelle wie bei `step_island`.

## Integration

- Abhängigkeiten: `island` (`Island`), `boss`, `enemies`, `events`, `world`, `k3c_data` (`DifficultyFile`).
- Konsumenten: `k3c-room` (`room/dev.rs`, `dev_combat.rs`, `dev_extra.rs`, `fullsave.rs`), `links.rs` (`pure`), `k3c-game` (Cheat-Fenster über den Raum).
