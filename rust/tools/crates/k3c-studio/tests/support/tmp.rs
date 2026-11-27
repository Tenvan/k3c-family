//! Temporäre Ordner der Tests liegen im Repo (Regel „Schreiben nur im Repo“): unter `K3C_TMP`, sonst `.work/tmp`.

/// Frischer Ordner; er wird beim Fallenlassen gelöscht.
pub fn tmp_dir() -> tempfile::TempDir {
    let base = std::env::var_os("K3C_TMP").filter(|v| !v.is_empty()).map_or_else(
        || std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../.work/tmp")),
        std::path::PathBuf::from,
    );
    std::fs::create_dir_all(&base).expect("Temp-Basis anlegen");
    tempfile::Builder::new()
        .prefix("k3c-studio-")
        .tempdir_in(base)
        .expect("Temp-Ordner anlegen")
}
