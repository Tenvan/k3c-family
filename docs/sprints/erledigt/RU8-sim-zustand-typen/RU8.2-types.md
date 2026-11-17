# RU8.2 · Zustands-Typen: types

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru8
- **Abhängig von:** RU8.1
- **Tickets:** B-1014
- **Kriterien:** AC-02, AC-03

## Ziel

Die Kern-Typen aus `engine/sim/types.go` existieren in Rust mit denselben JSON-Namen.

## Kontext

Quelle `engine/sim/types.go` (394). Ziel `rust/crates/k3c-sim/src/state/`, Dateinamen nach Leitfaden § 2, Regeln aus `docs/standards/rust-sim-zustand.md`. Fehlen Level-Typen, kurze Platzhalter in `rust/crates/k3c-level/src/types.rs` anlegen (RU8.3 vervollständigt). `BTreeMap` statt `HashMap`.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/state/**`
- `rust/crates/k3c-sim/src/lib.rs`
- `rust/crates/k3c-level/src/types.rs`
- `rust/crates/k3c-level/src/lib.rs`

## Nicht-Ziele

Sim-Logik. Änderungen an Go-Code.

## Schritte

1. `types.go` ganz lesen.
2. Typen portieren, `serde`-Namen wie die Go-JSON-Tags.
3. Test: JSON-Rundlauf je Typ mit einem Beispiel aus Go-Tests oder Golden.
4. `cargo test -p k3c-sim`, `task rust:lint`.

## Fertig, wenn

- [ ] AC-02/AC-03 (Teil types): Typen mit Go-JSON-Namen, Listen als `Vec` in Go-Reihenfolge.
- [ ] `task rust:lint` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-02/AC-03 (Teil types) geprüft: Alle Typen aus `engine/sim/types.go` stehen in `rust/crates/k3c-sim/src/state/` (`types.rs`: PlayerCommand, Player, Coin, Job, Troop; `types_things.rs`: ResourceNode, Plot, Site, Castle; `types_enemy.rs`: Event, Enemy, Storm, Projectile, Hazard, Pickup, Camp, QueuedSpawn, Travel; `types_world.rs`: World samt `new_id`; `Stock` = `k3c_data::hub::Stock`). Typen anderer Go-Dateien, die der Zustand braucht, stehen als reine Daten in `shared.rs` (Drop, Crown, Merchant, WarnZone, ActiveEvent, SwitchState, SwitchTarget, CycleInfo, CombatRuntime, CombatSettings, SkillTable, WorldValues). Level-Typen in `rust/crates/k3c-level/src/types.rs` (Chunk, Entity, Layout, WallSite, Ruin, Guard, NatureValues, Planned*; `Biome`, `Cycle`, `Trait`, `WeightGroup` aus k3c-data wiedergegeben). JSON-Namen wie die Go-Tags; `omitempty` über `state/omit.rs`; Listen als `Vec` in Go-Reihenfolge, Maps als `BTreeMap`. Tests: 14 Rundlauf-Tests (Sim-Golden, alle Snapshots, nur lesen) in `types_tests.rs`, 3 Layout-Tests über die Level-Golden. `cargo test -p k3c-sim -p k3c-level`, `task rust:lint`, `task check:diff` grün.
Abweichungen: (1) `World` hat kein Feld `island` (Vorentscheidung 4). (2) Die Zwischenspeicher `Site.hubW/hubLv/wall` entfallen (Rückverweis auf die Welt, reine Ableitung). (3) `Plot.tree` ist die Knoten-ID statt Zeiger. (4) `k3c-level/Cargo.toml` bekam `serde` und `serde_json` (dev), nötig für die Level-Typen. (5) `Island`, `WorldDef`, `BossData`-Verhalten gehören nicht zu types.go und kommen mit RU10; `lair_boss` nutzt `k3c_data::bosses::BossData`. Keine Tests fehlen.
