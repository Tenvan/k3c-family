# rust/crates/k3c-sim/src/cargo/

## Responsibility

Fracht auf dem Schiff (RU62.1, B-967, docs/rules/stufen.md § 1, wirtschaft.md § 8): Laderaum mit Plätzen, drei Ladeplätze
am Steg (Münzen, Truppen, Material), Fracht fährt mit zur nächsten Insel. Neue Mechanik ohne Go-Orakel (Entscheidung 013).

## Design

- `mod.rs`: `Bay` (Coins, Troops, Crates), `CargoItem`, `CrateOrder`, `PayState`, `Cargo` (gehört der Insel, `Island::cargo`),
  `KIND_*`-Konstanten, `world_has_cargo`, `with_hold` (Laderaum als 4. Schiffsstufe im Wertesatz, `data/ship.json` bleibt
  unberührt), `enable_cargo`.
- `load.rs`: `check` (ohne Abzug: Laderaum voll, Vorrat unter einer Kiste, keine Truppe), `perform`, `Refusal`,
  `largest_stock`, `nearest_fighter`.
- `orders.rs`: `step_orders` (Träger-Aufträge: Träger suchen, `eta` herunterzählen, Ankunft belegt Platz), `cancel_order`.
- `arrive.rs`: `release_for_departure`, `board_next`, `unload`, `landing`, `spawn_troop`, `emit_arrival` (Ereignis `cargoArrived`).
- `step.rs`: `step_cargo` (Takt), `Ship`, `slots_of`, `refund_pay`; `view.rs`: `build_state`/`publish` (`World::cargo`, `cargo_bays`).
- `save.rs`: `CargoSave`, `to_save`, `restore` (`ISLAND_SAVE_VERSION` 10); laufende Einwürfe werden nicht gespeichert.

## Flow

`island::step_island` ruft `step_cargo` nach den Stufen und vor Reise und Wechsel. Reihenfolge: Ankunft melden, Plätze
(Verlust des Laderaums gibt Fracht zurück), Träger-Aufträge, Einwürfe der Spieler (nach Spieler-Index), Zustand für die
Anzeige. Ohne `cfg` (Insel ohne Fracht, auch `standard`) kehrt der Takt sofort zurück. Beim Wechsel: `release_for_departure`
auf der alten, `board_next` auf der neuen Insel.

## Integration

- Abhängigkeiten: `k3c_data::cargo` (`CargoFile`), `island` (`Island`, Vorrat), `state` (`PlayerCommand`, `World`).
- Konsumenten: `island` (Takt, Wechsel, Spielstand), `k3c-room`, `k3c-game` (Anzeige der Ladeplätze).
- Daten: `data/cargo.json`, `data/ship.json`.
