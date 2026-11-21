//! Adern (Port von `engine/sim/world_vein.go`, B-114, materialien-gebaeude.md § 1): unendliche Ressourcen unter Tage. Eine
//! Ader wird einmal markiert und bleibt es, verschwindet nie und nimmt höchstens `max_workers` Bauern gleichzeitig.
//! Zielrate mit 2 Bauern = 2 × Amount ÷ WorkSeconds (data/economy.json › veins); der Trageweg zum Lager zählt nicht dazu.
//! Adern stehen nicht in `economy.gatherables`: Der endliche Startvorrat („Alles abbauen“) kennt sie nicht. `VeinData`
//! steht in `k3c_data::economy`.

use k3c_data::economy::Gatherable;

use super::values_room::values;
use crate::state::common::is_worker;
use crate::state::{Job, ResourceNode, Troop, World, WorldValues};

/// Go: `gatherOf`: die Abbau-Werte einer Knoten-Art (Ader oder endliche Ressource); `None`, wenn sie keine hat.
pub fn gather_of<'a>(vals: &'a WorldValues, kind: &str) -> Option<&'a Gatherable> {
    if let Some(v) = vals.economy.veins.get(kind) {
        return Some(&v.gatherable);
    }
    vals.economy.gatherables.get(kind)
}

/// Go: `isVein`.
pub fn is_vein(v: &WorldValues, n: &ResourceNode) -> bool {
    v.economy.veins.contains_key(&n.kind)
}

/// Go: `veinWorkers`: zählt die Bauern mit einem Sammel-Auftrag an der Ader.
pub fn vein_workers(w: &World, n: &ResourceNode) -> i64 {
    let count = w
        .troops
        .iter()
        .filter(|t| {
            t.job
                .as_ref()
                .is_some_and(|j| j.r#type == "gather" && j.node_id == n.id)
        })
        .count();
    i64::try_from(count).unwrap_or(i64::MAX)
}

/// Go: `nodeFree`: Nimmt der Knoten noch einen Bauern? Endliche Ressourcen einen, Adern bis `max_workers`.
pub fn node_free(w: &World, n: &ResourceNode) -> bool {
    let v = values(w);
    match v.economy.veins.get(&n.kind) {
        Some(vein) => vein_workers(w, n) < vein.max_workers,
        None => !is_worker(w, n.worker_id),
    }
}

/// Go: `gatherVein`: baut an der Ader ab; jeder Bauer hat seinen eigenen Fortschritt im Auftrag, die Ader bleibt.
pub fn gather_vein(t: &mut Troop, g: &Gatherable, dt: f64) {
    let Some(job) = t.job.as_mut() else {
        return;
    };
    job.progress += dt / g.work_seconds;
    if job.progress >= 1.0 {
        t.job = Some(Job {
            r#type: "carry".to_string(),
            resource: g.resource.clone(),
            amount: g.amount,
            ..Job::default()
        });
    }
}

/// ECO1.8 (B-1198): Umkreis um eine Ader, in dem Felsen sie versperren (Units).
pub const VEIN_CLEARANCE: f64 = 6.0;

/// ECO1.8: Ist die Ader noch versperrt? Mit nativen Regeln baut niemand an einer Ader ab, solange ein Fels in
/// `VEIN_CLEARANCE` steht; Felsen räumt der Handwerker mit der Spitzhacke (`unit/jobs.rs`). Ohne native Regeln nie (Go).
pub fn vein_blocked(w: &World, v: &WorldValues, n: &ResourceNode) -> bool {
    v.native_rules
        && is_vein(v, n)
        && w.nodes
            .iter()
            .any(|r| r.kind == "rock" && !r.gone && (r.x - n.x).abs() <= VEIN_CLEARANCE)
}

/// ECO1.8: Jede Ader bekommt 1 bis 3 Felsen in ihrem Umkreis (bereits stehende zählen mit); deterministisch aus `w.rng`.
/// Nur mit nativen Regeln, damit Level und Go-Läufe gleich bleiben.
pub fn place_vein_rocks(w: &mut World, v: &WorldValues) {
    if !v.native_rules {
        return;
    }
    let veins: Vec<f64> = w.nodes.iter().filter(|n| is_vein(v, n)).map(|n| n.x).collect();
    for vx in veins {
        let have = w
            .nodes
            .iter()
            .filter(|r| r.kind == "rock" && !r.gone && (r.x - vx).abs() <= VEIN_CLEARANCE)
            .count();
        let want = 1 + (w.rng.next() * 3.0).floor().min(2.0) as usize;
        for i in have..want {
            let off = (i as f64 + 1.0) * 1.5 * if i % 2 == 0 { -1.0 } else { 1.0 };
            let id = w.new_id();
            let x = (vx + off).clamp(0.0, w.width_units);
            w.nodes.push(ResourceNode {
                id,
                kind: "rock".into(),
                x,
                ..ResourceNode::default()
            });
        }
    }
}
