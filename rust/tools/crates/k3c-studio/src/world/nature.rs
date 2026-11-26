//! Insel einer Welt-Datei finden (Port aus `nature.go`). RU70.4 zieht nur `islandSpan`, `noIslandError` und die Typen
//! vor, weil Inseln und Chunks sie brauchen; die Natur-Werte selbst folgen in RU70.5.

use std::collections::BTreeMap;
use std::fmt;

use serde::Deserialize;
use serde_json::value::RawValue;

use super::null_default;
use super::patch::{Span, value_span};

/// Fest gesetzte Natur-Werte einer Tiefe.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
#[allow(dead_code)] // Gelesen ab RU70.5; hier prüft das Lesen nur die Form wie Go.
pub(super) struct NatureValues {
    #[serde(deserialize_with = "null_default")]
    pub traits: BTreeMap<String, i64>,
    #[serde(deserialize_with = "null_default")]
    pub weights: BTreeMap<String, BTreeMap<String, i64>>,
}

/// Der Teil einer Insel, den das Bearbeiten braucht.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub(super) struct IslandHead {
    #[serde(deserialize_with = "null_default")]
    pub id: String,
    #[serde(deserialize_with = "null_default")]
    pub depths: Vec<i64>,
    #[serde(rename = "natureByDepth", deserialize_with = "null_default")]
    #[allow(dead_code)] // Gelesen ab RU70.5.
    pub nature_by_depth: BTreeMap<String, NatureValues>,
}

/// Fehler von [`island_span`]; `Missing` erkennt das Anlegen (Gos `noIslandError`).
#[derive(Debug)]
pub(super) enum SpanError {
    Missing(String),
    Other(String),
}

impl fmt::Display for SpanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpanError::Missing(id) => write!(f, "die Insel {id:?} gibt es nicht"),
            SpanError::Other(text) => f.write_str(text),
        }
    }
}

impl From<SpanError> for String {
    fn from(e: SpanError) -> String {
        e.to_string()
    }
}

/// Findet die Insel `id` in `islands`: Byte-Bereich ihres Objekts und ihr Kopf.
pub(super) fn island_span(raw: &[u8], id: &str) -> Result<(Span, IslandHead), SpanError> {
    let missing_list = |cause: String| SpanError::Other(format!("islands fehlt in der Welt-Datei ({cause})"));
    let arr = value_span(raw, Span::whole(raw), "islands")
        .map_err(missing_list)?
        .ok_or_else(|| missing_list("<nil>".to_owned()))?;
    let items = serde_json::from_slice::<Vec<&RawValue>>(&raw[arr.from..arr.to])
        .map_err(|_| SpanError::Other("islands ist keine Liste".to_owned()))?;
    for item in items {
        let head = serde_json::from_str::<Option<IslandHead>>(item.get())
            .map_err(|e| SpanError::Other(format!("islands: {e}")))?
            .unwrap_or_default();
        if head.id == id {
            // Der Wert ist ein Ausschnitt von `raw`; seine Adresse gibt die Stelle.
            let from = item.get().as_ptr() as usize - raw.as_ptr() as usize;
            let to = from + item.get().len();
            return Ok((Span { from, to }, head));
        }
    }
    Err(SpanError::Missing(id.to_owned()))
}
