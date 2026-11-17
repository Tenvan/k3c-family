//! data/ruins.json: Go `level.RuinRules` (engine/level/ruins.go, `loadRuinRules`, `checkRuinRules`).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::buildings::BuildingData;
use crate::{Fehler, decode};

/// Go: `level.RuinRules`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RuinRules {
    #[serde(rename = "hpShare")]
    pub hp_share: f64,
    #[serde(rename = "kinds")]
    pub kinds: Vec<String>,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "ruins.json";

/// Gebäude, die dem Hub gehören und keine Ruine sein dürfen (engine/level `hubOnlyKinds`).
const HUB_ONLY_KINDS: [&str; 6] = ["castle", "wall", "tower", "gate", "stairsUp", "stairsDown"];

/// `load` liest ruins.json und prüft gegen die Gebäude wie `loadRuinRules` (dort `panic` mit `ruins.json: …`).
pub fn load(raw: &str, buildings: &BTreeMap<String, BuildingData>) -> Result<RuinRules, Fehler> {
    let r: RuinRules = decode("ruins.json", raw)?;
    check_ruin_rules(&r, buildings).map_err(|e| Fehler::Pruefung(format!("ruins.json: {e}")))?;
    Ok(r)
}

/// `check_ruin_rules` prüft: Anteil in (0, 1], jede Art bekannt und kein Hub-Gebäude.
fn check_ruin_rules(r: &RuinRules, buildings: &BTreeMap<String, BuildingData>) -> Result<(), String> {
    if !(r.hp_share > 0.0 && r.hp_share <= 1.0) {
        return Err(format!(
            "hpShare {} liegt nicht in (0, 1]",
            k3c_det::golden::go_float(r.hp_share)
        ));
    }
    if r.kinds.is_empty() {
        return Err("keine Art erlaubt".into());
    }
    for k in &r.kinds {
        if HUB_ONLY_KINDS.contains(&k.as_str()) {
            return Err(format!("{k:?} gehört dem Hub und ist keine Ruine"));
        }
        if !buildings.contains_key(k) {
            return Err(format!("unbekannte Gebäudeart {k:?}"));
        }
    }
    Ok(())
}
