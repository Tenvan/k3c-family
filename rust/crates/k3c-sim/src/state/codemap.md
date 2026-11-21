# rust/crates/k3c-sim/src/state/

## Responsibility

Zustand der Simulation (Port von `engine/sim/types.go` und Verwandten): reine Daten für Eingabe, Spieler, Truppen, Gegner, Plätze und `World`, dazu
Spielstand-Format Version 1 und die Zusatzfelder für die Anzeige (Protokoll). Positionen in Units, Zeiten in Sekunden, JSON-Namen wie die Go-Tags.

## Design

- Typen: `types.rs` (`PlayerCommand`, `Player`, `Coin`, `Job`, `Troop`), `types_things.rs` (`ResourceNode`, `Plot`, `Site`, `Castle`),
  `types_enemy.rs` (`Enemy`, `Storm`, `Projectile`, `Hazard`), `types_world.rs` (`World`), `shared.rs` (Typen aus anderen Go-Dateien, `CROWN_*`).
- `island_ref.rs`: `IslandRef`, `IslandView`, `IslandEvents`, `GradeValues` (Gos `w.island`).
- Spielstand: `save.rs` (`SaveGame`, `HubSave`, `PlayerSave`, `SAVE_VERSION` 1), `save_apply.rs` (`apply_hub`, `apply_site`), `save_economy.rs` (Version 7).
- Sichten (nur lesend, sonst fehlt das Feld): `weather_view.rs`, `festival_view.rs`, `hermit_view.rs`, `cargo_view.rs`, `coop_view.rs`, `mount_view.rs`,
  `action_view.rs`, `site_view.rs`.
- Helfer: `common.rs`, `purse.rs`, `torches.rs`, `node_mark.rs`, `data.rs`, `omit.rs` (`skip_serializing_if`), `fehler.rs` (`Fehler`), `links.rs`.

## Flow

Module anderer Gruppen mutieren `World`; `state` besitzt nur Daten und kleine Zugriffe (`troop`, `node`, `new_id`). Spielstand: `SaveGame` serialisieren,
beim Laden `apply_hub` auf eine frisch erzeugte Stufe anwenden.

## Integration

- Abhängigkeiten: `k3c-data`, `serde`, `serde_json`.
- Konsumenten: jedes Modul der Crate; serialisiert über `k3c-room` in Protokoll und Spielstände, gelesen von `k3c-game`.
