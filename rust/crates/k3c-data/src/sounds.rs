//! data/sounds.json: Sound-Slots und Mischung; Go liest es in native/k3c-game/audio/register.go.
//!
//! Das native Spiel ist ein eigenes Go-Modul mit Teil-Lesern je Paket; Vorbild ist deshalb der Aufbau der Datei
//! selbst (Schlüssel wie in der Datei, fehlende Schlüssel bleiben weg). Prüfungen der Leser portiert der Client-Port.

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Datei-Aufbau `.`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Sounds {
    #[serde(rename = "_comment")]
    pub comment: String,
    #[serde(rename = "mischung")]
    pub mischung: SoundsMischung,
    #[serde(rename = "slots")]
    pub slots: Vec<SoundSlot>,
}

/// Datei-Aufbau `.slots.[]`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SoundSlot {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "art")]
    pub art: String,
    #[serde(rename = "kategorie")]
    pub kategorie: String,
    #[serde(rename = "beschreibung")]
    pub beschreibung: String,
    #[serde(rename = "bus")]
    pub bus: String,
    #[serde(rename = "wahl")]
    pub wahl: Vec<SoundChoice>,
}

/// Datei-Aufbau `.slots.[].wahl.[]`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SoundChoice {
    #[serde(rename = "datei")]
    pub datei: String,
}

/// Datei-Aufbau `.mischung`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SoundsMischung {
    #[serde(rename = "_comment")]
    pub comment: String,
    #[serde(rename = "effekte")]
    pub effekte: f64,
    #[serde(rename = "musik")]
    pub musik: f64,
    #[serde(rename = "duckPegel")]
    pub duck_pegel: f64,
    #[serde(rename = "duckAnstieg")]
    pub duck_anstieg: f64,
    #[serde(rename = "duckHalten")]
    pub duck_halten: f64,
    #[serde(rename = "duckAbklingen")]
    pub duck_abklingen: f64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "sounds.json";

/// `load` liest sounds.json.
pub fn load(raw: &str) -> Result<Sounds, Fehler> {
    decode("data/sounds.json", raw)
}
