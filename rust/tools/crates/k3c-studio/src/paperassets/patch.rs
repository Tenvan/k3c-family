//! Reihenfolgetreuer JSON-Patch und Format der Specs (Port von `patch.go`): rohe Werte bleiben, wie sie sind.

use std::fmt;
use std::sync::LazyLock;

use regex::{Captures, Regex};
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

use super::Error;
use crate::kitx::{compact, go_quote, indent, marshal_raw};

/// Tiefste Verschachtelung eines Patches; der Patch wird rekursiv angewandt.
const MAX_TIEFE: usize = 128;

/// Die Felder eines Objekts (Schlüssel, roher Wert) in Reihenfolge des Dokuments.
pub(super) type Felder = Vec<(String, String)>;

struct Gelesen(Felder);

impl<'de> Deserialize<'de> for Gelesen {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Sammle;

        impl<'de> Visitor<'de> for Sammle {
            type Value = Gelesen;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("ein Objekt")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Gelesen, A::Error> {
                let mut felder = Vec::new();
                while let Some((key, value)) = map.next_entry::<String, Box<RawValue>>()? {
                    felder.push((key, value.get().to_owned()));
                }
                Ok(Gelesen(felder))
            }
        }

        deserializer.deserialize_map(Sammle)
    }
}

/// Zerlegt `raw` in seine Felder; `None`, wenn es kein Objekt ist.
pub(super) fn objekt(raw: &str) -> Result<Option<Felder>, Error> {
    let raw = raw.trim();
    if !raw.starts_with('{') {
        return Ok(None);
    }
    // Wie Gos Decoder: nur der erste Wert zählt.
    let gelesen = Gelesen::deserialize(&mut serde_json::Deserializer::from_str(raw))?;
    Ok(Some(gelesen.0))
}

pub(super) fn index(felder: &[(String, String)], key: &str) -> Option<usize> {
    felder.iter().position(|(k, _)| k == key)
}

pub(super) fn ist_null(raw: &str) -> bool {
    raw.trim() == "null"
}

/// Go `json.Compact`: Fehler bei ungültigem JSON.
pub(super) fn kompakt(raw: &str) -> Result<String, Error> {
    serde_json::from_str::<&RawValue>(raw)?;
    Ok(compact(raw))
}

pub(super) fn schreibe_objekt(felder: &[(String, String)]) -> Result<String, Error> {
    let teile = felder
        .iter()
        .map(|(k, v)| Ok(format!("{}:{}", marshal_raw(&go_quote(k)), kompakt(v)?)))
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(format!("{{{}}}", teile.join(",")))
}

/// Verschachtelungstiefe eines gültigen JSON-Texts (ohne Rekursion gezählt).
fn tiefe(raw: &str) -> usize {
    let (mut jetzt, mut max) = (0usize, 0usize);
    let (mut in_text, mut maskiert) = (false, false);
    for b in raw.bytes() {
        if in_text {
            in_text = maskiert || b != b'"';
            maskiert = !maskiert && b == b'\\';
            continue;
        }
        match b {
            b'"' => in_text = true,
            b'{' | b'[' => {
                jetzt += 1;
                max = max.max(jetzt);
            }
            b'}' | b']' => jetzt = jetzt.saturating_sub(1),
            _ => {}
        }
    }
    max
}

/// Wendet einen JSON-Merge-Patch (RFC 7396) an: `null` löscht, Objekte rekursiv, alles andere ersetzt. Die
/// Reihenfolge der Schlüssel des Ziels bleibt, neue kommen hinten an; Zahlen bleiben roh. Leeres Ziel = keines.
pub fn merge_patch(ziel: &str, patch: &str) -> Result<String, Error> {
    if serde_json::from_str::<&RawValue>(patch).is_err() {
        return Err(Error::Sonst("Patch ist kein gültiges JSON".to_owned()));
    }
    if tiefe(patch) > MAX_TIEFE {
        return Err(Error::Sonst(format!("Patch tiefer als {MAX_TIEFE} Ebenen")));
    }
    mische(ziel, patch)
}

fn mische(ziel: &str, patch: &str) -> Result<String, Error> {
    let Some(aenderungen) = objekt(patch)? else {
        return kompakt(patch);
    };
    let mut felder = objekt(ziel)?.unwrap_or_default();
    for (key, value) in aenderungen {
        let i = index(&felder, &key);
        if ist_null(&value) {
            if let Some(i) = i {
                felder.remove(i);
            }
            continue;
        }
        match i {
            Some(i) => felder[i].1 = mische(&felder[i].1, &value)?,
            None => {
                let neu = mische("", &value)?;
                felder.push((key, neu));
            }
        }
    }
    schreibe_objekt(&felder)
}

/// Eingerückte Liste nur aus Zahlen; [`format`] schreibt sie in eine Zeile wie aus-prototyp.ts.
static ZAHLEN_LISTE: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"\[[\t\n\f\r ]*(-?[0-9.eE+-]+(?:,[\t\n\f\r ]*-?[0-9.eE+-]+)*)[\t\n\f\r ]*\]").ok());
static TRENNER: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r",[\t\n\f\r ]*").ok());

/// Rückt eine Spec wie die Fixtures ein (2 Leerzeichen, Zahlenlisten in einer Zeile, Zeilenende am Schluss).
pub fn format(raw: &str) -> Result<String, Error> {
    let eingerueckt = indent(&kompakt(raw)?, "", "  ");
    let (Some(liste), Some(trenner)) = (ZAHLEN_LISTE.as_ref(), TRENNER.as_ref()) else {
        return Err(Error::Sonst("Muster der Zahlenlisten ungültig".to_owned()));
    };
    let mut out = liste
        .replace_all(&eingerueckt, |m: &Captures<'_>| {
            format!("[{}]", trenner.replace_all(&m[1], ", "))
        })
        .into_owned();
    out.push('\n');
    Ok(out)
}

/// Die Schlüssel des Objekts unter `pfad` (leer = oberste Ebene) in Reihenfolge des Dokuments.
pub fn schluessel(raw: &str, pfad: &[&str]) -> Vec<String> {
    let mut felder = objekt(raw).ok().flatten().unwrap_or_default();
    for p in pfad {
        let Some(i) = index(&felder, p) else {
            return Vec::new();
        };
        felder = objekt(&felder[i].1).ok().flatten().unwrap_or_default();
    }
    felder.into_iter().map(|(k, _)| k).collect()
}

/// Kürzt ein Objekt auf die genannten obersten Schlüssel (Reihenfolge des Dokuments); ohne Liste alle außer `$schema`.
/// Ein unbekannter Schlüssel ist ein Fehler mit den vorhandenen.
pub fn felder(raw: &str, keys: &[&str]) -> Result<String, Error> {
    let Ok(Some(alle)) = objekt(raw) else {
        return Err(Error::Sonst("Spec ist kein JSON-Objekt".to_owned()));
    };
    let vorhanden: Vec<&str> = alle
        .iter()
        .map(|(k, _)| k.as_str())
        .filter(|k| *k != "$schema")
        .collect();
    if let Some(k) = keys.iter().find(|k| !vorhanden.contains(k)) {
        return Err(Error::Sonst(format!(
            "Feld „{k}“ fehlt; vorhanden: {}",
            vorhanden.join(", ")
        )));
    }
    let out: Felder = alle
        .into_iter()
        .filter(|(k, _)| k != "$schema" && (keys.is_empty() || keys.contains(&k.as_str())))
        .collect();
    schreibe_objekt(&out)
}

#[cfg(test)]
#[path = "patch_tests.rs"]
mod tests;
