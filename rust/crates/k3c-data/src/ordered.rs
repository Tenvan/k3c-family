//! `Ordered` ist ein JSON-Objekt in Datei-Reihenfolge (Go: `level.Ordered[V]`, engine/level/biome.go).
//!
//! TS durchläuft Objekte mit `Object.entries` in Einfügereihenfolge; eine Map hätte eine andere Reihenfolge und
//! damit andere Level. Gelesen wird ein Objekt, geschrieben wie in Go eine Liste aus `{"Key", "Value"}`
//! (Go hat für `Ordered` nur `UnmarshalJSON`).

use std::fmt;
use std::marker::PhantomData;

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Go: `level.Entry[V]`, ein Schlüssel eines JSON-Objekts mit seinem Wert.
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Entry<V> {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: V,
}

/// Go: `level.Ordered[V]`.
#[derive(Debug, Clone, PartialEq)]
pub struct Ordered<V>(pub Vec<Entry<V>>);

impl<V> Default for Ordered<V> {
    fn default() -> Self {
        Ordered(Vec::new())
    }
}

impl<V> Ordered<V> {
    /// `get` liefert den Wert zu key.
    pub fn get(&self, key: &str) -> Option<&V> {
        self.0.iter().find(|e| e.key == key).map(|e| &e.value)
    }

    /// `len` ist die Zahl der Schlüssel.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// `is_empty`: keine Schlüssel.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<V: Serialize> Serialize for Ordered<V> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

struct OrderedVisitor<V>(PhantomData<V>);

impl<'de, V: Deserialize<'de>> Visitor<'de> for OrderedVisitor<V> {
    type Value = Ordered<V>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("JSON-Objekt")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Ordered<V>, A::Error> {
        let mut out = Vec::new();
        while let Some((key, value)) = map.next_entry::<String, V>()? {
            out.push(Entry { key, value });
        }
        Ok(Ordered(out))
    }
}

impl<'de, V: Deserialize<'de>> Deserialize<'de> for Ordered<V> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(OrderedVisitor(PhantomData))
    }
}

#[cfg(test)]
mod ordered_tests {
    use super::Ordered;

    #[test]
    fn reihenfolge_der_datei() {
        let o: Ordered<i64> = serde_json::from_str(r#"{"z": 1, "a": 2, "m": 3}"#).expect("lesen");
        let keys: Vec<&str> = o.0.iter().map(|e| e.key.as_str()).collect();
        assert_eq!(keys, ["z", "a", "m"]);
        assert_eq!(o.get("a"), Some(&2));
        assert_eq!(
            serde_json::to_string(&o).expect("schreiben"),
            r#"[{"Key":"z","Value":1},{"Key":"a","Value":2},{"Key":"m","Value":3}]"#
        );
    }
}
