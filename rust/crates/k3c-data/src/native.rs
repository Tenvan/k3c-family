//! data/native.json: Werte des nativen Spiels; Go liest es in native/k3c-game (art/voyage.go, devtools/chronicle.go).
//!
//! Das native Spiel ist ein eigenes Go-Modul mit Teil-Lesern je Paket; Vorbild ist deshalb der Aufbau der Datei
//! selbst (Schlüssel wie in der Datei, fehlende Schlüssel bleiben weg). Prüfungen der Leser portiert der Client-Port.

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Datei-Aufbau `.`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Native {
    #[serde(rename = "_hinweis")]
    pub hinweis: String,
    #[serde(rename = "stepBudgetMs")]
    pub step_budget_ms: i64,
    #[serde(rename = "slowLogEverySeconds")]
    pub slow_log_every_seconds: i64,
    #[serde(rename = "voyage")]
    pub voyage: NativeVoyage,
}

/// Datei-Aufbau `.voyage`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct NativeVoyage {
    #[serde(rename = "seconds")]
    pub seconds: i64,
    #[serde(rename = "arcPx")]
    pub arc_px: i64,
    #[serde(rename = "bobPx")]
    pub bob_px: f64,
    #[serde(rename = "bobHz")]
    pub bob_hz: f64,
    #[serde(rename = "wavePx")]
    pub wave_px: i64,
    #[serde(rename = "waveHz")]
    pub wave_hz: f64,
    #[serde(rename = "flagAt")]
    pub flag_at: f64,
    #[serde(rename = "scale")]
    pub scale: f64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "native.json";

/// `load` liest native.json.
pub fn load(raw: &str) -> Result<Native, Fehler> {
    decode("data/native.json", raw)
}
