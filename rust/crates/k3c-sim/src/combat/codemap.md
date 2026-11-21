# rust/crates/k3c-sim/src/combat/

## Responsibility

Aktiver Angriff (Port von `engine/sim/combat*.go`, docs/rules/kampf.md § 3, B-932, B-933): Variante `groups` (Schild an der
Burg macht einen Gruppenführer, Auslöser am äußersten Tor schickt die Gruppen los) und Variante `general` (Banner je
Monarch an jedem Tor, ihm folgt ein Trupp). Kein rng.

## Design

- `mod.rs`: `step_combat`, Hilfen `player_by_id`, `is_soldier`, `troop_side`, `leave_squad`, `squad_active`, `squad_target`,
  `squad_post` (Posten der Truppe im Angriff).
- `data.rs`: `CombatMode`, `combat_mode`, `group_size`, `settings` (prüft `combat` einer Welt-Datei), `chain_combat`
  (Kind-Welt gewinnt), `DEFAULT_GROUP_SIZE` 5.
- `spots.rs`: Zahlziele `CombatSpot`/`SpotKind` (`shield`, `trigger`, `banner`), `combat_spots`, `combat_target`,
  `combat_payable`, `pay_combat`, `refund_combat`, `shield_rack`, `gate_spot_x`, `combat_cost`.
- `squad.rs`: Zuordnung der Soldaten: `step_groups`, `send_groups`, `free_warriors`, `leaders_of`, `guarded_leader`,
  `take_banner`, `step_banners`, `return_banner`. Zeiger sind Indizes, bei Gleichstand gewinnt die frühere Truppe.

## Flow

`world::step::step` ruft `step_combat` je Tick vor `step_troops`: Zuordnungen pflegen, Gruppen bzw. Banner und Trupps führen.
Zahlungen an Zahlziele laufen über `economy::pay` (`pay_to_target`); `guarded_leader` hält im Tick-Ende einen Führer am
Leben, solange Soldaten seiner Gruppe stehen. Angreifende Truppen setzen `Troop::attacking`.

## Integration

- Abhängigkeiten: `state` (`World`, `Troop`, `Player`, `WorldValues`), `k3c_data` (Welt-Datei `combat`), `economy`.
- Konsumenten: `world::step`, `economy` (Zahlziele), `unit` (Posten, Angriff), `k3c-game` (Anzeige von Schild, Auslöser, Banner).
- Daten: Welt-Datei › `combat` {mode, groupSize}, `data/economy.json` › `combat`.
