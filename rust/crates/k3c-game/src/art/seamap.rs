//! Assets der Seekarte (B-964): `data/paper.json › seekarte` ordnet je Welt-ID eine Insel-Miniatur zu, dazu die drei
//! Papierschichten des Meers, das Schiff, die Aufsätze und die Zustandszeichen, je als Asset `gruppe/id`. Hat eine
//! Welt keinen Eintrag, gilt die Miniatur von `standard`. Die Assets sind weich wie die Weltbilder: Fehlt eins,
//! zeichnet das HUD die Karte aus Formen. Vorlage: `native/k3c-game/art/seamap.go`.

use std::collections::BTreeMap;

use serde::Deserialize;

use super::STANDARD_WORLD;

/// Papierschichten des Meers von hinten nach vorn.
#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct SeaLayers {
    #[serde(default, rename = "hinten")]
    pub back: String,
    #[serde(default, rename = "mitte")]
    pub mid: String,
    #[serde(default, rename = "vorn")]
    pub front: String,
}

/// Zustandszeichen einer Insel.
#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct SeaMarks {
    #[serde(default, rename = "abgeschlossen")]
    pub done: String,
    #[serde(default, rename = "gesperrt")]
    pub locked: String,
}

/// Zuordnung der Seekarte.
#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct SeaMap {
    #[serde(default, rename = "inseln")]
    pub islands: BTreeMap<String, String>,
    #[serde(default, rename = "meer")]
    pub sea: SeaLayers,
    #[serde(default, rename = "schiff")]
    pub ship: String,
    #[serde(default, rename = "aufsaetze")]
    pub toppers: Vec<String>,
    #[serde(default, rename = "zeichen")]
    pub marks: SeaMarks,
}

impl SeaMap {
    /// Liest den Abschnitt `seekarte` aus dem Inhalt von paper.json; jedes Feld muss gesetzt sein, `inseln` braucht
    /// `standard`, `aufsaetze` mindestens einen Eintrag.
    pub fn parse(raw: &[u8]) -> Result<SeaMap, String> {
        #[derive(Deserialize)]
        struct P {
            seekarte: Option<SeaMap>,
        }
        let p: P = serde_json::from_slice(raw).map_err(|e| format!("paper.json › seekarte: {e}"))?;
        let m = p.seekarte.ok_or("paper.json › seekarte fehlt")?;
        if m.islands.get(STANDARD_WORLD).is_none_or(String::is_empty) {
            return Err(format!(
                "paper.json › seekarte.inseln: Eintrag {STANDARD_WORLD:?} fehlt"
            ));
        }
        if m.toppers.is_empty() {
            return Err("paper.json › seekarte.aufsaetze ist leer".into());
        }
        if m.assets().iter().any(String::is_empty) {
            return Err("paper.json › seekarte: leerer Eintrag (inseln, meer, schiff, aufsaetze, zeichen)".into());
        }
        Ok(m)
    }

    /// Liest die Zuordnung aus data/paper.json; ein Fehler steht im Log und liefert `None` (Seekarte aus Formen).
    pub fn from_data() -> Option<SeaMap> {
        match super::datafile::read("paper.json").and_then(|raw| Self::parse(&raw)) {
            Ok(m) => Some(m),
            Err(e) => {
                k3c_platform::log::error("❌ Assets der Seekarte nicht lesbar", &[("err", &e)]);
                None
            }
        }
    }

    /// Miniatur der Welt `id`, ersatzweise die der Standard-Welt (`""` ohne Zuordnung).
    pub fn island(&self, id: &str) -> &str {
        self.islands
            .get(id)
            .or_else(|| self.islands.get(STANDARD_WORLD))
            .map_or("", String::as_str)
    }

    /// Aufsatz zum Wert `seed` (fester Hash der Insel-ID): dieselbe Insel, derselbe Aufsatz.
    pub fn topper(&self, seed: u32) -> &str {
        if self.toppers.is_empty() {
            return "";
        }
        &self.toppers[seed as usize % self.toppers.len()]
    }

    /// Alle Assets der Seekarte, sortiert und ohne Doppelte.
    pub fn assets(&self) -> Vec<String> {
        let mut out = vec![
            self.sea.back.clone(),
            self.sea.mid.clone(),
            self.sea.front.clone(),
            self.ship.clone(),
            self.marks.done.clone(),
            self.marks.locked.clone(),
        ];
        out.extend(self.toppers.iter().cloned());
        out.extend(self.islands.values().cloned());
        out.sort();
        out.dedup();
        out
    }
}
