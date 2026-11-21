# rust/crates/k3c-sim/src/campaign/

## Responsibility

Kampagne als Altpfad vor den Inseln (Port von `engine/sim/campaign.go`): alle Stufen eines Spielstands, jede mit eigenem Hub.
Dient Golden-Kampagnen, Tests und dem lokalen Spiel (`k3c-game/src/local/campaign.rs`).

## Design

- `mod.rs`: `Campaign` (`id`, `seed`, `cycle_speed`, `depth`, `unlocked_depth`, `worlds` je Tiefe, `pending_hubs`,
  `player_gold`, `vals`), `new_world` (erzeugt eine Stufe, neutrale Natur, `Err` statt Go-`panic` bei fehlendem Biom),
  `create`/`create_with`, `join_player`, `current_world(_mut)`, `world_for`, `travel`.
- `save.rs`: `Campaign::to_save`, `from_save`, `from_save_with` (Rest des Spielstands in `state/save.rs`).

## Flow

Der Aufrufer rechnet `step`; ist `w.travel.progress >= 1`, ruft er `travel(to_depth)`. `world_for` erzeugt die Zielstufe
bei Bedarf (mit gespeichertem Hub per `apply_hub`), gleicht Zeit und Zyklus an (wartende Gegner behalten Abstand).
`travel` nimmt Spieler mit neuer ID neben die Burg, löscht Krone und Zahlung, lässt Truppen, Gebäude und Vorräte im Hub und
meldet das Ereignis `arrived`.

## Integration

- Abhängigkeiten: `world` (`create_world`, `add_player`, `Options`, `base`, `cycle_at`), `state::save` (`SaveGame`, `HubSave`), `island::data::biome_for_depth`.
- Konsumenten: `k3c-game` (`local/campaign`, `local/session`, HUD und View), `k3c-room`.
