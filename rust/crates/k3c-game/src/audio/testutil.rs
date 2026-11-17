//! Hilfen der Tests des Pakets audio (Leitfaden § 6).
//!
//! Ein Ton-Gerät gibt es im Test nicht: Tests, die später eine Ausgabe brauchen, nehmen `k3c_platform::NullAudio`
//! (RU17.5); in dieser Session braucht keiner eine.

use std::path::{Path, PathBuf};

use super::register::{Register, load_register};

/// Ordner `data/` des Repos (Go: `data.Files`).
pub fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data")
}

/// `public/` des Repos (Go: `repoPublic`).
pub fn public_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public")
}

/// Das Register aus den echten Dateien data/sounds.json und data/monarch.json.
pub fn real_register() -> Register {
    let dir = data_dir();
    let read = |name: &str| std::fs::read(dir.join(name)).map_err(|e| e.to_string());
    load_register(&read).expect("Register aus data/")
}
