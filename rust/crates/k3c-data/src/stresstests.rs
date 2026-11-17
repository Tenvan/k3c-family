//! data/stresstests.json: Stresstest-Vorgaben; Go liest es in native/k3c-game/menu/stress_presets.go.
//!
//! Das native Spiel ist ein eigenes Go-Modul mit Teil-Lesern je Paket; Vorbild ist deshalb der Aufbau der Datei
//! selbst (Schlüssel wie in der Datei, fehlende Schlüssel bleiben weg). Prüfungen der Leser portiert der Client-Port.

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Datei-Aufbau `.`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Stresstests {
    #[serde(rename = "_doc")]
    pub doc: String,
    #[serde(rename = "presets")]
    pub presets: Vec<StressPreset>,
}

/// Datei-Aufbau `.presets.[]`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StressPreset {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "players")]
    pub players: i64,
    #[serde(rename = "depth")]
    pub depth: i64,
    #[serde(rename = "seed")]
    pub seed: i64,
    #[serde(rename = "enemies")]
    pub enemies: i64,
    #[serde(rename = "growth")]
    pub growth: String,
    #[serde(rename = "factor")]
    pub factor: f64,
    #[serde(rename = "build")]
    pub build: i64,
    #[serde(rename = "troops")]
    pub troops: i64,
    #[serde(rename = "timescale")]
    pub timescale: i64,
    #[serde(rename = "waves")]
    pub waves: i64,
    #[serde(rename = "minutes")]
    pub minutes: i64,
    #[serde(rename = "enemy")]
    pub enemy: String,
    #[serde(rename = "troop")]
    pub troop: String,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "stresstests.json";

/// `load` liest stresstests.json.
pub fn load(raw: &str) -> Result<Stresstests, Fehler> {
    decode("data/stresstests.json", raw)
}
