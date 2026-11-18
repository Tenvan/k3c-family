//! Aktiver Angriff (Port von `engine/sim/combat.go`, docs/rules/kampf.md § 3, B-932, B-933). Variante groups: Schild an der
//! Burg (Zahlziel `shield`) macht einen Soldaten zum Gruppenführer, ein Auslöser am äußersten Tor (`trigger`, je Seite)
//! schickt alle Gruppen der Seite los. Variante general: ein Banner je Monarch an jedem Tor (`banner`), ihm folgt ein Trupp.
//! Angreifende Truppen setzen `Troop::attacking` und beschädigen so Portale (portal.go). Zahlziele laufen über `input.pay`
//! (economy.go › findPayTarget). Kein rng; Reihenfolge der Truppen und Spieler bestimmt jede Auswahl.
//!
//! `spots.rs` hält die Zahlziele (erster Teil von `combat.go`), hier stehen der Takt und die Posten der Trupps.

pub mod data;
pub mod spots;
pub mod squad;

use data::{CombatMode, combat_mode};
use squad::{step_banners, step_groups};

use crate::state::common::troop_by_id;
use crate::state::{Player, Troop, World, WorldValues};

pub use spots::{
    CombatSpot, CombatSpotState, SpotKind, combat_cost, combat_payable, combat_spot_near, combat_spots, combat_target,
    gate_spot_x, pay_combat, refund_combat, shield_rack, shields_unlocked, trigger_useful,
};

/// Go: `stepCombat` (läuft je Tick vor `stepTroops`): Zuordnungen pflegen, Gruppen bzw. Banner und Trupps führen,
/// Angriffsflag setzen.
pub fn step_combat(w: &mut World, v: &WorldValues) {
    if combat_mode(v) == CombatMode::General {
        step_banners(w, v);
    } else {
        step_groups(w, v);
    }
    for ti in 0..w.troops.len() {
        if w.troops[ti].group != 0 {
            w.troops[ti].attacking = squad_active(w, v, &w.troops[ti]);
        }
    }
}

/// Go: `playerByID`: der Monarch mit der ID `id`, sonst `None`.
pub fn player_by_id(w: &World, id: i64) -> Option<&Player> {
    w.players.iter().find(|p| p.id == id)
}

/// Go: `isSoldier`: Krieger oder Elite-Krieger (Gruppen nehmen nur Soldaten).
pub fn is_soldier(t: &Troop) -> bool {
    t.kind == "warrior" || t.kind == "eliteWarrior"
}

/// Go: `troopSide`: Seite (-1 links, 1 rechts) der Truppe, nach ihrem Anker.
pub fn troop_side(w: &World, t: &Troop) -> f64 {
    if t.anchor_x < w.hub_x { -1.0 } else { 1.0 }
}

/// Go: `leaveSquad`: löst die Truppe `ti` aus Gruppe oder Trupp.
pub fn leave_squad(w: &mut World, ti: usize) {
    if w.troops[ti].shield_fetch {
        w.combat.shields += 1; // das Schild ging nicht verloren: ein anderer Soldat holt es
    }
    let t = &mut w.troops[ti];
    t.leader = false;
    t.group = 0;
    t.sortie = 0.0;
    t.shield_fetch = false;
    t.attacking = false;
}

/// Go: `squadActive`: Ist die Gruppe bzw. der Trupp von `t` gerade im Angriff (Führer mit Auftrag, Monarch mit Banner)?
pub fn squad_active(w: &World, v: &WorldValues, t: &Troop) -> bool {
    if combat_mode(v) == CombatMode::General {
        return player_by_id(w, t.group).is_some_and(|p| p.banner != 0);
    }
    troop_by_id(w, t.group).is_some_and(|l| l.leader && l.sortie != 0.0)
}

/// Go: `squadTarget`: Ort des nächsten Ziels vor `anchor` in Richtung `dir`: der nächste Gegner innerhalb der
/// Anspähweite, sonst das nächste Portal (mit `all_portals` unbegrenzt, sonst ebenfalls innerhalb der Anspähweite).
pub fn squad_target(w: &World, v: &WorldValues, anchor: f64, dir: f64, all_portals: bool) -> Option<f64> {
    let c = &v.economy.combat;
    let reach = c.engage_units;
    let mut best: Option<f64> = None;
    for e in &w.enemies {
        let d = (e.x - anchor) * dir;
        if d >= -c.behind_units && d <= reach && best.is_none_or(|b| d < (b - anchor) * dir) {
            best = Some(e.x);
        }
    }
    if best.is_some() {
        return best;
    }
    for &x in &w.portals {
        let d = (x - anchor) * dir;
        if d >= -c.behind_units && (all_portals || d <= reach) && best.is_none_or(|b| d < (b - anchor) * dir) {
            best = Some(x);
        }
    }
    best
}

/// Go: `squadPost`: Ort, zu dem `t` im Angriff geht (Posten der Gruppe bzw. des Trupps); `None` = normaler Posten am Tor.
pub fn squad_post(w: &World, v: &WorldValues, t: &Troop) -> Option<f64> {
    if t.shield_fetch {
        return Some(w.hub_x);
    }
    if t.group == 0 || !squad_active(w, v, t) {
        return None;
    }
    let general = combat_mode(v) == CombatMode::General;
    let (anchor, dir, all) = if general {
        let p = player_by_id(w, t.group)?;
        (p.x, p.banner as f64, false)
    } else {
        let l = troop_by_id(w, t.group)?;
        (l.x, l.sortie, true)
    };
    let c = &v.economy.combat;
    let range = v.troops.get(t.kind.as_str()).map_or(0.0, |d| d.range);
    let reach = (range * c.stand_range_factor).max(1.0);
    if let Some(x) = squad_target(w, v, anchor, dir, all) {
        return Some(x - dir * reach);
    }
    if general {
        // ohne Ziel folgt der Trupp dem Monarchen: Soldaten voraus, Schützen dahinter
        return Some(if is_soldier(t) {
            anchor + dir * c.follow_units
        } else {
            anchor - dir * c.follow_units
        });
    }
    None
}

#[cfg(test)]
mod data_tests;
#[cfg(test)]
mod gate_tests;
#[cfg(test)]
mod real_tests;
#[cfg(test)]
mod review_tests;
#[cfg(test)]
mod squad_tests;
#[cfg(test)]
mod state_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) mod testutil;
