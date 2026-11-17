//! data/guards.json: Go `level.GuardRules` (engine/level/guards.go) mit `checkGuardRules`.

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Go: `level.GuardRules`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GuardRules {
    #[serde(rename = "aggroRadiusUnits")]
    pub aggro_radius_units: f64,
    #[serde(rename = "maxPerChunk")]
    pub max_per_chunk: i64,
    #[serde(rename = "minGapUnits")]
    pub min_gap_units: f64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "guards.json";

/// `load` liest guards.json und prüft wie `loadGuardRules` (dort `panic` mit `guards.json: …`).
pub fn load(raw: &str) -> Result<GuardRules, Fehler> {
    let r: GuardRules = decode("guards.json", raw)?;
    check_guard_rules(&r).map_err(|e| Fehler::Pruefung(format!("guards.json: {e}")))?;
    Ok(r)
}

/// `check_guard_rules` prüft die Werte: Radius, Anzahl und Abstand sind positiv.
fn check_guard_rules(r: &GuardRules) -> Result<(), String> {
    let g = k3c_det::golden::go_float;
    #[allow(clippy::neg_cmp_op_on_partial_ord)] // wie Go: NaN fällt durch
    if !(r.aggro_radius_units > 0.0) {
        return Err(format!(
            "aggroRadiusUnits {} ist nicht positiv",
            g(r.aggro_radius_units)
        ));
    }
    if r.max_per_chunk < 1 {
        return Err(format!("maxPerChunk {} ist kleiner als 1", r.max_per_chunk));
    }
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if !(r.min_gap_units > 0.0) {
        return Err(format!("minGapUnits {} ist nicht positiv", g(r.min_gap_units)));
    }
    Ok(())
}
