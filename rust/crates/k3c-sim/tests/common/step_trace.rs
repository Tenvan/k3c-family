//! Leser der Go-Spur `.work/trace/sim-<name>.jsonl` (Go: `engine/sim/golden_trace_test.go`, Schalter `-trace`): je Schritt
//! eine Zeile `{"tick":N,"world":<Baum>}`. Die Spur ist nur Hilfe für die Fehlersuche und nicht eingecheckt; fehlt sie,
//! meldet der Abgleich nur den Snapshot.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use std::path::PathBuf;

use k3c_det::golden::{self, SIM_TOL};
use k3c_sim::world::step;
use serde_json::Value;

use super::golden_run::{GoldenRun, build_world};

/// Die Zeilen einer Go-Spur.
pub struct Trace {
    lines: Vec<String>,
}

/// `path` ist der Ort der Spur eines Laufs im Repo (`.work/trace/sim-<name>.jsonl`).
pub fn path(name: &str) -> PathBuf {
    golden::testdata()
        .parent()
        .expect("Repo-Wurzel")
        .join(".work")
        .join("trace")
        .join(format!("sim-{name}.jsonl"))
}

impl Trace {
    /// `open` liest die Spur von `name`; `None`, wenn es sie nicht gibt.
    pub fn open(name: &str) -> Option<Trace> {
        let text = std::fs::read_to_string(path(name)).ok()?;
        Some(Trace {
            lines: text.lines().map(str::to_string).collect(),
        })
    }

    /// `first_step_diff` rechnet den Lauf in Rust nach und liefert den ersten Schritt nach Takt `from` (bis `to`), dessen
    /// Zustand von der Go-Spur abweicht, samt Pfad.
    pub fn first_step_diff(&self, run: &GoldenRun, from: usize, to: usize) -> Option<(usize, String)> {
        let mut w = build_world(run);
        let mut tick = 0usize;
        for seg in &run.inputs {
            for _ in 0..seg.ticks {
                step(&mut w, &seg.commands, None, run.dt);
                tick += 1;
                if tick <= from {
                    continue;
                }
                if tick > to {
                    return None;
                }
                let line = self.lines.get(tick)?;
                let v: Value = serde_json::from_str(line).ok()?;
                let d = golden::diff_tol("world", &v["world"], &golden::tree(&w), SIM_TOL);
                if !d.is_empty() {
                    return Some((tick, d));
                }
            }
        }
        None
    }
}
