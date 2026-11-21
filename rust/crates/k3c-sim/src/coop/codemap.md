# rust/crates/k3c-sim/src/coop/

## Responsibility

Koop-Aktionen (RU62.3, B-968, docs/rules/koop.md; neue Mechanik ohne Go-Orakel): Eine Doppel-Truhe öffnet nur, wenn zwei
gesteuerte Monarchen gleichzeitig davorstehen, und wirft mehr Gold (`goldFactor`).

## Design

- `mod.rs`: `world_has_coop` (`coop.json › worlds`, nicht `standard`), `is_double_chest(seed, stage, x, every_nth)`
  (deterministisch, ohne rng-Strom), `enable_coop`, `prepare_coop` (Methoden der Insel).
- `chest.rs`: `begin` (Zustand für die Anzeige je Takt neu), `is_on` (Welt mit Koop-Regeln und mindestens zwei gesteuerten
  Monarchen), `is_marked` (Zwischenspeicher je Truhen-ID), `try_open` (`true` = Truhe bleibt liegen).

## Flow

`island::step_island` ruft `prepare_coop` vor den Stufen. Beim Einsammeln (`economy::players::collect_pickups`) fragt die
Wirtschaft `try_open` je Truhe; offene Truhen werfen Gold wie sonst, mit Faktor.

## Integration

- Abhängigkeiten: `k3c_data::coop`, `state` (`World`, `Pickup`, `IslandRef`), `island` (`Island`).
- Konsumenten: `island` (Vorbereitung), `economy` (Einsammeln), `k3c-game` (Anzeige der Doppel-Truhe).
- Daten: `data/coop.json`.
