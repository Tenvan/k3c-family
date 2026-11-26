//! Benannte Baselines (B-640, BS7.2): ein durchgelaufener Bericht unter einem Namen wie M1, mit der Konfiguration des
//! Laufs. Ablage `reports/baseline/<name>/` (`balance.json` = Bericht, `run.json` = [`BaselineInfo`]).
//!
//! Bis RU70.15 stehen hier nur der Typ und der Ordner.

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

/// Ordner der benannten Baselines, relativ zum Checkout.
pub const BASELINE_DIR: &str = "reports/baseline";

/// Name, Zeitpunkt, Herkunft und Konfiguration einer gesicherten Baseline.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BaselineInfo {
    pub name: String,
    pub saved: String,
    /// Berichtsordner, aus dem gesichert wurde.
    pub run: String,
    pub seeds: i64,
    pub seed_mode: String,
    /// Leere Listen als `[]`: Go legt sie so an (`baselines.go`).
    pub bots: Vec<String>,
    pub worlds: Vec<String>,
    pub pass: bool,
    /// Eingefrorene Gruppen zum Zeitpunkt des Sicherns.
    pub frozen: Vec<String>,
    /// Auftrag des Laufs als JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<RawValue>>,
}
