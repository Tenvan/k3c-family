//! Platzhalter für Funktionen späterer Sessions (Port-Leitfaden § 7). Der liefernde Sprint löscht den Platzhalter; RU22
//! prüft, dass keiner übrig ist.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint

/// Go: `metrics` (metrics.go), die Spielmetrik des Raumlaufs; RU24. Ohne Sammler ist `Room::met` `None`.
#[derive(Debug, Clone, Default)]
pub struct Metrics;

impl Metrics {
    /// Go: `metrics.join`.
    pub fn join(&mut self, _id: &str) {
        unimplemented!("RU24: metrics join")
    }
}
