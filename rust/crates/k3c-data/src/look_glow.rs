//! Lichthöfe des Looks (`data/look.json` › `profiles.*.glow`): Lagerfeuer, Fackeln, Mond und das folgende Licht der
//! Monarchen (LA1.8). Aus `look.rs` ausgelagert (Dateigrenze).

use serde::{Deserialize, Serialize};

/// Lichthöfe (`.profiles.*.glow`): weicher Hof um Lagerfeuer und Fackeln, Mond mit weitem Hof am Himmel.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Glow {
    pub enabled: bool,
    /// Faktor auf den Radius der Lichtquelle aus `data/light.json` (dort schon Darstellungs-Reichweite).
    pub radius: f64,
    /// Faktor auf die Stärke der Quelle (Deckkraft des Hofs in der Mitte).
    pub strength: f64,
    /// Anteil des Radius mit vollem Kern (0 = nur Verlauf).
    pub core: f64,
    /// Stärke des Hofs auf halbem Weg nach außen (Anteil der Mitte; kleiner = steilerer Abfall).
    pub falloff: f64,
    /// Höchstzahl Höfe je Zelle (ohne Mond).
    pub max: usize,
    pub moon: Moon,
    /// Folgendes Licht am Monarchen, nur nachts (LA1.8, B-1161).
    pub monarch: MonarchLight,
}

/// Folgendes Licht des Monarchen (`glow.monarch`): kleiner warmer Hof über dem Kopf, je Spieler.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct MonarchLight {
    /// Halbmesser in Units (nicht mit `glow.radius` skaliert).
    pub radius: f64,
    /// Deckkraft in der Mitte, mal Nacht-Stärke (`1 − daylight`).
    pub strength: f64,
    /// Höhe der Mitte über der Gehlinie in Units.
    pub height: f64,
}

const MONARCH_DEFAULT: MonarchLight = MonarchLight {
    radius: 2.5,
    strength: 0.5,
    height: 3.2,
};

impl Default for MonarchLight {
    fn default() -> Self {
        MONARCH_DEFAULT
    }
}

/// Mond mit Hof (`glow.moon`): Lage als Anteil der Welt-Zelle, Radius in Units.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Moon {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub strength: f64,
    /// Anteil des Radius, den die Scheibe selbst einnimmt.
    pub disc: f64,
}

const MOON_DEFAULT: Moon = Moon {
    x: 0.78,
    y: 0.2,
    radius: 9.0,
    strength: 0.55,
    disc: 0.12,
};

pub(crate) const GLOW_OFF: Glow = Glow {
    enabled: false,
    radius: 1.0,
    strength: 0.55,
    core: 0.1,
    falloff: 0.4,
    max: 8,
    moon: MOON_DEFAULT,
    monarch: MONARCH_DEFAULT,
};

impl Default for Glow {
    fn default() -> Self {
        GLOW_OFF
    }
}

impl Default for Moon {
    fn default() -> Self {
        MOON_DEFAULT
    }
}
