use std::path::Path;
use std::sync::Arc;

use super::{Engine, exe};
use crate::exeengine;
use crate::gameexe::BUILD_COMMAND;
use crate::test_helpers::tmp_dir;

/// Engine der Tool- und Methoden-Tests (Gos `exeengine.Engine("world", root)`): die Spiel-EXE der Repo-Wurzel liest die
/// Daten des Fixture-Checkouts `root`. Eigener Thread, damit sie auch aus `spawn_blocking` läuft.
pub(crate) fn fixture_engine(root: &Path) -> Engine {
    let root = root.to_path_buf();
    Arc::new(move |input| {
        std::thread::scope(|scope| {
            let run = scope.spawn(|| {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(|e| e.to_string())?;
                runtime.block_on(exeengine::engine("world", &root, input))
            });
            run.join()
                .unwrap_or_else(|_| Err("Aufruf der Spiel-EXE abgestürzt".to_owned()))
        })
    })
}

// RU34.3/AC-03: `exe` rechnet über die Spiel-EXE des Checkouts; ohne EXE nennt der Fehler den Bau-Befehl.
#[test]
fn exe_ohne_spiel_exe() {
    let root = tmp_dir();
    let error = exe(root.path())(br#"{"op":"check"}"#.to_vec()).unwrap_err();
    assert!(error.contains(BUILD_COMMAND), "{error}");
}
