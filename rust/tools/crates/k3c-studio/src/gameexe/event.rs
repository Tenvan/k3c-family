//! JSON-Zeilen von stdout der EXE (Port von `event.go`).

use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

/// Eine JSON-Zeile von stdout; die Felder entsprechen den Zeilen der Schnittstelle (progress, result, report, error,
/// aborted).
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Event {
    #[serde(deserialize_with = "null_default")]
    pub event: String,
    #[serde(deserialize_with = "null_default")]
    pub done: i64,
    #[serde(deserialize_with = "null_default")]
    pub total: i64,
    #[serde(deserialize_with = "null_default")]
    pub text: String,
    #[serde(deserialize_with = "null_default")]
    pub message: String,
    #[serde(deserialize_with = "null_default")]
    pub files: Vec<String>,
    /// Bericht unverändert als Text (Reihenfolge der Schlüssel und Zahlenform bleiben).
    pub report: Option<Box<RawValue>>,
}

/// `null` lässt wie in Go den Nullwert stehen.
fn null_default<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(d)?.unwrap_or_default())
}

/// Liest eine stdout-Zeile; `None`, wenn sie keine JSON-Zeile mit `event` ist.
pub fn parse_event(line: &str) -> Option<Event> {
    // serde läse auch eine Liste als Struktur.
    if !line.trim_start().starts_with('{') {
        return None;
    }
    serde_json::from_str::<Event>(line).ok().filter(|e| !e.event.is_empty())
}
