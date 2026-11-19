//! data/hermits.json: Einsiedler mit Hütte und Bauwerk (RU62.10, B-972). Neue Datei neben `hub.json` und `buildings.json`,
//! die Go-Ladepfade kennen sie nicht; die Sim liest sie in `k3c-sim/src/hermit/`.
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Fehler, check, decode};

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "hermits.json";

/// Der Katalog der Einsiedler (vorläufige Werte).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HermitsFile {
    /// Die Einsiedler des Katalogs.
    #[serde(rename = "hermits")]
    pub hermits: Vec<HermitEntry>,
    /// Lage und Reichweite der Hütte.
    #[serde(rename = "hut")]
    pub hut: HutData,
    /// Der Hub-Platz des Bauwerks.
    #[serde(rename = "hubSite")]
    pub hub_site: HubSiteData,
    /// Welt-ID auf Einsiedler-ID; `standard` gehört nicht dazu, Welten ohne Eintrag haben keinen Einsiedler.
    #[serde(rename = "worlds")]
    pub worlds: BTreeMap<String, String>,
}

/// Ein Einsiedler: Preis, Bauwerk und genau ein Bonus.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HermitEntry {
    #[serde(rename = "id")]
    pub id: String,
    /// Das Bauwerk (`dojo`, `shrine`), nur Name für die Anzeige.
    #[serde(rename = "building")]
    pub building: String,
    /// Münzen an der Hütte.
    #[serde(rename = "price")]
    pub price: i64,
    /// Bauzeit des Bauwerks in Sekunden.
    #[serde(rename = "buildSeconds")]
    pub build_seconds: f64,
    /// HP des Bauwerks.
    #[serde(rename = "hp")]
    pub hp: f64,
    /// Dojo: Schadensfaktor der Krieger und Schützen (> 1); 0 = kein solcher Bonus.
    #[serde(rename = "damageFactor")]
    pub damage_factor: f64,
    /// Schrein: Anteil der vollen HP, den jede stehende Mauer bei Tagesanbruch heilt (0 bis 1); 0 = kein solcher Bonus.
    #[serde(rename = "wallHealFraction")]
    pub wall_heal_fraction: f64,
}

/// Lage der Hütte (Tiefe 1) und Reichweite des Einwurfs.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HutData {
    /// Kleinster Abstand der Hütte zum fernen Rand der Stufe in Units.
    #[serde(rename = "edgeMarginMinUnits")]
    pub edge_margin_min_units: f64,
    /// Größter Abstand zum Rand in Units.
    #[serde(rename = "edgeMarginMaxUnits")]
    pub edge_margin_max_units: f64,
    /// Reichweite des Einwurfs in Units.
    #[serde(rename = "rangeUnits")]
    pub range_units: f64,
}

/// Der feste Platz des Bauwerks auf der Hub-Oberfläche.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HubSiteData {
    /// Art des Platzes (`hermitHall`).
    #[serde(rename = "id")]
    pub id: String,
    /// Versatz von der Hub-Mitte in Units.
    #[serde(rename = "offsetUnits")]
    pub offset_units: f64,
    /// Ab dieser Hub-Stufe nimmt der Platz den Bau an.
    #[serde(rename = "hubLevel")]
    pub hub_level: i64,
}

impl HermitsFile {
    /// Der Einsiedler mit der ID `id`.
    pub fn entry(&self, id: &str) -> Option<&HermitEntry> {
        self.hermits.iter().find(|h| h.id == id)
    }
}

/// Liest und prüft hermits.json: IDs eindeutig, Preis, Bauzeit und HP größer 0, je Einsiedler genau ein Bonus (Schadensfaktor
/// über 1 oder Heilanteil in (0, 1]), jede ID in `worlds` im Katalog, kein Eintrag für `standard`, Hütte und Platz gültig.
pub fn load(raw: &str) -> Result<HermitsFile, Fehler> {
    let f: HermitsFile = decode("data/hermits.json", raw)?;
    check("data/hermits.json", validate(&f))?;
    Ok(f)
}

fn positive(x: f64) -> bool {
    x.partial_cmp(&0.0) == Some(std::cmp::Ordering::Greater)
}

fn validate_entry(h: &HermitEntry) -> Result<(), String> {
    let id = &h.id;
    if h.building.is_empty() {
        return Err(format!("Einsiedler {id:?}: building darf nicht leer sein"));
    }
    if h.price <= 0 {
        return Err(format!("Einsiedler {id:?}: price muss größer 0 sein"));
    }
    if !positive(h.build_seconds) || !positive(h.hp) {
        return Err(format!("Einsiedler {id:?}: buildSeconds und hp müssen größer 0 sein"));
    }
    let damage = h.damage_factor != 0.0;
    let heal = h.wall_heal_fraction != 0.0;
    if damage == heal {
        return Err(format!(
            "Einsiedler {id:?}: genau ein Bonus erwartet (damageFactor oder wallHealFraction)"
        ));
    }
    let ok = if damage {
        h.damage_factor.partial_cmp(&1.0) == Some(std::cmp::Ordering::Greater)
    } else {
        positive(h.wall_heal_fraction) && h.wall_heal_fraction <= 1.0
    };
    if !ok {
        return Err(format!(
            "Einsiedler {id:?}: damageFactor muss über 1 liegen, wallHealFraction in (0, 1]"
        ));
    }
    Ok(())
}

fn validate(f: &HermitsFile) -> Result<(), String> {
    for (i, h) in f.hermits.iter().enumerate() {
        if h.id.is_empty() {
            return Err(format!("hermits[{i}]: id darf nicht leer sein"));
        }
        if f.hermits[..i].iter().any(|o| o.id == h.id) {
            return Err(format!("hermits[{i}]: id {:?} doppelt", h.id));
        }
        validate_entry(h)?;
    }
    for (world, id) in &f.worlds {
        if world == "standard" {
            return Err("worlds: die Welt standard hat keinen Einsiedler".to_string());
        }
        if f.entry(id).is_none() {
            return Err(format!(
                "worlds: Welt {world:?} nennt den unbekannten Einsiedler {id:?}"
            ));
        }
    }
    let hut = &f.hut;
    if !positive(hut.range_units) || !positive(hut.edge_margin_min_units) {
        return Err("hut: rangeUnits und edgeMarginMinUnits müssen größer 0 sein".to_string());
    }
    if hut.edge_margin_max_units.partial_cmp(&hut.edge_margin_min_units) == Some(std::cmp::Ordering::Less)
        || hut.edge_margin_max_units.is_nan()
    {
        return Err("hut: edgeMarginMaxUnits darf nicht unter edgeMarginMinUnits liegen".to_string());
    }
    if f.hub_site.id.is_empty() || f.hub_site.hub_level < 1 {
        return Err("hubSite: id darf nicht leer und hubLevel muss mindestens 1 sein".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: &str = include_str!("../../../../data/hermits.json");

    /// AC-05 `hermits_load`: die Datei lädt, beide Einsiedler stimmen, `standard` und `britannien` haben keinen.
    #[test]
    fn hermits_load() {
        let f = load(RAW).expect("hermits.json");
        assert_eq!(f.hermits.len(), 2);
        assert_eq!(f.worlds.get("sengoku").map(String::as_str), Some("swordsmith"));
        assert_eq!(f.worlds.get("nil").map(String::as_str), Some("priestess"));
        assert!(!f.worlds.contains_key("standard") && !f.worlds.contains_key("britannien"));
        let smith = f.entry("swordsmith").expect("Schwertschmied");
        assert_eq!(
            (smith.building.as_str(), smith.price, smith.damage_factor),
            ("dojo", 20, 1.15)
        );
        let priestess = f.entry("priestess").expect("Priesterin");
        assert_eq!(
            (priestess.building.as_str(), priestess.wall_heal_fraction),
            ("shrine", 0.25)
        );
        assert_eq!((f.hub_site.id.as_str(), f.hub_site.hub_level), ("hermitHall", 2));
    }

    /// AC-05: eine unbekannte ID in `worlds` lässt das Laden mit klarer Meldung scheitern.
    #[test]
    fn hermits_unbekannte_id_scheitert() {
        let raw = RAW.replace("\r\n", "\n"); // Windows-Checkout mit CRLF (autocrlf)
        let bad = raw.replace("\"nil\": \"priestess\"", "\"nil\": \"walfaenger\"");
        let e = load(&bad).expect_err("unbekannte ID").to_string();
        assert!(e.contains("walfaenger") && e.contains("nil"), "{e}");
    }

    #[test]
    fn hermits_lehnt_ungueltige_werte_ab() {
        let raw = RAW.replace("\r\n", "\n");
        for (from, to) in [
            ("\"damageFactor\": 1.15", "\"damageFactor\": 1.0"),
            ("\"damageFactor\": 1.15", "\"damageFactor\": 0"),
            ("\"wallHealFraction\": 0.25", "\"wallHealFraction\": 1.5"),
            (
                "\"wallHealFraction\": 0.25",
                "\"wallHealFraction\": 0.25, \"damageFactor\": 1.2",
            ),
            (
                "\"price\": 20, \"buildSeconds\": 20, \"hp\": 250",
                "\"price\": 0, \"buildSeconds\": 20, \"hp\": 250",
            ),
            ("\"hp\": 200", "\"hp\": 0"),
            ("\"rangeUnits\": 2.5", "\"rangeUnits\": 0"),
            ("\"edgeMarginMaxUnits\": 16", "\"edgeMarginMaxUnits\": 5"),
            ("\"hubLevel\": 2", "\"hubLevel\": 0"),
            ("\"nil\": \"priestess\"", "\"standard\": \"priestess\""),
        ] {
            assert!(raw.contains(from), "Vorlage fehlt: {from}");
            assert!(load(&raw.replace(from, to)).is_err(), "{to}");
        }
    }
}
