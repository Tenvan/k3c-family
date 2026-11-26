//! Hilfen der Planer-Tests (Port von `editor_test.go`): der Planer mit der Spiel-EXE der Repo-Wurzel (`exeengine`; ohne
//! EXE überspringen die Tests) und die Datei-Hilfen. Die Dateien des Checkouts werden nur gelesen.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::Value;
use tempfile::TempDir;

use super::hubplan_rules::RULES_FILE;
use super::{Editor, Engine, HUB_FILE, HubMatrix, HubRow};
use crate::exeengine;
use crate::test_helpers::tmp_dir;

/// Die Rüstkammer in `data/hub.json` ab Stufe 4 und ab Stufe 3 (Beispiel 2 aus B-462).
pub(super) const ARMORY4: &str = r#"{ "kind": "armory", "offsetUnits": 40, "hubLevel": 4, "topOnly": true }"#;
pub(super) const ARMORY3: &str = r#"{ "kind": "armory", "offsetUnits": 40, "hubLevel": 3, "topOnly": true }"#;

/// Wurzel des Checkouts.
pub(crate) fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

/// Engine über die Spiel-EXE der Repo-Wurzel mit den Daten des Fixture-Checkouts `root`. Eigener Thread, damit sie auch
/// aus `spawn_blocking` läuft.
pub(crate) fn fixture_engine(root: &Path) -> Engine {
    let root = root.to_path_buf();
    Arc::new(move |input| {
        std::thread::scope(|scope| {
            let run = scope.spawn(|| {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(|e| e.to_string())?;
                runtime.block_on(exeengine::engine("hub", &root, input))
            });
            run.join()
                .unwrap_or_else(|_| Err("Aufruf der Spiel-EXE abgestürzt".to_owned()))
        })
    })
}

/// Der Planer unter `root`; `None` ohne frische Spiel-EXE.
pub(super) fn editor(root: &Path) -> Option<Editor> {
    exeengine::require().then(|| Editor::new(root, fixture_engine(root)))
}

pub(super) fn read_in(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).unwrap()
}

pub(crate) fn read_repo(rel: &str) -> String {
    read_in(&repo_root(), rel)
}

pub(super) fn write(root: &Path, rel: &str, text: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

/// Legt `hub_rel` (Repo) als `data/hub.json` und die Regeltabelle des Checkouts in ein temporäres Verzeichnis.
pub(super) fn planer_copy(hub_rel: &str) -> TempDir {
    let dir = tmp_dir();
    write(dir.path(), HUB_FILE, &read_repo(hub_rel));
    write(dir.path(), RULES_FILE, &read_repo(RULES_FILE));
    dir
}

/// `data/hub.json` mit der Rüstkammer ab Hub-Stufe 3, nur im Speicher.
pub(super) fn planer_armory_from3() -> String {
    let raw = read_repo(HUB_FILE);
    assert_eq!(
        raw.matches(ARMORY4).count(),
        1,
        "data/hub.json enthält {ARMORY4} nicht genau einmal; Test an die Datei anpassen"
    );
    raw.replacen(ARMORY4, ARMORY3, 1)
}

pub(super) fn row_of<'a>(matrix: &'a HubMatrix, key: &str) -> &'a HubRow {
    matrix
        .rows
        .iter()
        .find(|r| r.key == key)
        .unwrap_or_else(|| panic!("Zeile {key} fehlt"))
}

/// `raw` nach `edit`, kompakt und mit sortierten Schlüsseln (eine ganz andere Form als die Datei).
pub(super) fn remarshal(raw: &str, edit: impl FnOnce(&mut Value)) -> String {
    let mut value: Value = serde_json::from_str(raw).unwrap();
    edit(&mut value);
    value.to_string()
}

/// Entfernte und hinzugekommene Zeilen von `a` nach `b` (längste gemeinsame Teilfolge, ohne CR).
pub(super) fn line_diff(a: &str, b: &str) -> (Vec<String>, Vec<String>) {
    let (a, b) = (a.replace("\r\n", "\n"), b.replace("\r\n", "\n"));
    let x: Vec<&str> = a.split('\n').collect();
    let y: Vec<&str> = b.split('\n').collect();
    let mut lcs = vec![vec![0usize; y.len() + 1]; x.len() + 1];
    for i in (0..x.len()).rev() {
        for j in (0..y.len()).rev() {
            lcs[i][j] = if x[i] == y[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }
    let (mut i, mut j) = (0, 0);
    let (mut removed, mut added) = (Vec::new(), Vec::new());
    while i < x.len() || j < y.len() {
        if i < x.len() && j < y.len() && x[i] == y[j] {
            (i, j) = (i + 1, j + 1);
        } else if j < y.len() && (i == x.len() || lcs[i][j + 1] >= lcs[i + 1][j]) {
            added.push(y[j].to_owned());
            j += 1;
        } else {
            removed.push(x[i].to_owned());
            i += 1;
        }
    }
    (removed, added)
}
