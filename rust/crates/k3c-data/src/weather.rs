//! data/weather.json: Wetter je Welt (RU62.5, B-969, docs/rules/wetter.md). Neue Datei neben `events.json`, die Go-Ladepfade
//! kennen sie nicht; die Sim liest sie in `k3c-sim/src/weather/`. `worlds` ist der Schalter: eine Welt ohne Eintrag hat kein
//! Wetter (`standard`, `britannien`).
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Fehler, check, decode};

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "weather.json";

/// Die Wetterarten des Katalogs (`worlds` darf nur diese nennen).
pub const KINDS: [&str; 2] = ["taifun", "flut"];

/// Der Katalog der Wetterarten und ihre Zuordnung zu den Welten (vorläufige Werte).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WeatherFile {
    /// Welt-ID → Wetterart (genau eine je Welt; fehlt die Welt, gibt es dort kein Wetter).
    #[serde(rename = "worlds")]
    pub worlds: BTreeMap<String, String>,
    /// Der Taifun (Welt `sengoku`).
    #[serde(rename = "taifun")]
    pub taifun: Taifun,
    /// Die Flut (Welt `nil`, RU62.6).
    #[serde(rename = "flut")]
    pub flut: Flut,
}

/// Werte des Taifuns.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Taifun {
    /// Jede n-te Nacht ist eine Wetternacht.
    #[serde(rename = "everyNights")]
    pub every_nights: i64,
    /// Frühestens in dieser Nacht.
    #[serde(rename = "firstNight")]
    pub first_night: i64,
    /// Tempo aller Einheiten in der Wetternacht (Faktor).
    #[serde(rename = "speedFactor")]
    pub speed_factor: f64,
    /// Fernkampf-Reichweite aller Schützen in der Wetternacht (Faktor).
    #[serde(rename = "rangeFactor")]
    pub range_factor: f64,
}

/// Werte der Flut.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Flut {
    /// Jede n-te Nacht ist eine Wetternacht.
    #[serde(rename = "everyNights")]
    pub every_nights: i64,
    /// Frühestens in dieser Nacht.
    #[serde(rename = "firstNight")]
    pub first_night: i64,
    /// Breite der überfluteten Randzone je Rand in Units.
    #[serde(rename = "floodUnits")]
    pub flood_units: i64,
    /// Tempo in Units je Sekunde, mit dem Einheiten aus der Randzone laufen (muss über dem schnellsten Bodentempo liegen).
    #[serde(rename = "runUnits")]
    pub run_units: f64,
}

impl WeatherFile {
    /// Die Wetterart der Welt `world_id`; `None` = kein Wetter.
    pub fn kind_of(&self, world_id: &str) -> Option<&str> {
        self.worlds.get(world_id).map(String::as_str)
    }
}

/// Liest und prüft weather.json: Zahlen und Faktoren größer 0, jede Art in `worlds` ist im Katalog.
pub fn load(raw: &str) -> Result<WeatherFile, Fehler> {
    let f: WeatherFile = decode("data/weather.json", raw)?;
    check("data/weather.json", validate(&f))?;
    Ok(f)
}

fn positive(v: f64) -> bool {
    v.partial_cmp(&0.0) == Some(std::cmp::Ordering::Greater)
}

fn validate(f: &WeatherFile) -> Result<(), String> {
    for (world, kind) in &f.worlds {
        if !KINDS.contains(&kind.as_str()) {
            return Err(format!(
                "worlds.{world}: unbekannte Wetterart \"{kind}\" (erlaubt: {})",
                KINDS.join(", ")
            ));
        }
    }
    let t = &f.taifun;
    if t.every_nights <= 0 || t.first_night <= 0 {
        return Err("taifun.everyNights und taifun.firstNight müssen größer 0 sein".to_string());
    }
    if !positive(t.speed_factor) || !positive(t.range_factor) {
        return Err("taifun.speedFactor und taifun.rangeFactor müssen größer 0 sein".to_string());
    }
    let fl = &f.flut;
    if fl.every_nights <= 0 || fl.first_night <= 0 || fl.flood_units <= 0 || !positive(fl.run_units) {
        return Err(
            "flut.everyNights, flut.firstNight, flut.floodUnits und flut.runUnits müssen größer 0 sein".to_string(),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: &str = include_str!("../../../../data/weather.json");

    /// AC-03 `weather_load`: die Datei lädt, die Zahlen sind > 0, `standard` hat kein Wetter.
    #[test]
    fn weather_load() {
        let f = load(RAW).expect("weather.json");
        assert_eq!(f.kind_of("sengoku"), Some("taifun"));
        assert_eq!(f.kind_of("nil"), Some("flut"));
        assert_eq!(f.kind_of("standard"), None);
        assert_eq!(f.kind_of("britannien"), None);
        assert!(f.taifun.speed_factor < 1.0 && f.taifun.range_factor < 1.0);
        assert_eq!(f.flut.flood_units, 24);
    }

    /// AC-03: Eine unbekannte Art in `worlds` scheitert mit klarer Meldung (Welt und Art genannt).
    #[test]
    fn weather_unbekannte_art_scheitert_klar() {
        let bad = RAW.replace("\"sengoku\": \"taifun\"", "\"sengoku\": \"schneesturm\"");
        let e = load(&bad).expect_err("unbekannte Art").to_string();
        assert!(e.contains("schneesturm") && e.contains("sengoku"), "{e}");
    }

    #[test]
    fn weather_lehnt_null_ab() {
        for (from, to) in [
            ("\"speedFactor\": 0.7", "\"speedFactor\": 0"),
            ("\"rangeFactor\": 0.7", "\"rangeFactor\": 0"),
            ("\"floodUnits\": 24", "\"floodUnits\": 0"),
            ("\"runUnits\": 20", "\"runUnits\": 0"),
            (
                "\"firstNight\": 3,\n    \"speedFactor\"",
                "\"firstNight\": 0,\n    \"speedFactor\"",
            ),
        ] {
            let raw = RAW.replace("\r\n", "\n"); // Windows-Checkout mit CRLF (autocrlf)
            assert!(raw.contains(from), "{from}");
            assert!(load(&raw.replace(from, to)).is_err(), "{to}");
        }
    }
}
