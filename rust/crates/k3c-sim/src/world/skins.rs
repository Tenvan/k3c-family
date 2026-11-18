//! Welt-Feld `skins` (Port von `engine/sim/world_skins.go`, B-984, welten.md § 2.3): Basis-Asset → Ersatz-Asset, beide
//! `<gruppe>/<asset>` wie in data/paper.json. Die Engine prüft nur die Form und reicht die Zuordnung über `WorldDef::skins`
//! weiter; sie liest keine Grafik und rechnet nicht damit (das Spiel zeichnet, WE6.2).
//!
//! Abweichung von Go: Die Regex `^([a-z0-9][a-z0-9_-]*)/([a-z0-9][a-z0-9_-]*)$` ist von Hand geprüft (keine neue
//! Abhängigkeit). Die Fehlertexte sind wortgleich.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::island::WorldDef;

/// Ein Name der Form `[a-z0-9][a-z0-9_-]*`.
fn is_asset_name(s: &str) -> bool {
    let mut chars = s.chars();
    let first_ok = chars
        .next()
        .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
    first_ok && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
}

/// Go: `assetPattern.FindStringSubmatch`: Gruppe und Asset eines Pfads `<gruppe>/<asset>` (Kleinbuchstaben, Ziffern, - und
/// _); `None`, wenn die Form nicht stimmt.
fn asset_parts(s: &str) -> Option<(&str, &str)> {
    let (group, asset) = s.split_once('/')?;
    (is_asset_name(group) && is_asset_name(asset)).then_some((group, asset))
}

/// Go: `decodeWorldSkins`: liest `skins` der Welt streng (Objekt aus Text-Paaren) und prüft jedes Paar in
/// Schlüssel-Reihenfolge; ein Nicht-Objekt (auch `null`) ist ein Fehler. Fehler: `skins › <Basis-Asset>: <Grund>`. Das Feld
/// fehlt: der Aufrufer ruft nicht (Go: Ergebnis nil).
pub fn decode_world_skins(raw: &Value) -> Result<BTreeMap<String, String>, String> {
    let Value::Object(obj) = raw else {
        return Err("skins: Objekt Basis-Asset → Ersatz-Asset erwartet".to_string());
    };
    let mut pairs: Vec<(&String, &Value)> = obj.iter().collect();
    pairs.sort_by(|a, b| a.0.cmp(b.0)); // Schlüssel-Reihenfolge wie slices.Sorted(maps.Keys)
    let mut out = BTreeMap::new();
    for (base, value) in pairs {
        let to = match value {
            Value::String(s) => s.as_str(),
            Value::Null => "", // Go: null lässt den Text leer, die Prüfung meldet es
            _ => return Err(format!("skins › {base}: Ersatz-Asset muss ein Text sein")),
        };
        check_skin(base, to).map_err(|e| format!("skins › {base}: {e}"))?;
        out.insert(base.clone(), to.to_string());
    }
    Ok(out)
}

/// Go: `checkSkin`: prüft ein Paar: beide Seiten `<gruppe>/<asset>`, gleiche Gruppe, kein Verweis auf sich selbst.
fn check_skin(base: &str, to: &str) -> Result<(), String> {
    let Some((bg, _)) = asset_parts(base) else {
        return Err(format!("Basis-Asset {base:?} ist kein <gruppe>/<asset>"));
    };
    let Some((tg, _)) = asset_parts(to) else {
        return Err(format!("Ersatz-Asset {to:?} ist kein <gruppe>/<asset>"));
    };
    if bg != tg {
        return Err(format!("Ersatz-Asset {to:?} wechselt die Gruppe ({bg} → {tg})"));
    }
    if base == to {
        return Err("verweist auf sich selbst".to_string());
    }
    Ok(())
}

/// Go: `chainSkins`: mischt die Skins der Kette (älteste Welt zuerst); das Kind überschreibt je Schlüssel. Ohne Skins leer.
pub fn chain_skins(chain: &[WorldDef]) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for w in chain {
        out.extend(w.skins.iter().map(|(k, v)| (k.clone(), v.clone())));
    }
    out
}
