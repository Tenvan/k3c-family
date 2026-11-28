//! Hilfen für Tests. Temporäre Ordner liegen im Repo (Regel „Schreiben nur im Repo“), nie unter `std::env::temp_dir()`.

use std::path::PathBuf;

/// Frischer Ordner unter `K3C_TMP`, ohne die Variable unter `<Repo>/.work/tmp`; er wird beim Fallenlassen gelöscht.
pub(crate) fn tmp_dir() -> tempfile::TempDir {
    let base = std::env::var_os("K3C_TMP").filter(|v| !v.is_empty()).map_or_else(
        || {
            let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(4);
            repo.expect("Repo-Wurzel").join(".work").join("tmp")
        },
        // Neu zusammengesetzt, damit unter Windows keine Schrägstriche gemischt stehen.
        |v| PathBuf::from(v).components().collect(),
    );
    std::fs::create_dir_all(&base).expect("Temp-Basis anlegen");
    tempfile::Builder::new()
        .prefix("k3c-workbench-")
        .tempdir_in(base)
        .expect("Temp-Ordner anlegen")
}
