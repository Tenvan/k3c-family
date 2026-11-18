//! Ausbau-Stufen (Port von `engine/sim/hub_level.go`; B-112, materialien-gebaeude.md §§ 2-4): Hub-Stufe 1 bis Anzahl
//! hub.json › levels an der Burg, Mauer, Turm und Tor 1 bis Anzahl ihrer levels am selben Platz. Ein Ausbau läuft wie jeder
//! Bau: Gold zahlen, Material aus dem Vorrat, ein Bauer baut. Während des Ausbaus bleibt die alte Stufe gebaut. Der Hub-
//! Ausbau läuft in Schritten (`hub_step`, B-592); er ist ein Platz ohne Liste (`World::hub_site`, ID der Burg).
//!
//! Zwei Abweichungen vom Go-Aufbau (Zustandsmodell): Plätze werden über die ID angesprochen (`step_upgrade`, `pay_upgrade`,
//! `finish_upgrade`), und Funktionen, die Gos `cheap(w, …)` brauchen, bekommen die Insel als `isl` (Vorentscheidung 4).

use k3c_data::buildings::{Cost, LevelData};
use serde_json::json;

use crate::building::ruin::kind_hub_level;
use crate::economy::util::{can_afford, spend};
use crate::state::common::event;
use crate::state::island_ref::{IslandRef, cheap};
use crate::state::{Job, Site, World};
use crate::world::ship::SHIP_KIND;
use crate::world::{hub_of, vals};

/// Go: `newHubSite`: der Zahlplatz für den Hub-Ausbau an der Burg.
pub fn new_hub_site(w: &World) -> Site {
    Site {
        id: w.castle.id,
        kind: "castle".into(),
        x: w.castle.x,
        state: "built".into(),
        build_progress: 1.0,
        ..Site::default()
    }
}

/// Go: `eachSite`: ruft `f` für jeden Bauplatz der Stufe und zuletzt für den Hub-Ausbau.
pub fn each_site(w: &World, f: &mut dyn FnMut(&Site)) {
    for s in &w.sites {
        f(s);
    }
    if let Some(h) = &w.hub_site {
        f(h);
    }
}

/// Der Platz ist der Zahlplatz des Hub-Ausbaus (Go: `s == w.hubSite`).
fn is_hub_site(w: &World, s: &Site) -> bool {
    w.hub_site.as_ref().is_some_and(|h| h.id == s.id)
}

/// Go: `siteHubLevel`: die Hub-Stufe, ab der ein Hub-Platz bezahlbar ist (data/hub.json › sites, islandSites); für eine
/// Ruine die Hub-Stufe ihrer Art, 1 für andere Plätze ohne Eintrag. Go merkt sich den Wert im Platz; Rust rechnet je Aufruf
/// (Vorentscheidung 4: der Zwischenspeicher hing nur an einem Rückverweis).
pub fn site_hub_level(w: &World, s: &Site) -> i64 {
    hub_level_of(w, s)
}

/// Go: `hubLevelOf`.
#[allow(clippy::float_cmp)] // Go vergleicht die Lage exakt (`w.HubX+h.OffsetUnits == s.X`)
pub fn hub_level_of(w: &World, s: &Site) -> i64 {
    if s.ruin {
        return kind_hub_level(w, &s.kind);
    }
    let h = hub_of(w);
    for hs in h.sites.iter().chain(&h.island_sites) {
        if hs.kind == s.kind && w.hub_x + hs.offset_units == s.x {
            return hs.hub_level.max(1);
        }
    }
    1
}

/// Go: `levelOf`: die Stufe eines gebauten Platzes (Hub-Ausbau: die Hub-Stufe).
pub fn level_of(w: &World, s: &Site) -> i64 {
    if is_hub_site(w, s) {
        return w.hub_level;
    }
    s.level.max(1)
}

/// Go: `siteCost`: die Baukosten eines unbezahlten Platzes: der Eintrag der Biom-Tiefe in `cost_by_depth` (Treppen, B-631),
/// ohne Eintrag dieser Tiefe (fehlt oder leer) die Kosten `cost` seiner Art.
pub fn site_cost(w: &World, isl: Option<&IslandRef>, s: &Site) -> Cost {
    let v = vals(w);
    let b = v.buildings.get(&s.kind);
    let by = usize::try_from(w.biome.depth)
        .ok()
        .and_then(|d| b.and_then(|b| b.cost_by_depth.get(d)));
    match by {
        Some(c) if *c != Cost::default() => cheap(isl, c),
        _ => cheap(isl, &b.map(|b| b.cost.clone()).unwrap_or_default()),
    }
}

/// Go: `nextLevel`: die nächste Ausbau-Stufe eines gebauten Platzes, beim Hub der nächste Ausbauschritt (Kosten samt
/// Material-Tor); `None`, wenn er keine hat oder die Hub-Stufe fehlt (Stufe n von Mauer, Turm und Tor erst ab Hub-Stufe n).
pub fn next_level(w: &World, isl: Option<&IslandRef>, s: &Site) -> Option<LevelData> {
    let is_hub = is_hub_site(w, s);
    let v = vals(w);
    let levels: &[LevelData] = if is_hub {
        &hub_of(w).levels
    } else {
        v.buildings.get(&s.kind).map_or(&[], |b| &b.levels)
    };
    let n = level_of(w, s) + 1;
    if s.state != "built" || n > levels.len() as i64 || (!is_hub && s.kind != SHIP_KIND && w.hub_level < n) {
        return None;
    }
    let mut next = levels.get(usize::try_from(n - 1).ok()?)?.clone();
    if is_hub {
        next = hub_step_of(&next, w.hub_step);
    }
    next.cost = cheap(isl, &next.cost);
    Some(next)
}

/// Go: `hubStepOf`: Schritt `step` (ab 1) der Hub-Stufe `lv` mit dem Material-Tor in den Kosten; ohne Steps die Stufe selbst.
pub fn hub_step_of(lv: &LevelData, step: i64) -> LevelData {
    if lv.steps.is_empty() {
        return lv.clone();
    }
    let i = step.max(1).min(lv.steps.len() as i64) - 1;
    let mut st = lv.steps[i as usize].clone();
    st.cost = add_cost(&st.cost, &st.gate);
    st
}

/// Go: `hubSteps`: die Zahl der Ausbauschritte auf die nächste Hub-Stufe (mindestens 1); 0 auf der letzten Stufe.
pub fn hub_steps(w: &World) -> i64 {
    let levels = &hub_of(w).levels;
    if w.hub_level >= levels.len() as i64 {
        return 0;
    }
    usize::try_from(w.hub_level)
        .ok()
        .and_then(|i| levels.get(i))
        .map_or(1, |l| (l.steps.len() as i64).max(1))
}

/// Go: `hubGate`: das Material-Tor des laufenden Ausbauschritts (leer ohne Tor oder auf der letzten Stufe).
pub fn hub_gate(w: &World, isl: Option<&IslandRef>) -> Cost {
    let levels = &hub_of(w).levels;
    let Some(lv) = usize::try_from(w.hub_level).ok().and_then(|i| levels.get(i)) else {
        return Cost::default();
    };
    if lv.steps.is_empty() {
        return Cost::default();
    }
    let i = w.hub_step.max(1).min(lv.steps.len() as i64) - 1;
    cheap(isl, &lv.steps[i as usize].gate)
}

/// Go: `addCost`: die Kosten `a` und `b` zusammen.
pub fn add_cost(a: &Cost, b: &Cost) -> Cost {
    Cost {
        gold: a.gold + b.gold,
        wood: a.wood + b.wood,
        stone: a.stone + b.stone,
        copper: a.copper + b.copper,
        iron: a.iron + b.iron,
        crystal: a.crystal + b.crystal,
    }
}

/// Go: `upgradePayable`: Nimmt der gebaute Platz Münzen für die nächste Stufe? Die Kosten spielen hier keine Rolle, darum
/// ohne `cheap`.
pub fn upgrade_payable(w: &World, s: &Site) -> bool {
    s.upgrade.is_empty() && next_level(w, None, s).is_some()
}

/// Go: `payUpgrade`: zahlt eine Münze in den Ausbau des Platzes `id`; mit vollem Gold wartet er auf Material.
pub fn pay_upgrade(w: &mut World, isl: Option<&IslandRef>, id: i64) {
    let Some(s) = w.site(id) else {
        return;
    };
    let gold = next_level(w, isl, s).map(|l| l.cost.gold);
    let Some(s) = w.site_mut(id) else {
        return;
    };
    s.upgrade_paid += 1;
    if gold.is_some_and(|g| s.upgrade_paid >= g) {
        s.upgrade = "waitingMaterial".into();
    }
}

/// Go: `stepUpgrade`: bucht das Material der nächsten Stufe des Platzes `id` ab, sobald genug im Vorrat ist.
pub fn step_upgrade(w: &mut World, isl: Option<&IslandRef>, id: i64) {
    let Some(s) = w.site(id) else {
        return;
    };
    if s.upgrade != "waitingMaterial" {
        return;
    }
    let Some(next) = next_level(w, isl, s) else {
        return;
    };
    if can_afford(&w.stock, &next.cost) {
        spend(&mut w.stock, &next.cost);
        if let Some(s) = w.site_mut(id) {
            s.upgrade = "waitingWorker".into();
            s.build_progress = 0.0;
        }
    }
}

/// Go: `upgradeJob`: Baut der Auftrag gerade einen Ausbau?
pub fn upgrade_job(w: &World, j: &Job) -> bool {
    j.r#type == "build" && w.site(j.site_id).is_some_and(|s| s.upgrade == "waitingWorker")
}

/// Go: `finishUpgrade`: setzt die neue Stufe des Platzes `id`: Hub-Stufe + 1 bzw. Platz-Stufe + 1 mit vollen HP der Stufe.
/// Beim Hub hebt nur der letzte Ausbauschritt die Stufe, davor zählt `hub_step` weiter (ohne Ereignis).
pub fn finish_upgrade(w: &mut World, id: i64) {
    let Some(s) = w.site(id) else {
        return;
    };
    let next = next_level(w, None, s);
    let level = level_of(w, s) + 1;
    let (is_hub, kind) = (is_hub_site(w, s), s.kind.clone());
    if let Some(s) = w.site_mut(id) {
        s.upgrade.clear();
        s.upgrade_paid = 0;
        s.build_progress = 1.0;
        s.worker_id = None;
    }
    let step = w.hub_step.max(1);
    if is_hub && step < hub_steps(w) {
        w.hub_step = step + 1;
        return;
    }
    if is_hub {
        w.hub_level = level;
        w.hub_step = 1;
    } else if let Some(s) = w.site_mut(id) {
        let hp = next.map_or(0.0, |n| n.hp);
        s.level = level;
        s.hp = hp;
        s.max_hp = hp;
    }
    w.events
        .push(event(json!({"type": "upgraded", "kind": kind, "level": level})));
}
