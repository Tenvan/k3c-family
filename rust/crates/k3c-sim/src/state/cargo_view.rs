//! Zustand der Schiffs-Fracht für die Anzeige (RU62.1, B-967; Quelle `cargo/view.rs`). Der Zustand trägt `cargo` nur auf
//! Inseln, die Fracht kennen (Welten aus data/cargo.json); sonst fehlt das Feld.

use serde::{Deserialize, Serialize};

/// Plätze, Fracht, Ladeplätze und offene Träger-Aufträge des Schiffs.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CargoState {
    /// Plätze des Schiffs gesamt (Grundschiff, mit Laderaum mehr).
    #[serde(rename = "slots")]
    pub slots: i64,
    /// Belegte Plätze (Fracht an Bord).
    #[serde(rename = "used")]
    pub used: i64,
    /// Für offene Träger-Aufträge reservierte Plätze.
    #[serde(rename = "reserved")]
    pub reserved: i64,
    /// Der Abschnitt Laderaum steht.
    #[serde(rename = "hold")]
    pub hold: bool,
    /// Die Fracht an Bord in Lade-Reihenfolge.
    #[serde(rename = "load")]
    pub load: Vec<CargoEntry>,
    /// Die drei Ladeplätze (leer, solange sie nicht nutzbar sind).
    #[serde(rename = "bays")]
    pub bays: Vec<CargoBayState>,
    /// Offene Träger-Aufträge.
    #[serde(rename = "orders")]
    pub orders: Vec<CargoOrderState>,
}

/// Eine Ladung an Bord.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CargoEntry {
    /// `coins` (Beutel), `troop` (Truppe) oder `crate` (Kiste Material).
    #[serde(rename = "kind")]
    pub kind: String,
    /// Truppenart oder Material; beim Beutel leer.
    #[serde(rename = "id")]
    pub id: String,
    /// Münzen im Beutel, Einheiten in der Kiste, bei der Truppe 1.
    #[serde(rename = "amount")]
    pub amount: i64,
    /// Platz an Deck (0 = zuerst geladen).
    #[serde(rename = "index")]
    pub index: i64,
}

/// Ein Ladeplatz am Steg.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CargoBayState {
    /// `coins`, `troops` oder `crates`.
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "x")]
    pub x: f64,
    /// Münzen für einen Einwurf.
    #[serde(rename = "price")]
    pub price: i64,
    /// `ready`, `full` (Laderaum voll) oder `empty` (nichts zu laden: keine Truppe, Vorrat unter einer Kiste).
    #[serde(rename = "state")]
    pub state: String,
}

/// Ein Bauer trägt eine Kiste zum Steg.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CargoOrderState {
    #[serde(rename = "resource")]
    pub resource: String,
    #[serde(rename = "units")]
    pub units: i64,
    /// ID der Truppe des Trägers; 0, solange kein Bauer frei ist.
    #[serde(rename = "carrier")]
    pub carrier: i64,
    /// Sekunden bis zur Ankunft am Steg.
    #[serde(rename = "eta")]
    pub eta: f64,
}

/// Ein nutzbarer Ladeplatz der Stufe (nur im Speicher): Wer hier `pay` hält, zahlt in die Fracht, nicht auf den Boden.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CargoBay {
    pub x: f64,
    pub range: f64,
}
