//! Klartext der Auswertung.
//!
//! Bis RU70.17 steht hier nur der Typ.

use std::path::Path;

use serde::{Deserialize, Serialize};

use super::insights::TargetInsight;

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

/// Stubs RU70.17: noch ohne Klartext.
impl TargetInsight {
    pub(crate) fn plain(&self) -> String {
        String::new()
    }

    pub(crate) fn gap_text(&self) -> String {
        String::new()
    }

    pub(crate) fn trend_plain(&self) -> String {
        String::new()
    }

    /// `root` für [`super::tuneparam::parse_tune_param`].
    pub(crate) fn plans(&self, _root: &Path) -> Vec<LeverPlan> {
        Vec::new()
    }
}
