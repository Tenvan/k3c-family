//! data/cargo.json: Fracht auf dem Schiff (RU62.1, B-967). Neue Datei neben `ship.json`, die Go-Ladepfade kennen sie nicht;
//! die Sim liest sie in `k3c-sim/src/cargo/`.

use serde::{Deserialize, Serialize};

use crate::buildings::Cost;
use crate::{Fehler, check, decode};

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "cargo.json";

/// Die Fracht-Regeln (vorläufige Werte).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CargoFile {
    /// Münzen, die ein Beutel fasst (und die der Einwurf am Münz-Ladeplatz kostet).
    #[serde(rename = "purseCoins")]
    pub purse_coins: i64,
    /// Einheiten Material einer Kiste.
    #[serde(rename = "crateUnits")]
    pub crate_units: i64,
    /// Plätze des Grundschiffs.
    #[serde(rename = "baseSlots")]
    pub base_slots: i64,
    /// Münzen für den Ruf einer Truppe an Bord.
    #[serde(rename = "troopCoins")]
    pub troop_coins: i64,
    /// Münzen für den Auftrag an einen Bauern, eine Kiste zu tragen.
    #[serde(rename = "crateCoins")]
    pub crate_coins: i64,
    /// Tempo des Bauern mit der Kiste in Units je Sekunde.
    #[serde(rename = "carrierSpeed")]
    pub carrier_speed: f64,
    /// Reichweite eines Ladeplatzes in Units.
    #[serde(rename = "bayRangeUnits")]
    pub bay_range_units: f64,
    /// Lage der Ladeplätze zum Schiff in Units.
    #[serde(rename = "bays")]
    pub bays: CargoBays,
    /// Welten, die Fracht tragen (`standard` gehört nicht dazu).
    #[serde(rename = "worlds")]
    pub worlds: Vec<String>,
    /// Optionaler vierter Abschnitt des Schiffs.
    #[serde(rename = "hold")]
    pub hold: CargoHold,
}

/// Versatz der drei Ladeplätze zum Schiff (Units, negativ = links).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CargoBays {
    #[serde(rename = "coins")]
    pub coins: f64,
    #[serde(rename = "troops")]
    pub troops: f64,
    #[serde(rename = "crates")]
    pub crates: f64,
}

/// Der Abschnitt „Laderaum“: wie ein Schiffsabschnitt, dazu `extraSlots`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CargoHold {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "optional")]
    pub optional: bool,
    #[serde(rename = "extraSlots")]
    pub extra_slots: i64,
    #[serde(rename = "cost")]
    pub cost: Cost,
    #[serde(rename = "hp")]
    pub hp: f64,
    #[serde(rename = "buildSeconds")]
    pub build_seconds: f64,
}

/// Liest und prüft cargo.json: alle Zahlen größer 0, der Laderaum ist optional.
pub fn load(raw: &str) -> Result<CargoFile, Fehler> {
    let f: CargoFile = decode("data/cargo.json", raw)?;
    check("data/cargo.json", validate(&f))?;
    Ok(f)
}

fn validate(f: &CargoFile) -> Result<(), String> {
    let ints = [
        ("purseCoins", f.purse_coins),
        ("crateUnits", f.crate_units),
        ("baseSlots", f.base_slots),
        ("troopCoins", f.troop_coins),
        ("crateCoins", f.crate_coins),
        ("hold.extraSlots", f.hold.extra_slots),
    ];
    for (name, v) in ints {
        if v <= 0 {
            return Err(format!("{name} muss größer 0 sein"));
        }
    }
    let floats = [
        ("carrierSpeed", f.carrier_speed),
        ("bayRangeUnits", f.bay_range_units),
        ("hold.hp", f.hold.hp),
        ("hold.buildSeconds", f.hold.build_seconds),
    ];
    for (name, v) in floats {
        if v.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
            return Err(format!("{name} muss größer 0 sein"));
        }
    }
    if !f.hold.optional {
        return Err("hold.optional muss true sein (die Abfahrt verlangt den Laderaum nicht)".to_string());
    }
    if f.hold.cost.gold <= 0 {
        return Err("hold.cost.gold muss größer 0 sein".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: &str = include_str!("../../../../data/cargo.json");

    /// AC-01 `cargo_load`: die Datei lädt und alle Werte sind > 0.
    #[test]
    fn cargo_load() {
        let f = load(RAW).expect("cargo.json");
        assert_eq!((f.purse_coins, f.crate_units, f.base_slots), (10, 20, 6));
        assert!(f.hold.optional && f.hold.extra_slots > 0);
        assert!(f.worlds.iter().all(|w| w != "standard"));
    }

    #[test]
    fn cargo_load_lehnt_null_ab() {
        let bad = RAW.replace("\"baseSlots\": 6", "\"baseSlots\": 0");
        assert!(load(&bad).is_err());
        let bad = RAW.replace("\"optional\": true", "\"optional\": false");
        assert!(load(&bad).is_err());
    }
}
