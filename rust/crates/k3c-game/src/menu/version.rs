//! Die Version des Spiels fürs Hauptmenü. Go setzt `menu.Version` beim Bau per Linker-Flag; in Rust ruft der Bau
//! `set_version` mit `option_env!("K3C_VERSION")` auf. Einzige Datei ohne Go-Gegenstück (Sprint RU9, Vorentscheidung).

use std::sync::RwLock;

static VERSION: RwLock<String> = RwLock::new(String::new());

/// Die gesetzte Version (leer, solange keine gesetzt wurde).
pub fn version() -> String {
    VERSION.read().map(|v| v.clone()).unwrap_or_default()
}

/// Setzt die Version.
pub fn set_version(v: &str) {
    if let Ok(mut g) = VERSION.write() {
        *g = v.to_string();
    }
}
