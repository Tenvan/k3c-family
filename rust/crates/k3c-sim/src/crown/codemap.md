# rust/crates/k3c-sim/src/crown/

## Responsibility

Krone (Port von `engine/sim/crown.go`, docs/rules/kampf.md §§ 5 und 6, B-930, B-931): Der Monarch hat keine HP, jeder Treffer
kostet Münzen, ohne Gold fällt die Krone nach hinten. Gegner tragen sie ins Portal (Kronenraub = Game Over).

## Design

- `mod.rs`: Zustände `CROWN_FLYING`, `CROWN_GROUND`, `CROWN_ENEMY`, `CROWN_PLAYER`; `crowned`, `crown_direction` (vom
  nächsten Gegner weg, deterministisch), `drop_crown(_with)` (Ereignis `crownFallen`), `step_crowns`, `restore_crown`,
  `take_crown_or_loot`, `drop_crown_of`, `crown_stolen`, `hit_coins(_with)` (Münzen eines Treffers fallen neben den Monarchen).

## Flow

`world::step::step` ruft `step_crowns` nach `step_players`: Flug, Mitnehmen durch Träger, Aufheben durch Monarche. Gegner
heben Kronen in `enemies::step_enemies` über `take_crown_or_loot` auf; im Portal gilt `crown_stolen`, ohne Portale in der
Stufe lässt der Gegner die Krone fallen. Treffer am Monarchen laufen über `hit_coins`.

## Integration

- Abhängigkeiten: `state` (`World`, `Player`, `Enemy`, `IslandRef`, `WorldValues`), `events`.
- Konsumenten: `links.rs` (`crown_stolen`, `drop_crown_of`, `take_crown_or_loot`), `world::step`, `enemies`, `monarch`, `k3c-game` (`crowned` für Aktionssperre).
