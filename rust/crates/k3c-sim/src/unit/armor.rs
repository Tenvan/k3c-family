//! Rüstung abholen (EQ1.1, B-1205): Liegt im Regal des Rüstungsschmieds eine Rüstung, schickt `assign` den nächsten freien
//! Krieger, sonst den nächsten freien Bogenschützen zum Gebäude (Auftrag `fetchArmor`, je Rüstung einer). Erst beim
//! Abholen wird er zur Elite (`step_errand`). Nur ohne Gefahr (Nacht oder Welle) und nur mit den nativen Regeln; bei Gefahr
//! lässt der Kämpfer den Auftrag fallen, die Rüstung bleibt im Regal.

use super::walk_to;
use crate::economy::nearest;
use crate::state::common::{event, is_dangerous};
use crate::state::{Job, World, WorldValues};

/// Der Auftrag eines Kämpfers, der zum Rüstungsschmied läuft; `site_id` ist das Gebäude.
pub const FETCH_ARMOR: &str = "fetchArmor";

/// Wie viele Kämpfer gerade zum Gebäude `site_id` unterwegs sind.
fn on_the_way(w: &World, site_id: i64) -> i64 {
    let n = w
        .troops
        .iter()
        .filter(|t| {
            t.job
                .as_ref()
                .is_some_and(|j| j.r#type == FETCH_ARMOR && j.site_id == site_id)
        })
        .count();
    n as i64
}

/// Gibt dem nächsten freien Krieger (sonst Bogenschützen) einen Auftrag, wenn im Regal des Platzes `si` mehr Rüstungen
/// liegen, als Kämpfer unterwegs sind. Ohne Kandidat oder bei Gefahr bleibt die Rüstung im Regal.
pub fn assign(w: &mut World, si: usize) {
    let (id, sx, armors) = (w.sites[si].id, w.sites[si].x, w.sites[si].armors);
    if armors <= on_the_way(w, id) || is_dangerous(w) {
        return;
    }
    for kind in ["warrior", "archer"] {
        let free = |t: &crate::state::Troop| {
            t.kind == kind && t.job.is_none() && t.group == 0 && !t.attacking && !t.shield_fetch
        };
        if let Some(ti) = nearest(&w.troops, |t| t.x, sx, f64::INFINITY, free) {
            w.troops[ti].job = Some(Job {
                r#type: FETCH_ARMOR.into(),
                site_id: id,
                ..Job::default()
            });
            return;
        }
    }
}

/// Ein Schritt des Kämpfers mit Auftrag `fetchArmor`; `true`, wenn er damit beschäftigt war (der normale Schritt entfällt).
/// Gefahr, ein verschwundenes Gebäude oder eine schon vergebene Rüstung beenden den Auftrag. Am Gebäude wird er zur Elite
/// mit vollen Leben der neuen Figur.
pub fn step_errand(w: &mut World, v: &WorldValues, ti: usize, dt: f64) -> bool {
    let Some(site_id) = w.troops[ti]
        .job
        .as_ref()
        .filter(|j| j.r#type == FETCH_ARMOR)
        .map(|j| j.site_id)
    else {
        return false;
    };
    let site = w
        .site(site_id)
        .filter(|s| s.state == "built" && s.armors > 0)
        .map(|s| s.x);
    let Some(sx) = site.filter(|_| v.native_rules && !is_dangerous(w)) else {
        w.troops[ti].job = None;
        return false;
    };
    if !walk_to(w, v, ti, sx, dt) {
        return true;
    }
    let to = if w.troops[ti].kind == "warrior" {
        "eliteWarrior"
    } else {
        "eliteArcher"
    };
    let hp = v.troops.get(to).map_or(0.0, |t| t.hp);
    let t = &mut w.troops[ti];
    t.kind = to.to_string();
    t.hp = hp;
    t.max_hp = hp;
    t.job = None;
    if let Some(s) = w.site_mut(site_id) {
        s.armors -= 1;
    }
    w.events.push(event(serde_json::json!({"type": "armed"})));
    true
}
