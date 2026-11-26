//! Pfade aus Tool-Argumenten (Port von `mcpkit/paths.go`).

use std::path::{Path, PathBuf};

use super::workspace::{ambiguous, join, rel_under, resolve};

/// Liefert den Pfad nur, wenn er (auch nach Links) unterhalb der Repo-Wurzel liegt (`replay_run` der Workbench,
/// `assets_import` des Studios; WZ63).
pub fn inside_root(root: impl AsRef<Path>, name: &str) -> Result<PathBuf, String> {
    let root = root.as_ref();
    if name.is_empty() {
        return Err("path fehlt: Datei relativ zur Repo-Wurzel".to_owned());
    }
    if ambiguous(name) {
        return Err(format!("path {name:?} abgelehnt: Pfad ohne Laufwerk oder mit „:“"));
    }
    let path = if Path::new(name).is_absolute() {
        PathBuf::from(name)
    } else {
        join(root, name)
    };
    let real_root = resolve(root).map_err(|e| format!("Repo-Wurzel {:?}: {e}", root.display().to_string()))?;
    let real = resolve(&path).map_err(|e| format!("{name} nicht gefunden: {e}"))?;
    if rel_under(&real_root, &real).is_none() {
        return Err(format!(
            "path {name:?} abgelehnt: nur Dateien unterhalb der Repo-Wurzel"
        ));
    }
    Ok(real)
}
