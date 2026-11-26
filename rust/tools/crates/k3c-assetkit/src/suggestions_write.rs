//! Schreibweg der Vorschlagsliste (Go `suggestions_write.go`): Verwerfen und atomares Speichern.

use std::fs;
use std::io;
use std::path::Path;
use std::sync::{Mutex, MutexGuard, PoisonError};

use serde_json::{Map, Value};

use crate::error::{Error, Result};
use crate::gojson;
use crate::register::write_atomic;
use crate::suggestions::{SUGGESTIONS_PATH, Suggestions, load_suggestions};
use crate::vorrat::VORRAT_DROPPED;

/// Reiht Schreiber der Vorschlagsliste in diesem Prozess hintereinander (Verwerfen aus dem Chat, B-600).
static SUGGESTIONS_MU: Mutex<()> = Mutex::new(());

pub(crate) fn lock_suggestions() -> MutexGuard<'static, ()> {
    SUGGESTIONS_MU.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Felder, die [`Suggestions`] und [`crate::Suggestion`] kennen; nur sie überstehen das Zurückschreiben.
const LIST_KEYS: &[&str] = &["_comment", "vorschlaege"];
const ENTRY_KEYS: &[&str] = &[
    "id",
    "slot",
    "art",
    "titel",
    "quelle",
    "seite",
    "dateiUrl",
    "urheber",
    "lizenz",
    "dateiname",
    "groesse",
    "prioritaet",
    "hinweis",
    "status",
];

/// Setzt den Status des Eintrags `id` auf „verworfen“ und schreibt die Liste atomar (B-600, WZ29.7); `slot` ≠ "" muss
/// der Slot des Eintrags sein. Ein schon verworfener Eintrag schreibt nichts.
pub fn drop_suggestion(root: &Path, slot: &str, id: &str) -> Result<()> {
    let _guard = lock_suggestions();
    let mut list = load_suggestions(root)?;
    let Some(entry) = list.list.iter_mut().find(|s| s.id == id) else {
        return Err(Error::Msg(format!("eintrag {id:?} nicht in der Vorschlagsliste")));
    };
    if !slot.is_empty() && entry.slot != slot {
        return Err(Error::Msg(format!(
            "eintrag {id:?} gehört zu {}, nicht zu {slot}",
            entry.slot
        )));
    }
    if entry.status == VORRAT_DROPPED {
        return Ok(());
    }
    entry.status = VORRAT_DROPPED.to_string();
    list.save(root)
}

fn unknown_key<'a>(object: &'a Map<String, Value>, known: &[&str]) -> Option<&'a String> {
    object.keys().find(|k| !known.contains(&k.as_str()))
}

/// Die Datei wird von Hand gepflegt: Ein Feld, das Rust nicht kennt (auch abweichend groß geschrieben), ginge beim
/// Zurückschreiben verloren. Dann wird nicht geschrieben (strenger als Go, das es verwirft).
fn check_lossless(path: &Path) -> Result<()> {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(Error::Io(e)),
    };
    let value: Value = serde_json::from_slice(&bytes).map_err(|e| Error::Msg(format!("{SUGGESTIONS_PATH}: {e}")))?;
    let Some(root) = value.as_object() else {
        return Ok(());
    };
    let entries = root.get("vorschlaege").and_then(Value::as_array);
    let unknown = unknown_key(root, LIST_KEYS).or_else(|| {
        entries
            .into_iter()
            .flatten()
            .filter_map(Value::as_object)
            .find_map(|entry| unknown_key(entry, ENTRY_KEYS))
    });
    match unknown {
        None => Ok(()),
        Some(key) => Err(Error::Msg(format!(
            "{SUGGESTIONS_PATH}: unbekanntes Feld {key:?}, nichts geschrieben (es ginge verloren)"
        ))),
    }
}

impl Suggestions {
    /// Schreibt die Liste wie das Register: zwei Leerzeichen Einrückung, Umlaute und & im Klartext, Zeilenende `\n`.
    pub(crate) fn save(&self, root: &Path) -> Result<()> {
        let path = root.join(SUGGESTIONS_PATH);
        check_lossless(&path)?;
        let mut bytes = gojson::marshal_indent(self, false).map_err(|e| Error::Msg(e.to_string()))?;
        bytes.push(b'\n');
        write_atomic(&path, &bytes)
    }
}

#[cfg(test)]
#[path = "suggestions_write_tests.rs"]
mod tests;
