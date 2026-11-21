//! data/sizes.json: Größenstufen in Monarchenhöhen (LA1.1, Standard `docs/standards/bildaufbau.md` § 2). Nur Rust liest
//! die Datei; sie gilt im Look-Profil `k2c` (`look::Look::sizes`), im Profil `go` bleiben die alten Höhen.
//!
//! Der berittene Monarch ist der Bezugswert (1 M = `bezugPx` Spiel-px). Jede Stufe nennt die **sichtbare** Höhe eines
//! Dings als Vielfaches von M; jedes Asset gehört zu einer Stufe und nennt `sicht`, den Anteil der sichtbaren Höhe an
//! der Rahmenhöhe seines Bildes (gemessen am ersten Bild der Ruhe-Animation). Daraus rechnet das Spiel die Zeichengröße.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::source::Source;
use crate::{Fehler, check, decode};

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "sizes.json";

/// Datei-Aufbau `.`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Sizes {
    pub notes: String,
    /// Höhe des berittenen Monarchen in Spiel-px (1 M).
    #[serde(rename = "bezugPx")]
    pub reference_px: f64,
    /// Stufen nach Name.
    #[serde(rename = "stufen")]
    pub stages: BTreeMap<String, Stage>,
    /// Zuordnung Asset `gruppe/id` → Stufe.
    pub assets: BTreeMap<String, AssetSize>,
    /// Breiten der Hub-Gebäude für den Abstand der Bauplätze (LA3.1, B-1182).
    #[serde(rename = "bauplaetze")]
    pub sites: SiteWidths,
}

/// Breiten der Hub-Gebäude (`.bauplaetze`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SiteWidths {
    pub notes: String,
    /// Mindestlücke zwischen zwei Gebäuden in Units.
    #[serde(rename = "luecke")]
    pub gap: f64,
    /// Mindestabstand zweier Zahlziele in Units (2 × payRangeUnits).
    #[serde(rename = "zahlAbstand")]
    pub pay_gap: f64,
    /// Breite der Burg in Units (Hub-Mitte).
    #[serde(rename = "burg")]
    pub castle: f64,
    /// Breite je Platzart in Units.
    #[serde(rename = "breiten")]
    pub widths: BTreeMap<String, f64>,
}

/// Eine Stufe (`.stufen.*`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Stage {
    /// Sichtbare Höhe in Monarchenhöhen (M).
    pub m: f64,
    /// Wofür die Stufe gilt.
    #[serde(rename = "was")]
    pub what: String,
}

/// Ein Asset (`.assets.*`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetSize {
    #[serde(rename = "stufe")]
    pub stage: String,
    /// Sichtbare Höhe geteilt durch Rahmenhöhe des Bildes (0 … 1].
    #[serde(rename = "sicht")]
    pub visible: f64,
}

/// `load` liest und prüft sizes.json.
pub fn load(raw: &str) -> Result<Sizes, Fehler> {
    let s: Sizes = decode("data/sizes.json", raw)?;
    check("data/sizes.json", s.validate())?;
    Ok(s)
}

/// Liest sizes.json aus der Quelle; fehlt die Datei oder ist sie ungültig, gibt es keine Stufen (Meldung auf stderr).
pub fn from_source(src: &Source) -> Option<Sizes> {
    match src
        .read(FILE)
        .map_err(|e| e.to_string())
        .and_then(|raw| load(&raw).map_err(|e| e.to_string()))
    {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("❌ Größenstufen nicht lesbar: {e}");
            None
        }
    }
}

impl Sizes {
    /// Prüft Bezug, Stufen und Zuordnung.
    pub fn validate(&self) -> Result<(), String> {
        if self.reference_px <= 0.0 {
            return Err("bezugPx muss größer 0 sein".to_string());
        }
        for (name, s) in &self.stages {
            if s.m <= 0.0 {
                return Err(format!("stufen.{name}.m muss größer 0 sein"));
            }
        }
        for (asset, a) in &self.assets {
            if !self.stages.contains_key(&a.stage) {
                return Err(format!("assets.{asset}: Stufe {:?} fehlt unter stufen", a.stage));
            }
            if !(a.visible > 0.0 && a.visible <= 1.0) {
                return Err(format!("assets.{asset}.sicht muss in (0, 1] liegen"));
            }
        }
        Ok(())
    }

    /// Soll der sichtbaren Höhe von `asset` in Spiel-px; `None`, wenn das Asset keine Stufe hat.
    pub fn target_px(&self, asset: &str) -> Option<f64> {
        let a = self.assets.get(asset)?;
        Some(self.stages.get(&a.stage)?.m * self.reference_px)
    }

    /// Rahmenhöhe in Spiel-px, bei der `asset` seine Soll-Höhe sichtbar hat (Figuren).
    pub fn frame_px(&self, asset: &str) -> Option<f64> {
        Some(self.target_px(asset)? / self.assets.get(asset)?.visible)
    }

    /// Faktor auf `units_per_px` eines Sheets mit Rahmenhöhe `fh` px und `upp` Spiel-px je Bild-px (Gebäude, Objekte).
    pub fn sheet_factor(&self, asset: &str, fh: i64, upp: f64) -> Option<f64> {
        let now = self.assets.get(asset)?.visible * fh as f64 * upp;
        (now > 0.0).then(|| self.target_px(asset).unwrap_or(0.0) / now)
    }
}

#[cfg(test)]
#[path = "sizes_tests.rs"]
mod tests;
