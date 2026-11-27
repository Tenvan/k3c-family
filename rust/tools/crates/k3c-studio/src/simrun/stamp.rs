//! Auftrag im Bericht (Port von `stamp.go`, B-662): Nach dem Rechnen trägt das Studio Label, Auftrag und Lauf-ID in den
//! Bericht (`balance-*.json`, `tuning-*.json`) ein. Der Bericht kommt von der EXE des Checkouts, deren Format neuer sein
//! kann als das des Studios; deshalb wird er nicht neu kodiert, sondern vorn ergänzt und nur neu eingerückt (Reihenfolge
//! und unbekannte Felder bleiben).

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::json;
use serde_json::value::RawValue;

use super::registry::{Registry, report_dir};
use super::reports::{newest_file, read_result};
use super::run::Run;
use super::spec::Spec;
use crate::balance::report_types::read_go;
use crate::balance::writeguard;
use crate::kitx::{compact, indent, marshal_raw};

/// Felder, die das Studio in einen Bericht schreibt; ältere Berichte haben sie nicht.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Stamp {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,
    #[serde(rename = "runId", skip_serializing_if = "String::is_empty")]
    pub run_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<Spec>,
}

/// Liest ein JSON-Objekt wie Go `json.Unmarshal` in eine Struktur (`null` lässt sie leer, eine Liste ist ein Fehler;
/// serde läse sie sonst als Struktur).
pub(crate) fn read_object<T: DeserializeOwned + Default>(raw: &[u8]) -> Result<T, String> {
    let text = raw.trim_ascii();
    if !text.starts_with(b"{") && text != b"null" {
        // ungültiges JSON meldet der Parser selbst
        serde_json::from_slice::<serde::de::IgnoredAny>(text).map_err(|e| e.to_string())?;
        return Err("json: kein Objekt".to_owned());
    }
    read_go(raw)
}

/// Die Felder von [`Stamp`], wie sie in einem Bericht stehen; `spec` bleibt roh, damit ein unlesbarer Auftrag nicht
/// den ganzen Bericht kostet.
#[derive(Debug, Default)]
pub(super) struct StampRaw {
    pub label: String,
    pub run_id: String,
    pub spec: Option<Box<RawValue>>,
}

impl StampRaw {
    pub(super) fn read(raw: &[u8]) -> Result<StampRaw, String> {
        if raw.trim_ascii() == b"null" {
            return Ok(StampRaw::default());
        }
        // Bei doppelten Schlüsseln gilt wie in Go der letzte.
        let mut fields: BTreeMap<String, Box<RawValue>> = serde_json::from_slice(raw).map_err(|e| e.to_string())?;
        let spec = fields.remove("spec");
        let mut text = |key: &str| -> Result<String, String> {
            let Some(value) = fields.remove(key) else {
                return Ok(String::new());
            };
            let value: Option<String> = serde_json::from_str(value.get()).map_err(|e| format!("{key}: {e}"))?;
            Ok(value.unwrap_or_default())
        };
        Ok(StampRaw {
            label: text("label")?,
            run_id: text("runId")?,
            spec,
        })
    }

    /// Auftrag des Berichts; `None`, wenn er fehlt oder unlesbar ist.
    pub(super) fn spec(&self) -> Option<Spec> {
        read_object(self.spec.as_ref()?.get().as_bytes()).ok()
    }
}

impl Registry {
    /// Stempelt den neuesten Bericht des Laufs und liest dann Urteil und Zeilen. Ein Fehler beim Stempeln steht im
    /// Log; der Lauf zählt trotzdem.
    pub(super) fn read_stamped(&self, run: &Arc<Run>, spec: &Spec) -> Result<(String, Vec<String>), String> {
        let full = run.root().join(report_dir(run));
        let tune = spec.tune.is_some();
        let kind = if tune { "tuning-" } else { "balance-" };
        if let Ok(file) = newest_file(&full, kind, run.started()) {
            let st = Stamp {
                label: run.label().to_owned(),
                run_id: run.id().to_owned(),
                spec: Some(spec.clone()),
            };
            if let Err(e) = stamp(run.root(), &file, &st) {
                let attrs = [("ns", json!("simtest")), ("id", json!(run.id())), ("error", json!(e))];
                self.log.error("❌ auftrag nicht im bericht", &attrs);
            }
        }
        read_result(run.root(), &full, run.started(), tune)
    }
}

/// Schreibt `st` an den Anfang des JSON-Objekts in `file`; ein schon gestempelter Bericht bleibt, wie er ist.
pub(super) fn stamp(root: &Path, file: &Path, st: &Stamp) -> Result<(), String> {
    let raw = fs::read(file).map_err(|e| e.to_string())?;
    let have = StampRaw::read(&raw).map_err(|e| format!("bericht unlesbar: {e}"))?;
    let head = marshal_raw(&serde_json::to_string(st).map_err(|e| e.to_string())?);
    if !have.run_id.is_empty() || head.len() <= 2 {
        return Ok(());
    }
    let body = std::str::from_utf8(&raw).map_err(|e| e.to_string())?.trim();
    // `rest`: Felder des Berichts samt schließender Klammer
    let Some(rest) = body.strip_prefix('{').map(str::trim) else {
        return Err("bericht ist kein JSON-Objekt".to_owned());
    };
    let sep = if rest.starts_with('}') { "" } else { "," };
    let open = head.strip_suffix('}').unwrap_or(&head);
    let mut out = indent(&compact(&format!("{open}{sep}{rest}")), "", "  ");
    out.push('\n');
    let mut tmp = OsString::from(file);
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    writeguard::write_file(root, &tmp, out.as_bytes()).map_err(|e| e.to_string())?;
    replace_file(root, &tmp, file)
}

/// Ersetzt `file` durch `tmp`. Unter Windows scheitert das Umbenennen, solange ein anderer Leser (z. B. die
/// Berichtsliste nach `balance:reports`) die Datei offen hat; deshalb einige kurze Wiederholungen. Bleibt es dabei,
/// fällt `tmp` weg und der Bericht bleibt unverändert (nur ohne Stempel).
fn replace_file(root: &Path, tmp: &Path, file: &Path) -> Result<(), String> {
    let mut last = String::new();
    for _ in 0..5 {
        match writeguard::rename(root, tmp, file) {
            Ok(()) => return Ok(()),
            Err(e) => last = e.to_string(),
        }
        thread::sleep(Duration::from_millis(20));
    }
    let _ = writeguard::remove(root, tmp); // der Fehler des Umbenennens zählt
    Err(last)
}

#[cfg(test)]
#[path = "stamp_tests.rs"]
mod tests;
