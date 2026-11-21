//! Gruppen (Variante groups, B-932) und Banner-Trupps (Variante general, B-933): Zuordnung der Soldaten und Schützen
//! (Port von `engine/sim/combat_squad.go`). Kein rng. Go-Zeiger auf Truppen und Spieler sind Indizes (`ti`, `pi`);
//! Listen von Truppen sind `Vec<usize>` in Truppen-Reihenfolge, bei Gleichstand gewinnt die frühere (nur `<`).

use serde_json::json;

use super::data::group_size;
use super::{is_soldier, leave_squad, player_by_id, squad_target, troop_side};
use crate::state::common::{event, outer_wall};
use crate::state::links::{crowned, emit, unit_x};
use crate::state::{World, WorldValues};
use crate::unit::warrior::is_fighter;

/// Go: `freeWarriors`: Soldaten ohne Gruppe, Schild-Weg und Führerrolle, in Truppen-Reihenfolge (Indizes).
pub fn free_warriors(w: &World) -> Vec<usize> {
    (0..w.troops.len()).filter(|&ti| is_free_warrior(w, ti)).collect()
}

fn is_free_warrior(w: &World, ti: usize) -> bool {
    let t = &w.troops[ti];
    is_soldier(t) && t.group == 0 && !t.shield_fetch && t.job.is_none() // EQ1.1: Rüstungsholer bleibt frei von Gruppen
}

/// Go: `countFetching`: Zahl der Truppen, die gerade ihr Schild holen.
pub fn count_fetching(w: &World) -> i64 {
    w.troops.iter().filter(|t| t.shield_fetch).count() as i64
}

/// Go: `leadersOf`: die Gruppenführer der Seite `side` (Indizes).
pub fn leaders_of(w: &World, side: f64) -> Vec<usize> {
    (0..w.troops.len())
        .filter(|&ti| w.troops[ti].leader && troop_side(w, &w.troops[ti]) == side)
        .collect()
}

/// Go: `nearestTroop`: aus `ts` die Truppe, die `x` am nächsten ist (bei Gleichstand die frühere); `None` bei leerer
/// Liste.
pub fn nearest_troop(w: &World, ts: impl IntoIterator<Item = usize>, x: f64) -> Option<usize> {
    let mut best: Option<usize> = None;
    for ti in ts {
        if best.is_none_or(|b| (w.troops[ti].x - x).abs() < (w.troops[b].x - x).abs()) {
            best = Some(ti);
        }
    }
    best
}

/// Go: `groupCount`: Soldaten der Gruppe des Führers `li` neben ihm.
pub fn group_count(w: &World, li: usize) -> i64 {
    let id = w.troops[li].id;
    w.troops
        .iter()
        .enumerate()
        .filter(|&(ti, t)| t.group == id && ti != li)
        .count() as i64
}

/// Go: `guardedLeader`: Ein Führer fällt nicht, solange Soldaten seiner Gruppe stehen (kampf.md § 9: Vorschlag keine
/// Wirkung).
pub fn guarded_leader(w: &World, ti: usize) -> bool {
    w.troops[ti].leader && group_count(w, ti) > 0
}

/// Go: `stepGroups`: pflegt die Gruppen: Schilde vergeben, Führer ernennen, Gruppen füllen, Einsatz beenden ohne Ziel.
pub fn step_groups(w: &mut World, v: &WorldValues) {
    for ti in 0..w.troops.len() {
        // Mitglieder ohne Führer (er ist gefallen) sind wieder frei
        let t = &w.troops[ti];
        if t.group != 0 && !t.leader && !crate::state::common::troop_by_id(w, t.group).is_some_and(|l| l.leader) {
            leave_squad(w, ti);
        }
    }
    while w.combat.shields > 0 {
        let free = (0..w.troops.len()).filter(|&ti| is_free_warrior(w, ti));
        let Some(ti) = nearest_troop(w, free, w.hub_x) else {
            break;
        };
        w.troops[ti].shield_fetch = true;
        w.combat.shields -= 1;
    }
    let arrive = v.economy.combat.shield_arrive_units;
    for ti in 0..w.troops.len() {
        let t = &w.troops[ti];
        if t.shield_fetch && (t.x - w.hub_x).abs() < arrive {
            let (id, x) = (t.id, t.x);
            let t = &mut w.troops[ti];
            t.shield_fetch = false;
            t.leader = true;
            t.group = id;
            emit(w, "shield", event(json!({"troop": id, "x": unit_x(x)})));
        } else if t.leader {
            fill_group(w, v, ti);
            end_sortie(w, v, ti);
        }
    }
}

/// Go: `fillGroup`: holt einen freien Soldaten der Seite in die Gruppe des Führers `li` (einen je Tick), bis `groupSize`
/// erreicht ist.
fn fill_group(w: &mut World, v: &WorldValues, li: usize) {
    if group_count(w, li) >= group_size(v) {
        return;
    }
    let side = troop_side(w, &w.troops[li]);
    let cand = (0..w.troops.len()).filter(|&ti| is_free_warrior(w, ti) && troop_side(w, &w.troops[ti]) == side);
    if let Some(ti) = nearest_troop(w, cand, w.troops[li].x) {
        w.troops[ti].group = w.troops[li].id;
    }
}

/// Go: `endSortie`: Hat die Gruppe kein Ziel mehr voraus, kehrt sie ans Tor zurück.
fn end_sortie(w: &mut World, v: &WorldValues, li: usize) {
    let l = &w.troops[li];
    if l.sortie == 0.0 {
        return;
    }
    if squad_target(w, v, l.x, l.sortie, true).is_none() {
        let (id, x) = (l.id, l.x);
        w.troops[li].sortie = 0.0;
        emit(w, "groupReturned", event(json!({"troop": id, "x": unit_x(x)})));
    }
}

/// Go: `sendGroups`: schickt alle Gruppen der Seite los (Auslöser am Tor, kampf.md § 3.1).
pub fn send_groups(w: &mut World, side: f64) {
    let leaders = leaders_of(w, side);
    for &li in &leaders {
        w.troops[li].sortie = side;
    }
    emit(
        w,
        "attackOrdered",
        event(json!({"side": side as i64, "groups": leaders.len()})),
    );
}

/// Go: `takeBanner`: Der Monarch `pi` nimmt das Banner am Tor der Seite `side` auf (Variante general).
pub fn take_banner(w: &mut World, pi: usize, side: i64) {
    let p = &mut w.players[pi];
    p.banner = side;
    p.banner_out = false;
    let index = p.index;
    emit(w, "bannerTaken", event(json!({"player": index, "side": side})));
}

/// Go: `stepBanners`: Banner zurückstellen, Trupps der Monarchen mit Banner füllen.
pub fn step_banners(w: &mut World, v: &WorldValues) {
    for pi in 0..w.players.len() {
        if w.players[pi].banner == 0 {
            continue;
        }
        let side = w.players[pi].banner as f64;
        let Some(wall_x) = outer_wall(w, side).map(|s| s.x) else {
            return_banner(w, pi);
            continue;
        };
        if !crowned(&w.players[pi]) {
            return_banner(w, pi);
            continue;
        }
        if (w.players[pi].x - wall_x) * side > 0.0 {
            w.players[pi].banner_out = true;
        } else if w.players[pi].banner_out {
            // hinter das äußerste Tor zurück
            return_banner(w, pi);
            continue;
        }
        fill_squad(w, v, pi, wall_x);
    }
    for ti in 0..w.troops.len() {
        // Trupp ohne Monarch mit Banner löst sich auf
        let g = w.troops[ti].group;
        if g != 0 && !player_by_id(w, g).is_some_and(|p| p.banner != 0) {
            leave_squad(w, ti);
        }
    }
}

/// Go: `returnBanner`: stellt das Banner des Monarchen `pi` am Tor wieder auf; sein Trupp kehrt zurück.
pub fn return_banner(w: &mut World, pi: usize) {
    let (id, index) = (w.players[pi].id, w.players[pi].index);
    w.players[pi].banner = 0;
    w.players[pi].banner_out = false;
    for ti in 0..w.troops.len() {
        if w.troops[ti].group == id {
            leave_squad(w, ti);
        }
    }
    emit(w, "bannerReturned", event(json!({"player": index})));
}

/// Go: `fillSquad`: ergänzt den Trupp des Monarchen `pi` auf Soldaten und Schützen (je Art einen je Tick) vom Tor
/// `gate_x`; ein gefallener Kämpfer ist so nach einem Tick ersetzt (kampf.md § 3.2).
fn fill_squad(w: &mut World, v: &WorldValues, pi: usize, gate_x: f64) {
    let c = &v.economy.combat;
    let (pid, banner) = (w.players[pi].id, w.players[pi].banner as f64);
    let (mut soldiers, mut archers) = (0, 0);
    let (mut best_s, mut best_a): (Option<usize>, Option<usize>) = (None, None);
    let nearer = |best: Option<usize>, ti: usize| {
        best.is_none_or(|b| (w.troops[ti].x - gate_x).abs() < (w.troops[b].x - gate_x).abs())
    };
    for (ti, t) in w.troops.iter().enumerate() {
        if !is_fighter(&v.troops, &t.kind) {
            continue;
        }
        if t.group == pid {
            if is_soldier(t) {
                soldiers += 1;
            } else {
                archers += 1;
            }
        } else if t.group != 0 || t.job.is_some() || troop_side(w, t) != banner {
            continue;
        } else if is_soldier(t) {
            if nearer(best_s, ti) {
                best_s = Some(ti);
            }
        } else if nearer(best_a, ti) {
            best_a = Some(ti);
        }
    }
    if soldiers < c.troop_soldiers
        && let Some(ti) = best_s
    {
        w.troops[ti].group = pid;
    }
    if archers < c.troop_archers
        && let Some(ti) = best_a
    {
        w.troops[ti].group = pid;
        w.troops[ti].tower_id = None;
    }
}
