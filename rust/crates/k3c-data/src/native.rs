//! data/native.json: Werte des nativen Spiels; Go liest es in native/k3c-game (art/voyage.go, devtools/chronicle.go).
//!
//! Das native Spiel ist ein eigenes Go-Modul mit Teil-Lesern je Paket; Vorbild ist deshalb der Aufbau der Datei
//! selbst (Schlüssel wie in der Datei, fehlende Schlüssel bleiben weg). Zahlen sind `f64` wie `float64` in Go. Prüfungen der Leser portiert der Client-Port.

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Datei-Aufbau `.`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Native {
    #[serde(rename = "_hinweis")]
    pub hinweis: String,
    #[serde(rename = "stepBudgetMs")]
    /// `None`: Schlüssel fehlt (Go: Standardwert bleibt); ein ausdrückliches `0` bleibt `Some(0.0)`
    pub step_budget_ms: Option<f64>,
    #[serde(rename = "slowLogEverySeconds")]
    /// wie `step_budget_ms`
    pub slow_log_every_seconds: Option<f64>,
    #[serde(rename = "voyage")]
    pub voyage: NativeVoyage,
}

/// Datei-Aufbau `.voyage`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct NativeVoyage {
    #[serde(rename = "seconds")]
    pub seconds: f64,
    #[serde(rename = "arcPx")]
    pub arc_px: f64,
    #[serde(rename = "bobPx")]
    pub bob_px: f64,
    #[serde(rename = "bobHz")]
    pub bob_hz: f64,
    #[serde(rename = "wavePx")]
    pub wave_px: f64,
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

#[cfg(test)]
mod tests {
    use super::load;

    // B-1110: Bruchwert wird gelesen, ausdrückliche 0 bleibt 0, fehlendes Feld ist `None` (Go-Standard greift im Aufrufer).
    #[test]
    fn bruch_null_und_fehlend() {
        let n = load(r#"{"stepBudgetMs": 4.5, "slowLogEverySeconds": 0, "voyage": {"seconds": 2.5, "arcPx": 28.5}}"#)
            .expect("laden");
        assert_eq!(n.step_budget_ms, Some(4.5));
        assert_eq!(n.slow_log_every_seconds, Some(0.0));
        assert_eq!((n.voyage.seconds, n.voyage.arc_px), (2.5, 28.5));
        let leer = load("{}").expect("laden");
        assert_eq!((leer.step_budget_ms, leer.slow_log_every_seconds), (None, None));
    }
}
