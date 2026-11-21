# rust/crates/k3c-sim/src/skills/

## Responsibility

Aktive Skills (Port von `engine/sim/skills*.go`, docs/rules/monarch.md § 3): Der Befehl `skill` löst den Skill in Slot 1 bis 4 aus; die Wirkung
steht in `monarch.json › skills › effect`, `effect.type` wählt die Funktion.

## Design

- `mod.rs`: `cast_effect` (`match` statt Gos `skillEffects`-Map, liefert `Option<bool>`), `step_skills` (Abklingzeiten je Spieler und Slot).
- `caster.rs`: Fireball, Ice Wall, Lightning Storm, Meteor (`cast_area_damage`, `cast_slow`, `cast_damage_over_time`, `step_storms`).
- `healer.rs`: `cast_heal`, `cast_group_heal`, `cast_revive_crown`; Divine Shield nutzt `cast_shield`.
- `tank.rs`: `cast_taunt`, `cast_stun`, `cast_shield`, `cast_last_stand`.
- `thief.rs`: `cast_backstab`, `cast_stealth`, `behind_enemy`.
- `status.rs`: `Aura`, `auras_of`: Status für die Anzeige, nur lesend, ohne rng.
- Alle `cast_*` haben dieselbe Signatur; Schaden läuft über `apply_damage`.

## Flow

Befehl `skill` mit Slot → `step_skills` prüft Abklingzeit → `cast_effect` verzweigt nach `effect.type` → Wirkung auf Gegner, Truppen oder Krone,
Abklingzeit startet nur bei Erfolg (ohne Ziel keine Abklingzeit).

## Integration

- Abhängigkeiten: `state`, `monarch` (Passive, `skill_ref`), `combat`/`enemies` (`apply_damage`), `k3c-data` (`SkillEffect`).
- Daten: `data/monarch.json`.
- Konsumenten: `monarch`, `economy/players.rs`, `world/step.rs` (`step_storms` über `step`).
