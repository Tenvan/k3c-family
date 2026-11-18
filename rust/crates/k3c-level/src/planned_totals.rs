//! Biom-Summen der Planung (Port von `engine/level/planned_totals.go`).

use crate::biome::Biome;
use crate::level::EVENT_KINDS;
use crate::planned::{Geometry, Placed};
use crate::types::PlannedChunk;

/// Go: `counts`, zählt feste Objekte je Art, die für die Biom-Summen zählen.
#[derive(Default)]
struct Counts {
    portal: i64,
    chest: i64,
    camp: i64,
    vein: i64,
}

impl Counts {
    fn add(&mut self, kind: &str) {
        match kind {
            "portal" => self.portal += 1,
            "chest" => self.chest += 1,
            "recruitCamp" => self.camp += 1,
            _ => {}
        }
    }

    fn of(&self, kind: &str) -> i64 {
        if kind == "chest" { self.chest } else { self.camp }
    }
}

/// Höchstzahl der Ereignis-Chunks einer Art im Biom (0 ohne Eintrag, wie Gos Map-Zugriff).
fn event_max(b: &Biome, kind: &str) -> i64 {
    b.event_chunks.get(kind).map_or(0, |r| r.max)
}

/// Go: `checkTotals`, prüft die Biom-Summen inklusive Planung und ob für den Rest der Level genug freie Plätze bleiben
/// (docs/rules/geplante-chunks.md § 3 und § 5). Gerechnet wird mit der kürzesten Länge und dem ungünstigsten Würfelwurf.
pub(crate) fn check_totals(b: &Biome, g: &Geometry, plan: &[PlannedChunk], all: &[Placed]) -> Result<(), String> {
    let mut count = Counts::default();
    for p in all {
        count.add(&p.kind);
        if b.veins.count > 0 && p.kind == b.veins.kind {
            count.vein += 1;
        }
    }
    if count.portal > b.portals.count {
        return Err(format!(
            "{} feste Portale, das Biom hat {}",
            count.portal, b.portals.count
        ));
    }
    for kind in EVENT_KINDS {
        if count.of(kind) > event_max(b, kind) {
            return Err(format!(
                "{} feste {kind}, das Biom erlaubt höchstens {}",
                count.of(kind),
                event_max(b, kind)
            ));
        }
    }
    if b.veins.count > 0 && count.vein > b.veins.count {
        return Err(format!(
            "{} feste Adern ({}), das Biom hat {}",
            count.vein, b.veins.kind, b.veins.count
        ));
    }
    check_room(b, g, plan, &count)
}

/// Go: `checkRoom`: Die übrigen Portale, Ereignisse (im ungünstigsten Fall bis zum Maximum) und Adern brauchen freie
/// Chunks.
fn check_room(b: &Biome, g: &Geometry, plan: &[PlannedChunk], count: &Counts) -> Result<(), String> {
    let free = 2 * g.usable - plan.len() as i64;
    let remaining_portals = b.portals.count - count.portal;
    let n = eligible_free(b, g, plan);
    if n < remaining_portals {
        return Err(format!(
            "kein Platz mehr für {remaining_portals} weitere Portale (frei und weit genug vom Hub: {n})"
        ));
    }
    let events: i64 = EVENT_KINDS
        .iter()
        .map(|kind| i64::max(0, event_max(b, kind) - count.of(kind)))
        .sum();
    let remaining_veins = if b.veins.count > 0 {
        b.veins.count - count.vein
    } else {
        0
    };
    let need = remaining_portals + events + remaining_veins;
    if free < need {
        return Err(format!(
            "kein Platz mehr: {free} freie Chunks, gebraucht bis zu {need} (Portale {remaining_portals}, Ereignisse {events}, Adern {remaining_veins})"
        ));
    }
    Ok(())
}

/// Go: `eligibleFree`, zählt freie Plätze, deren Mitte mindestens `minDistanceFromHubUnits` vom Hub entfernt liegt.
fn eligible_free(b: &Biome, g: &Geometry, plan: &[PlannedChunk]) -> i64 {
    let mut n = 0;
    for slot in 1..=g.usable {
        if g.dist(slot, g.cw / 2.0) < b.portals.min_distance_from_hub_units {
            continue;
        }
        for s in [slot, -slot] {
            if !plan.iter().any(|c| c.slot == s) {
                n += 1;
            }
        }
    }
    n
}
