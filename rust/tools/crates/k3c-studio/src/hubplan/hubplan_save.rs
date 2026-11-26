//! Speichern im Gebäude-Planer (Port von `hubplan_save.go`, B-462/AC-02, AC-05, AC-06): nur ein geprüfter Kandidat ohne
//! Befund; erst `data/hub.json` (format-stabil, `patch.rs`), dann der erzeugte Block der Regeltabelle, beide atomar.
//! Schreibregel: Hat sich eine der Dateien seit dem Laden geändert, wird nichts geschrieben. Eine Datei, die kein
//! UTF-8 ist, wird nie zurückgeschrieben.

use std::path::Path;
use std::sync::{Mutex, PoisonError};

use serde::{Deserialize, Serialize};

use super::hubplan_rules::{RULES_FILE, RULES_TASK};
use super::{Editor, HUB_FILE, HubFindingView, null_default, patch, table};
use crate::kitx::sha256_hex;

#[cfg(test)]
#[path = "hubplan_save_tests.rs"]
pub(super) mod tests;

const REG_NOTICE: &str = "REG-Änderung: Commit mit Begründung (data/hub.json und Regeltabelle)";
const STALE_HINT: &str =
    "seit dem Laden geändert: nicht gespeichert, bitte neu laden (Eingaben bleiben bis dahin erhalten)";

/// Reiht die Speichervorgänge dieses Prozesses (Stand prüfen und schreiben am Stück).
static SAVE_LOCK: Mutex<()> = Mutex::new(());

/// Der Inhalt von `data/hub.json` und der Regeltabelle beim Laden (SHA-256, hex); leer = Datei fehlte.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default)]
pub struct HubStand {
    #[serde(deserialize_with = "null_default")]
    pub hub: String,
    #[serde(deserialize_with = "null_default")]
    pub rules: String,
}

impl HubStand {
    /// Stand zum gelesenen Inhalt `hub` und der Regeltabelle unter `root`.
    pub(super) fn of(root: &Path, hub: &[u8]) -> HubStand {
        HubStand {
            hub: sha256_hex(hub),
            rules: file_stamp(root, RULES_FILE),
        }
    }
}

/// Das Ergebnis des Speicherns. `saved`: `data/hub.json` geschrieben. `stale`: eine Datei wurde extern geändert
/// (nichts geschrieben). `rules_error`: hub.json gespeichert, der Block der Regeltabelle nicht. `notice`: Hinweis auf
/// die REG-Änderung. `unchanged`: der Kandidat bedeutet dasselbe wie die Datei, nichts geschrieben. `stand`: neuer
/// Stand beider Dateien.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HubSaveResult {
    pub saved: bool,
    pub unchanged: bool,
    pub stale: String,
    pub findings: Vec<HubFindingView>,
    pub error: String,
    pub rules_error: String,
    pub notice: String,
    pub stand: HubStand,
}

/// Stand der Datei `rel` unter `root`; leer, wenn sie nicht lesbar ist.
fn file_stamp(root: &Path, rel: &str) -> String {
    std::fs::read(root.join(rel)).map_or_else(|_| String::new(), |bytes| sha256_hex(&bytes))
}

/// Nennt die Datei, die nicht mehr den Stand beim Laden hat.
fn stale_file(root: &Path, hub: &[u8], stand: &HubStand) -> Option<&'static str> {
    if sha256_hex(hub) != stand.hub {
        return Some(HUB_FILE);
    }
    (file_stamp(root, RULES_FILE) != stand.rules).then_some(RULES_FILE)
}

/// Die alte Datei mit den Werten des Kandidaten; `None`: gleicher Inhalt.
fn patched(old: &[u8], candidate: &str) -> Result<Option<String>, String> {
    let old = std::str::from_utf8(old).map_err(|e| format!("kein UTF-8: {e}"))?;
    let next = patch::patch_json(old, candidate)?;
    Ok((next != old).then_some(next))
}

fn write(path: &Path, content: &str) -> Result<(), String> {
    k3c_assetkit::write_atomic(path, content.as_bytes()).map_err(|e| e.to_string())
}

/// Schreibt den aus `hub_json` erzeugten Block zwischen die Marken der Regeltabelle; der Rest bleibt byte-gleich, ein
/// unveränderter Block wird nicht geschrieben.
fn write_rules(root: &Path, hub_json: &str) -> Result<(), String> {
    let path = root.join(RULES_FILE);
    let doc = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let block = table::block(hub_json.as_bytes())?;
    let next = table::ersetzen(&doc, &block)?;
    if next == doc {
        return Ok(());
    }
    write(&path, &next)
}

impl Editor {
    /// Speichert den Kandidaten, wenn beide Dateien noch den Stand `stand` haben.
    pub fn save(&self, candidate: &str, stand: HubStand) -> HubSaveResult {
        let _lock = SAVE_LOCK.lock().unwrap_or_else(PoisonError::into_inner);
        let mut result = HubSaveResult {
            stand,
            ..HubSaveResult::default()
        };
        if let Err(error) = self.save_into(candidate, &mut result) {
            result.error = error;
        }
        result
    }

    /// Prüft den Kandidaten über die Engine, vergleicht den Stand und schreibt `data/hub.json` und danach die
    /// Regeltabelle.
    fn save_into(&self, candidate: &str, result: &mut HubSaveResult) -> Result<(), String> {
        let report = self
            .ask(candidate)
            .map_err(|e| format!("Kandidat nicht geprüft: {e}"))?;
        if !report.parse_error.is_empty() {
            return Err(format!("Kandidat nicht lesbar: {}", report.parse_error));
        }
        result.findings = report.findings;
        if !result.findings.is_empty() {
            let count = result.findings.len();
            return Err(format!("nicht gespeichert: {count} Befunde der Prüfung"));
        }
        let path = self.root.join(HUB_FILE);
        let old = std::fs::read(&path).map_err(|e| format!("{HUB_FILE} nicht lesbar: {e}"))?;
        if let Some(stale) = stale_file(&self.root, &old, &result.stand) {
            result.stale = format!("{stale} {STALE_HINT}");
            return Ok(());
        }
        let not_written = |e: String| format!("{HUB_FILE} nicht geschrieben: {e}");
        let Some(next) = patched(&old, candidate).map_err(not_written)? else {
            // Gleicher Inhalt: nichts geschrieben, keine REG-Änderung.
            result.unchanged = true;
            return Ok(());
        };
        write(&path, &next).map_err(not_written)?;
        result.saved = true;
        result.notice = REG_NOTICE.to_owned();
        result.stand.hub = sha256_hex(next.as_bytes());
        if let Err(e) = write_rules(&self.root, &next) {
            result.rules_error = format!("Regeltabelle veraltet, {RULES_TASK} ({e})");
        }
        result.stand.rules = file_stamp(&self.root, RULES_FILE);
        Ok(())
    }
}
