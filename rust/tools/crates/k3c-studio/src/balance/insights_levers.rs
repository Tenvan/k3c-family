//! Stellschrauben nach Wirkung.
//!
//! Bis RU70.17 steht hier nur der Typ.

use serde::{Deserialize, Serialize};

use super::insights_plain::LeverPlan;

/// Ein Wert in `data/` mit allen verletzten Zielen, die er verbessern soll. `conflict`: ein anderes verletztes Ziel
/// braucht denselben Wert in die Gegenrichtung.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct LeverGroup {
    pub path: String,
    pub direction: String,
    pub plan: LeverPlan,
    pub targets: Vec<String>,
    pub conflict: String,
    pub text: String,
}
