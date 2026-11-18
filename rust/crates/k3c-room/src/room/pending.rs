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

/// Aus `dev_combat.go` (RU24.2).
impl Room {
    /// Go: `devCombat`: Kampf-Aktionen (`boss`, `event`, `bossPhase`, `night`, `spawn`, `spawnCastle`, B-603, B-766). Eine
    /// Aktion ist bis RU24.2 abgelehnt (`None`), die Kampf-Aktionen warten auf RU24.2.
    pub fn dev_combat(
        &mut self,
        _env: &mut Env,
        _who: &str,
        a: &super::dev::DevAction,
        _map: &super::dev::SlotMap,
    ) -> Option<super::dev::Attrs> {
        // Bis RU24.2 lehnt der Raum auch die Kampf-Aktionen ab (BadRequest) statt abzubrechen.
        let _ = a;
        None
    }
}
