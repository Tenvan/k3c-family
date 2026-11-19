//! Reparatur (Port von `engine/sim/building_repair.go`, B-112, materialien-gebaeude.md § 4): Bauern reparieren beschädigte,
//! gebaute Gebäude zwischen den Wellen kostenlos. Lesart „Anteil der Bauzeit“: Die Reparatur dauert den fehlenden HP-Anteil
//! mal die Bauzeit der Stufe (halb kaputt = halbe Bauzeit). Ein zerstörtes Gebäude (`destroy_site`) ist leer und wird neu
//! bezahlt, nicht repariert.

use serde_json::json;

use crate::hub::level_of;
use crate::state::common::{event, is_dangerous, is_worker};
use crate::state::{Job, Site, World, WorldValues};
use crate::unit::jobs::release_job;
use crate::unit::professions::work_factor;
use crate::unit::walk_to;

/// Go: `levelSeconds` ist die Bauzeit der aktuellen Stufe eines gebauten Platzes.
pub fn level_seconds(w: &World, v: &WorldValues, s: &Site) -> f64 {
    let Some(b) = v.buildings.get(&s.kind) else {
        return crate::hermit::bonus::hall_seconds(w, s); // RU62.10: das Bauwerk des Einsiedlers steht in keiner buildings.json
    };
    if b.levels.is_empty() {
        return b.build_seconds;
    }
    let idx = usize::try_from(level_of(w, s) - 1).unwrap_or(0);
    b.levels.get(idx).map_or(0.0, |l| l.build_seconds)
}

/// Go: `damaged`: gebaut, beschädigt, nicht im Ausbau und ohne lebenden Bauer.
fn damaged(w: &World, s: &Site) -> bool {
    s.state == "built" && s.hp > 0.0 && s.hp < s.max_hp && s.upgrade.is_empty() && !is_worker(w, s.worker_id)
}

/// Go: `repairJob`: nächstes beschädigtes Gebäude, nur ohne Gefahr; `None`, wenn es keins gibt. Der Bauer `w.troops[ti]`
/// wird als Bauer des Platzes eingetragen.
pub fn repair_job(w: &mut World, ti: usize) -> Option<Job> {
    if is_dangerous(w) || !w.troops[ti].hammer {
        return None;
    }
    let (tid, tx) = (w.troops[ti].id, w.troops[ti].x);
    let mut best: Option<(i64, f64)> = None;
    for s in &w.sites {
        if damaged(w, s) && best.is_none_or(|(_, bx)| (s.x - tx).abs() < (bx - tx).abs()) {
            best = Some((s.id, s.x));
        }
    }
    let (sid, _) = best?;
    if let Some(s) = w.site_mut(sid) {
        s.worker_id = Some(tid);
    }
    Some(Job {
        r#type: "repair".into(),
        site_id: sid,
        ..Job::default()
    })
}

/// Go: `repair` stellt HP her, ohne Gold und Material, bis `max_hp`.
pub fn repair(w: &mut World, v: &WorldValues, ti: usize, dt: f64) {
    let Some(sid) = w.troops[ti].job.as_ref().map(|j| j.site_id) else {
        return;
    };
    let ok = w
        .site(sid)
        .is_some_and(|s| s.state == "built" && s.upgrade.is_empty() && s.hp < s.max_hp);
    if !ok {
        release_job(w, ti);
        return;
    }
    let sx = w.site(sid).map_or(0.0, |s| s.x);
    if !walk_to(w, v, ti, sx, dt) {
        return;
    }
    let factor = work_factor(w, v, ti, "builder");
    let secs = w.site(sid).map_or(0.0, |s| level_seconds(w, v, s));
    let Some(s) = w.site_mut(sid) else {
        return;
    };
    s.hp = s.max_hp.min(s.hp + (s.max_hp * dt * factor) / secs.max(0.1));
    if s.hp >= s.max_hp {
        let kind = s.kind.clone();
        release_job(w, ti);
        w.events.push(event(json!({"type": "repaired", "kind": kind})));
    }
}
