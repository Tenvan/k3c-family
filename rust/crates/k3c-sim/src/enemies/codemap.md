# rust/crates/k3c-sim/src/enemies/

## Responsibility

Gegner (Port von `engine/sim/enemies*.go`): Spawnen, Wellen, Zielwahl und Angriff, Traits, Lager-Wachen, Ausrüstungsraub,
Geschosse und Drops. Sie laufen vom Portal auf die Burg zu, eine intakte Mauer hält sie auf.

## Design

- `mod.rs`: `spawn_enemy`, `spawn_scaled` (Grad-Faktoren der Insel), `spawn_with`, `step_spawns`, `step_enemies`,
  `send_enemies_home` (Tagesanbruch), `leave_through_portal`, `stunned`, `is_barrier`, `is_building`, `BODY`. Zeiger sind Indizes (`ei`).
- `target.rs`: `Target`/`TargetKind`/`Wall`, `blocking_wall`, `choose_target` (Taunt, Händler, Rang), `attack`;
  `for_each_candidate` läuft ohne Liste (kein Allokieren im Takt).
- `traits.rs`: `attack_rate`, `strike` (Trait `aoe`), `invulnerable(_in)` (`phases`), `kite` (`kiting`).
- `waves.rs`: `plan_wave`, `spawn_orders`, `SpawnOrder`, `start_wave`, `queue_wave`, `step_projectiles`, `target_x`,
  `remove_dead_enemies`, `enemy_drop`. Die Würfe haben feste Reihenfolge (Schleife, Schwarm, Elite, Entscheidung 012).
- `guard.rs`: Wachen der Planung (`spawn_guards`, `step_guard`, `keep_guards`, `mark_cleared`, `guards_cleared_keys`, `apply_guards`).
- `equipment_taken.rs`: `take_equipment`, `drop_carried` (Gegner mit `grabRange` tragen Drops zum Portal).

## Flow

`world::step::step`: `spawn_guards` und `step_spawns` früh; `step_enemies` nach `step_endboss`; danach `step_projectiles` und
`remove_dead_enemies` (Gold, Ressource, Kronen und Ausrüstung fallen). `start_wave` kommt vom Zyklus bei Nachtbeginn
(`full_moon_extra` ergänzt Vollmond). Bei Frieden (`w.peace`) rechnet `festival::flee` statt `step_enemies`.

## Integration

- Abhängigkeiten: `k3c_data::enemies`, `state`, `crown`, `boss`, `events`, `economy`, `portal`, `building`, `unit` (über `links.rs`).
- Konsumenten: `world::step`, `portal`, `boss`, `dev`, `merchant`, `k3c-game` (Zeichnen der Gegner).
- Daten: `data/enemies.json`, `data/difficulty.json`.
