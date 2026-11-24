//! data/bottle.json: Rhythmus, Ort und Reichweiten der Flaschenpost (GS4.6, B-971, R1 bis R6). Vorläufige Startwerte;
//! Balancing kommt später. Gelesen von `k3c-sim` (Flaschenpost), die Regel steht in docs/rules/wirtschaft.md.
use serde::{Deserialize, Serialize};

use crate::{Fehler, check, decode};

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "bottle.json";

/// Rhythmus, Ort und Reichweiten der Flaschenpost.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BottleData {
    /// Erster Tag, an dem eine Flasche erscheinen kann (R1).
    #[serde(rename = "firstDay")]
    pub first_day: i64,
    /// Danach frühestens alle so vielen Tage (R1).
    #[serde(rename = "everyDays")]
    pub every_days: i64,
    /// Kleinster Abstand der Flasche zum Inselende in Units (R2).
    #[serde(rename = "edgeMarginMinUnits")]
    pub edge_margin_min_units: f64,
    /// Größter Abstand zum Inselende in Units (R2).
    #[serde(rename = "edgeMarginMaxUnits")]
    pub edge_margin_max_units: f64,
    /// Aufhebe-Reichweite für einen lebenden Monarchen in Units (R4).
    #[serde(rename = "pickupRangeUnits")]
    pub pickup_range_units: f64,
    /// Erreichen des Ziels durch einen Monarchen in Units (R6).
    #[serde(rename = "targetRangeUnits")]
    pub target_range_units: f64,
}

/// Liest und prüft bottle.json: Tage positiv und `everyDays` nicht kleiner als 1, Abstände 0 bis Maximum, Reichweiten größer 0.
pub fn load(raw: &str) -> Result<BottleData, Fehler> {
    let b: BottleData = decode("data/bottle.json", raw)?;
    check("data/bottle.json", validate(&b))?;
    Ok(b)
}

fn validate(b: &BottleData) -> Result<(), String> {
    if b.first_day < 0 {
        return Err(format!("firstDay {} darf nicht negativ sein", b.first_day));
    }
    if b.every_days < 1 {
        return Err(format!("everyDays {} muss mindestens 1 sein", b.every_days));
    }
    let margin_ok = b.edge_margin_min_units >= 0.0 && b.edge_margin_min_units <= b.edge_margin_max_units;
    if !margin_ok {
        return Err("edgeMarginMinUnits muss zwischen 0 und edgeMarginMaxUnits liegen".into());
    }
    if b.pickup_range_units <= 0.0 || b.target_range_units <= 0.0 {
        return Err("pickupRangeUnits und targetRangeUnits müssen größer 0 sein".into());
    }
    Ok(())
}

#[cfg(test)]
#[path = "bottle_tests.rs"]
mod bottle_tests;
