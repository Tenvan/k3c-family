//! Delta-Bericht (B-640, BS7.2): Vergleich mit der Baseline eines Meilensteins, nach Werte-Gruppen geordnet.
//!
//! Bis RU70.15 stehen hier nur die Typen.

use serde::{Deserialize, Serialize};

/// Trend einer Gruppe gegenüber der Baseline.
pub const TREND_BETTER: &str = "besser";
pub const TREND_WORSE: &str = "schlechter";
pub const TREND_SAME: &str = "gleich";

/// Eine Werte-Gruppe im Delta-Bericht.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupDelta {
    pub group: String,
    pub frozen: bool,
    /// Schlechteste Ampel der Ziele der Gruppe.
    pub light: String,
    pub trend: String,
    pub better: i64,
    pub worse: i64,
    /// Ziele der Gruppe im Lauf.
    pub targets: i64,
    /// Ziele, deren Pass gegenüber der Baseline wechselt.
    pub tipped: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub warning: String,
}
