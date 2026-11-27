//! Automatisches Balancing (WZ28.6): Sweep oder Suche über ein bis zwei Werte aus `data/`; jeder Punkt spielt alle
//! Ziele mit den Werten als Varianten im Speicher. `data/` wird nie geschrieben.
//!
//! RU70.14 hat die Typen und Grenzen vorgezogen.

use serde::{Deserialize, Serialize};

use super::report_types::nil_empty;
use super::tuneparam::TuneParam;
use crate::kitx::go_marshal_indent;

/// Modi des Abstimmens.
pub const TUNE_SWEEP: &str = "sweep";
pub const TUNE_SEARCH: &str = "search";

/// Grenzen und Vorgaben der CLI-Schalter `--steps` und `--budget`.
pub const DEFAULT_TUNE_STEPS: i64 = 5;
pub const MAX_TUNE_STEPS: i64 = 21;
pub const DEFAULT_TUNE_BUDGET: i64 = 40;
pub const MAX_TUNE_PARAMS: usize = 2;

/// Ein ausgewerteter Punkt. `score`: Summe der normierten Abstände außerhalb der Korridore (0 = alle getroffen).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TunePoint {
    #[serde(serialize_with = "nil_empty")]
    pub values: Vec<f64>,
    pub score: f64,
    pub pass: bool,
    #[serde(serialize_with = "nil_empty")]
    pub failed: Vec<String>,
}

/// Der beste Punkt (kleinster Score, bei Gleichstand der erste).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TuneBest {
    #[serde(serialize_with = "nil_empty")]
    pub values: Vec<f64>,
    pub score: f64,
    pub pass: bool,
}

/// Inhalt von `reports/…/tuning-<Zeit>.json`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Tuning {
    pub version: i64,
    pub mode: String,
    #[serde(serialize_with = "nil_empty")]
    pub params: Vec<TuneParam>,
    #[serde(serialize_with = "nil_empty")]
    pub points: Vec<TunePoint>,
    pub best: TuneBest,
    pub reason: String,
    pub diff: String,
}

impl Tuning {
    /// Das Ergebnis eingerückt mit Zeilenende.
    pub fn json(&self) -> Result<String, String> {
        Ok(go_marshal_indent(self)? + "\n")
    }
}
