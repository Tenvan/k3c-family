//! Natur-Werte im Welten-Editor (Port von `nature.go`, B-891, docs/rules/welten.md § 2.1): Die Seite zeigt je Insel-Stufe
//! die geltenden Werte (fest oder aus dem Seed) und setzt einen Wert fest oder zurück. Geschrieben wird nur
//! `natureByDepth` der Insel; alle übrigen Bytes der Welt-Datei bleiben gleich. Vor dem Schreiben prüft die Spiel-EXE den
//! Entwurf. Hier steht auch das Finden einer Insel in der Welt-Datei.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::value::{RawValue, to_raw_value};

use super::edit::write_checked;
use super::overrides::write_lock;
use super::patch::{Span, remove_key, set_key, value_span};
use super::{Editor, LevelView, null_default};
use crate::kitx;

/// Fest gesetzte Natur-Werte einer Tiefe.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub(super) struct NatureValues {
    #[serde(deserialize_with = "null_default", skip_serializing_if = "BTreeMap::is_empty")]
    pub traits: BTreeMap<String, i64>,
    #[serde(deserialize_with = "null_default", skip_serializing_if = "BTreeMap::is_empty")]
    pub weights: BTreeMap<String, BTreeMap<String, i64>>,
}

impl NatureValues {
    fn is_empty(&self) -> bool {
        self.traits.is_empty() && self.weights.is_empty()
    }

    /// Setzt `key` (Eigenschaft oder „gruppe.variante“) auf `value`; `None` entfernt den Eintrag, leere Gruppen
    /// entfallen.
    fn set(&mut self, key: &str, value: Option<i64>) {
        match (key.split_once('.'), value) {
            (None, None) => {
                self.traits.remove(key);
            }
            (None, Some(value)) => {
                self.traits.insert(key.to_owned(), value);
            }
            (Some((group, option)), None) => {
                if let Some(options) = self.weights.get_mut(group) {
                    options.remove(option);
                    if options.is_empty() {
                        self.weights.remove(group);
                    }
                }
            }
            (Some((group, option)), Some(value)) => {
                let options = self.weights.entry(group.to_owned()).or_default();
                options.insert(option.to_owned(), value);
            }
        }
    }
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

/// `natureByDepth` als eine kompakte Zeile wie Gos `json.Marshal`.
fn by_depth_text(by_depth: &BTreeMap<String, NatureValues>) -> Result<String, String> {
    let mut raw = BTreeMap::new();
    for (depth, values) in by_depth {
        raw.insert(depth.clone(), to_raw_value(values).map_err(|e| e.to_string())?);
    }
    Ok(kitx::marshal_map(&raw))
}

/// Setzt in der Insel `island` den Natur-Wert `key` der Stufe `stage` (`None` = zurück auf „aus Seed“).
fn patch_nature(raw: &[u8], island: &str, stage: i64, key: &str, value: Option<i64>) -> Result<Vec<u8>, String> {
    let (obj, head) = island_span(raw, island)?;
    let Some(depth) = usize::try_from(stage).ok().and_then(|i| head.depths.get(i)) else {
        return Err(format!("die Stufe {stage} gibt es auf der Insel {island:?} nicht"));
    };
    if key.is_empty() || key.matches('.').count() > 1 {
        return Err(format!("ungültiger Natur-Schlüssel {key:?}"));
    }
    let depth = depth.to_string();
    let mut by_depth = head.nature_by_depth;
    let mut entry = by_depth.remove(&depth).unwrap_or_default();
    entry.set(key, value);
    if !entry.is_empty() {
        by_depth.insert(depth, entry);
    }
    if by_depth.is_empty() {
        let at = value_span(raw, obj, "natureByDepth")?;
        return Ok(at.map_or_else(|| raw.to_vec(), |at| remove_key(raw, at, "natureByDepth")));
    }
    set_key(raw, obj, "natureByDepth", by_depth_text(&by_depth)?.as_bytes())
}

impl Editor {
    /// Setzt den Natur-Wert `key` (Eigenschaft wie „forestDensity“ oder „gruppe.variante“ wie „treeSpecies.tanne“) der
    /// Insel `island` in der Stufe `stage` fest; `value` `None` setzt ihn auf „aus Seed“ zurück. Ungültig (Bereich,
    /// unbekannter Schlüssel, alle Varianten 0) bleibt die Datei, wie sie ist, und `rejected` nennt den Grund. Die
    /// Antwort ist das neue Level.
    pub fn set_nature(
        &self,
        id: &str,
        island: &str,
        stage: i64,
        key: &str,
        value: Option<i64>,
        seed: &str,
    ) -> LevelView {
        let written = self.write_nature(id, island, stage, key, value);
        let mut view = self.level(id, island, stage, seed);
        match written {
            Ok(warnings) => view.warnings.extend(warnings),
            Err(error) => view.rejected = error,
        }
        view
    }

    /// Schreibt den Wert unter der Schreibsperre; die Antwort sind die Warnungen der Prüfung.
    fn write_nature(
        &self,
        id: &str,
        island: &str,
        stage: i64,
        key: &str,
        value: Option<i64>,
    ) -> Result<Vec<String>, String> {
        let _guard = write_lock();
        let raw = self.read(id)?;
        let out = patch_nature(&raw, island, stage, key, value)?;
        if out == raw {
            return Ok(Vec::new());
        }
        let warnings = self.check_draft(id, &out)?;
        write_checked(&self.file(id), Some(&raw), &out, &self.log)?;
        Ok(warnings.unwrap_or_default())
    }
}

#[cfg(test)]
#[path = "nature_tests.rs"]
mod tests;
