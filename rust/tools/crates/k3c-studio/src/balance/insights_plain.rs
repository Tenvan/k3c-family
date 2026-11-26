//! Klartext der Auswertung.
//!
//! Bis RU70.17 steht hier nur der Typ.

use serde::{Deserialize, Serialize};

/// Zahlen zu einem Hebel.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct LeverPlan {
    pub path: String,
    pub label: String,
    pub known: bool,
    pub current: String,
    pub min: String,
    pub max: String,
    pub start: String,
    /// Ohne Wirkungssatz: aktueller Wert, Bereich, Startvorschlag.
    pub short: String,
    pub text: String,
}
