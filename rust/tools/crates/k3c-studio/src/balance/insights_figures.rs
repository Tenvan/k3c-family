//! Ampeln der Kennzahlen (B-632) aus den Figures und den Richtwerten.
//!
//! Bis RU70.17 stehen hier nur die Typen.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::figures::NightFigures;
use super::figures_island::IslandFigures;
use super::hints::Band;
use super::ship_figures::ShipFigures;

/// Kennzahlen mit Ampel, die Nächte für die Diagramme und die grünen Bänder je Diagramm.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FigureInsights {
    pub note: String,
    pub bands: BTreeMap<String, Band>,
    pub lights: Vec<FigureLight>,
    pub nights: Vec<NightFigures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub island: Option<IslandFigures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship: Option<ShipFigures>,
}

/// Eine Kennzahl mit Ampel; `value` `None`: kein Messwert (graue Ampel).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FigureLight {
    pub key: String,
    pub name: String,
    pub unit: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub grade: String,
    pub value: Option<f64>,
    pub light: String,
    pub text: String,
}
