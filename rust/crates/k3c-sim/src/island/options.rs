//! Raum-Optionen der Insel (Port von `engine/sim/island_options.go`, B-101): Schwierigkeitsgrad, Ziel und Niederlage-Modus.
//! Das Ziel prüft `island_victory` (RU10.4); der Niederlage-Modus ist hier nur ein Wert, seine Wirkung steht in `defeat.rs`.
//! Die Regeln gelten nur für Inseln.
//!
//! Gos Paket-Variable `difficulty` (`loadDifficulty`) gibt es nicht: Die Tabelle kommt als `&DifficultyFile`.
//! `waveFactors` und `protectedNight` sind seit RU11.1 `IslandRef::wave_factors` und `protected_night`; hier steht nur
//! `Island::wave_factors`, das denselben Rechenweg benutzt. `scaled` steht seit RU11 in `enemies/waves.rs`.

use k3c_data::difficulty::{DifficultyFile, GRADE_NAMES};
use serde::{Deserialize, Serialize};

use super::Island;
use crate::state::IslandRef;

/// Go: `scaled`: Gegnerzahl skalieren (gerundet, mindestens 1 bei Zahl > 0); steht seit RU11 bei `plan_wave`.
pub use crate::enemies::waves::scaled;

/// Go: `IslandOptions`, die Optionen einer Insel. Grad: dev, easy, normal, hard, ultra.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct IslandOptions {
    #[serde(rename = "grade")]
    pub grade: String,
    /// endboss, gold, days, mineAll, buildAll.
    #[serde(rename = "goal")]
    pub goal: String,
    /// resources, stage (Kronenraub ist in jedem Grad Game Over, kein Modus).
    #[serde(rename = "defeat")]
    pub defeat: String,
}

/// Go: `goalNames`.
pub const GOAL_NAMES: [&str; 5] = ["endboss", "gold", "days", "mineAll", "buildAll"];
/// Go: `defeatNames`.
pub const DEFEAT_NAMES: [&str; 2] = ["resources", "stage"];

/// Go: `DefaultOptionsFor` liefert die Standard-Optionen eines Grads (für den Anlegen-Dialog). Unbekannter Grad: Normal.
pub fn default_options_for(grade: &str, difficulty: &DifficultyFile) -> IslandOptions {
    let (grade, g) = match difficulty.grades.get(grade) {
        Some(g) => (grade, g.clone()),
        // `k3c_data::difficulty::load` verlangt jeden Grad (Go bricht dort mit `panic` ab).
        None => ("normal", difficulty.grades.get("normal").cloned().unwrap_or_default()),
    };
    IslandOptions {
        grade: grade.to_string(),
        goal: "endboss".to_string(),
        defeat: g.default_defeat,
    }
}

/// Go: `DefaultOptions`: Normal, Endboss, Gold/Material-Verlust.
pub fn default_options(difficulty: &DifficultyFile) -> IslandOptions {
    default_options_for("normal", difficulty)
}

/// Go: `validateOptions`: Grad, Ziel und Niederlage-Modus müssen bekannte Namen sein.
pub fn validate_options(o: &IslandOptions) -> Result<(), String> {
    let checks: [(&str, &str, &[&str]); 3] = [
        ("Grad", &o.grade, &GRADE_NAMES),
        ("Ziel", &o.goal, &GOAL_NAMES),
        ("Niederlage-Modus", &o.defeat, &DEFEAT_NAMES),
    ];
    for (name, v, allowed) in checks {
        if !allowed.contains(&v) {
            return Err(format!("insel: unbekannter Wert {v:?} für {name}"));
        }
    }
    Ok(())
}

/// Go: `SetOptions` setzt die Optionen der Insel; Grad dev nur im Dev-Mode. Die Faktoren gelten ab der nächsten Welle.
pub fn set_options(isl: &mut Island, o: IslandOptions, dev_mode: bool) -> Result<(), String> {
    validate_options(&o)?;
    if o.grade == "dev" && !dev_mode {
        return Err("insel: Grad dev nur im Dev-Mode".to_string());
    }
    isl.options = o;
    Ok(())
}

/// Go: `SetGrade` wechselt nur den Grad; Ziel und Niederlage-Modus bleiben (Standards: `default_options_for`).
pub fn set_grade(isl: &mut Island, grade: &str, dev_mode: bool) -> Result<(), String> {
    let mut o = isl.options.clone();
    o.grade = grade.to_string();
    set_options(isl, o, dev_mode)
}

impl Island {
    /// Go: `Island.waveFactors`: Wellengröße (Spieleranzahl der Insel mal Grad), HP- und Schadensfaktor zum Zeitpunkt des
    /// Wellenstarts. Gleicher Rechenweg wie bei der Stufe: `IslandRef::wave_factors`.
    pub fn wave_factors(&self, difficulty: &DifficultyFile) -> (f64, f64, f64) {
        IslandRef {
            grade: difficulty.grades.get(&self.options.grade).cloned().unwrap_or_default(),
            players: self.player_count(),
            per_extra_player: self.per_extra_player(),
            ..IslandRef::default()
        }
        .wave_factors()
    }
}
