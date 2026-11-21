# rust/crates/k3c-sim/src/festival/

## Responsibility

Friedliche Insel und Fest (RU62.12, B-973): Nach dem Sieg über den Endboss enden alle Angriffe in allen Stufen der Insel, der Zustand nennt
`festival` bis zur Abfahrt. Neue Mechanik nur in Rust, ohne Go-Orakel.

## Design

- `mod.rs`: `Festival` (Zustand der Insel), `is_peace(w, isl)`, `Island::enable_festival`, `Island::prepare_festival`, `step_festival`; Ereignis
  `festivalStart` genau einmal je Sieg. Der Frieden ist abgeleitet (`Island::endboss_defeated`), nichts steht im Spielstand.
- `flee.rs`: `step_flee` und `exit_x`: lebende Gegner brechen ab, lassen Krone, Ausrüstung und Gold fallen (`portal::drop_loot`), fliehen zum
  nächsten Portal oder Inselrand und verschwinden.
- Gilt nur in Welten aus `festival.json › worlds` und beim Ziel „Endboss“ (`options.goal`).

## Flow

`island::step_island` ruft `prepare_festival` vor den Stufen und `step_festival` danach. In `world::step` ersetzt `step_flee` den Gegner-Takt,
solange `World::peace` gilt; `world/cycle.rs`, `portal` und `events/merchant.rs` lesen dasselbe Flag (keine Wellen, keine Verteidiger, Besuch
ohne Überfall).

## Integration

- Abhängigkeiten: `state` (`World`, `FestivalState`), `island`, `portal`, `k3c-data` (`festival.rs`).
- Daten: `data/festival.json`.
- Konsumenten: `world`, `portal`, `events`, `island`; die Anzeige (RU62.13) liest dieselbe Datei.
