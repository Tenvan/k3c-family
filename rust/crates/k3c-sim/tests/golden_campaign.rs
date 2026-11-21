//! AC-02 (Sprint RU22), Teil Kampagnen: Die Kampagnen-Golden (`campaign-*.json.gz`) rechnet die Rust-Sim mit gleichem
//! Verhalten nach, einschließlich Spielstand, Laden und Reise zwischen Tiefen (Go: `runGoldenKampagne` in
//! `engine/sim/campaign_test.go`; Entscheidung 012). Golden werden nie geschrieben.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

mod common;

use std::sync::Arc;

use common::golden_run::Segment;
use k3c_det::golden::{self, SIM_TOL};
use k3c_sim::campaign::Campaign;
use k3c_sim::state::save::SaveGame;
use k3c_sim::state::{World, WorldValues};
use k3c_sim::world::base::base;
use k3c_sim::world::step;
use k3c_sim::world::values_room::default_values;
use serde::Deserialize;
use serde_json::Value;

/// Go: `campaignFile`.
#[derive(Debug, Deserialize)]
#[serde(default)]
struct CampaignFile {
    name: String,
    seed: String,
    #[serde(rename = "cycleSpeed")]
    cycle_speed: f64,
    players: usize,
    dt: f64,
    ticks: usize,
    #[serde(rename = "snapshotEvery")]
    snapshot_every: usize,
    #[serde(rename = "saveAt")]
    save_at: usize,
    #[serde(rename = "savedAt")]
    saved_at: String,
    save: Value,
    inputs: Vec<Segment>,
    snapshots: Vec<CampaignSnap>,
}

impl Default for CampaignFile {
    fn default() -> Self {
        CampaignFile {
            name: String::new(),
            seed: String::new(),
            cycle_speed: 0.0,
            players: 0,
            dt: 0.0,
            ticks: 0,
            snapshot_every: 0,
            save_at: 0,
            saved_at: String::new(),
            save: Value::Null,
            inputs: Vec::new(),
            snapshots: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct CampaignSnap {
    tick: usize,
    depth: i64,
    #[serde(rename = "unlockedDepth")]
    unlocked_depth: i64,
    world: Value,
}

fn load(name: &str) -> CampaignFile {
    let path = golden::testdata()
        .join("golden")
        .join(format!("campaign-{name}.json.gz"));
    let f: CampaignFile =
        serde_json::from_value(golden::read(&path)).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    assert!(
        !f.snapshots.is_empty() && f.save_at > 0 && !f.save.is_null(),
        "{} unbrauchbar",
        path.display()
    );
    f
}

/// Go: `goldenPurse`: Beutel 100 Gold, nur im Test.
fn purse_values() -> Arc<WorldValues> {
    let mut v = (*default_values()).clone();
    v.economy.purse.start_gold = 100;
    v.economy.purse.max_gold = 100;
    v.native_rules = false; // Go kennt die nativen Regeln (HB1, ECO1.6) nicht
    Arc::new(v)
}

fn join_all(c: &mut Campaign, n: usize) {
    for _ in 0..n {
        c.join_player().expect("Spieler");
    }
}

/// Go: `compareCampaign`: erst Tiefe, dann die Welt; die Meldung nennt Name, Takt und Pfad.
fn compare(f: &CampaignFile, snap: usize, c: &Campaign) -> Result<(), String> {
    let s = &f.snapshots[snap];
    if (s.depth, s.unlocked_depth) != (c.depth, c.unlocked_depth) {
        return Err(format!(
            "{} Takt {}: Tiefe {}/{}, erwartet {}/{}",
            f.name, s.tick, c.depth, c.unlocked_depth, s.depth, s.unlocked_depth
        ));
    }
    let w = c.current_world().expect("Welt");
    let d = golden::diff_tol("world", &s.world, &golden::tree(w), SIM_TOL);
    if d.is_empty() {
        Ok(())
    } else {
        Err(format!("{} Takt {}: {d}", f.name, s.tick))
    }
}

/// Go: `saveAndLoad`: der Spielstand gleicht dem Golden, dann wird der Golden-Spielstand geladen.
fn save_and_load(f: &CampaignFile, c: &Campaign, vals: &Arc<WorldValues>) -> Result<Campaign, String> {
    let save = c.to_save(&f.saved_at)?;
    let d = golden::diff_tol("save", &f.save, &golden::tree(&save), SIM_TOL);
    if !d.is_empty() {
        return Err(format!("{} Takt {}: {d}", f.name, f.save_at));
    }
    let biomes = &base()?.data.biomes;
    let parsed = k3c_sim::state::save::parse_save(&f.save.to_string(), biomes, vals).map_err(|e| e.to_string())?;
    let mut loaded = Campaign::from_save_with(&parsed, f.cycle_speed, Some(Arc::clone(vals)))?;
    join_all(&mut loaded, f.players);
    Ok(loaded)
}

/// Go: `runGoldenKampagne`: spielt die Datei ab und liefert den gespeicherten Stand.
fn run(name: &str, want_depth: i64) -> Result<SaveGame, String> {
    let f = load(name);
    let vals = purse_values();
    let mut c = Campaign::create_with(&f.seed, "golden", f.cycle_speed, Some(Arc::clone(&vals)))?;
    join_all(&mut c, f.players);
    compare(&f, 0, &c)?;
    let (mut tick, mut snap) = (0usize, 0usize);
    for seg in &f.inputs {
        for _ in 0..seg.ticks {
            let w: &mut World = c.current_world_mut().expect("Welt");
            step(w, &seg.commands, None, f.dt);
            tick += 1;
            let to = w.travel.as_ref().filter(|t| t.progress >= 1.0).map(|t| t.to_depth);
            if let Some(to) = to {
                c.travel(to)?;
            }
            if tick == f.save_at {
                c = save_and_load(&f, &c, &vals)?;
            }
            if tick % f.snapshot_every == 0 {
                snap += 1;
                compare(&f, snap, &c)?;
            }
        }
    }
    if tick != f.ticks || snap + 1 != f.snapshots.len() || c.depth != want_depth {
        return Err(format!(
            "{} Takte, {} Snapshots, Tiefe {}; Datei hat {} Takte, {} Snapshots, Ziel Tiefe {want_depth}",
            tick,
            snap + 1,
            c.depth,
            f.ticks,
            f.snapshots.len()
        ));
    }
    serde_json::from_value(f.save.clone()).map_err(|e| e.to_string())
}

#[test]
fn campaign_abstieg() {
    run("abstieg", 2).unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn campaign_ausbau() {
    let save = run("ausbau", 0).unwrap_or_else(|e| panic!("{e}"));
    let h = &save.hubs[0];
    let built = h.sites.iter().filter(|s| s.state != "unpaid").count();
    let archers = h.troops.iter().filter(|t| t.kind == "archer").count();
    assert!(
        !h.nodes_gone.is_empty()
            && !h.nodes_marked.is_empty()
            && !h.pickups_taken.is_empty()
            && built > 0
            && archers > 0,
        "Stand zu leer: nodesGone {}, nodesMarked {}, pickupsTaken {}, gebaut {built}, Bogenschützen {archers}",
        h.nodes_gone.len(),
        h.nodes_marked.len(),
        h.pickups_taken.len()
    );
}
