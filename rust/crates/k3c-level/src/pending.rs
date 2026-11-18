//! Platzhalter für Funktionen aus späteren Sessions von RU6 (Leitfaden § 7). Die Session, die die Funktion liefert,
//! löscht den Platzhalter; RU6.3 räumt diese Datei leer.

#![allow(clippy::unimplemented)] // Platzhalter nach Leitfaden § 7, RU6.3 löscht sie

use crate::biome::Biome;
use crate::planned::{Geometry, Placed};
use crate::rules::Rules;
use crate::types::PlannedChunk;

/// RU6.3: `ruins.go`.
pub(crate) fn check_buildings(
    _c: &PlannedChunk,
    _g: &Geometry,
    _near: &[Placed],
    _rules: &Rules,
) -> Result<Vec<Placed>, String> {
    unimplemented!("RU6.3: check_buildings")
}

/// RU6.3: `guards.go`.
pub(crate) fn check_guards(_c: &PlannedChunk, _b: &Biome, _g: &Geometry, _rules: &Rules) -> Result<(), String> {
    unimplemented!("RU6.3: check_guards")
}

/// RU6.3: `ruins.go`.
pub(crate) fn check_ruins_across(_ruins: &[Placed], _objects: &[Placed]) -> Result<(), String> {
    unimplemented!("RU6.3: check_ruins_across")
}

/// RU6.3: `planned_totals.go`.
pub(crate) fn check_totals(
    _b: &Biome,
    _g: &Geometry,
    _plan: &[PlannedChunk],
    _all: &[Placed],
    _rules: &Rules,
) -> Result<(), String> {
    unimplemented!("RU6.3: check_totals")
}
