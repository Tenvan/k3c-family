//! data/light.json: Licht und Fackeln; Go liest es in Teilen: engine/sim/torches.go (`torches`) und native/k3c-game/world/light.go.
//!
//! Das native Spiel ist ein eigenes Go-Modul mit Teil-Lesern je Paket; Vorbild ist deshalb der Aufbau der Datei
//! selbst (Schlüssel wie in der Datei, fehlende Schlüssel bleiben weg). Prüfungen der Leser portiert der Client-Port.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Datei-Aufbau `.`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Light {
    #[serde(rename = "notes")]
    pub notes: String,
    #[serde(rename = "nightBrightness")]
    pub night_brightness: f64,
    #[serde(rename = "backgroundBrightness")]
    pub background_brightness: f64,
    #[serde(rename = "glowColor")]
    pub glow_color: String,
    #[serde(rename = "sources")]
    pub sources: BTreeMap<String, LightSource>,
    #[serde(rename = "torches")]
    pub torches: BTreeMap<String, Vec<f64>>,
}

/// Datei-Aufbau `.sources.*`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LightSource {
    #[serde(rename = "radius")]
    pub radius: i64,
    #[serde(rename = "strength")]
    pub strength: f64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "light.json";

/// `load` liest light.json.
pub fn load(raw: &str) -> Result<Light, Fehler> {
    decode("data/light.json", raw)
}
