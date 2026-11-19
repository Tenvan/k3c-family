//! Zustand des Reittier-Gatters für die Anzeige (RU62.8, B-970; Quelle `mounts/`). `World::mount_gate` trägt das Gatter
//! nur in der Oberflächen-Stufe einer Welt mit Welt-Reittier (data/mounts.json); sonst fehlt das Feld.
use serde::{Deserialize, Serialize};

/// Das Gatter einer Insel: hier kauft ein Monarch das Welt-Reittier.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MountGate {
    /// Ort des Gatters in Units.
    #[serde(rename = "x")]
    pub x: f64,
    /// Reichweite in Units.
    #[serde(rename = "range")]
    pub range: f64,
    /// Münzen für das Reittier.
    #[serde(rename = "price")]
    pub price: i64,
    /// ID des Welt-Reittiers im Katalog.
    #[serde(rename = "mount")]
    pub mount: String,
    /// Schlüssel der Grafik (`mounts` in data/paper.json).
    #[serde(rename = "sprite")]
    pub sprite: String,
}
