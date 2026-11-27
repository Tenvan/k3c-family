//! Werte-Ansicht (Port von `values.go`, docs/rules/welten.md § 1): je Feld der Basiswert (Basis `data/`, darüber die
//! Overrides der Eltern-Welten; gerechnet von `k3c-game sim world` op values) und der Override der Welt. Dazu die
//! Helfer, die ein JSON-Objekt in der Reihenfolge der Datei lesen.

use std::collections::BTreeMap;
use std::fmt;

use serde::Serialize;
use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde_json::json;
use serde_json::value::RawValue;

use super::{Editor, check_id, null_default};
use crate::kitx;

/// Ein Feld einer Basis-Datei: `file` ist der Schlüssel in `overrides`, `path` der Weg darin (Listen von Objekten je
/// `id` bzw. Index). `base` ist `null`, wenn die Basis das Feld nicht nennt; `override` fehlt, wenn die Welt es erbt.
#[derive(Clone, Debug, Serialize)]
pub struct ValueEntry {
    pub file: String,
    pub path: Vec<String>,
    pub base: Box<RawValue>,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    pub override_value: Option<Box<RawValue>>,
}

/// Werte der Welt: Eltern-Welt, Nachfolger und die Felder. `error` nennt einen Lese- oder Prüffehler, `warnings`
/// Nachfolger, die schon vorher ungültig waren.
#[derive(Clone, Debug, Default, Serialize)]
pub struct ValuesView {
    pub world: String,
    pub extends: String,
    pub children: Vec<String>,
    pub entries: Vec<ValueEntry>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    pub error: String,
}

/// Antwort von `k3c-game sim world` op values.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
struct ValuesReport {
    #[serde(deserialize_with = "null_default")]
    extends: String,
    #[serde(deserialize_with = "null_default")]
    children: Vec<String>,
    #[serde(deserialize_with = "null_default")]
    base: BTreeMap<String, Box<RawValue>>,
    #[serde(deserialize_with = "null_default")]
    overrides: BTreeMap<String, Box<RawValue>>,
}

impl Editor {
    /// Werte der Welt `id` für die Ansicht „Werte“.
    pub fn values(&self, id: &str) -> ValuesView {
        let mut view = ValuesView {
            world: id.to_owned(),
            ..ValuesView::default()
        };
        let report = check_id(id).and_then(|()| self.call::<ValuesReport>(&json!({"op": "values", "world": id})));
        match report {
            Ok(r) => {
                view.entries = value_entries(&r.base, &r.overrides);
                (view.extends, view.children) = (r.extends, r.children);
            }
            Err(e) => view.error = e,
        }
        view
    }
}

/// Je Basis-Datei (sortiert) ihre Felder in der Reihenfolge der Datei mit dem Override der Welt, danach übersteuerte
/// Felder, die die Basis nicht nennt.
fn value_entries(
    base: &BTreeMap<String, Box<RawValue>>,
    overrides: &BTreeMap<String, Box<RawValue>>,
) -> Vec<ValueEntry> {
    let mut out = Vec::new();
    for (file, values) in base {
        let over = overrides.get(file).map_or("", |v| v.get());
        let mut own: BTreeMap<Vec<String>, Box<RawValue>> = BTreeMap::new();
        flatten_json(over, &[], &mut |p, v| {
            own.insert(p, v);
        });
        flatten_json(values.get(), &[], &mut |p, v| {
            let override_value = own.remove(&p);
            out.push(ValueEntry {
                file: file.clone(),
                path: p,
                base: v,
                override_value,
            });
        });
        flatten_json(over, &[], &mut |p, v| {
            if own.contains_key(&p) {
                out.push(ValueEntry {
                    file: file.clone(),
                    path: p,
                    base: RawValue::NULL.to_owned(),
                    override_value: Some(v),
                });
            }
        });
    }
    out
}

/// Empfänger eines Blatts: Pfad und Wert.
pub(super) type Emit<'a> = dyn FnMut(Vec<String>, Box<RawValue>) + 'a;

/// Ruft `emit` für jedes Blatt von `raw`: Zahl, Text, true/false und Listen von Werten (ganz); Objekte und Listen von
/// Objekten (je `id`, sonst Index wie der Merge der Engine) werden durchlaufen. Kommentare (`_…`) entfallen.
pub(super) fn flatten_json(raw: &str, path: &[String], emit: &mut Emit<'_>) {
    flatten_skip(raw, path, "", emit);
}

fn child(path: &[String], key: String) -> Vec<String> {
    let mut next = path.to_vec();
    next.push(key);
    next
}

/// Wie [`flatten_json`]; `skip` ist ein Schlüssel des Objekts `raw`, der nicht ausgegeben wird (die `id` eines
/// Listeneintrags ist sein Schlüssel im Override, kein Feld).
fn flatten_skip(raw: &str, path: &[String], skip: &str, emit: &mut Emit<'_>) {
    let raw = raw.trim();
    match raw.bytes().next() {
        None => {}
        Some(b'{') => {
            for (key, value) in object_items(raw) {
                if !key.starts_with('_') && key != skip {
                    flatten_json(value.get(), &child(path, key), emit);
                }
            }
        }
        Some(b'[') => flatten_list(raw, path, emit),
        Some(_) => emit_text(raw.to_owned(), path, emit),
    }
}

fn emit_text(text: String, path: &[String], emit: &mut Emit<'_>) {
    if let Ok(value) = RawValue::from_string(text) {
        emit(path.to_vec(), value);
    }
}

/// Eine Liste von Objekten je `id` (gibt es eine, sind nur Einträge mit `id` ansprechbar), sonst je Index; jede andere
/// Liste ist ein Blatt.
fn flatten_list(raw: &str, path: &[String], emit: &mut Emit<'_>) {
    let items = serde_json::from_str::<Vec<&RawValue>>(raw).unwrap_or_default();
    if !items.first().is_some_and(|v| v.get().starts_with('{')) {
        emit_text(kitx::compact(raw), path, emit);
        return;
    }
    let ids: Vec<Option<String>> = items.iter().map(|v| item_id(v.get())).collect();
    let by_id = ids.iter().any(Option::is_some);
    for (i, (item, id)) in items.iter().zip(ids).enumerate() {
        match id {
            Some(id) => flatten_skip(item.get(), &child(path, id), "id", emit),
            None if !by_id => flatten_skip(item.get(), &child(path, i.to_string()), "", emit),
            None => {}
        }
    }
}

/// Die `id` (Text) eines Listeneintrags.
fn item_id(raw: &str) -> Option<String> {
    let (_, id) = object_items(raw).into_iter().rfind(|(k, _)| k == "id")?;
    serde_json::from_str(id.get()).ok()
}

/// Schlüssel und Werte eines Objekts in der Reihenfolge der Datei, auch doppelte.
struct Entries<'a>(Vec<(String, &'a RawValue)>);

impl<'de> Deserialize<'de> for Entries<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Collect;
        impl<'de> Visitor<'de> for Collect {
            type Value = Entries<'de>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("JSON-Objekt")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = Vec::new();
                while let Some(entry) = map.next_entry::<String, &'de RawValue>()? {
                    out.push(entry);
                }
                Ok(Entries(out))
            }
        }
        deserializer.deserialize_map(Collect)
    }
}

/// Die Einträge des Objekts `raw`; die Werte sind Ausschnitte von `raw`.
pub(super) fn parse_entries(raw: &[u8]) -> serde_json::Result<Vec<(String, &RawValue)>> {
    serde_json::from_slice::<Entries<'_>>(raw).map(|e| e.0)
}

/// Wie [`parse_entries`]; kein Objekt ergibt keine Einträge.
pub(super) fn object_items(raw: &str) -> Vec<(String, &RawValue)> {
    parse_entries(raw.as_bytes()).unwrap_or_default()
}
