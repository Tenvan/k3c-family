//! Platzhalter für Funktionen späterer Sessions (Port-Leitfaden § 7). Der liefernde Sprint löscht den Platzhalter; RU22
//! prüft, dass keiner übrig ist.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint

use k3c_sim::island::Island;

use super::env::{Env, Store};
use super::log::Log;
use super::manager::Manager;
use super::types::{Options, Room};
use crate::fehler::Fehler;

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

/// Aus `island_switch.go` und `observer.go` (RU24). `Room::tick` ruft `switch_island` jeden Tick; ohne bereiten Wechsel
/// ist es wie in Go `false`.
impl Room {
    /// Go: `switchIsland`: tauscht die Insel, wenn die Sim `switch_ready` meldet und es eine nächste gibt.
    pub fn switch_island(&mut self, _env: &mut Env) -> bool {
        if !self.isl.switch_ready {
            return false;
        }
        unimplemented!("RU24: switch_island")
    }

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

/// Aus `worlds.go` (RU24): Welt-Auflösung und Insel-Erzeugung des Managers.
impl<S: Store, L: Log> Manager<S, L> {
    /// Go: `Manager.createIn`: legt eine neue Insel in der Welt `opts.world` an (Standard-Welt ohne Angabe).
    pub fn create_in(&mut self, _seed: &str, _depth: i64, _opts: &Options) -> Result<Island, Fehler> {
        unimplemented!("RU24: create_in")
    }

    /// Go: `ParseIslandSave` und `Manager.loadIn`: liest einen Spielstand und baut die Insel in seiner Welt.
    pub fn load_in(&mut self, _data: &[u8]) -> Result<Island, Fehler> {
        unimplemented!("RU24: load_in")
    }
}
