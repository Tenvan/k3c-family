//! Größte Fließkomma-Abweichung eines Golden-Laufs (G3-Bericht, AC-03): Der Lauf wird wie `replay` nachgerechnet; je
//! Snapshot zählt die größte relative Abweichung aller Fließkommawerte (absolut höchstens 1e-9 zählt als 0). Ganzzahlen und
//! Struktur prüft `replay`; hier zählt nur die Fließkomma-Abweichung.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use k3c_det::golden;
use k3c_sim::world::step;
use serde_json::Value;

use super::golden_run::{GoldenRun, build_world};

/// `max_dev` ist die größte relative Abweichung der Fließkommawerte (`got` als JSON-Fließkomma) zwischen zwei Bäumen
/// gleicher Form; `(Abweichung, Anzahl verglichener Fließkommawerte, Anzahl bitgleicher)`.
pub fn max_dev(want: &Value, got: &Value) -> (f64, usize, usize) {
    match (want, got) {
        (Value::Object(w), Value::Object(g)) => w
            .iter()
            .filter_map(|(k, wv)| g.get(k).map(|gv| max_dev(wv, gv)))
            .fold((0.0, 0, 0), merge),
        (Value::Array(w), Value::Array(g)) => w.iter().zip(g).map(|(wv, gv)| max_dev(wv, gv)).fold((0.0, 0, 0), merge),
        (Value::Number(w), Value::Number(g)) if g.is_f64() => {
            let (w, g) = (w.as_f64().unwrap(), g.as_f64().unwrap());
            let d = (w - g).abs();
            let rel = if d <= 1e-9 { 0.0 } else { d / w.abs().max(g.abs()) };
            (rel, 1, usize::from(w.to_bits() == g.to_bits()))
        }
        _ => (0.0, 0, 0),
    }
}

fn merge(a: (f64, usize, usize), b: (f64, usize, usize)) -> (f64, usize, usize) {
    (a.0.max(b.0), a.1 + b.1, a.2 + b.2)
}

/// `run_deviation` rechnet den Lauf nach und liefert die größte relative Fließkomma-Abweichung über alle Snapshots samt
/// Zählung `(Abweichung, Fließkommawerte, davon bitgleich)`.
pub fn run_deviation(run: &GoldenRun) -> (f64, usize, usize) {
    let mut w = build_world(run);
    let mut acc = max_dev(&run.snapshots[0].world, &golden::tree(&w));
    let (mut tick, mut snap) = (0usize, 0usize);
    for seg in &run.inputs {
        for _ in 0..seg.ticks {
            step(&mut w, &seg.commands, None, run.dt);
            tick += 1;
            if tick.is_multiple_of(run.snapshot_every) {
                snap += 1;
                acc = merge(acc, max_dev(&run.snapshots[snap].world, &golden::tree(&w)));
            }
        }
    }
    acc
}
