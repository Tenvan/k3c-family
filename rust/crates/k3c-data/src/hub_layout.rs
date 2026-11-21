//! Abstand der Hub-Plätze nach der Gebäudebreite (LA3.1, B-1182). Nur Rust: `data/hub.json` bleibt wie für Go
//! (Golden), `load_game` rückt die Plätze beim Laden nach außen, bis zwischen zwei Gebäuden die Lücke aus
//! `data/sizes.json › bauplaetze` frei ist. Die Reihenfolge je Seite bleibt, die Burg steht in der Mitte. Treppen
//! (Breite 0) bleiben stehen. Der feste Baugrund der Zone 1 (Turm, Mauer) rückt hinter das äußerste Gebäude beider
//! Seiten, damit er kein Gebäude überdeckt.

use crate::hub::{HubData, HubSite};
use crate::sizes::SiteWidths;

/// Breite einer Platzart in Units; unbekannte Arten gelten als Treppe (0, bleiben stehen).
fn width_of(w: &SiteWidths, kind: &str) -> f64 {
    w.widths.get(kind).copied().unwrap_or(0.0)
}

/// Äußerster Rand der Gebäude einer Seite nach dem Spreizen; schiebt die Plätze von der Burg nach außen. Ein Platz
/// bleibt, wo er ist, wenn er schon genug Abstand hat. Fehler: ein Mittelpunkt läge jenseits von `limit` Units.
fn spread_side(h: &mut HubData, w: &SiteWidths, side: f64, limit: f64) -> Result<f64, String> {
    let mut refs: Vec<&mut HubSite> = h
        .sites
        .iter_mut()
        .chain(h.island_sites.iter_mut())
        .filter(|s| s.offset_units * side > 0.0 && width_of(w, &s.kind) > 0.0)
        .collect();
    refs.sort_by(|a, b| a.offset_units.abs().total_cmp(&b.offset_units.abs()));
    let mut edge = w.castle / 2.0;
    for s in refs {
        let half = width_of(w, &s.kind) / 2.0;
        let at = s.offset_units.abs().max(edge + w.gap + half);
        if at > limit {
            return Err(format!(
                "Platz {} liegt bei {at} jenseits der Hub-Breite (±{limit})",
                s.kind
            ));
        }
        s.offset_units = side * at;
        edge = at + half;
    }
    Ok(edge)
}

/// `spread` spreizt die Plätze beider Seiten und schiebt Turm und Mauer der Zone 1 hinter das äußerste Gebäude. Der Aufruf
/// ist stabil: ein zweiter ändert nichts.
pub fn spread(h: &mut HubData, w: &SiteWidths, limit: f64) -> Result<(), String> {
    let edge = spread_side(h, w, -1.0, limit)?.max(spread_side(h, w, 1.0, limit)?);
    let (tower, wall) = (width_of(w, "tower"), width_of(w, "wall"));
    let p = &mut h.site_placement;
    p.zone1_tower_units = p.zone1_tower_units.max(edge + w.gap + tower / 2.0);
    let beside = (tower / 2.0 + w.gap + wall / 2.0).max(w.pay_gap);
    p.zone1_wall_units = p.zone1_wall_units.max(p.zone1_tower_units + beside);
    Ok(())
}

#[cfg(test)]
#[path = "hub_layout_tests.rs"]
mod tests;
