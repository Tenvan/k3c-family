//! Flucht der Bürger (Port von `engine/sim/unit_flee.go`, docs/rules/kampf.md § 4, B-928): Ein getroffener Bürger lässt
//! seine Arbeit liegen und läuft zum Schutzplatz an der Burg, ohne Statusverlust. Er arbeitet wieder, sobald er dort ist
//! und kein Gegner in Hörweite ist.

use super::jobs::release_job;
use super::{shelter_x, walk_to};
use crate::state::{World, WorldValues};

/// Go: `enemyWithin`: Steht ein lebender Gegner höchstens `dist` Units von `x` entfernt (Wachen eines Lagers zählen mit)?
pub fn enemy_within(w: &World, x: f64, dist: f64) -> bool {
    w.enemies.iter().any(|e| e.hp > 0.0 && (e.x - x).abs() <= dist)
}

/// Go: `flee` läuft zum Schutzplatz; `true`, solange der Bürger noch flieht (der Aufrufer macht dann nichts weiter).
pub fn flee(w: &mut World, v: &WorldValues, ti: usize, dt: f64) -> bool {
    // getragenes Material bleibt beim Bauern, er trägt es nach der Flucht weiter
    if w.troops[ti].job.as_ref().is_none_or(|j| j.r#type != "carry") {
        release_job(w, ti);
    }
    let x = shelter_x(w, ti);
    let near = v.troops.get("peasant").map_or(0.0, |t| t.fetch_enemy_distance);
    if walk_to(w, v, ti, x, dt) && !enemy_within(w, w.troops[ti].x, near) {
        let t = &mut w.troops[ti];
        t.fleeing = false;
        t.shelter = 0;
        return false;
    }
    true
}
