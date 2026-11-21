//! Himmel des Look-Profils (LA1.5, B-1164): ausgelagert aus `look.rs` (Dateigrenze).

use serde::{Deserialize, Serialize};

/// Himmel (`.profiles.*.sky`): Verlauf von der Himmelsfarbe zu `top` (Deckkraft `gradient` am oberen Rand), warmes
/// Abendrot am Horizont in Dämmerung und Morgengrauen (`glowHorizon`) und Sterne nachts (`stars`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Sky {
    pub enabled: bool,
    /// Farbe am oberen Rand (RGB 0 … 255).
    pub top: [f64; 3],
    /// Deckkraft von `top` ganz oben (0 = kein Verlauf), nach unten auf 0.
    pub gradient: f64,
    /// Farbe des Abendrots (RGB 0 … 255) und seine Deckkraft am Horizont bei vollem Dämmerlicht.
    pub horizon: [f64; 3],
    pub glow_horizon: f64,
    /// Anzahl Sterne je Zelle und ihre Deckkraft in tiefster Nacht.
    pub stars: usize,
    pub star_alpha: f64,
    /// Aufhellung der tiefsten Nacht gegenüber `light.json` (Vordergrund / Hintergrund), nur mit `enabled`: die
    /// Silhouetten der Kulisse bleiben sichtbar, die Nacht ist nicht ganz schwarz. `light.json` selbst bleibt (Go-Golden).
    pub night_lift: f64,
    pub back_lift: f64,
    /// Wolken (LA2.2, B-1183): Anzahl je Zelle (0 = keine), Deckkraft am Tag, Tempo (Zellenbreiten je Sekunde) und der
    /// Anteil der Deckkraft, der in tiefer Nacht bleibt.
    pub clouds: usize,
    pub cloud_alpha: f64,
    pub cloud_speed: f64,
    pub cloud_night: f64,
}

pub(crate) const SKY_OFF: Sky = Sky {
    enabled: false,
    top: [38.0, 74.0, 150.0],
    gradient: 0.55,
    horizon: [255.0, 150.0, 92.0],
    glow_horizon: 0.7,
    stars: 90,
    star_alpha: 0.9,
    night_lift: 0.06,
    back_lift: 0.09,
    clouds: 7,
    cloud_alpha: 0.5,
    cloud_speed: 0.004,
    cloud_night: 0.3,
};

impl Default for Sky {
    fn default() -> Self {
        SKY_OFF
    }
}
