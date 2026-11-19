//! Win32-Schicht des Cheat-Fensters. Einzige Stelle im Rust-Workspace mit `unsafe` (RU56, rust-port.md § 12):
//! jeder Block trägt einen `// SAFETY:`-Kommentar, `tests/no_unsafe.rs` prüft beides.
#![allow(unsafe_code)]

mod theme;
mod window;

/// run öffnet das Fenster und läuft die Nachrichtenschleife bis zum Schließen; liefert den Exit-Code.
pub fn run() -> windows::core::Result<u8> {
    window::run()
}
