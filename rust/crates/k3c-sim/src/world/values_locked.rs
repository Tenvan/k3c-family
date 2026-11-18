//! Nicht übersteuerbare Felder (Port von `engine/sim/world_values_locked.go`, docs/rules/welten.md § 1.3, B-692): Der
//! Level-Generator (`k3c_level`) liest diese Werte aus den eingebetteten Dateien, nicht aus dem Wertesatz der Welt
//! (`hub.sites` und `hub.islandSites` bekommt er seit B-983 als Wert, `Options::hub_sites`). Ein Override darauf ließe Level
//! (Lage der Plätze, Zahlziele, Ruinen-Planung) und Simulation auseinanderlaufen; es wird beim Laden mit Pfad abgelehnt
//! (fail closed).
//!
//! Go: `lockedFields` (Map) ist hier die `match`-Funktion `locked_fields`; `checkLocked` arbeitet auf `serde_json::Value`
//! statt auf `json.RawMessage`.

use serde_json::Value;

/// Go: `lockedPlantation`: `buildings.<id>.plantation.stripUnits` prüft die Ruinen-Planung gegen die Basis.
const LOCKED_PLANTATION: &str = "stripUnits";

/// Go: `lockedFields`: je Basis-Datei die Felder der obersten Ebene, die keine Welt übersteuert.
pub fn locked_fields(name: &str) -> &'static [&'static str] {
    match name {
        "economy" => &["payRangeUnits"],
        "hub" => &["sitePlacement"],
        _ => &[],
    }
}

/// Go: `checkLocked`: lehnt ein Override der Datei `name` auf ein nicht übersteuerbares Feld ab; der Pfad folgt den
/// Meldungen von `merge_file`. Formfehler meldet `merge_file` vorher, hier gilt ein Override ohne Objekt als in Ordnung.
pub fn check_locked(name: &str, ov: &Value) -> Result<(), String> {
    let Some(obj) = ov.as_object() else {
        return Ok(());
    };
    for f in locked_fields(name) {
        if obj.contains_key(*f) {
            return Err(locked_err(f));
        }
    }
    if name != "buildings" {
        return Ok(());
    }
    let mut ids: Vec<&String> = obj.keys().collect();
    ids.sort();
    for id in ids {
        let plantation = obj.get(id).and_then(|b| b.get("plantation")).and_then(Value::as_object);
        // Go: `b.Plantation[lockedPlantation] != nil`; ein `null`-Wert ist dort ein Eintrag ("null") und zählt.
        if plantation.is_some_and(|p| p.contains_key(LOCKED_PLANTATION)) {
            return Err(locked_err(&format!("{id}.plantation.{LOCKED_PLANTATION}")));
        }
    }
    Ok(())
}

/// Go: `lockedErr`.
fn locked_err(path: &str) -> String {
    format!("{path}: nicht übersteuerbar (der Level-Generator liest die Basis, welten.md § 1.3)")
}
