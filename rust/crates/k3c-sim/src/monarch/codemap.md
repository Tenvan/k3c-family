# rust/crates/k3c-sim/src/monarch/

## Responsibility

Monarch (Port von `engine/sim/monarch*.go`, docs/rules/monarch.md): Schlag, Fund-Pool der Insel, Skills und Passive lernen (Verteilen, Respec,
Presets), passive Werte und Reittier.

## Design

- `mod.rs`: `step_attack`, `pool_of`, `add_pool_points`, `chest_skill_point`, `available_points` (Pool minus gelernte Skills), `line_count`,
  `tier_need`, `can_learn`; Zeiger auf Spieler sind `pi`, die Insel `Option<&mut IslandRef>`.
- `data.rs`: `new_skill_table`, `skill_ref`, `skill_by_id`, `check_presets` (Lese-Typen in `k3c_data::monarch`).
- `passives.rs`: Gelernte Passive wirken immer: `each_passive`, `speed_of`, `damage_mult_of`, `cooldown_mult_of`, `troop_damage`, `step_passives`,
  `frost_armor_hit`; Eigenschaften werden berechnet, nicht gehalten; nur lebende Besitzer.
- `mount.rs`: `mount_of` (Reittier aus dem Wertesatz).

## Flow

Fund (Truhe, Boss) erhöht den Pool; `can_learn` prüft Punkte und Tier, Lernen trägt den Skill beim Spieler ein. Der Tick liest die Passive für Tempo,
Schaden und Abklingzeit; aktive Skills laufen in `skills`.

## Integration

- Abhängigkeiten: `state`, `world` (`WorldValues`), `island`, `skills`, `k3c-data`.
- Daten: `data/monarch.json` (Skills, Presets, Mounts, Krone).
- Konsumenten: `world/step.rs`, `skills`, `combat`, `unit`, `mounts`; Spielstand `PlayerSave`.
