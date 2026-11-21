# rust/crates/k3c-sim/src/boss/

## Responsibility

Minibosse und Endbosse (Port von `engine/sim/boss*.go`, docs/rules/bosse.md): Erscheinen, Fähigkeiten, Phasen, Belohnung.
Ein besiegter Boss steht in `IslandRef::defeated_bosses` und kommt nie wieder.

## Design

- `mod.rs`: `trigger_boss` (Miniboss kommt am Wellenstart ab seiner Welle), `spawn_boss`/`place_boss` (Werte des
  Bezugs-Gegners mal Boss-Faktoren, kein rng), `boss_ability` (Fähigkeit fällig?), `boss_defeated` (Belohnung: Gold als
  Münzen, Material in den Insel-Vorrat bis zum Lager-Maximum). `*_with`-Varianten nehmen einen bereits geholten `WorldValues`.
- `data.rs`: Abfragen `boss_by_id`, `end_boss`, `mini_boss` über `k3c_data::bosses::BossesFile` (Lader liegen in `k3c-data`).
- `abilities.rs`: `arm_boss`, `flame_trail` (Lava-Golem, Bodenflächen), `shard_throw` (Splitter-Wurf), `hit_splash`
  (Splash-Einschlag von Geschossen), `step_hazards` (Flächen auslaufen lassen, Schaden einmal je Sekunde).
- `endboss.rs`: `EndbossFight` (Boss-ID, Phase, Heimat-Stufe, `escort`, `parked`), `step_endboss`, `endboss_phase`
  (Phasenwechsel an HP-Schwellen, einmal gemeldet als `bossPhase`), `phase_tempo`, `endboss_defeated`, `boss_ability_of`,
  `at_lair`, `lair_x`, `player_near`, `park_endboss` (Burgfall nimmt den Endboss aus `w.enemies`, `step_endboss` stellt ihn zurück).

## Flow

`world::step::step` ruft `step_endboss` vor `step_enemies` und `step_hazards` danach. Der Endboss liegt im Bau (`lair`) und
existiert nicht als Gegner, bis ein lebender Spieler näher als `lair.triggerUnits` ist; dann entsteht er (Ereignis
`bossSpawned`), kämpft am Bau (Tempo 0) und bleibt bis zum Tod aktiv, auch über einen Burgfall. Sieg setzt
`IslandRef::endboss_defeated`. Minibosse laufen als normale Gegner über das erste Portal, ihre Fähigkeit feuert in `step_enemies`.

## Integration

- Abhängigkeiten: `k3c_data::bosses`, `state` (`World`, `Enemy`, `IslandRef`, `WorldValues`), `enemies`.
- Konsumenten: `links.rs` (`trigger_boss`, `boss_ability`, `boss_defeated`, `hit_splash`), `world::step`, `dev::combat`
  (`spawn_boss`, `endboss_phase`), `k3c-game` (`at_lair`).
- Daten: `data/bosses.json`.
