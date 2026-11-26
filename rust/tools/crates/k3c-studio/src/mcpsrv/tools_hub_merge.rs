//! JSON-Merge-Patch (RFC 7396) für `hub_check` und `hub_save` (Port von `MergePatch` aus `paperassets/patch.go`, hier
//! vorgezogen, V11): `null` löscht, Objekte werden rekursiv gemischt, alles andere ersetzt. Die Reihenfolge der
//! Schlüssel des Ziels bleibt, neue kommen hinten an; Zahlen bleiben roh.

use std::fmt;

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use serde_json::value::RawValue;

use crate::kitx::{compact, go_marshal};

/// Die Felder eines Objekts in Reihenfolge des Dokuments, Werte roh.
struct Fields(Vec<(String, String)>);

impl<'de> Deserialize<'de> for Fields {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Collect;

        impl<'de> Visitor<'de> for Collect {
            type Value = Fields;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("ein Objekt")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Fields, A::Error> {
                let mut fields = Vec::new();
                while let Some((key, value)) = map.next_entry::<String, Box<RawValue>>()? {
                    fields.push((key, value.get().to_owned()));
                }
                Ok(Fields(fields))
            }
        }

        deserializer.deserialize_map(Collect)
    }
}

/// Die Felder von `raw`; `None`, wenn es kein Objekt ist.
fn object(raw: &str) -> Result<Option<Fields>, String> {
    if !raw.trim_start().starts_with('{') {
        return Ok(None);
    }
    serde_json::from_str(raw).map(Some).map_err(|e| e.to_string())
}

/// Wendet `patch` auf `target` an (leer = kein Ziel). Die Tiefe der Rekursion ist die des Patches; der kommt als
/// begrenzt gelesener JSON-Wert.
pub(super) fn merge_patch(target: &str, patch: &str) -> Result<String, String> {
    serde_json::from_str::<&RawValue>(patch).map_err(|_| "Patch ist kein gültiges JSON".to_owned())?;
    let Some(changes) = object(patch)? else {
        return Ok(compact(patch));
    };
    let mut fields = object(target)?.map_or_else(Vec::new, |f| f.0);
    for (key, value) in changes.0 {
        let at = fields.iter().position(|(k, _)| *k == key);
        if value.trim() == "null" {
            if let Some(i) = at {
                fields.remove(i);
            }
            continue;
        }
        match at {
            Some(i) => fields[i].1 = merge_patch(&fields[i].1, &value)?,
            None => fields.push((key, merge_patch("", &value)?)),
        }
    }
    let parts: Vec<String> = fields
        .into_iter()
        .map(|(key, value)| format!("{}:{}", go_marshal(&Value::String(key)), compact(&value)))
        .collect();
    Ok(format!("{{{}}}", parts.join(",")))
}

#[cfg(test)]
mod tests {
    use super::merge_patch;

    /// Neu: Reihenfolge und Zahlform des Ziels bleiben, `null` löscht, Listen werden ersetzt.
    #[test]
    fn mischt_wie_go() {
        let target = r#"{ "z": 1.50, "a": { "k": 1, "weg": 2 }, "l": [1, 2], "x": 3 }"#;
        let patch = r#"{"a":{"weg":null,"neu":{"n":null,"m":1}},"l":[3],"x":null,"b":"<"}"#;
        let got = merge_patch(target, patch).unwrap();
        assert_eq!(got, r#"{"z":1.50,"a":{"k":1,"neu":{"m":1}},"l":[3],"b":"<"}"#);
        assert_eq!(merge_patch(target, "[ 1 ]").unwrap(), "[1]");
        assert!(merge_patch(target, "{").is_err());
    }
}
