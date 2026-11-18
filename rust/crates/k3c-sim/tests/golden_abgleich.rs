//! AC-01 (Sprint RU22): Der Abgleich nennt bei einer Abweichung den ersten Snapshot und das erste Feld. Die Tests
//! verändern ein geladenes Golden im Speicher (nie auf der Platte) und prüfen Meldung und Toleranz.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

mod common;

use common::golden_run::{load, replay, report};
use serde_json::json;

const LAUF: &str = "forest-ohne-spieler";

#[test]
fn abweichung_im_startzustand_nennt_takt_null_und_feld() {
    let mut run = load(LAUF);
    run.snapshots[0].world["time"] = json!(123.5);
    let d = replay(&run).expect("Abweichung erwartet");
    assert_eq!((d.tick, d.last_equal), (0, None));
    let msg = report(&run, &d);
    assert!(
        msg.starts_with("sim-forest-ohne-spieler: erster abweichender Snapshot bei Takt 0 (vorheriger gleicher Snapshot: keiner): world.time"),
        "{msg}"
    );
}

#[test]
fn abweichung_im_zweiten_snapshot_nennt_den_vorherigen_gleichen() {
    let mut run = load(LAUF);
    let (first, second) = (run.snapshots[1].tick, run.snapshots[2].tick);
    run.snapshots[2].world["nextId"] = json!(run.snapshots[2].world["nextId"].as_i64().unwrap() + 1);
    let d = replay(&run).expect("Abweichung erwartet");
    assert_eq!((d.tick, d.last_equal), (second, Some(first)));
    assert!(d.path.starts_with("world.nextId = "), "{}", d.path);
}

#[test]
fn fliesskomma_innerhalb_der_toleranz_ist_keine_abweichung() {
    let mut run = load(LAUF);
    let t = run.snapshots[1].world["time"].as_f64().unwrap();
    run.snapshots[1].world["time"] = json!(t * (1.0 + 1e-6));
    assert_eq!(replay(&run), None);
    run.snapshots[1].world["time"] = json!(t * 1.01);
    let d = replay(&run).expect("Abweichung außerhalb der Toleranz");
    assert!(d.path.starts_with("world.time = "), "{}", d.path);
}

#[test]
fn ganzzahl_weicht_schon_bei_eins_ab() {
    let mut run = load(LAUF);
    let n = run.snapshots[1].world["nextId"].as_i64().unwrap();
    run.snapshots[1].world["nextId"] = json!(n + 1);
    assert!(replay(&run).is_some());
}
