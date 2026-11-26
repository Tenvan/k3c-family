//! Hilfen für Tests. Temporäre Ordner liegen im Repo (Regel „Schreiben nur im Repo“), nie unter `std::env::temp_dir()`.

use std::path::PathBuf;

/// Legt einen frischen Temp-Ordner unter `K3C_TMP` an, ohne die Variable unter `<Repo>/.work/tmp`; er wird beim
/// Fallenlassen gelöscht.
pub(crate) fn tmp_dir() -> tempfile::TempDir {
    let base = std::env::var_os("K3C_TMP")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../.work/tmp")));
    std::fs::create_dir_all(&base).expect("Temp-Basis anlegen");
    tempfile::Builder::new()
        .prefix("k3c-kit-")
        .tempdir_in(base)
        .expect("Temp-Ordner anlegen")
}
