//! Die Version des Spiels fürs Hauptmenü. Go setzt `menu.Version` beim Bau per Linker-Flag; in Rust ruft der Bau
//! `set_version` mit `option_env!("K3C_VERSION")` auf. Einzige Datei ohne Go-Gegenstück (Sprint RU9, Vorentscheidung).

use std::sync::RwLock;

use super::texts::{TextKey, tr, tr1};

static VERSION: RwLock<String> = RwLock::new(String::new());

/// Die gesetzte Version (leer, solange keine gesetzt wurde).
pub fn version() -> String {
    VERSION.read().map(|v| v.clone()).unwrap_or_default()
}

/// Meldet, ob keine Version gesetzt ist (Entwicklungs-Bau), ohne den Text zu kopieren.
pub fn is_unset() -> bool {
    VERSION.read().is_ok_and(|v| v.is_empty())
}

/// Setzt die Version.
pub fn set_version(v: &str) {
    if let Ok(mut g) = VERSION.write() {
        *g = v.to_string();
    }
}

/// Die Versionszeile des Hauptmenüs („Version …“, ohne Angabe „unbekannt“; Go: `versionText` in `licenses.go`).
pub fn version_text(lang: &str) -> String {
    let mut v = version();
    if v.is_empty() {
        v = tr(lang, TextKey::VersionUnknown);
    }
    tr1(lang, TextKey::Version, &v)
}
