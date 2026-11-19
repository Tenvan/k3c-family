//! data/festival.json: Friedliche Insel und Fest nach dem Sieg über den Endboss (RU62.12, B-973). Neue Datei neben
//! `hub.json` und den Welt-Dateien, die Go-Ladepfade kennen sie nicht; die Sim liest sie in `k3c-sim/src/festival/`, die
//! Anzeige (RU62.13) liest dieselbe Datei.
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Fehler, check, decode};

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "festival.json";

/// Die erlaubten Himmels-Effekte des Fests (Anzeige RU62.13).
pub const EFFECTS: [&str; 2] = ["lanterns", "torches"];

/// Welten mit Fest und die Werte der Flucht (vorläufig).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FestivalFile {
    /// Laufgeschwindigkeit fliehender Gegner als Vielfaches ihres Tempos.
    #[serde(rename = "fleeSpeedFactor")]
    pub flee_speed_factor: f64,
    /// Abstand zum Portal oder Inselrand in Units, ab dem ein fliehender Gegner verschwindet.
    #[serde(rename = "exitMarginUnits")]
    pub exit_margin_units: f64,
    /// Welt-ID auf Himmels-Effekt; `standard` gehört nicht dazu, Welten ohne Eintrag haben kein Fest (Verhalten wie heute).
    #[serde(rename = "worlds")]
    pub worlds: BTreeMap<String, String>,
}

impl FestivalFile {
    /// Der Himmels-Effekt des Fests der Welt `world`; `None` = Welt ohne Fest.
    pub fn effect_of(&self, world: &str) -> Option<&str> {
        self.worlds.get(world).map(String::as_str)
    }
}

/// Liest und prüft festival.json: Flucht-Werte größer 0, jeder Effekt aus [`EFFECTS`], kein Eintrag für `standard`.
pub fn load(raw: &str) -> Result<FestivalFile, Fehler> {
    let f: FestivalFile = decode("data/festival.json", raw)?;
    check("data/festival.json", validate(&f))?;
    Ok(f)
}

fn positive(x: f64) -> bool {
    x.partial_cmp(&0.0) == Some(std::cmp::Ordering::Greater)
}

fn validate(f: &FestivalFile) -> Result<(), String> {
    if !positive(f.flee_speed_factor) || !positive(f.exit_margin_units) {
        return Err("fleeSpeedFactor und exitMarginUnits müssen größer 0 sein".to_string());
    }
    for (world, effect) in &f.worlds {
        if world == "standard" {
            return Err("worlds: die Welt standard hat kein Fest".to_string());
        }
        if !EFFECTS.contains(&effect.as_str()) {
            return Err(format!(
                "worlds: Welt {world:?} nennt den unbekannten Effekt {effect:?} (erlaubt: {})",
                EFFECTS.join(", ")
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: &str = include_str!("../../../../data/festival.json");

    /// AC-06 `festival_load`: die Datei lädt, beide Welten haben ihren Effekt, `standard` und `britannien` kein Fest.
    #[test]
    fn festival_load() {
        let f = load(RAW).expect("festival.json");
        assert_eq!(f.effect_of("sengoku"), Some("lanterns"));
        assert_eq!(f.effect_of("nil"), Some("torches"));
        assert_eq!(f.effect_of("standard"), None);
        assert_eq!(f.effect_of("britannien"), None);
        assert!(f.flee_speed_factor > 0.0 && f.exit_margin_units > 0.0);
    }

    /// AC-06: ein unbekannter Effekt-Name lässt das Laden mit klarer Meldung scheitern.
    #[test]
    fn festival_unbekannter_effekt_scheitert() {
        let raw = RAW.replace("\r\n", "\n"); // Windows-Checkout mit CRLF (autocrlf)
        let bad = raw.replace("\"nil\": \"torches\"", "\"nil\": \"feuerwerk\"");
        let e = load(&bad).expect_err("unbekannter Effekt").to_string();
        assert!(e.contains("feuerwerk") && e.contains("nil"), "{e}");
    }

    #[test]
    fn festival_lehnt_ungueltige_werte_ab() {
        let raw = RAW.replace("\r\n", "\n");
        for (from, to) in [
            ("\"fleeSpeedFactor\": 1.2", "\"fleeSpeedFactor\": 0"),
            ("\"exitMarginUnits\": 0.3", "\"exitMarginUnits\": -1"),
            ("\"nil\": \"torches\"", "\"standard\": \"torches\""),
        ] {
            assert!(raw.contains(from), "Vorlage fehlt: {from}");
            assert!(load(&raw.replace(from, to)).is_err(), "{to}");
        }
    }
}
