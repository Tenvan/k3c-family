//! data/render.json: Speicherbudget je Plattform (B-772); kein Go-Code liest die Datei.
//!
//! Das native Spiel ist ein eigenes Go-Modul mit Teil-Lesern je Paket; Vorbild ist deshalb der Aufbau der Datei
//! selbst (Schlüssel wie in der Datei, fehlende Schlüssel bleiben weg). Prüfungen der Leser portiert der Client-Port.

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Datei-Aufbau `.`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Render {
    #[serde(rename = "_hinweis")]
    pub hinweis: String,
    #[serde(rename = "xbox")]
    pub xbox: RenderXbox,
}

/// Datei-Aufbau `.xbox`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RenderXbox {
    #[serde(rename = "textureMiB")]
    pub texture_mi_b: i64,
    #[serde(rename = "near")]
    pub near: f64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "render.json";

/// `load` liest render.json.
pub fn load(raw: &str) -> Result<Render, Fehler> {
    decode("data/render.json", raw)
}
