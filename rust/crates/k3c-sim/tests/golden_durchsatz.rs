//! RU22.5: Durchsatz der Rust-Sim an `sim-forest-aufbau` (2 Spieler, 6000 Takte, viele Bauten) ohne Vergleich der
//! Snapshots; gemessen wird nur `step`, nicht das Laden. Das Gegenstück in Go ist `BenchmarkGoldenLauf`
//! (`engine/sim/golden_trace_test.go`). Lauf nur auf Anweisung (Abnahme-Ticket, ruhiger Rechner):
//! `cargo test --release -p k3c-sim --test golden_durchsatz -- --ignored --nocapture`
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

mod common;

use std::time::Instant;

use common::golden_run::{build_world, load};
use k3c_sim::world::step;

const LAEUFE_JE_MESSUNG: usize = 10;
const MESSUNGEN: usize = 5;

#[test]
#[ignore = "Messung: nur im Release-Bau auf ruhigem Rechner (Abnahme-Ticket)"]
fn durchsatz_forest_aufbau() {
    let run = load("forest-aufbau");
    let mut werte = Vec::new();
    for m in 0..MESSUNGEN {
        let (mut schritte, mut sekunden) = (0usize, 0.0f64);
        for _ in 0..LAEUFE_JE_MESSUNG {
            let mut w = build_world(&run);
            let t = Instant::now();
            for seg in &run.inputs {
                for _ in 0..seg.ticks {
                    step(&mut w, &seg.commands, None, run.dt);
                    schritte += 1;
                }
            }
            sekunden += t.elapsed().as_secs_f64();
        }
        let je_s = schritte as f64 / sekunden;
        println!("Messung {}: {je_s:.0} schritte/s", m + 1);
        werte.push(je_s);
    }
    werte.sort_by(f64::total_cmp);
    println!(
        "forest-aufbau Rust: Median {:.0} schritte/s, Spanne {:.0} bis {:.0}",
        werte[werte.len() / 2],
        werte[0],
        werte[werte.len() - 1]
    );
    assert!(werte[0] > 0.0);
}
