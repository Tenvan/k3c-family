//! Kampfvariante je Welt (Port der Abfragen aus `engine/sim/combat_data.go`, docs/rules/kampf.md § 3, B-932, B-933):
//! Welt-Datei › `combat` {mode, groupSize}, Werte in data/economy.json › combat. Die Lader und Prüfungen
//! (`combatData`, Rohform und Prüfung der Welt-Datei) liegen in `k3c_data`; hier stehen die Konstanten, `settings`
//! (Go: `rawCombat.settings`), `chain_combat`, `combat_mode` und `group_size`.

use k3c_data::world::RawCombat;

use crate::state::WorldValues;
use crate::state::shared::CombatSettings;

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

/// Go: `rawCombat.settings`: prüft `combat` einer Welt-Datei; `None` gibt die Standardwerte (leer). Meldungstexte wie Go.
pub fn settings(r: Option<&RawCombat>) -> Result<CombatSettings, String> {
    let mut c = CombatSettings::default();
    let Some(r) = r else {
        return Ok(c);
    };
    if let Some(mode) = &r.mode {
        if mode != COMBAT_GROUPS && mode != COMBAT_GENERAL {
            return Err(format!("combat.mode {mode:?} unbekannt (groups oder general)"));
        }
        c.mode.clone_from(mode);
    }
    if let Some(n) = r.group_size {
        if n < 1 {
            return Err("combat.groupSize muss eine ganze Zahl ≥ 1 sein".to_string());
        }
        c.group_size = n;
    }
    Ok(c)
}

/// Go: `chainCombat`: setzt die Kampfvariante aus der Kette (älteste Welt zuerst) zusammen; die Kind-Welt gewinnt
/// feldweise. Go liest `WorldDef.Combat`; bis `WorldDef` mit RU10.5 kommt, ist die Kette die Liste der `combat`-Einstellungen
/// je Welt (`&[CombatSettings]`).
pub fn chain_combat(chain: &[CombatSettings]) -> CombatSettings {
    let mut out = CombatSettings::default();
    for c in chain {
        if !c.mode.is_empty() {
            out.mode.clone_from(&c.mode);
        }
        if c.group_size > 0 {
            out.group_size = c.group_size;
        }
    }
    out
}
