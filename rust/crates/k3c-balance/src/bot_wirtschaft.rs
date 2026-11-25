//! Profil „Wirtschaft“ (Port von `engine/balance/bot_wirtschaft.go`, BAL6, B-347): der Bot des Standardszenarios. Er
//! spielt die Wirtschaft so, dass jeder Korridor aus docs/rules/zielkorridore.md › BR1 einen Messwert hat: Mauern, Turm,
//! Baumschule und Werkstatt bauen, den Hub ausbauen (Material markieren oder beim Händler kaufen), Holz nachmarkieren
//! und Kämpfer schaffen (Bögen, Landstreicher). Ob ein Platz Münzen nimmt und wie viel noch fehlt, liest er über
//! `gold_left`; er zahlt nur, wenn das eigene Gold den Rest deckt, sonst kämen die Münzen beim Loslassen zurück.
//! Die Materialsuche (Knoten, Schiff) steht in `bot_material.rs`.

use k3c_data::buildings::Cost;
use k3c_sim::island::Island;
use k3c_sim::state::{Player, PlayerCommand, Site, World};

use crate::bot_clear::{WantSet, clear_node};
use crate::bot_material::{amount_of, cost_of, covers, node_for, ship_step};
use crate::bots::{closer, economy_view, go_to, gold_left, nearest_vagrant, recruit_or_hub};
use crate::prices::prices;

/// Holz, das er im Vorrat halten will (Korridor ≥ 100 plus ein Bogen); darunter markiert er Bäume.
pub const WOOD_WANT: i64 = 150;
/// So viele Bauern wirbt er höchstens an (Arbeiter und Träger der Bögen); ab 6 fehlt das Gold für Mauern und Bögen, Nacht
/// 5 vernichtet alles (B-645, Hebel 3).
pub const PEASANT_LIMIT: i64 = 4;
/// So viele Kämpfer will er (Korridor Tag 6 ≥ 8); darüber spart er das Gold für den Hub.
pub const FIGHTER_WANT: i64 = 8;

/// Unbezahlte Plätze in dieser Reihenfolge, je Art der nächste bezahlbare.
const BUILD_ORDER: [&str; 4] = ["wall", "tower", "nursery", "bowyer"];

/// Gilt unter Tage: die Werkstatt zuerst, ohne sie entstehen keine Bögen, und jede Welle räumt sie ab (B-652).
const DEEP_BUILD_ORDER: [&str; 4] = ["bowyer", "wall", "tower", "nursery"];

fn build_order_of(w: &World) -> &'static [&'static str; 4] {
    if w.biome.depth >= 1 {
        &DEEP_BUILD_ORDER
    } else {
        &BUILD_ORDER
    }
}

/// Wohin der Hub-Ausbau den Bot schickt: an die Burg (Go: Zeiger auf `w.Castle.X`, zahlt sofort) oder an einen Ort.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HubGo {
    Castle,
    At(f64),
}

impl HubGo {
    /// Der Ort in Units.
    pub fn x(self, w: &World) -> f64 {
        match self {
            HubGo::Castle => w.castle.x,
            HubGo::At(x) => x,
        }
    }
}

/// Wirtschaft: nachts an der Burg; tags in dieser Reihenfolge: Bauten aus `BUILD_ORDER`, deren Material im Vorrat liegt;
/// der Hub-Ausbau; Holz nachmarkieren; Bögen an der Werkstatt; Landstreicher anwerben. Liegt das Material des
/// Hub-Ausbaus bereit, spart er dessen Gold: alles danach nur mit Gold über dieser Rücklage.
pub fn wirtschaft(isl: &Island, w: &World, p: &Player) -> PlayerCommand {
    if w.cycle.phase == "night" {
        return go_to(p, w.hub_x, false);
    }
    if peasants(w) == 0 && nearest_vagrant(w, p).is_some() {
        // ohne Arbeiter bleibt jeder bezahlte Bau liegen (waitingWorker): erst anwerben
        return recruit_or_hub(w, p);
    }
    let (hub, reserve) = hub_step(isl, w, p);
    if hub == Some(HubGo::Castle) {
        // Material und Gold für den Hub-Ausbau liegen bereit: zuerst zahlen, vor neuen Bauten
        return go_to(p, w.castle.x, true);
    }
    if let Some(x) = fighter_step(w, p, reserve) {
        // Kämpfer vor Wiederaufbau: Unter Tage räumt jede Welle Mauern und Türme ab, die Armee trägt
        return go_to(p, x, true);
    }
    if let Some(x) = build_x(isl, w, p, reserve) {
        return go_to(p, x, true);
    }
    if let Some(x) = ship_step(isl, w, p, reserve) {
        return go_to(p, x, true);
    }
    let mut x = hub.map(|h| h.x(w));
    if x.is_none() && reserve == 0 {
        x = node_for(
            isl,
            w,
            p,
            &Cost {
                wood: WOOD_WANT,
                ..Cost::default()
            },
        )
        .map(|n| n.x);
    }
    // Go ruft hier `fighter_step` ein zweites Mal; mit denselben Eingaben lieferte der Aufruf oben schon nichts.
    if let Some(x) = x {
        return go_to(p, x, true);
    }
    if p.gold >= prices().recruit_gold + reserve && peasants(w) < PEASANT_LIMIT {
        return recruit_or_hub(w, p);
    }
    go_to(p, w.hub_x, false)
}

/// Der Ort des nächsten Baus (`next_build`) oder, wenn nichts bezahlbar ist und keine Rücklage nötig, des nächsten Baums
/// oder Felsens, der einen Mauer- oder Turm-Platz sperrt (B-652); `None` sonst.
pub fn build_x(isl: &Island, w: &World, p: &Player, reserve: i64) -> Option<f64> {
    if let Some(s) = next_build(isl, w, p, reserve) {
        return Some(s.x);
    }
    if reserve == 0 {
        return clear_node(w, p).map(|n| n.x);
    }
    None
}

/// Der nächste Bau aus `BUILD_ORDER`, dessen Material im Vorrat liegt und dessen Rest plus Rücklage das Gold deckt.
pub fn next_build<'a>(isl: &Island, w: &'a World, p: &Player, reserve: i64) -> Option<&'a Site> {
    if waiting_material(w) {
        return None;
    }
    let none = Cost::default();
    for kind in build_order_of(w) {
        if let Some(s) = nearest_payable(w, p, kind, "unpaid", reserve)
            && covers(&isl.stock, prices().cost.get(*kind).unwrap_or(&none))
        {
            return Some(s);
        }
    }
    None
}

/// Ort zum Zahlen für den Hub-Ausbau (Burg, Knoten oder Händler) oder die Rücklage, solange das Gold fehlt.
pub fn hub_step(isl: &Island, w: &World, p: &Player) -> (Option<HubGo>, i64) {
    let Some(up) = economy_view(w).hub_upgrade else {
        return (None, 0);
    };
    if !up.state.is_empty() {
        return (None, 0);
    }
    let need = Cost {
        wood: up.material.wood,
        stone: up.material.stone,
        copper: up.material.copper,
        iron: up.material.iron,
        crystal: up.material.crystal,
        ..Cost::default()
    };
    let left = up.gold - up.paid;
    let covered = covers(&isl.stock, &need);
    if covered && p.gold >= left {
        return (Some(HubGo::Castle), 0);
    }
    if covered {
        return (None, left);
    }
    if let Some(n) = node_for(isl, w, p, &need) {
        return (Some(HubGo::At(n.x)), 0);
    }
    if let Some(m) = &w.merchant
        && amount_of(&isl.stock, &m.resource) < cost_of(&need, &m.resource)
        && p.gold >= prices().merchant_gold - m.buy_paid
    {
        return (Some(HubGo::At(w.hub_x + prices().merchant_buy_offset)), 0);
    }
    (None, 0)
}

/// Der nächste Bogner, der einen Bogen nimmt (Gold für den Rest plus Rücklage), solange ein Bauer ihn holen kann und
/// weniger als `fighter_target` Kämpfer da sind; `None` sonst.
pub fn fighter_step(w: &World, p: &Player, reserve: i64) -> Option<f64> {
    if peasants(w) == 0 || economy_view(w).fighters >= fighter_target(w) {
        return None;
    }
    nearest_payable(w, p, "bowyer", "built", reserve).map(|s| s.x)
}

/// Der nächste Platz der Art im Zustand `state`, der Münzen nimmt und dessen Rest plus Rücklage das Gold deckt.
pub fn nearest_payable<'a>(w: &'a World, p: &Player, kind: &str, state: &str, reserve: i64) -> Option<&'a Site> {
    let mut best: Option<&Site> = None;
    let want = WantSet::new(w);
    for s in &w.sites {
        if s.kind != kind || s.state != state || (state == "unpaid" && !want.wanted(s)) {
            continue;
        }
        let (left, ok) = gold_left(w, s);
        if ok && left > 0 && p.gold >= left + reserve && best.is_none_or(|b| closer(p.x, s.x, b.x)) {
            best = Some(s);
        }
    }
    best
}

/// Ein bezahlter Bau wartet noch auf Material; dann zahlt er keinen weiteren, damit das Material nicht auf viele Plätze
/// verteilt liegen bleibt.
pub fn waiting_material(w: &World) -> bool {
    w.sites.iter().any(|s| s.state == "waitingMaterial")
}

/// Die Zahl der Bauern der Stufe.
pub fn peasants(w: &World) -> i64 {
    w.troops.iter().filter(|t| t.kind == "peasant").count() as i64
}

/// So viele Kämpfer will der Bot in dieser Stufe; in der Stufe mit dem Bau des Endbosses sind es `ENDBOSS_WANT`, damit das
/// Heer den Endboss tragen kann (B-652), sonst `FIGHTER_WANT`.
pub fn fighter_target(w: &World) -> i64 {
    if w.level.entities.iter().any(|e| e.kind == "lair") {
        return crate::pending::ENDBOSS_WANT;
    }
    FIGHTER_WANT
}
