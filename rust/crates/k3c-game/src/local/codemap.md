# rust/crates/k3c-game/src/local/

## Responsibility

Die lokale Sitzung des nativen Spiels (Port von `native/k3c-game/local/`): führt die Simulation im selben Prozess über `k3c-room`, synchron aus der Spielschleife getaktet, mit Einstellungen, Spielständen, Welten, Kampagnen-Stand und Seekarten-Route.

## Design

- `session.rs`: `Session` (`open`, `Started`): ein Gerät mit bis zu `MAX_PLAYERS` Slots, kein Thread je Raum. `view_impl.rs` implementiert `view::session::Session` für die Menüs, `dev.rs` die `DevSession` (Cheats, Zeitraffer, Halt, Neu-Laden).
- Takt: `step.rs` (`STEP_HZ`, Befehle, Ausgang, `impl Sim for Session`), `peer.rs` (`LocalPeer` als `room::Peer`, `StageState`: Stufen eines Takts), `actions.rs` (Geräte tauschen, Skills, Insel wählen).
- Stand: `campaign.rs` (`Campaign::follow`/`frame`), `route.rs` (`RouteCache`, `route_list`), `worlds.rs` (Standard-Welt aus `k3c_data`, weitere Welten aus einem Ordner, Umgebungsvariable `K3C_WORLDS_DIR`).
- Ablage: `saves.rs` (`<saves>/<name>.json`, Liste, Löschen, freier Name), `watched.rs` (`WatchedStore`, `SaveResults`), `settings.rs` (`_settings.json`, `load_settings`/`save_settings`), `config.rs`, `error.rs` (`UserError`).

## Flow

`game::advance` ruft `Sim::step`: Befehle der Eingabe gehen je Slot an den Raum, `LocalPeer` empfängt Zustand und Ereignisse, `Frame` (mit Kampagne und Route) wird gebaut; Speicherungen laufen über `WatchedStore` und werden gemeldet. Menüs erreichen die Sitzung nur über den `view::session::Session`-Vertrag.

## Integration

- Abhängigkeiten: `k3c_room`, `k3c_sim`, `k3c_level`, `k3c_data`, `view` (Vertrag), `game` (`Sim`).
- Konsumenten: `game::launch` (`open_session`), `game::shots_run`, `game::devwinhost` (`game_data`).
