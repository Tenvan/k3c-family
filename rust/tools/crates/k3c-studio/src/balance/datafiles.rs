//! Daten-Dateien von der Platte (V9): `K3C_DATA_DIR`, sonst `<root>/data/`. Go liest dieselben Dateien über
//! `data.Files`; anders als dort ist nichts eingebettet und der Ordner wird bei jedem Lesen neu gewählt.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Verlegt den Daten-Ordner (wie Go `data.DirEnv`).
pub const DIR_ENV: &str = "K3C_DATA_DIR";

/// Ein Eintrag eines Daten-Ordners.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirEntry {
    pub name: String,
    pub is_dir: bool,
}

/// Daten-Ordner: `env` (Wert von [`DIR_ENV`]), wenn er ein vorhandener Ordner ist, sonst `<root>/data`.
pub fn data_dir(root: &Path, env: Option<&OsStr>) -> PathBuf {
    match env.map(Path::new) {
        Some(dir) if !dir.as_os_str().is_empty() && dir.is_dir() => dir.to_path_buf(),
        _ => root.join("data"),
    }
}

/// Pfad der Datei `name` unter dem Daten-Ordner. Wie Go `fs.ValidPath` auf `os.DirFS`: nur relative Namen mit `/`,
/// ohne `.` und `..`, ohne Backslash, Laufwerk und NUL.
fn resolve(root: &Path, env: Option<&OsStr>, name: &str) -> Result<PathBuf, String> {
    let bad = name
        .split('/')
        .any(|part| part.is_empty() || part == "." || part == ".." || part.contains(['\\', ':', '\0']));
    if bad {
        return Err(format!("open {name}: invalid argument"));
    }
    let mut path = data_dir(root, env);
    path.extend(name.split('/'));
    Ok(path)
}

/// Liest `data/<name>` des Checkouts `root`.
pub fn read(root: &Path, name: &str) -> Result<Vec<u8>, String> {
    let path = resolve(root, std::env::var_os(DIR_ENV).as_deref(), name)?;
    std::fs::read(path).map_err(|e| format!("open {name}: {e}"))
}

/// Einträge von `data/<name>`, nach Namen (Bytes) sortiert wie Go `fs.ReadDir`.
pub fn read_dir(root: &Path, name: &str) -> Result<Vec<DirEntry>, String> {
    let path = resolve(root, std::env::var_os(DIR_ENV).as_deref(), name)?;
    let fail = |e: std::io::Error| format!("open {name}: {e}");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(path).map_err(fail)? {
        let entry = entry.map_err(fail)?;
        out.push(DirEntry {
            name: entry.file_name().to_string_lossy().into_owned(),
            is_dir: entry.file_type().map_err(fail)?.is_dir(),
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}
