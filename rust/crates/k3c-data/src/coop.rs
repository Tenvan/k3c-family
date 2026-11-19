//! data/coop.json: Koop-Aktionen (RU62.3, B-968). Neue Datei neben `economy.json`, die Go-Ladepfade kennen sie nicht;
//! die Sim liest sie in `k3c-sim/src/coop/`.
use serde::{Deserialize, Serialize};

use crate::{Fehler, check, decode};

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "coop.json";

/// Die Koop-Regeln (vorläufige Werte).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CoopFile {
    /// Welten mit Koop-Aktionen (`standard` gehört nicht dazu).
    #[serde(rename = "worlds")]
    pub worlds: Vec<String>,
    /// Die Doppel-Truhe.
    #[serde(rename = "doubleChest")]
    pub double_chest: DoubleChest,
}

/// Werte der Doppel-Truhe.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DoubleChest {
    /// Im Mittel jede n-te Truhe ist doppelt.
    #[serde(rename = "everyNth")]
    pub every_nth: i64,
    /// Multiplikator auf das normale Truhen-Gold.
    #[serde(rename = "goldFactor")]
    pub gold_factor: f64,
}

/// Liest und prüft coop.json: `everyNth` und `goldFactor` größer 0.
pub fn load(raw: &str) -> Result<CoopFile, Fehler> {
    let f: CoopFile = decode("data/coop.json", raw)?;
    check("data/coop.json", validate(&f))?;
    Ok(f)
}

fn validate(f: &CoopFile) -> Result<(), String> {
    if f.double_chest.every_nth <= 0 {
        return Err("doubleChest.everyNth muss größer 0 sein".to_string());
    }
    if f.double_chest.gold_factor.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return Err("doubleChest.goldFactor muss größer 0 sein".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: &str = include_str!("../../../../data/coop.json");

    /// AC-02 `coop_load`: die Datei lädt, die Werte sind > 0, `standard` ist unwirksam.
    #[test]
    fn coop_load() {
        let f = load(RAW).expect("coop.json");
        assert_eq!(f.double_chest.every_nth, 4);
        assert!(f.double_chest.gold_factor > 1.0);
        assert!(f.worlds.iter().all(|w| w != "standard"));
    }

    #[test]
    fn coop_load_lehnt_null_ab() {
        let bad = RAW.replace("\"everyNth\": 4", "\"everyNth\": 0");
        assert!(load(&bad).is_err());
        let bad = RAW.replace("\"goldFactor\": 3.0", "\"goldFactor\": 0");
        assert!(load(&bad).is_err());
    }
}
