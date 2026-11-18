//! Liest Dateien aus `data/` (Go: `data.Files`, `data/embed.go`): Ordner aus `K3C_DATA_DIR`, sonst `data/` neben der
//! EXE, sonst die eingebetteten Dateien (`k3c_data::Source::from_env`), unabhängig vom Arbeitsordner (B-1066).
//! Typisierte Daten liegen nicht hier (k3c-data), nur die Zuordnungen der Grafik.

#[cfg(test)]
use std::path::Path;

/// Liest `name` aus der Datenquelle wie Go; der Fehlertext nennt den Pfad (`data/<name>: …`).
pub fn read(name: &str) -> Result<Vec<u8>, String> {
    read_from(&k3c_data::Source::from_env(), name)
}

/// Wie `read`, mit ausdrücklicher Quelle.
pub fn read_from(src: &k3c_data::Source, name: &str) -> Result<Vec<u8>, String> {
    src.read(name).map(String::into_bytes).map_err(|e| e.to_string())
}

/// Wie `read`, mit ausdrücklichem Ordner (Tests).
#[cfg(test)]
pub fn read_in(dir: &Path, name: &str) -> Result<Vec<u8>, String> {
    let path = dir.join(name);
    std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))
}
