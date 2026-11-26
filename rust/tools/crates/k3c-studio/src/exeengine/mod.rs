//! Engine-Seite der Studio-Tests (Port von `internal/exeengine`): `k3c-game sim world|hub` über die Spiel-EXE der
//! Repo-Wurzel mit den Daten eines Fixture-Checkouts (`Options::data`). Fehlt die EXE oder ist sie veraltet
//! (`gameexe::find`), überspringen die Tests mit dem Bau-Befehl im Grund. Nur Tests nutzen das Modul; Welten- und
//! Hub-Tests späterer Sessions rufen `require` und `engine`.

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use serde_json::value::RawValue;

use crate::gameexe::{self, Options};

/// Repo-Wurzel mit frischer EXE oder der Grund, warum sie fehlt.
static PROBE: LazyLock<Result<PathBuf, String>> = LazyLock::new(|| probe(Path::new(env!("CARGO_MANIFEST_DIR"))));

fn is_repo_root(dir: &Path) -> bool {
    dir.join("Taskfile.yml").exists() && dir.join("rust").join("Cargo.toml").exists()
}

/// Sucht die Repo-Wurzel ab `start` aufwärts und prüft dort die EXE.
fn probe(start: &Path) -> Result<PathBuf, String> {
    let root = start
        .ancestors()
        .find(|dir| is_repo_root(dir))
        .ok_or_else(|| "Repo-Wurzel nicht gefunden".to_owned())?;
    gameexe::find(root)?;
    Ok(root.to_path_buf())
}

/// Die Repo-Wurzel mit frischer EXE oder der Grund, warum sie fehlt.
pub fn available() -> Result<PathBuf, String> {
    PROBE.clone()
}

/// `false` ohne frische EXE; der Test meldet sich dann mit `return` ab.
pub fn require() -> bool {
    match available() {
        Ok(_) => true,
        Err(e) => {
            eprintln!("übersprungen: k3c-game.exe nicht verfügbar: {e}");
            false
        }
    }
}

/// Ein Aufruf der Engine-Seite für `sub` (world oder hub) mit den Daten unter `fixture/data`. Ohne EXE der Grund als
/// Fehler.
pub async fn engine(sub: &str, fixture: &Path, input: Vec<u8>) -> Result<Option<Box<RawValue>>, String> {
    let root = available()?;
    // Die EXE läuft in ihrem eigenen Arbeitsordner.
    let data = std::path::absolute(fixture.join("data")).map_err(|e| e.to_string())?;
    let options = Options {
        root,
        data: Some(data),
        sub: sub.to_owned(),
        input,
        ..Options::default()
    };
    let event = gameexe::call(options, gameexe::never()).await;
    event.map(|ev| ev.report).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::tmp_dir;

    #[test]
    fn ohne_exe_nennt_der_grund_den_bau_befehl() {
        let root = tmp_dir();
        std::fs::write(root.path().join("Taskfile.yml"), "").unwrap();
        std::fs::create_dir_all(root.path().join("rust")).unwrap();
        std::fs::write(root.path().join("rust").join("Cargo.toml"), "").unwrap();
        let reason = probe(&root.path().join("rust")).unwrap_err();
        assert!(reason.contains(gameexe::BUILD_COMMAND), "{reason}");
    }

    #[tokio::test]
    async fn engine_reicht_den_fehler_der_exe_durch() {
        if !require() {
            return;
        }
        let fixture = tmp_dir();
        let error = engine("hub", fixture.path(), b"{}".to_vec()).await.unwrap_err();
        assert!(!error.is_empty());
    }
}
