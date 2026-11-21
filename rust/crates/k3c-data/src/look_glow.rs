//! Lichthöfe des Look-Profils (RU55.5, LA1.8): ausgelagert aus `look.rs` (Dateigrenze).

use serde::{Deserialize, Serialize};

/// Lichthöfe (`.profiles.*.glow`): weicher Hof um Lagerfeuer und Fackeln, Mond mit weitem Hof am Himmel.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Glow {
    pub enabled: bool,
    /// Lagerfeuer, Fackeln und Monarch zeichnen einen sichtbaren Hof. Aus (🧑 2026-11-21): das Licht darf nicht selbst
    /// sichtbar sein, es erhellt nur die Gegend (Lichtkarte, `light.json`); der Mond behält seinen Hof.
    pub sources: bool,
    /// Faktor auf den Radius der Lichtquelle aus `data/light.json` (dort schon Darstellungs-Reichweite).
    pub radius: f64,
    /// Faktor auf die Stärke der Quelle (Deckkraft des Hofs in der Mitte).
    pub strength: f64,
    /// Anteil des Radius mit vollem Kern (0 = nur Verlauf).
    pub core: f64,
    /// Stärke des Hofs auf halbem Weg nach außen (Anteil der Mitte; kleiner = steilerer Abfall).
    pub falloff: f64,
    /// Abstand der Fackeln zur Mitte des Platzes als Faktor auf `light.json › torches` (nur Darstellung; die Sim bleibt),
    /// damit sie neben der Tür statt auf ihr hängen.
    pub torch_spread: f64,
    /// Abstand der zwei Fackeln der Burg zur Burgmitte in Units (0 = keine).
    pub castle_torches: f64,
    /// Exponent auf die Kanäle von `glowColor` im Lichtschein: über 1 wird er gelber und oranger (Fackel-Stimmung).
    pub warmth: f64,
    /// Wie stark der Schein der Fackeln mit der Flamme flackert (0 = gar nicht, 1 = bis ganz aus).
    pub torch_flicker: f64,
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
    radius: 3.6,
    strength: 0.3,
    height: 3.4,
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
    radius: 14.0,
    strength: 0.6,
    disc: 0.08,
};

pub(crate) const GLOW_OFF: Glow = Glow {
    enabled: false,
    sources: false,
    radius: 1.5,
    strength: 0.32,
    core: 0.0,
    falloff: 0.5,
    torch_spread: 2.0,
    castle_torches: 2.6,
    warmth: 1.6,
    torch_flicker: 0.35,
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
