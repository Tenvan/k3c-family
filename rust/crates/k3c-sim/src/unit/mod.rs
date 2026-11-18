//! Eigene Truppen (Port von `engine/sim/unit_common.go`, `units.ts`):
//!   - Landstreicher wandern um ihr Camp, bis ein Monarch sie mit einer Münze rekrutiert.
//!   - Bauern holen Bögen, bauen bezahlte Gebäude und holen markierte Ressourcen. Bei Gefahr bleiben sie in der Burg.
//!   - Bogenschützen besetzen Türme oder stehen hinter der äußersten Mauer und schießen automatisch.
//!
//! Zeiger auf Truppen sind Indizes (`ti`); die Werte (`v`) holt der Aufrufer einmal je Schritt (kein `values(w)` je
//! Truppe). Die Aufträge (`find_job` …) stehen in `jobs.rs`.

pub mod archer;
pub mod escort;
pub mod flee;
pub mod hammer;
pub mod jobs;
pub mod professions;
pub mod warrior;

#[cfg(test)]
mod archer_tests;
#[cfg(test)]
mod common_tests;
#[cfg(test)]
mod disarm_tests;
#[cfg(test)]
mod escort_tests;
#[cfg(test)]
mod fetch_tests;
#[cfg(test)]
mod hammer_tests;
#[cfg(test)]
mod professions_tests;
#[cfg(test)]
mod shelter_tests;
#[cfg(test)]
mod warrior_tests;

use serde_json::json;

use crate::economy::add_stock;
use crate::island::storage::carry_to_stock;
use crate::state::common::{approach, event, is_dangerous};
use crate::state::island_ref::IslandRef;
use crate::state::links::{
    add_pool, build_progress_event, finish_upgrade, gather_of, gather_vein, is_vein, next_level, outpost_job, pure,
    repair, scatter_coins, unit_x, upgrade_job, vagrants_at, values, wander_radius,
};
use crate::state::{Job, World, WorldValues};
use archer::step_archer;
use flee::flee;
use hammer::spawn_vagrant_with;
use jobs::{find_job, release_job};
use professions::{mine_factor, work_factor};
use warrior::{fetch_weapon, step_warrior};

/// Go: `arrive`: so nah gilt ein Ziel als erreicht.
pub const ARRIVE: f64 = 0.3;

/// Go: `arrowSpeed`: Fluggeschwindigkeit der Pfeile.
pub const ARROW_SPEED: f64 = 25.0;

/// Go: `spawnVagrant`: legt einen Landstreicher an und gibt seinen Index in `w.troops` zurück.
pub fn spawn_vagrant(w: &mut World, camp_x: f64, x: f64) -> usize {
    let v = values(w);
    spawn_vagrant_with(w, &v, camp_x, x)
}

/// Go: `stepCamps`.
pub fn step_camps(w: &mut World, v: &WorldValues, dt: f64) {
    let c = &v.economy.recruit_camp;
    if pure(w) {
        return;
    }
    for ci in 0..w.camps.len() {
        let camp_x = w.camps[ci].x;
        if vagrants_at(w, camp_x) >= c.max_vagrants {
            w.camps[ci].respawn_in = c.respawn_seconds;
            continue;
        }
        w.camps[ci].respawn_in -= dt;
        if w.camps[ci].respawn_in <= 0.0 {
            let x = camp_x + (w.rng.next() - 0.5) * c.wander_units;
            spawn_vagrant_with(w, v, camp_x, x);
            w.camps[ci].respawn_in = c.respawn_seconds;
        }
    }
}

/// Go: `stepTroops`.
pub fn step_troops(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, dt: f64) {
    for ti in 0..w.troops.len() {
        match w.troops[ti].kind.as_str() {
            "vagrant" => {
                let anchor = w.troops[ti].anchor_x;
                let radius = wander_radius(w, ti);
                wander(w, v, ti, anchor, radius, dt);
            }
            "peasant" => {
                if !w.troops[ti].fleeing || !flee(w, v, ti, dt) {
                    step_peasant(w, v, ti, isl, dt);
                }
            }
            "warrior" | "eliteWarrior" => step_warrior(w, v, isl, ti, dt),
            _ => step_archer(w, v, isl, ti, dt),
        }
    }
}

/// Go: `wander`: um `center` herumlaufen; die zwei Würfe (Ziel, dann Wartezeit) stehen in dieser Reihenfolge.
pub fn wander(w: &mut World, v: &WorldValues, ti: usize, center: f64, radius: f64, dt: f64) {
    let t = &mut w.troops[ti];
    let speed = v.troops.get(t.kind.as_str()).map_or(0.0, |d| d.speed) * 0.5;
    if (t.target_x - center).abs() > radius {
        t.target_x = center;
    }
    if (t.x - t.target_x).abs() < ARRIVE {
        t.cooldown -= dt;
        if t.cooldown <= 0.0 {
            t.target_x = center + (w.rng.next() * 2.0 - 1.0) * radius;
            t.cooldown = 1.0 + w.rng.next() * 3.0;
        }
        return;
    }
    t.x = approach(t.x, t.target_x, speed, dt);
}

/// Go: `walkTo`: `true`, sobald die Truppe `x` erreicht hat.
pub fn walk_to(w: &mut World, v: &WorldValues, ti: usize, x: f64, dt: f64) -> bool {
    let t = &mut w.troops[ti];
    t.x = approach(t.x, x, v.troops.get(t.kind.as_str()).map_or(0.0, |d| d.speed), dt);
    (t.x - x).abs() < ARRIVE
}

/// Die Art des Auftrags, aus dem Text von Go einmal gelesen (kein Text-Vergleich je Zweig).
#[derive(Clone, Copy, PartialEq, Eq)]
enum JobKind {
    Fetch,
    Build,
    Repair,
    Gather,
    Carry,
    Other,
}

fn job_kind(j: &Job) -> JobKind {
    match j.r#type.as_str() {
        "fetchBow" | "fetchSword" | "fetchHammer" | "pickup" => JobKind::Fetch,
        "build" => JobKind::Build,
        "repair" => JobKind::Repair,
        "gather" => JobKind::Gather,
        "carry" => JobKind::Carry,
        _ => JobKind::Other,
    }
}

/// Go: `stepPeasant`.
pub fn step_peasant(w: &mut World, v: &WorldValues, ti: usize, isl: Option<&IslandRef>, dt: f64) {
    let t = &mut w.troops[ti];
    if t.job.is_none() && t.carried.is_some() {
        t.job = t.carried.take(); // nach dem Bauauftrag das behaltene Material weitertragen
    }
    if w.troops[ti].job.is_none() {
        w.troops[ti].job = find_job(w, v, ti, isl);
    }
    let danger = is_dangerous(w);
    // Bei Gefahr: Arbeit außerhalb der Mauern und Ausbauten liegen lassen und in die Burg.
    if danger && leave_on_danger(w, w.troops[ti].job.as_ref()) {
        release_job(w, ti);
    }
    if !danger {
        w.troops[ti].shelter = 0; // nächste Gefahr legt den Platz neu fest
    }
    let Some(kind) = w.troops[ti].job.as_ref().map(job_kind) else {
        if danger {
            let x = shelter_x(w, ti);
            walk_to(w, v, ti, x, dt);
        } else {
            let hub_x = w.hub_x;
            wander(w, v, ti, hub_x, v.hub.home_radius_units, dt);
        }
        return;
    };
    match kind {
        JobKind::Fetch => fetch_weapon(w, v, ti, dt),
        JobKind::Build => build(w, v, ti, dt),
        JobKind::Repair => repair(w, v, ti, dt),
        JobKind::Gather => gather(w, v, ti, dt),
        JobKind::Carry => carry(w, v, ti, isl, dt),
        JobKind::Other => {}
    }
}

/// Go: `shelterX`: Schutzplatz bei Gefahr, über 5 Plätze um den Hub gestreut. Maßgeblich ist der Index der Truppe in
/// `w.troops`, nicht ihre Entity-ID: Neue Bauplätze oder andere Entities verschieben so keine Schutzplätze (B-260). Der
/// Platz wird beim ersten Aufruf je Gefahr gemerkt (`Troop::shelter`): Sterben andere Truppen oder kommen welche hinzu,
/// bleibt er bis zum Ende der Gefahr (B-605).
pub fn shelter_x(w: &mut World, ti: usize) -> f64 {
    let t = &mut w.troops[ti];
    if t.shelter == 0 {
        t.shelter = ti as i64 % 5 + 1;
    }
    w.hub_x + (t.shelter - 3) as f64
}

/// Go: `leaveOnDanger`: Sammeln, Reparatur, Ausbau und jeder Auftrag an einem Vorposten (building_outpost.go) bleiben bei
/// Gefahr liegen.
pub fn leave_on_danger(w: &World, j: Option<&Job>) -> bool {
    j.is_some_and(|j| j.r#type == "gather" || j.r#type == "repair" || upgrade_job(w, j) || outpost_job(w, j))
}

/// Go: `build`.
fn build(w: &mut World, v: &WorldValues, ti: usize, dt: f64) {
    let Some(sid) = w.troops[ti].job.as_ref().map(|j| j.site_id) else {
        return;
    };
    let Some(site) = w.site(sid) else {
        w.troops[ti].job = None;
        return;
    };
    let upgrading = site.upgrade == "waitingWorker";
    if site.state != "waitingWorker" && !upgrading {
        w.troops[ti].job = None;
        return;
    }
    let sx = site.x;
    if !walk_to(w, v, ti, sx, dt) {
        return;
    }
    let Some(site) = w.site(sid) else {
        return;
    };
    let fallback = Default::default();
    let b = v.buildings.get(site.kind.as_str()).unwrap_or(&fallback);
    let seconds = if upgrading {
        next_level(w, None, site).map_or(0.0, |l| l.build_seconds) // nur die Bauzeit: der Dev-Preis (`cheap`) ändert sie nicht
    } else {
        b.build_seconds
    };
    let before = site.build_progress;
    let hp = b.hp;
    let progress = before + dt * work_factor(w, v, ti, "builder") / seconds.max(0.1);
    if let Some(s) = w.site_mut(sid) {
        s.build_progress = progress;
    }
    build_progress_event(w, sid, before);
    if progress >= 1.0 && upgrading {
        finish_upgrade(w, sid);
        w.troops[ti].job = None;
    } else if progress >= 1.0 {
        let Some(s) = w.site_mut(sid) else {
            return;
        };
        s.state = "built".into();
        s.build_progress = 1.0;
        s.hp = hp;
        s.max_hp = hp;
        s.worker_id = None;
        let (kind, x) = (s.kind.clone(), s.x); // einmal je gebautem Platz
        w.troops[ti].job = None;
        w.events
            .push(event(json!({"type": "built", "kind": kind, "x": unit_x(x)})));
    }
}

/// Go: `gather`.
fn gather(w: &mut World, v: &WorldValues, ti: usize, dt: f64) {
    let Some(nid) = w.troops[ti].job.as_ref().map(|j| j.node_id) else {
        return;
    };
    let Some(node) = w.node(nid) else {
        w.troops[ti].job = None;
        return;
    };
    let nx = node.x;
    if !walk_to(w, v, ti, nx, dt) {
        return;
    }
    let Some(node) = w.node(nid) else {
        return;
    };
    let fallback = Default::default(); // unbekannte Art: Nullwerte wie Gos Map-Zugriff
    let g = gather_of(v, &node.kind).unwrap_or(&fallback);
    let mine = dt * mine_factor(w, v, ti, &g.resource);
    if is_vein(v, node) {
        gather_vein(&mut w.troops[ti], g, mine);
        return;
    }
    let progress = node.progress + mine / g.work_seconds;
    if let Some(n) = w.node_mut(nid) {
        n.progress = progress;
    }
    if progress < 1.0 {
        return;
    }
    let Some(i) = w.nodes.iter().position(|n| n.id == nid) else {
        return;
    };
    let mut node = w.nodes.remove(i);
    node.gone = true;
    if node.kind == "tree" && !node.plant {
        // Holzfällen bringt Gold (B-952)
        let tg = v.economy.tree_gold;
        let coins = w.rng.int(tg[0], tg[1]);
        scatter_coins(w, node.x, coins);
    }
    w.troops[ti].job = Some(Job {
        r#type: "carry".into(),
        resource: g.resource.clone(), // einmal je gefälltem Baum oder Stein
        amount: g.amount,
        ..Job::default()
    });
}

/// Go: `carry`.
fn carry(w: &mut World, v: &WorldValues, ti: usize, isl: Option<&IslandRef>, dt: f64) {
    if isl.is_some() {
        carry_to_stock(w, v, ti, isl, dt);
        return;
    }
    let hub_x = w.hub_x;
    if !walk_to(w, v, ti, hub_x, dt) {
        return;
    }
    let Some(job) = w.troops[ti].job.take() else {
        return;
    };
    add_stock(w, v, isl, &job.resource, job.amount);
    w.events.push(event(
        json!({"type": "gathered", "resource": job.resource, "amount": job.amount}),
    ));
    gather_pressure(w);
}

/// Go: `gatherPressure` erhöht den Aggressionspool je vollständig abgegebenem Sammel-Auftrag.
pub(crate) fn gather_pressure(w: &mut World) {
    let percent = w.biome.cycle.percent_per_gather;
    add_pool(w, percent);
}
