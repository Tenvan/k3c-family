//! Aufträge der Bauern (Port von `engine/sim/unit_common.go`, ab `findJob`): Auftrag suchen, Bauplatz wählen, Auftrag
//! abgeben, zum Kämpfer befördern.

use super::warrior::weapon_to_fetch;
use crate::island::storage::resource_full;
use crate::state::common::{is_dangerous, is_worker};
use crate::state::island_ref::IslandRef;
use crate::state::pending::{armored_hp, each_site, gather_of, is_vein, node_free, outpost, repair_job};
use crate::state::{Job, World, WorldValues};

/// Go: `findJob`: Bogen holen, sonst nächsten wartenden Bauplatz, sonst (ohne Gefahr) die markierte Ressource nächst am
/// Hub. Bei gleichem Abstand gewinnt das frühere Element, wie das stabile `sort(...)[0]` in TS.
pub fn find_job(w: &mut World, v: &WorldValues, ti: usize, isl: Option<&IslandRef>) -> Option<Job> {
    if let Some(j) = site_job(w, v, ti) {
        return Some(j);
    }
    if is_dangerous(w) {
        return None;
    }
    let mut best: Option<(i64, f64)> = None;
    for n in &w.nodes {
        if !n.marked || !node_free(w, n) {
            continue;
        }
        let resource = gather_of(v, &n.kind).map_or("", |g| g.resource.as_str());
        if !resource_full(w, v, isl, resource)
            && best.is_none_or(|(_, bx)| (n.x - w.hub_x).abs() < (bx - w.hub_x).abs())
        {
            best = Some((n.id, n.x));
        }
    }
    let (nid, _) = best?;
    let tid = w.troops[ti].id;
    if let Some(n) = w.node_mut(nid)
        && !is_vein(v, n)
    {
        n.worker_id = Some(tid);
    }
    Some(Job {
        r#type: "gather".into(),
        node_id: nid,
        ..Job::default()
    })
}

/// Go: `siteJob`: Bogen holen, den nächsten wartenden Bauplatz bauen oder (ohne Gefahr) reparieren; `None`, wenn es
/// nichts zu tun gibt.
pub fn site_job(w: &mut World, v: &WorldValues, ti: usize) -> Option<Job> {
    if let Some(j) = weapon_to_fetch(w, v, ti) {
        return Some(j);
    }
    if let Some(j) = build_job(w, ti) {
        return Some(j);
    }
    repair_job(w, v, ti)
}

/// Go: `buildJob`: nächster wartender Bauplatz ohne Bauer, Ausbauten und Vorposten nur ohne Gefahr; `None`, wenn es
/// keinen gibt.
pub fn build_job(w: &mut World, ti: usize) -> Option<Job> {
    if !w.troops[ti].hammer {
        return None; // nur Handwerker bauen (B-950)
    }
    let (tid, tx) = (w.troops[ti].id, w.troops[ti].x);
    let danger = is_dangerous(w);
    let mut best: Option<(i64, f64)> = None;
    each_site(w, &mut |s| {
        let waiting =
            (s.state == "waitingWorker" || (s.upgrade == "waitingWorker" && !danger)) && !(danger && outpost(w, s.x));
        if waiting && !is_worker(w, s.worker_id) && best.is_none_or(|(_, bx)| (s.x - tx).abs() < (bx - tx).abs()) {
            best = Some((s.id, s.x));
        }
    });
    let (sid, _) = best?;
    if let Some(s) = w.site_mut(sid) {
        s.worker_id = Some(tid);
    }
    Some(Job {
        r#type: "build".into(),
        site_id: sid,
        ..Job::default()
    })
}

/// Go: `releaseJob`: gibt den Auftrag ab und den Knoten oder Platz frei, den diese Truppe belegt hatte.
pub fn release_job(w: &mut World, ti: usize) {
    let tid = w.troops[ti].id;
    let Some(job) = w.troops[ti].job.take() else {
        return;
    };
    if job.r#type == "gather"
        && let Some(n) = w.node_mut(job.node_id)
        && n.worker_id == Some(tid)
    {
        n.worker_id = None;
    }
    if (job.r#type == "build" || job.r#type == "repair")
        && let Some(s) = w.site_mut(job.site_id)
        && s.worker_id == Some(tid)
    {
        s.worker_id = None;
    }
}

/// Go: `makeArcher` befördert zum Bogenschützen.
pub fn make_archer(w: &mut World, ti: usize) {
    make_fighter(w, ti, "archer");
}

/// Go: `makeFighter` befördert zur Figur `kind` (Werte aus troops.json). Die Seite wird so gewählt, dass beide Seiten mit
/// dieser Figur gleich stark besetzt sind (Krieger zählen für sich, Q38).
pub fn make_fighter(w: &mut World, ti: usize, kind: &str) {
    let (mut left, mut right) = (0, 0);
    for (oi, o) in w.troops.iter().enumerate() {
        if oi == ti || o.kind != kind {
            continue;
        }
        if o.anchor_x < w.hub_x {
            left += 1;
        } else {
            right += 1;
        }
    }
    let side = if left <= right { -1.0 } else { 1.0 };
    let hp = armored_hp(w, kind); // mit der Rüstung des Hubs (building_upgrades.go)
    let anchor = w.hub_x + side;
    let t = &mut w.troops[ti];
    t.kind = kind.to_string();
    t.hp = hp;
    t.max_hp = hp;
    t.cooldown = 0.0;
    t.job = None;
    t.anchor_x = anchor;
    t.profession.clear();
    t.work_site = 0;
}
