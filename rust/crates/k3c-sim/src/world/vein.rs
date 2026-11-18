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
