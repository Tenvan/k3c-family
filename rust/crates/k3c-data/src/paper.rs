//! data/paper.json: Zuordnung der Papierschnitt-Grafik; Go liest es in Teilen in native/k3c-game (actors, art, world).
//!
//! Das native Spiel ist ein eigenes Go-Modul mit Teil-Lesern je Paket; Vorbild ist deshalb der Aufbau der Datei
//! selbst (Schlüssel wie in der Datei, fehlende Schlüssel bleiben weg). Prüfungen der Leser portiert der Client-Port.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Datei-Aufbau `.`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Paper {
    #[serde(rename = "_hinweis")]
    pub hinweis: String,
    #[serde(rename = "spieler")]
    pub spieler: Vec<String>,
    #[serde(rename = "beritten")]
    pub beritten: String,
    #[serde(rename = "reittiere")]
    pub reittiere: BTreeMap<String, String>,
    #[serde(rename = "truppen")]
    pub truppen: BTreeMap<String, String>,
    #[serde(rename = "gegner")]
    pub gegner: BTreeMap<String, String>,
    #[serde(rename = "figuren")]
    pub figuren: PaperFiguren,
    #[serde(rename = "gebaeude")]
    pub gebaeude: String,
    #[serde(rename = "einsiedler")]
    pub einsiedler: PaperEinsiedler,
    #[serde(rename = "skalierung")]
    pub skalierung: BTreeMap<String, f64>,
    #[serde(rename = "hintergruende")]
    pub hintergruende: PaperHintergruende,
    #[serde(rename = "weltbilder")]
    pub weltbilder: BTreeMap<String, PaperWeltbilder>,
    #[serde(rename = "wetter")]
    pub wetter: BTreeMap<String, String>,
    #[serde(rename = "seekarte")]
    pub seekarte: PaperSeekarte,
    #[serde(rename = "objekte")]
    pub objekte: BTreeMap<String, PaperObjekte>,
    /// Waldwand-Hintergrund `gruppe/id` (BB1.3, B-1146); fehlt der Schlüssel, gibt es keine Waldwand.
    #[serde(rename = "waldwand")]
    pub waldwand: String,
    /// Leitmotive je Welt-ID `gruppe/id` (BB1.4, B-1146); fehlt der Schlüssel, gibt es keine.
    #[serde(rename = "leitmotive")]
    pub leitmotive: BTreeMap<String, String>,
}

impl Paper {
    /// Leitmotiv-Asset der Welt `world`, ersatzweise das der Standard-Welt (`None`: es gibt keine Leitmotive).
    pub fn motif_asset(&self, world: &str) -> Option<&str> {
        self.leitmotive
            .get(world)
            .or_else(|| self.leitmotive.get("standard"))
            .map(String::as_str)
    }
}

/// Datei-Aufbau `.einsiedler` (SI5, B-1001).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PaperEinsiedler {
    #[serde(rename = "_hinweis")]
    pub hinweis: String,
    #[serde(rename = "figuren")]
    pub figuren: BTreeMap<String, String>,
    #[serde(rename = "bauwerke")]
    pub bauwerke: BTreeMap<String, String>,
    #[serde(rename = "huette")]
    pub huette: String,
}

/// Datei-Aufbau `.objekte.*`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PaperObjekte {
    #[serde(rename = "gruppe")]
    pub gruppe: String,
    #[serde(rename = "frame")]
    pub frame: Vec<i64>,
    #[serde(rename = "frames")]
    pub frames: i64,
    #[serde(rename = "rahmen")]
    pub rahmen: Vec<i64>,
}

/// Datei-Aufbau `.seekarte`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PaperSeekarte {
    #[serde(rename = "inseln")]
    pub inseln: BTreeMap<String, String>,
    #[serde(rename = "meer")]
    pub meer: PaperSeekarteMeer,
    #[serde(rename = "schiff")]
    pub schiff: String,
    #[serde(rename = "aufsaetze")]
    pub aufsaetze: Vec<String>,
    #[serde(rename = "zeichen")]
    pub zeichen: PaperSeekarteZeichen,
}

/// Datei-Aufbau `.seekarte.zeichen`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PaperSeekarteZeichen {
    #[serde(rename = "abgeschlossen")]
    pub abgeschlossen: String,
    #[serde(rename = "gesperrt")]
    pub gesperrt: String,
}

/// Datei-Aufbau `.seekarte.meer`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PaperSeekarteMeer {
    #[serde(rename = "hinten")]
    pub hinten: String,
    #[serde(rename = "mitte")]
    pub mitte: String,
    #[serde(rename = "vorn")]
    pub vorn: String,
}

/// Datei-Aufbau `.weltbilder.*`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PaperWeltbilder {
    #[serde(rename = "gross")]
    pub gross: String,
    #[serde(rename = "klein")]
    pub klein: String,
}

/// Datei-Aufbau `.hintergruende`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PaperHintergruende {
    #[serde(rename = "gruppe")]
    pub gruppe: String,
    #[serde(rename = "varianten")]
    pub varianten: i64,
}

/// Datei-Aufbau `.figuren`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PaperFiguren {
    #[serde(rename = "_hinweis")]
    pub hinweis: String,
    #[serde(rename = "spieler")]
    pub spieler: Vec<PaperFigurenSpieler>,
    #[serde(rename = "truppen")]
    pub truppen: BTreeMap<String, PaperFigurenTruppen>,
    #[serde(rename = "gegner")]
    pub gegner: BTreeMap<String, PaperFigurenGegner>,
}

/// Datei-Aufbau `.figuren.gegner.*`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PaperFigurenGegner {
    #[serde(rename = "hoehe")]
    pub hoehe: f64,
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
    #[serde(rename = "alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f64>,
}

/// Datei-Aufbau `.figuren.truppen.*`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PaperFigurenTruppen {
    #[serde(rename = "hoehe")]
    pub hoehe: f64,
}

/// Datei-Aufbau `.figuren.spieler.[]`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PaperFigurenSpieler {
    #[serde(rename = "hoehe")]
    pub hoehe: f64,
    #[serde(rename = "tint", skip_serializing_if = "Option::is_none")]
    pub tint: Option<String>,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "paper.json";

/// `load` liest paper.json.
pub fn load(raw: &str) -> Result<Paper, Fehler> {
    decode("data/paper.json", raw)
}

#[cfg(test)]
#[path = "paper_tests.rs"]
mod tests;
