//! Schlüssel eines JSON-Werts in Datei-Reihenfolge (Hilfe für das strenge Lesen der Welt-Datei, `worlds_decode.rs`).
//!
//! Gos `checkKeys` liest die Schlüssel mit einem Token-Leser und sieht deshalb doppelte Schlüssel; `serde_json::Value`
//! behält nur den letzten. `Keys` liest die Schlüssel jedes Objekts (samt Unterobjekten) getrennt vom Wert, damit die
//! Prüfung auf unbekannte und doppelte Felder auf jeder Ebene wie in Go läuft.

use std::fmt;

use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Die Schlüssel eines Objekts mit den Schlüsseln seiner Werte; bei einer Liste die der Elemente, sonst leer.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Keys {
    entries: Vec<(String, Keys)>,
    items: Vec<Keys>,
}

/// Der leere Schlüsselbaum (ein fehlendes Feld).
static NONE: Keys = Keys {
    entries: Vec::new(),
    items: Vec::new(),
};

impl Keys {
    /// Aus einem gelesenen Wert; ohne doppelte Schlüssel (die kennt der Wert nicht mehr).
    pub fn from_value(v: &Value) -> Keys {
        match v {
            Value::Object(m) => Keys {
                entries: m.iter().map(|(k, x)| (k.clone(), Keys::from_value(x))).collect(),
                items: Vec::new(),
            },
            Value::Array(a) => Keys {
                entries: Vec::new(),
                items: a.iter().map(Keys::from_value).collect(),
            },
            _ => Keys::default(),
        }
    }

    /// Die Schlüssel des Feldes `key` (leer, wenn es fehlt).
    pub fn child(&self, key: &str) -> &Keys {
        self.entries.iter().find(|(k, _)| k == key).map_or(&NONE, |(_, c)| c)
    }

    /// Die Schlüsselbäume der Listenelemente.
    pub fn items(&self) -> &[Keys] {
        &self.items
    }

    /// Prüft die Schlüssel der obersten Ebene gegen `fields`: unbekannt und doppelt sind Fehler (Go `checkKeys`; die
    /// erste Verletzung in Datei-Reihenfolge wird gemeldet).
    pub fn check(&self, fields: &[&str]) -> Result<(), String> {
        for (i, (key, _)) in self.entries.iter().enumerate() {
            if !fields.contains(&key.as_str()) {
                return Err(format!("unbekanntes Feld {key:?}"));
            }
            if self.entries[..i].iter().any(|(k, _)| k == key) {
                return Err(format!("doppeltes Feld {key:?}"));
            }
        }
        Ok(())
    }
}

struct KeysVisitor;

impl<'de> Visitor<'de> for KeysVisitor {
    type Value = Keys;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ein JSON-Wert")
    }

    fn visit_bool<E>(self, _: bool) -> Result<Keys, E> {
        Ok(Keys::default())
    }

    fn visit_i64<E>(self, _: i64) -> Result<Keys, E> {
        Ok(Keys::default())
    }

    fn visit_u64<E>(self, _: u64) -> Result<Keys, E> {
        Ok(Keys::default())
    }

    fn visit_f64<E>(self, _: f64) -> Result<Keys, E> {
        Ok(Keys::default())
    }

    fn visit_str<E>(self, _: &str) -> Result<Keys, E> {
        Ok(Keys::default())
    }

    fn visit_unit<E>(self) -> Result<Keys, E> {
        Ok(Keys::default())
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Keys, A::Error> {
        let mut items = Vec::new();
        while let Some(k) = seq.next_element::<Keys>()? {
            items.push(k);
        }
        Ok(Keys {
            entries: Vec::new(),
            items,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Keys, A::Error> {
        let mut entries = Vec::new();
        while let Some(key) = map.next_key::<String>()? {
            entries.push((key, map.next_value::<Keys>()?));
        }
        Ok(Keys {
            entries,
            items: Vec::new(),
        })
    }
}

impl<'de> Deserialize<'de> for Keys {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Keys, D::Error> {
        d.deserialize_any(KeysVisitor)
    }
}
