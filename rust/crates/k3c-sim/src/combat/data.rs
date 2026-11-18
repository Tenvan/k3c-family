//! Kampfvariante je Welt (Port der Abfragen aus `engine/sim/combat_data.go`, docs/rules/kampf.md § 3, B-932, B-933):
//! Welt-Datei › `combat` {mode, groupSize}, Werte in data/economy.json › combat. Die Lader und Prüfungen
//! (`rawCombat.settings`, `chainCombat`, `combatData`) liegen in `k3c_data` und `state::data`; hier stehen
//! `combatMode` und `groupSize` samt Konstanten (RU15.2 holt sie für `combat/mod.rs` vor, die Session RU15.3 nennt sie).

use crate::state::WorldValues;

/// Go: `CombatGroups`: Gruppen mit Schild und Auslöser am Tor (Standard).
pub const COMBAT_GROUPS: &str = "groups";
/// Go: `CombatGeneral`: Banner je Monarch, ihm folgt ein Trupp.
pub const COMBAT_GENERAL: &str = "general";
/// Go: `DefaultGroupSize`: Soldaten je Gruppe neben dem Führer (kampf.md § 3.1).
pub const DEFAULT_GROUP_SIZE: i64 = 5;

/// Die Kampfvariante der Stufe: Go `CombatGroups` oder `CombatGeneral` als Aufzählung.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatMode {
    Groups,
    General,
}

/// Go: `combatMode`: die Kampfvariante der Stufe (fehlt sie, groups).
pub fn combat_mode(v: &WorldValues) -> CombatMode {
    if v.combat.mode == COMBAT_GENERAL {
        CombatMode::General
    } else {
        CombatMode::Groups
    }
}

/// Go: `groupSize`: die Soldaten je Gruppe neben dem Führer.
pub fn group_size(v: &WorldValues) -> i64 {
    if v.combat.group_size > 0 {
        v.combat.group_size
    } else {
        DEFAULT_GROUP_SIZE
    }
}
