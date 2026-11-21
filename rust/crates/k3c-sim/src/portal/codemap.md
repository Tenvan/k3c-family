# rust/crates/k3c-sim/src/portal/

## Responsibility

Portale (Port von `engine/sim/portal.go`, docs/rules/kampf.md § 2, B-927): Jedes Portal hat HP; nur Truppen im Angriff beschädigen es. Solange sie
angreifen, spawnt es Verteidiger. Sind alle zerstört, ist die Stufe dauerhaft sicher.

## Design

- `mod.rs` (einzige Datei): `safe`, `sync_portals`, `portal_in_reach`, `step_portals`, `defend_portal`, `destroy_portal`, `standing_portal_near`,
  `secure_stage`, `drop_loot`, `apply_portals`.
- Zerstörte Portale stehen in `World::portals_down` und im Spielstand (`HubSave::portals_down`).
- Gegner, Türme, Mauern und Monarch beschädigen Portale nie; im Frieden (`World::peace`) keine Verteidiger-Wellen.

## Flow

`world::step` ruft `step_portals` nach den Truppen. Greift eine Truppe an, sinkt die HP; `defend_portal` spawnt Verteidiger. Bei 0 HP
`destroy_portal`, am Ende `secure_stage`. `apply_portals` stellt den Zustand aus dem Spielstand wieder her.

## Integration

- Abhängigkeiten: `state` (`World`, `Troop`), `island` (`IslandRef`), `enemies`, `world` (`WorldValues`).
- Daten: `data/biomes/*.json › portals` (HP, Verteidigung).
- Konsumenten: `world/step.rs`, `festival/flee.rs` (Fluchtziel, `drop_loot`), `unit`, `state/save_apply.rs`.
