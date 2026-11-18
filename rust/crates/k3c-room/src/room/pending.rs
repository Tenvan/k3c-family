//! Platzhalter für Funktionen späterer Sessions (Port-Leitfaden § 7). Der liefernde Sprint löscht den Platzhalter; RU22
//! prüft, dass keiner übrig ist.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint

use k3c_sim::island::Island;

use super::env::Env;
use super::types::Room;

/// Go: `metrics` (metrics.go), die Spielmetrik des Raumlaufs; RU24. Ohne Sammler ist `Room::met` `None`.
#[derive(Debug, Clone, Default)]
pub struct Metrics;

impl Metrics {
    /// Go: `metrics.join`.
    pub fn join(&mut self, _id: &str) {
        unimplemented!("RU24: metrics join")
    }

    /// Go: `metrics.drop`: ein Gerät trennt sich.
    pub fn disconnect(&mut self, _id: &str) {
        unimplemented!("RU24: metrics disconnect")
    }

    /// Go: `metrics.observe`: wertet die Ereignisse und Stände eines Schritts aus.
    pub fn observe(&mut self, _isl: &Island) {
        unimplemented!("RU24: metrics observe")
    }
}

/// Aus `observer.go` (RU24).
impl Room {
    /// Go: `unobserve`: ein Beobachter verlässt den Raum; Beobachter entstehen erst mit RU24.
    pub fn unobserve(&mut self, _env: &mut Env, _id: &str) {
        unimplemented!("RU24: unobserve")
    }
}

/// Aus `metrics.go` (RU24).
impl Room {
    /// Go: `writeReport`: schreibt den Spielmetrik-Report beim Raumende; ohne Sammler (`met` ist `None`) nichts.
    pub fn write_report(&mut self, _env: &mut Env) {
        if self.met.is_some() {
            unimplemented!("RU24: write_report")
        }
    }
}
