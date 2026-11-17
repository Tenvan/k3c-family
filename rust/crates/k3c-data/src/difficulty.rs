//! data/difficulty.json: Go `struct{ Grades map[string]sim.Grade }` (engine/sim/island_options.go).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Go: `struct { Grades map[string]sim.Grade }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DifficultyFile {
    #[serde(rename = "Grades", alias = "grades")]
    pub grades: BTreeMap<String, Grade>,
}

/// Go: `sim.Grade`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Grade {
    #[serde(rename = "WaveSize", alias = "waveSize")]
    pub wave_size: f64,
    #[serde(rename = "EnemyHP", alias = "enemyHp")]
    pub enemy_hp: f64,
    #[serde(rename = "EnemyDamage", alias = "enemyDamage")]
    pub enemy_damage: f64,
    #[serde(rename = "DefaultDefeat", alias = "defaultDefeat")]
    pub default_defeat: String,
    #[serde(rename = "ProtectedNights", alias = "protectedNights")]
    pub protected_nights: i64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "difficulty.json";

/// Die Grade in Go-Reihenfolge (engine/sim `gradeNames`).
pub const GRADE_NAMES: [&str; 5] = ["dev", "easy", "normal", "hard", "ultra"];

/// `load` liest difficulty.json und verlangt jeden Grad wie `loadDifficulty` (dort `panic`).
pub fn load(raw: &str) -> Result<DifficultyFile, Fehler> {
    let d: DifficultyFile = decode("data/difficulty.json", raw)?;
    for g in GRADE_NAMES {
        if !d.grades.contains_key(g) {
            return Err(Fehler::Pruefung(format!("data/difficulty.json: Grad {g:?} fehlt")));
        }
    }
    Ok(d)
}
