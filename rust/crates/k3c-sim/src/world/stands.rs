//! WD1.2 (B-1204): Waldstücke und die Rodungsregel. Bäume, deren Abstand höchstens `STAND_GAP` Units ist, bilden ein
//! Waldstück; eine Lichtung überbrückt die Lücke (sie trennt nicht), der baumfreie Hub-Streifen trennt links und rechts.
//! Ein Waldstück ab `MIN_STAND_FRACTION` der Stufenbreite Länge heißt groß: Markierbar sind nur die `OUTER` äußersten
//! Bäume je Seite (nachrückend, wenn Randbäume fallen). Einzelbäume und kleine Gruppen sind frei markierbar, aber nur
//! `SMALL_LIMIT` zugleich je Stufe. Nur mit nativen Regeln auf der Oberfläche. Bäume der Plantage sind ausgenommen.
//!
//! Abweichung von „halbe Stufenbreite“ (Ticket B-1204, Annahme 2): Der Hub-Streifen teilt den Wald in zwei Hälften zu etwa
//! 0,45 der Breite; mit 0,5 wäre nie ein Waldstück groß. Darum 0,25.

use crate::state::{ResourceNode, World, WorldValues};
use crate::world::forest::{CLEARING_RADIUS, clearing_centres, forest_active};

/// Größte Lücke in Units zwischen zwei Bäumen eines Waldstücks.
pub const STAND_GAP: f64 = 3.0;
/// Zahl der markierbaren äußeren Bäume je Seite eines großen Waldstücks.
pub const OUTER: usize = 5;
/// Höchstzahl gleichzeitig markierter Bäume außerhalb großer Waldstücke je Stufe.
pub const SMALL_LIMIT: usize = 5;
/// Mindestlänge eines großen Waldstücks als Anteil der Stufenbreite.
pub const MIN_STAND_FRACTION: f64 = 0.25;

/// Ein Waldstück: Ausdehnung in x, ob groß, und die Bäume (IDs) von links nach rechts.
#[derive(Debug, Clone, PartialEq)]
pub struct Stand {
    pub start: f64,
    pub end: f64,
    pub big: bool,
    /// Alle stehenden Bäume des Stücks, nach x.
    pub trees: Vec<i64>,
    /// Die Bäume, die man gerade markieren darf (große Stücke: die äußeren 5 je Seite; kleine: leer, siehe `small_ok`).
    pub outer: Vec<i64>,
}

/// Bäume, die zählen: stehend, nicht aus der Plantage.
fn live_trees(w: &World) -> Vec<&ResourceNode> {
    let mut t: Vec<&ResourceNode> = w
        .nodes
        .iter()
        .filter(|n| n.kind == "tree" && !n.plant && !n.gone)
        .collect();
    t.sort_by(|a, b| a.x.total_cmp(&b.x).then(a.id.cmp(&b.id)));
    t
}

/// Überbrückt eine Lichtung die Lücke zwischen `a` und `b`?
fn bridged(clearings: &[f64], a: f64, b: f64) -> bool {
    clearings
        .iter()
        .any(|c| a >= c - CLEARING_RADIUS - STAND_GAP && b <= c + CLEARING_RADIUS + STAND_GAP)
}

/// Alle Waldstücke der Stufe (große und kleine Gruppen), nach x. Leer, wenn die Regel nicht gilt.
pub fn stands(w: &World, v: &WorldValues) -> Vec<Stand> {
    if !forest_active(w, v) {
        return Vec::new();
    }
    let clearings = clearing_centres(w);
    let min_len = w.level.width_units * MIN_STAND_FRACTION;
    let trees = live_trees(w);
    let mut out: Vec<Stand> = Vec::new();
    let mut i = 0;
    while i < trees.len() {
        let mut j = i;
        while j + 1 < trees.len() {
            let (a, b) = (trees[j].x, trees[j + 1].x);
            if b - a <= STAND_GAP || bridged(&clearings, a, b) {
                j += 1;
            } else {
                break;
            }
        }
        let ids: Vec<i64> = trees[i..=j].iter().map(|n| n.id).collect();
        let (start, end) = (trees[i].x, trees[j].x);
        let big = end - start >= min_len;
        let outer = if big {
            let n = ids.len();
            ids.iter()
                .enumerate()
                .filter(|(k, _)| *k < OUTER || *k + OUTER >= n)
                .map(|(_, id)| *id)
                .collect()
        } else {
            Vec::new()
        };
        out.push(Stand {
            start,
            end,
            big,
            trees: ids,
            outer,
        });
        i = j + 1;
    }
    out
}

/// Darf der Baum `n` gerade markiert werden (Regel dieses Moduls)? Bäume anderer Art und Plantage-Bäume: `true` (Aufrufer
/// prüft sie nicht hier).
pub fn tree_markable(w: &World, v: &WorldValues, n: &ResourceNode) -> bool {
    if n.kind != "tree" || n.plant || !forest_active(w, v) {
        return true;
    }
    let all = stands(w, v);
    let Some(st) = all.iter().find(|s| s.trees.contains(&n.id)) else {
        return true;
    };
    if st.big {
        return st.outer.contains(&n.id);
    }
    let marked_small = all
        .iter()
        .filter(|s| !s.big)
        .flat_map(|s| s.trees.iter())
        .filter(|id| w.node(**id).is_some_and(|t| t.marked))
        .count();
    marked_small < SMALL_LIMIT
}

#[cfg(test)]
#[path = "stands_tests.rs"]
mod stands_tests;
