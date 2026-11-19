//! Zustand der Koop-Aktionen für die Anzeige (RU62.3, B-968; Quelle `coop/`). `World::coop` trägt die Doppel-Truhen, deren
//! Kennzeichen gerade wirkt (mindestens zwei gesteuerte Monarchen, Welt aus data/coop.json); sonst fehlt das Feld.
use serde::{Deserialize, Serialize};

/// Eine wirksame Doppel-Truhe der Stufe.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CoopChest {
    /// ID der Truhe (`Pickup::id`).
    #[serde(rename = "id")]
    pub id: i64,
    /// Ort der Truhe in Units.
    #[serde(rename = "x")]
    pub x: f64,
    /// Die Truhe ist doppelt (immer `true`: nur wirksame Doppel-Truhen stehen in der Liste).
    #[serde(rename = "double")]
    pub double: bool,
    /// Index des Monarchen, der allein davorsteht und auf den zweiten wartet; fehlt, wenn niemand davorsteht.
    #[serde(rename = "waiting", skip_serializing_if = "Option::is_none")]
    pub waiting: Option<i64>,
}

/// Koop-Zustand der Stufe.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CoopState {
    /// Die wirksamen Doppel-Truhen (je Takt neu gefüllt, Kapazität bleibt).
    #[serde(rename = "chests")]
    pub chests: Vec<CoopChest>,
    /// Zwischenspeicher je Truhen-ID: ist die Truhe gekennzeichnet? Wird einmal je Truhe berechnet, nie gespeichert.
    #[serde(skip)]
    pub marks: Vec<(i64, bool)>,
}

impl CoopState {
    /// Für `skip_serializing_if`: nichts anzuzeigen.
    pub fn is_empty(&self) -> bool {
        self.chests.is_empty()
    }
}
