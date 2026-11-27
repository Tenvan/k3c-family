//! Welt-Sicht des Katalogs (B-1169, WZ62.2): Welche Assets gehören zu einer Welt? Die Zuordnung steht in zwei Quellen:
//! `skins` der Welt-Dateien (Basis-Asset → Ersatz-Asset, docs/rules/welten.md § 2.3) und die Einträge unter der
//! Welt-ID in `data/paper.json` (weltbilder, leitmotive, seekarte …). Alle übrigen Assets sind Basis und gelten für
//! jede Welt (Herkunft standard). Die Sicht rechnet nur lesend aus diesen Dateien, nichts wird geschrieben.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Dienst, datei, lies_json};
use crate::kitx::go_quoted;

#[cfg(test)]
#[path = "welt_tests.rs"]
mod tests;

/// Herkunft der Assets ohne Welt-Zuordnung.
pub const WELT_BASIS: &str = "standard";

/// Welche Assets eine Welt sieht. Schlüssel sind `<gruppe>/<asset>` wie in `data/paper.json`.
///
/// - `eigene`: Assets, die eine Welt der Kette (Welt, ihr `extends`, …) exklusiv hat, mit der Herkunftswelt. Ist die
///   Herkunft die Welt selbst, ist das Asset für sie exklusiv.
/// - `aus`: Assets, die die Welt nicht sieht: exklusiv für eine fremde Welt oder per Skin durch ein Ersatz-Asset ersetzt.
/// - Alles übrige ist Basis (Herkunft [`WELT_BASIS`]) und erscheint nur im Modus „gesamt“.
#[derive(Debug, Clone, Default, Serialize)]
pub struct WeltSicht {
    pub welt: String,
    pub basis: String,
    pub eigene: BTreeMap<String, String>,
    pub aus: Vec<String>,
    /// Feste Welt ohne Datei („noch nicht umgesetzt“): nur Basis plus paper.json, keine Skins.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub fehlt: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct WeltDatei {
    extends: Option<String>,
    skins: Option<BTreeMap<String, String>>,
}

type Welten = BTreeMap<String, WeltDatei>;

/// Welt-ID wie `^[a-z0-9][a-z0-9-]{0,31}$`.
fn gueltige_welt(id: &str) -> bool {
    let zeichen = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit();
    id.len() <= 32 && id.bytes().next().is_some_and(zeichen) && id.bytes().all(|b| zeichen(b) || b == b'-')
}

/// Asset-Verweis wie `^[a-z0-9_-]+/[a-z0-9_-]+$`.
fn ist_asset_verweis(s: &str) -> bool {
    let teil = |t: &str| {
        !t.is_empty()
            && t.bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
    };
    s.split_once('/').is_some_and(|(a, b)| teil(a) && teil(b))
}

/// Die Welt, ihr `extends`, dessen `extends` … (ein Zyklus bricht ab).
fn welt_kette(welten: &Welten, welt: &str) -> Vec<String> {
    let mut kette: Vec<String> = Vec::new();
    let mut w = welt;
    while !w.is_empty() && !kette.iter().any(|k| k == w) {
        kette.push(w.to_owned());
        w = welten.get(w).and_then(|d| d.extends.as_deref()).unwrap_or_default();
    }
    kette
}

/// Sammelt alle Asset-Verweise (ohne `@variante`), die irgendwo unter dem Schlüssel `welt` stehen.
fn gehe(wert: &Value, drin: bool, welt: &str, out: &mut Vec<String>) {
    match wert {
        Value::Object(map) => {
            for (k, v) in map {
                gehe(v, drin || k == welt, welt, out);
            }
        }
        Value::Array(liste) => {
            for v in liste {
                gehe(v, drin, welt, out);
            }
        }
        Value::String(s) => {
            let verweis = s.split('@').next().unwrap_or_default();
            if drin && ist_asset_verweis(verweis) {
                out.push(verweis.to_owned());
            }
        }
        _ => {}
    }
}

impl Dienst {
    /// Die lesbaren Welt-Dateien; unlesbare zählen nicht.
    fn welten(&self) -> Welten {
        let ordner = datei(&self.root, "data/worlds");
        let Ok(eintraege) = std::fs::read_dir(&ordner) else {
            return Welten::new();
        };
        eintraege
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_ok_and(|t| t.is_dir()))
            .filter_map(|e| {
                let name = e.file_name().into_string().ok()?;
                let id = name.strip_suffix(".json").filter(|id| gueltige_welt(id))?;
                let welt: Option<WeltDatei> = lies_json(&e.path()).ok()?;
                Some((id.to_owned(), welt.unwrap_or_default()))
            })
            .collect()
    }

    /// Asset → Welten, in deren Daten es steht.
    fn besitzer(&self, welten: &Welten) -> BTreeMap<String, BTreeSet<String>> {
        let paper: Value = lies_json(&datei(&self.root, "data/paper.json")).unwrap_or_default();
        let mut ids: BTreeSet<&str> = welten.keys().map(String::as_str).collect();
        if let Some(bilder) = paper.get("weltbilder").and_then(Value::as_object) {
            ids.extend(bilder.keys().map(String::as_str).filter(|k| gueltige_welt(k)));
        }
        let mut owners: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for id in ids {
            let mut assets = Vec::new();
            gehe(&paper, false, id, &mut assets);
            for a in assets {
                owners.entry(a).or_default().insert(id.to_owned());
            }
        }
        for (id, w) in welten {
            for ersatz in w.skins.iter().flat_map(BTreeMap::values) {
                owners.entry(ersatz.clone()).or_default().insert(id.clone());
            }
        }
        owners
    }

    /// Die Sicht der Welt `welt` (Methode `StudioWeltSicht`).
    pub fn welt_sicht(&self, welt: &str) -> Result<WeltSicht, String> {
        if !gueltige_welt(welt) {
            return Err(format!("ungültige Welt-ID {}", go_quoted(welt)));
        }
        let welten = self.welten();
        let kette = welt_kette(&welten, welt);
        let mut eigene = BTreeMap::new();
        let mut aus = BTreeSet::new();
        for (asset, besitzer) in self.besitzer(&welten) {
            match kette.iter().find(|k| besitzer.contains(*k)) {
                Some(herkunft) => {
                    eigene.insert(asset, herkunft.clone());
                }
                None => {
                    aus.insert(asset);
                }
            }
        }
        // Skins der Kette: das ersetzte Basis-Asset zeigt nur sein Ersatz-Asset.
        let ersetzt = kette
            .iter()
            .filter_map(|k| welten.get(k)?.skins.as_ref())
            .flat_map(BTreeMap::keys)
            .filter(|basis| !eigene.contains_key(*basis))
            .cloned();
        aus.extend(ersetzt);
        Ok(WeltSicht {
            welt: welt.to_owned(),
            basis: WELT_BASIS.to_owned(),
            fehlt: !welten.contains_key(welt),
            eigene,
            aus: aus.into_iter().collect(),
        })
    }
}
