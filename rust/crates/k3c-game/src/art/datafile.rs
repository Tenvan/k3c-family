//! Liest Dateien aus `data/` (Go: `data.Files`, `data/embed.go` ohne Einbetten): Ordner aus `K3C_DATA_DIR`, sonst
//! `data/` relativ zum Arbeitsordner. Typisierte Daten liegen nicht hier (k3c-data), nur die Zuordnungen der Grafik.

use std::path::{Path, PathBuf};

/// Ordner der Daten: `K3C_DATA_DIR`, sonst `data`.
pub fn data_dir() -> PathBuf {
    match std::env::var_os("K3C_DATA_DIR") {
        Some(d) if !d.is_empty() => PathBuf::from(d),
        _ => PathBuf::from("data"),
    }
}

/// Liest `name` aus dem Datenordner; der Fehlertext nennt den Pfad.
pub fn read(name: &str) -> Result<Vec<u8>, String> {
    read_in(&data_dir(), name)
}

/// Wie `read`, mit ausdrücklichem Ordner.
pub fn read_in(dir: &Path, name: &str) -> Result<Vec<u8>, String> {
    let path = dir.join(name);
    std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))
}
