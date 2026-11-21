//! Schutzplätze der Bürger (Go: `unit_shelter_test.go`, B-605).

use super::hammer::spawn_vagrant_with;
use super::{shelter_x, step_peasant};
use crate::enemies::testutil::*;

/// B-605/AC-01: Stirbt eine Truppe oder kommt eine hinzu, behalten die übrigen Truppen in der laufenden Gefahr ihren
/// Schutzplatz. Nach der Gefahr legt die nächste den Platz neu fest. (Die Bauern rechnen über `step_peasant`, das
/// `find_job` und damit RU14 und RU18 braucht.)
#[test]
fn schutzplatz_stabil_bei_truppenwechsel() {
    let mut w = quiet_world();
    let v = w.vals.clone().expect("Wertesatz");
    w.cycle.phase = "night".into();
    let hub = w.hub_x;
    for _ in 0..5 {
        let ti = spawn_vagrant_with(&mut w, &v, hub, hub);
        w.troops[ti].serve();
    }
    let step_all = |w: &mut crate::state::World| {
        for ti in 0..w.troops.len() {
            step_peasant(w, &v, ti, None, 0.1);
        }
    };
    step_all(&mut w);
    let ids: Vec<i64> = w
        .troops
        .iter()
        .map(|t| t.id)
        .filter(|&id| id != w.troops[1].id)
        .collect(); // Bauer 2 stirbt
    let before: Vec<f64> = ids.iter().map(|&id| shelter_of(&mut w, id)).collect();
    // Tod von Bauer 2, ein neuer Bauer vorn und einer hinten: alle Indizes der übrigen verschieben sich.
    w.troops.remove(1);
    let front = spawn_vagrant_with(&mut w, &v, hub, hub);
    w.troops[front].serve();
    let t = w.troops.pop().expect("neuer Bauer");
    w.troops.insert(0, t);
    let back = spawn_vagrant_with(&mut w, &v, hub, hub);
    w.troops[back].serve();
    step_all(&mut w);
    for (i, &id) in ids.iter().enumerate() {
        let got = shelter_of(&mut w, id);
        assert_eq!(
            got, before[i],
            "Bauer {i}: Schutzplatz {got} statt {} nach Truppenwechsel",
            before[i]
        );
    }
    w.cycle.phase = "day".into();
    step_all(&mut w);
    w.cycle.phase = "night".into();
    let idx = w.troops.iter().position(|t| t.id == ids[0]).expect("Bauer");
    assert_eq!(
        shelter_x(&mut w, idx),
        hub + ((idx % 5) as f64 - 2.0),
        "nächste Gefahr: neu nach Index"
    );
}

/// Der Schutzplatz einer Truppe nach ID (ihr Index ist im Test nicht stabil).
fn shelter_of(w: &mut crate::state::World, id: i64) -> f64 {
    let ti = w.troops.iter().position(|t| t.id == id).expect("Truppe");
    shelter_x(w, ti)
}

// B-605/AC-02 (Zusatzplatz ändert in den Golden-Läufen nur IDs) läuft als `tests/golden_zusatzplatz.rs` (RU22).
