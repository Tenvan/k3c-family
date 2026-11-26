//! Größenstufen für das Lineal des Größenvergleichs (B-1170, WZ62.3): liest `data/sizes.json` (LA1.1, Standard
//! docs/standards/bildaufbau.md § 2). Der Monarch auf dem Mount ist 1 M = bezugPx Spiel-Pixel hoch. Nur lesen.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Dienst, datei, go_zahl, lies};

#[cfg(test)]
#[path = "stufen_tests.rs"]
mod tests;

/// Eine Größenstufe: sichtbare Höhe als Vielfaches von M.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Stufe {
    pub id: String,
    #[serde(serialize_with = "go_zahl")]
    pub m: f64,
    pub was: String,
}

/// Ordnet ein Asset einer Stufe zu; `sicht` ist die gemessene sichtbare Höhe geteilt durch die Rahmenhöhe.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StufenAsset {
    pub stufe: String,
    #[serde(serialize_with = "go_zahl")]
    pub sicht: f64,
}

/// `data/sizes.json` für die Oberfläche: Stufen nach Höhe sortiert, Assets nach `<gruppe>/<id>`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Stufen {
    /// Höhe des berittenen Monarchen (1 M) in Spiel-Pixeln.
    #[serde(rename = "bezugPx", serialize_with = "go_zahl")]
    pub bezug_px: f64,
    pub stufen: Vec<Stufe>,
    pub assets: BTreeMap<String, StufenAsset>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Roh {
    #[serde(rename = "bezugPx")]
    bezug_px: f64,
    stufen: Option<BTreeMap<String, RohStufe>>,
    assets: Option<BTreeMap<String, StufenAsset>>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct RohStufe {
    m: f64,
    was: String,
}

impl Dienst {
    /// Liest `data/sizes.json`; fehlt die Datei oder ist sie leer, kommt eine klare Meldung.
    pub fn assets_stufen(&self) -> Result<Stufen, String> {
        let raw = lies(&datei(&self.root, "data/sizes.json")).map_err(|e| format!("data/sizes.json: {e}"))?;
        let roh: Roh = serde_json::from_slice::<Value>(&raw)
            .and_then(serde_json::from_value)
            .map_err(|e| format!("data/sizes.json: kein gültiges JSON: {e}"))?;
        let stufen = roh.stufen.unwrap_or_default();
        if roh.bezug_px <= 0.0 || stufen.is_empty() {
            return Err("data/sizes.json: bezugPx oder stufen fehlen".to_owned());
        }
        let mut stufen: Vec<Stufe> = stufen
            .into_iter()
            .map(|(id, s)| Stufe { id, m: s.m, was: s.was })
            .collect();
        stufen.sort_by(|a, b| a.m.total_cmp(&b.m).then_with(|| a.id.cmp(&b.id)));
        Ok(Stufen {
            bezug_px: roh.bezug_px,
            stufen,
            assets: roh.assets.unwrap_or_default(),
        })
    }
}
