//! Texte der Auswertung.
//!
//! Bis RU70.17 steht hier nur der Typ.

use serde::{Deserialize, Serialize};

use super::hints::Lever;
use super::insights::Insights;
use super::insights_plain::LeverPlan;

/// Ein Tuning-Hinweis zu einem Ziel.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Hint {
    pub target: String,
    pub light: String,
    pub text: String,
    pub why: String,
    pub levers: Vec<Lever>,
    pub plans: Vec<LeverPlan>,
}

/// Stub RU70.17: noch keine Stärken.
pub(crate) fn strengths(_insights: &Insights) -> Vec<String> {
    Vec::new()
}

/// Stub RU70.17: noch keine Schwächen.
pub(crate) fn weaknesses(_insights: &Insights) -> Vec<String> {
    Vec::new()
}

/// Stub RU70.17: noch keine Kurzauswertung.
pub(crate) fn summary(_insights: &Insights) -> Vec<String> {
    Vec::new()
}
