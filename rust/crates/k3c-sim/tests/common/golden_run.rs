//! Ein Sim-Golden-Lauf (`testdata/golden/sim-*.json.gz`): Eingaben lesen, Welt bauen, Takte rechnen (Go: `goldenRun`,
//! `goldenWorld`, `applySetup`, `replay` in `engine/sim/sim_test.go`). Der Lauf ist nur Orakel-Abgleich (Entscheidung 012):
//! Er schreibt nie ein Golden.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use std::sync::Arc;

use k3c_data::hub::{HubData, Stock};
use k3c_det::golden::{self, SIM_TOL};
use k3c_level::nature::neutral_nature;
use k3c_sim::monarch::{add_pool_points, learn_skill};
use k3c_sim::state::{PlayerCommand, World};
use k3c_sim::world::base::base;
use k3c_sim::world::{Options, add_player, create_world, step};
use serde::Deserialize;
use serde_json::Value;

use super::step_trace::Trace;

/// Go: `goldenSetup`: Start-Vorrat, Pool-Punkte und je Spieler die zu lernenden Skills.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Setup {
    #[serde(rename = "stock")]
    pub stock: Stock,
    #[serde(rename = "pool")]
    pub pool: i64,
    #[serde(rename = "skills")]
    pub skills: Vec<Vec<String>>,
}

/// Ein Abschnitt der Eingaben: `commands` gelten `ticks` Takte lang.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Segment {
    pub ticks: usize,
    pub commands: Vec<PlayerCommand>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Snapshot {
    pub tick: usize,
    pub world: Value,
}

/// Go: `goldenRun`.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct GoldenRun {
    pub name: String,
    pub biome: String,
    pub seed: String,
    #[serde(rename = "cycleSpeed")]
    pub cycle_speed: f64,
    pub players: usize,
    pub dt: f64,
    pub ticks: usize,
    #[serde(rename = "snapshotEvery")]
    pub snapshot_every: usize,
    pub setup: Option<Setup>,
    pub aggression: f64,
    pub inputs: Vec<Segment>,
    pub snapshots: Vec<Snapshot>,
}

impl Default for GoldenRun {
    fn default() -> Self {
        GoldenRun {
            name: String::new(),
            biome: String::new(),
            seed: String::new(),
            cycle_speed: 0.0,
            players: 0,
            dt: 0.0,
            ticks: 0,
            snapshot_every: 0,
            setup: None,
            aggression: 0.0,
            inputs: Vec::new(),
            snapshots: Vec::new(),
        }
    }
}

/// `load` liest `testdata/golden/sim-<name>.json.gz`.
pub fn load(name: &str) -> GoldenRun {
    let path = golden::testdata().join("golden").join(format!("sim-{name}.json.gz"));
    let v = golden::read(&path);
    let run: GoldenRun = serde_json::from_value(v).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    assert!(
        run.snapshot_every > 0 && !run.snapshots.is_empty(),
        "{} unbrauchbar",
        path.display()
    );
    run
}

/// Go: `goldenWorld` samt `goldenPurse`: baut die Startwelt eines Laufs. Der Beutel steht wie in Go auf 100 Gold (nur im
/// Test, nicht in den Daten).
pub fn build_world(run: &GoldenRun) -> World {
    build_world_with(run, None)
}

/// `build_world` mit abweichender hub.json (`None` = die des Wertesatzes), Go: `goldenWorld(t, run, h)`.
pub fn build_world_with(run: &GoldenRun, hub: Option<Arc<HubData>>) -> World {
    let data = &base().expect("Daten").data;
    let b = data
        .biomes
        .iter()
        .find(|b| b.id == run.biome)
        .unwrap_or_else(|| panic!("Biom {} fehlt", run.biome));
    let mut vals = (*k3c_sim::world::values_room::default_values()).clone();
    vals.economy.purse.start_gold = 100;
    vals.economy.purse.max_gold = 100;
    let opts = Options {
        cycle_speed: run.cycle_speed,
        nature: neutral_nature(b),
        vals: Some(Arc::new(vals)),
        hub,
        ..Options::default()
    };
    let mut w = create_world(b, &run.seed, &opts).unwrap_or_else(|e| panic!("sim-{}: {e}", run.name));
    for _ in 0..run.players {
        add_player(&mut w);
    }
    if let Some(s) = &run.setup {
        apply_setup(&mut w, run, s);
    }
    if run.aggression > 0.0 && w.aggression.is_some() {
        w.aggression = Some(run.aggression);
    }
    w
}

/// Go: `applySetup`.
fn apply_setup(w: &mut World, run: &GoldenRun, s: &Setup) {
    w.stock = s.stock.clone();
    add_pool_points(w, None, s.pool);
    let v = k3c_sim::world::vals(w);
    for (i, ids) in s.skills.iter().enumerate() {
        for id in ids {
            learn_skill(w, &v, i, id).unwrap_or_else(|e| panic!("sim-{}: {e}", run.name));
        }
    }
}

/// Der erste abweichende Snapshot eines Laufs.
#[derive(Debug, Clone, PartialEq)]
pub struct Divergence {
    /// Takt des abweichenden Snapshots.
    pub tick: usize,
    /// Takt des letzten gleichen Snapshots (`None`: schon der Start weicht ab).
    pub last_equal: Option<usize>,
    /// Pfad und Werte aus `golden::diff_tol`.
    pub path: String,
}

/// Go: `replay`: rechnet den Lauf nach und vergleicht jeden Snapshot nach Entscheidung 012 (Ganzzahlen, Ereignisse und
/// Reihenfolge exakt, Fließkomma 1e-4 relativ). Der Rückgabewert ist die erste Abweichung.
pub fn replay(run: &GoldenRun) -> Option<Divergence> {
    let mut w = build_world(run);
    let (mut tick, mut snap) = (0usize, 0usize);
    let check = |w: &World, snap: usize, tick: usize| {
        let s = &run.snapshots[snap];
        assert_eq!(s.tick, tick, "sim-{}: Snapshot {snap} hat nicht Takt {tick}", run.name);
        golden::diff_tol("world", &s.world, &golden::tree(w), SIM_TOL)
    };
    let d = check(&w, 0, 0);
    if !d.is_empty() {
        return Some(Divergence {
            tick: 0,
            last_equal: None,
            path: d,
        });
    }
    let mut last = Some(0);
    for seg in &run.inputs {
        for _ in 0..seg.ticks {
            step(&mut w, &seg.commands, None, run.dt);
            tick += 1;
            if !tick.is_multiple_of(run.snapshot_every) {
                continue;
            }
            snap += 1;
            let d = check(&w, snap, tick);
            if !d.is_empty() {
                return Some(Divergence {
                    tick,
                    last_equal: last,
                    path: d,
                });
            }
            last = Some(tick);
        }
    }
    assert_eq!(tick, run.ticks, "sim-{}: Takte gerechnet", run.name);
    assert_eq!(snap + 1, run.snapshots.len(), "sim-{}: Snapshots gerechnet", run.name);
    None
}

/// `report` ist die Meldung zu einer Abweichung (Vorentscheidung 2 von RU22.1); liegt eine Go-Spur vor, nennt sie zusätzlich
/// den ersten abweichenden Schritt.
pub fn report(run: &GoldenRun, d: &Divergence) -> String {
    let prev = d.last_equal.map_or("keiner".to_string(), |t| format!("Takt {t}"));
    let mut msg = format!(
        "sim-{}: erster abweichender Snapshot bei Takt {} (vorheriger gleicher Snapshot: {prev}): {}",
        run.name, d.tick, d.path
    );
    if let Some(trace) = Trace::open(&run.name) {
        let from = d.last_equal.unwrap_or(0);
        if let Some((k, p)) = trace.first_step_diff(run, from, d.tick) {
            msg.push_str(&format!("; erster abweichender Schritt {k}: {p}"));
        }
    }
    msg
}

/// `assert_equal_behavior` prüft einen Lauf und bricht mit der Meldung samt Pfad ab.
pub fn assert_equal_behavior(name: &str) {
    let run = load(name);
    if let Some(d) = replay(&run) {
        panic!("{}", report(&run, &d));
    }
}
