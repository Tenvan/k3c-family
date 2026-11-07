//! Biom-Summen einer Planung (Port von `engine/level/planned_totals.go`, geplante-chunks.md § 3 und § 5): feste
//! Objekte gegen die Biom-Werte und genug freie Plätze für den Rest, im ungünstigsten Würfelwurf.

use super::EVENT_KINDS;
use super::biome::Biome;
use super::plan::{Geometry, Placed, PlannedChunk};

/// Feste Objekte je Art, die für die Biom-Summen zählen.
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
        if kind == "chest" {
            self.chest
        } else {
            self.camp
        }
    }
}

/// Prüft die Biom-Summen inklusive Planung und den Platz für den Rest der Level.
pub fn check_totals(
    b: &Biome,
    g: &Geometry,
    plan: &[PlannedChunk],
    all: &[Placed],
) -> Result<(), String> {
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
        let want = b.event(kind);
        if count.of(kind) > want.max {
            return Err(format!(
                "{} feste {}, das Biom erlaubt höchstens {}",
                count.of(kind),
                kind,
                want.max
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

/// Die übrigen Portale, Ereignisse (bis zum Maximum) und Adern brauchen freie Chunks.
fn check_room(
    b: &Biome,
    g: &Geometry,
    plan: &[PlannedChunk],
    count: &Counts,
) -> Result<(), String> {
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
        .map(|k| (b.event(k).max - count.of(k)).max(0))
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

/// Freie Plätze, deren Mitte mindestens minDistanceFromHubUnits vom Hub entfernt liegt.
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
