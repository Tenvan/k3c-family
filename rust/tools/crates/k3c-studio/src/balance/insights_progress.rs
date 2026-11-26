//! Ergebnis-Datei eines Laufs (`result.json`) und Vergleich mit dem letzten Lauf gleicher Konfiguration.
//!
//! Bis RU70.16 steht hier nur der Typ.

use serde::{Deserialize, Serialize};

use super::progress::{GoalView, MilestoneRow, QualityView, RunConfig};
use super::report_types::nil_empty;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ResultFile {
    pub run: String,
    pub generated: String,
    pub goal_reached: bool,
    pub goal: Option<GoalView>,
    pub quality: Option<QualityView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<RunConfig>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub test_match: String,
    pub seeds: i64,
    #[serde(serialize_with = "nil_empty")]
    pub milestones: Vec<MilestoneRow>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub blocker: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub previous: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub next_lever: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub report: String,
}
