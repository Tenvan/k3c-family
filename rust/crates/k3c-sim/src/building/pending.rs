//! Platzhalter der Gruppe `building` für Funktionen späterer Sessions (Port-Leitfaden § 7). Die Session, die die Funktion
//! liefert (RU18.3), löscht den Platzhalter; RU22 prüft, dass keiner übrig ist.

/// Go: `levelOf` (hub_level.go, RU18.3): die Ausbau-Stufe des Platzes; der Platzhalter steht in `world/pending.rs`.
pub use crate::world::pending::level_of;
