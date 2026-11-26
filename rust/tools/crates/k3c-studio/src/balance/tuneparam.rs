//! Parameter des automatischen Balancings (WZ28.6): `datei.json:a.b.c=min..max` zeigt auf eine Zahl in `data/`.
//!
//! Bis RU70.16 steht hier nur der Typ.

use serde::{Deserialize, Serialize};

/// Ein abgestimmter Wert: Datei, Pfad, Bereich und Ausgangswert. `int`: der Wert ist in der Datei ganzzahlig.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TuneParam {
    pub file: String,
    pub path: String,
    pub min: f64,
    pub max: f64,
    pub base: f64,
    pub int: bool,
}
