//! Vorposten (Port von `engine/sim/building_outpost.go`, docs/rules/geplante-chunks.md § 8, B-468): Ein Platz jenseits der
//! äußersten Mauer-Platz-Lage seiner Seite ist ein Vorposten. Maßgeblich ist die Lage laut Layout der Stufe (äußerster
//! Mauer-Platz, B-652), nicht die äußerste gebaute Mauer (`outer_wall`): sonst griffe die Regel auch für Türme vor der
//! Mauer. Bauern bauen und reparieren dort nur ohne Gefahr, Bürger holen dort bei Gefahr keine Waffen; bei Gefahr lassen
//! sie den Auftrag liegen (`leave_on_danger`) und gehen in die Burg.

use crate::state::{Job, World};

/// Go: `outermostWall` ist das X des äußersten Mauer-Platzes auf Seite `side` (-1 links, +1 rechts); `None` ohne Mauer-Platz
/// dort.
pub fn outermost_wall(w: &World, side: i64) -> Option<f64> {
    let mut best: Option<f64> = None;
    for l in &w.level.sites {
        if l.kind != "wall" || l.side != side {
            continue;
        }
        if best.is_none_or(|b| side as f64 * (l.x - b) > 0.0) {
            best = Some(l.x);
        }
    }
    best
}

/// Go: `outpost`: Liegt `x` jenseits des äußersten Mauer-Platzes seiner Seite?
pub fn outpost(w: &World, x: f64) -> bool {
    let side = if x < w.hub_x { -1 } else { 1 };
    outermost_wall(w, side).is_some_and(|edge| side as f64 * (x - edge) > 0.0)
}

/// Go: `outpostJob`: Bau, Reparatur oder Waffen holen an einem Vorposten.
pub fn outpost_job(w: &World, j: &Job) -> bool {
    match j.r#type.as_str() {
        "build" | "repair" | "fetchBow" | "fetchSword" | "fetchHammer" => {
            w.site(j.site_id).is_some_and(|s| outpost(w, s.x))
        }
        _ => false,
    }
}
