//! Was sich an der Stil-Datei geändert hat und welche Specs das trifft; dazu die Animationsnamen einer Spec.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::Deserializer;
use serde::de::{IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde_json::value::RawValue;
use serde_json::{Map, Value};

use super::cli::Error;
use super::hash::{kanonisch, lies_wert};

/// Änderung an der Stil-Datei gegenüber der Basis: alles (Papier, Licht …) oder nur einzelne Materialien und
/// Einzeltöne. Ein Asset ist betroffen, wenn es einen geänderten Namen nutzt (B-701 D-Zeile zu `stil` je Asset).
pub(super) struct StilAenderung {
    alle: bool,
    namen: BTreeSet<String>,
}

/// Abschnitte der Stil-Datei, deren Einträge Specs per Namen nutzen.
const NAMENS_LISTEN: [&str; 2] = ["materialien", "einzeltoene"];

fn objekt(raw: &[u8]) -> Result<Map<String, Value>, Error> {
    let wert: Option<Map<String, Value>> = serde_json::from_value(lies_wert(raw)?)?;
    Ok(wert.unwrap_or_default())
}

fn vereinigung<'a>(a: &'a Map<String, Value>, b: &'a Map<String, Value>) -> BTreeSet<&'a String> {
    a.keys().chain(b.keys()).collect()
}

fn gleich(a: Option<&Value>, b: Option<&Value>) -> bool {
    kanonisch(a.unwrap_or(&Value::Null)) == kanonisch(b.unwrap_or(&Value::Null))
}

pub(super) fn vergleiche_stil(alt: &[u8], neu: &[u8]) -> Result<StilAenderung, Error> {
    let (a, n) = (objekt(alt)?, objekt(neu)?);
    let mut s = StilAenderung {
        alle: false,
        namen: BTreeSet::new(),
    };
    let leer = Map::new();
    for k in vereinigung(&a, &n) {
        if k == "$schema" {
            continue;
        }
        if !NAMENS_LISTEN.contains(&k.as_str()) {
            s.alle = s.alle || !gleich(a.get(k), n.get(k));
            continue;
        }
        let ma = a.get(k).and_then(Value::as_object).unwrap_or(&leer);
        let mn = n.get(k).and_then(Value::as_object).unwrap_or(&leer);
        for name in vereinigung(ma, mn) {
            if !gleich(ma.get(name), mn.get(name)) {
                s.namen.insert(name.clone());
            }
        }
    }
    Ok(s)
}

impl StilAenderung {
    /// Trifft die Änderung die Spec: alles geändert oder ein genutzter Name (jeder Text der Spec zählt, so sind Lagen,
    /// Umhang und Varianten erfasst).
    pub(super) fn betrifft(&self, spec: &[u8]) -> bool {
        if self.alle {
            return true;
        }
        if self.namen.is_empty() {
            return false;
        }
        lies_wert(spec).map_or(true, |v| self.nutzt_namen(&v))
    }

    fn nutzt_namen(&self, v: &Value) -> bool {
        match v {
            Value::String(s) => self.namen.contains(s),
            Value::Array(list) => list.iter().any(|e| self.nutzt_namen(e)),
            Value::Object(map) => map.values().any(|e| self.nutzt_namen(e)),
            _ => false,
        }
    }
}

/// Die Schlüssel eines Objekts in Datei-Reihenfolge; alles andere ergibt keine.
struct Schluessel;

impl<'de> Visitor<'de> for Schluessel {
    type Value = Vec<String>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ein JSON-Wert")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut namen = Vec::new();
        while let Some(name) = map.next_key::<String>()? {
            map.next_value::<IgnoredAny>()?;
            namen.push(name);
        }
        Ok(namen)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        while seq.next_element::<IgnoredAny>()?.is_some() {}
        Ok(Vec::new())
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(Vec::new())
    }

    fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
        Ok(Vec::new())
    }

    fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
        Ok(Vec::new())
    }

    fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
        Ok(Vec::new())
    }

    fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
        Ok(Vec::new())
    }

    fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
        Ok(Vec::new())
    }
}

/// Alle Animationsnamen in Datei-Reihenfolge.
pub fn animationen(spec: &[u8]) -> Result<Vec<String>, Error> {
    let text = String::from_utf8_lossy(spec);
    let top: Option<BTreeMap<String, Box<RawValue>>> = serde_json::from_str(&text)?;
    let Some(raw) = top.as_ref().and_then(|t| t.get("animationen")) else {
        return Ok(Vec::new());
    };
    let mut leser = serde_json::Deserializer::from_str(raw.get());
    Ok(leser.deserialize_any(Schluessel)?)
}

/// Der erste Schlüssel von `animationen` in Datei-Reihenfolge; leer ohne Animation.
pub fn erste_animation(spec: &[u8]) -> Result<String, Error> {
    Ok(animationen(spec)?.into_iter().next().unwrap_or_default())
}
