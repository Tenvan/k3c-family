//! Baseline sichern (Port von `baseline.go`, B-640, B-662): `sim_test {action: baseline}` und „Als Baseline sichern“ der
//! Balancing-Seite rufen beide [`Registry::save_baseline`], ein Weg für dieselben Dateien unter
//! `reports/baseline/<name>/`.

use std::fs;
use std::path::Path;

use serde_json::value::RawValue;

use super::registry::Registry;
use super::reports::{files_in, report_path};
use super::run::DONE;
use super::stamp::StampRaw;
use crate::balance::baselines::{self, BaselineInfo};
use crate::kitx::{go_quoted, marshal_raw};

impl Registry {
    /// Sichert den Bericht `rel` (`reports/<lauf>/balance-*.json`) unter `name`. Kennt das Register den Lauf
    /// (`simtest-<id>`), muss er durchgelaufen sein; der Auftrag kommt vom Lauf, sonst aus dem Bericht (Stempel).
    /// `open` sichert auch einen gültigen Lauf ohne Pass (offener Meilenstein).
    pub fn save_baseline(&self, root: &Path, rel: &str, name: &str, open: bool) -> Result<BaselineInfo, String> {
        let path = report_path(root, rel)?;
        let quoted = go_quoted(rel);
        let is_balance = path
            .file_name()
            .is_some_and(|n| n.to_string_lossy().starts_with("balance-"));
        let dir = path.parent().unwrap_or(root);
        if !is_balance || dir.parent() != Some(root.join("reports").as_path()) {
            return Err(format!(
                "bericht {quoted}: nur balance-*.json im Ordner eines Laufs (reports/<lauf>/)"
            ));
        }
        if !fs::symlink_metadata(&path).is_ok_and(|m| m.is_file()) {
            return Err(format!("bericht {quoted} gibt es nicht"));
        }
        let folder = dir.file_name().unwrap_or_default().to_string_lossy();
        let mut spec = match folder.strip_prefix("simtest-") {
            Some(id) => self.run_spec(root, id)?,
            None => None,
        };
        if spec.is_none() {
            spec = report_spec(&path);
        }
        let info = baselines::save_baseline(root, name, &path, spec, open)?;
        Self::reports_changed(&self.lock(), root);
        Ok(info)
    }

    /// Auftrag eines bekannten, durchgelaufenen Laufs als JSON; `None` ohne Eintrag im Register oder ohne Auftrag.
    fn run_spec(&self, root: &Path, id: &str) -> Result<Option<Box<RawValue>>, String> {
        let Ok(info) = self.get(root, id) else {
            return Ok(None);
        };
        if info.state != DONE {
            return Err(format!(
                "lauf {id} ist {}; gesichert werden nur durchgelaufene Läufe",
                info.state
            ));
        }
        let raw = info.spec.and_then(|s| serde_json::to_string(&s).ok());
        Ok(raw.and_then(|s| RawValue::from_string(marshal_raw(&s)).ok()))
    }
}

/// Neuester Balancing-Bericht des Laufs `id` (`reports/simtest-<id>/balance-*.json`, relativ zum Checkout).
pub fn run_report(root: &Path, id: &str) -> Result<String, String> {
    let missing = || format!("lauf {id} hat keinen Balancing-Bericht (nur offline-Läufe mit focus balance)");
    // eine Lauf-ID ist ein Name, nie ein Pfad
    if id.contains(['/', '\\', ':']) {
        return Err(missing());
    }
    let folder = format!("simtest-{id}");
    let files = files_in(&root.join("reports").join(&folder), "balance-");
    let last = files.last().and_then(|f| f.file_name()).ok_or_else(missing)?;
    Ok(format!("reports/{folder}/{}", last.to_string_lossy()))
}

/// Gestempelter Auftrag eines Berichts als JSON; `None` ohne Auftrag (ältere Berichte).
fn report_spec(path: &Path) -> Option<Box<RawValue>> {
    let st = StampRaw::read(&fs::read(path).ok()?).ok()?;
    st.spec()?;
    st.spec
}

#[cfg(test)]
#[path = "baseline_tests.rs"]
mod tests;
