//! Zustand des Wetters für die Anzeige (RU62.5, B-969; Quelle `weather/`). `World::weather` nennt Art und Phase, solange die
//! Welt eine Art hat und eine Wetternacht ansteht oder läuft; sonst fehlt das Feld. Beide Teile sind kleine `Copy`-Werte
//! (kein Text, keine Zuteilung je Takt).
use serde::{Deserialize, Serialize};

/// Die Wetterart (Katalog `data/weather.json`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WeatherKind {
    /// Taifun (Welt `sengoku`): bremst alle, kürzere Fernkampf-Reichweite.
    #[default]
    #[serde(rename = "taifun")]
    Taifun,
    /// Flut (Welt `nil`, Wirkung in RU62.6).
    #[serde(rename = "flut")]
    Flut,
}

/// Die Phase der Wetternacht.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WeatherPhase {
    /// Vorboten: ab Tagesanbruch des Vortags bis zum Nachtbeginn.
    #[default]
    #[serde(rename = "forecast")]
    Forecast,
    /// Wirkung: von Nachtbeginn bis Tagesanbruch.
    #[serde(rename = "active")]
    Active,
}

/// Art und Phase des Wetters der Stufe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WeatherState {
    /// Die Art der Welt.
    #[serde(rename = "kind")]
    pub kind: WeatherKind,
    /// Vorboten oder Wirkung.
    #[serde(rename = "phase")]
    pub phase: WeatherPhase,
}

/// Ein überfluteter Bereich der Flut (RU62.6): Welt-X in Units, von `from` bis `to`, am Inselrand; nur während der aktiven
/// Flut an der Oberfläche gesetzt (`World::flood_zones`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FloodZone {
    /// Anfang in Units.
    #[serde(rename = "from")]
    pub from: f64,
    /// Ende in Units.
    #[serde(rename = "to")]
    pub to: f64,
}
