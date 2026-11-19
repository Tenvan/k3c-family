//! data/mounts.json: Welt-Reittiere (RU62.8, B-970). Neue Datei neben `monarch.json`, die Go-Ladepfade kennen sie nicht;
//! die Sim liest sie in `k3c-sim/src/mounts/`.
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Fehler, check, decode};

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "mounts.json";

/// Der Katalog der Welt-Reittiere (vorläufige Werte).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MountsFile {
    /// Die Reittiere des Katalogs.
    #[serde(rename = "mounts")]
    pub mounts: Vec<MountEntry>,
    /// Das Gatter, an dem das Reittier gekauft wird.
    #[serde(rename = "mountGate")]
    pub mount_gate: MountGateData,
    /// Welt-ID auf Reittier-ID; `standard` gehört nicht dazu, Welten ohne Eintrag haben kein Reittier.
    #[serde(rename = "worlds")]
    pub worlds: BTreeMap<String, String>,
}

/// Ein Reittier des Katalogs (Faktoren gelten absolut).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MountEntry {
    #[serde(rename = "id")]
    pub id: String,
    /// Schlüssel unter `mounts` in data/paper.json.
    #[serde(rename = "sprite")]
    pub sprite: String,
    #[serde(rename = "speedFactor")]
    pub speed_factor: f64,
    #[serde(rename = "sprintFactor")]
    pub sprint_factor: f64,
    /// Münzen für den Kauf am Gatter.
    #[serde(rename = "price")]
    pub price: i64,
}

/// Lage und Reichweite des Gatters.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MountGateData {
    /// Mindestabstand zum Hub in Units.
    #[serde(rename = "minHubDistanceUnits")]
    pub min_hub_distance_units: f64,
    /// Reichweite des Gatters in Units.
    #[serde(rename = "rangeUnits")]
    pub range_units: f64,
}

impl MountsFile {
    /// Das Reittier mit der ID `id`.
    pub fn entry(&self, id: &str) -> Option<&MountEntry> {
        self.mounts.iter().find(|m| m.id == id)
    }
}

/// Liest und prüft mounts.json: Faktoren >= 1, Preis > 0, IDs eindeutig, jede ID in `worlds` im Katalog, kein Eintrag für
/// `standard`, Abstand und Reichweite größer 0.
pub fn load(raw: &str) -> Result<MountsFile, Fehler> {
    let f: MountsFile = decode("data/mounts.json", raw)?;
    check("data/mounts.json", validate(&f))?;
    Ok(f)
}

fn at_least_one(x: f64) -> bool {
    x.partial_cmp(&1.0).is_some_and(|o| o.is_ge())
}

fn positive(x: f64) -> bool {
    x.partial_cmp(&0.0) == Some(std::cmp::Ordering::Greater)
}

fn validate(f: &MountsFile) -> Result<(), String> {
    for (i, m) in f.mounts.iter().enumerate() {
        if m.id.is_empty() || m.sprite.is_empty() {
            return Err(format!("mounts[{i}]: id und sprite dürfen nicht leer sein"));
        }
        if f.mounts[..i].iter().any(|o| o.id == m.id) {
            return Err(format!("mounts[{i}]: id {:?} doppelt", m.id));
        }
        if !at_least_one(m.speed_factor) {
            return Err(format!("Reittier {:?}: speedFactor muss mindestens 1 sein", m.id));
        }
        if !at_least_one(m.sprint_factor) {
            return Err(format!("Reittier {:?}: sprintFactor muss mindestens 1 sein", m.id));
        }
        if m.price <= 0 {
            return Err(format!("Reittier {:?}: price muss größer 0 sein", m.id));
        }
    }
    for (world, id) in &f.worlds {
        if world == "standard" {
            return Err("worlds: die Welt standard hat kein Welt-Reittier".to_string());
        }
        if f.entry(id).is_none() {
            return Err(format!("worlds: Welt {world:?} nennt das unbekannte Reittier {id:?}"));
        }
    }
    if !positive(f.mount_gate.min_hub_distance_units) {
        return Err("mountGate.minHubDistanceUnits muss größer 0 sein".to_string());
    }
    if !positive(f.mount_gate.range_units) {
        return Err("mountGate.rangeUnits muss größer 0 sein".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: &str = include_str!("../../../../data/mounts.json");

    /// AC-04 `mounts_load`: die Datei lädt, Faktoren und Preise stimmen, `standard` hat kein Reittier.
    #[test]
    fn mounts_load() {
        let f = load(RAW).expect("mounts.json");
        assert_eq!(f.mounts.len(), 2);
        assert_eq!(f.worlds.get("sengoku").map(String::as_str), Some("sengokuHorse"));
        assert_eq!(f.worlds.get("nil").map(String::as_str), Some("camel"));
        assert!(!f.worlds.contains_key("standard"));
        assert!(
            f.mounts
                .iter()
                .all(|m| m.price > 0 && m.sprint_factor >= 1.0 && m.speed_factor >= 1.0)
        );
        assert_eq!(f.mount_gate.min_hub_distance_units, 40.0);
    }

    /// AC-04: eine unbekannte ID in `worlds` lässt das Laden mit klarer Meldung scheitern.
    #[test]
    fn mounts_unbekannte_id_scheitert() {
        let bad = RAW.replace("\"nil\": \"camel\"", "\"nil\": \"drache\"");
        let e = load(&bad).expect_err("unbekannte ID").to_string();
        assert!(e.contains("drache") && e.contains("nil"), "{e}");
    }

    #[test]
    fn mounts_lehnt_ungueltige_werte_ab() {
        for (from, to) in [
            ("\"sprintFactor\": 1.3", "\"sprintFactor\": 0.9"),
            (
                "\"speedFactor\": 1.0, \"sprintFactor\": 1.5",
                "\"speedFactor\": 0.5, \"sprintFactor\": 1.5",
            ),
            (
                "\"sprintFactor\": 1.3, \"price\": 30",
                "\"sprintFactor\": 1.3, \"price\": 0",
            ),
            ("\"minHubDistanceUnits\": 40", "\"minHubDistanceUnits\": 0"),
            ("\"nil\": \"camel\"", "\"standard\": \"camel\""),
        ] {
            assert!(RAW.contains(from), "Vorlage fehlt: {from}");
            assert!(load(&RAW.replace(from, to)).is_err(), "{to}");
        }
    }
}
