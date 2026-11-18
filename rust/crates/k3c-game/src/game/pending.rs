//! Platzhalter für Teile, die spätere Sprints liefern (Leitfaden § 7). RU30.2 liefert `script.go` samt `ParseScript`,
//! tauscht die `pub use`-Zeile in `mod.rs` aus und löscht diese Datei. RU22 prüft, dass keiner übrig ist.
#![allow(clippy::unimplemented)] // Platzhalter nach Leitfaden § 7: wartet auf RU30

use super::scene::Hold;

/// Go: `ParseScript` (`script.go`, RU30.2): liest die feste Eingabe-Folge „p@t=…,…“.
pub fn parse_script(_s: &str) -> Result<Vec<Hold>, String> {
    unimplemented!("RU30: parse_script")
}
