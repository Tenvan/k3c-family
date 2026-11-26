//! Spielfortschritt eines Insel-Laufs (B-648): Meilenstein-Leiter, Blocker-Satz und Konfiguration.
//!
//! Bis RU70.16 stehen hier nur die Typen und der Stub, den die Zusammenfassung braucht.

use std::path::Path;

use serde::{Deserialize, Serialize};

use super::report_types::{Report, Scenario, nil_empty};

/// Urteil über das Ziel: `reached`, wenn alle Ziele mit Rolle goal erreicht sind.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct GoalView {
    pub reached: bool,
    pub hit: i64,
    pub total: i64,
}

/// Korridore mit Rolle guard, die im Korridor liegen.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct QualityView {
    pub passed: i64,
    pub total: i64,
}

/// Konfiguration des Ziel-Szenarios (ohne Seeds, die stehen im Summary).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct RunConfig {
    pub world: String,
    pub bot: String,
    pub players: i64,
    pub days: i64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub grade: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub helps: String,
}

/// Ein Meilenstein über alle Seeds: wie oft erreicht, Median des Tags und ein Pfeil gegen den letzten gleichen Lauf.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct MilestoneRow {
    pub id: String,
    pub label: String,
    pub reached: i64,
    pub runs: i64,
    pub median_day: Option<i64>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub arrow: String,
}

/// Spielfortschritt des Ziel-Szenarios.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Progress {
    pub config: RunConfig,
    pub test_match: String,
    #[serde(serialize_with = "nil_empty")]
    pub ladder: Vec<MilestoneRow>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub blocker: String,
}

/// Wertet die Läufe des Ziel-Szenarios aus; `None`, wenn keine Läufe mit Welt dazu vorliegen.
#[allow(clippy::unimplemented)] // V11: Stub bis RU70.16, kein Test dieser Session hat ein Ziel mit Welt.
pub(crate) fn progress_of(_root: &Path, _r: &Report, _sc: &Scenario, _seeds: &[String]) -> Option<Progress> {
    unimplemented!("RU70.16")
}
