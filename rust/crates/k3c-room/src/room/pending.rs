//! Platzhalter für Funktionen späterer Sessions (Port-Leitfaden § 7). Der liefernde Sprint löscht den Platzhalter; RU22
//! prüft, dass keiner übrig ist.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint

use k3c_sim::island::Island;

use super::env::{Env, Store};
use super::log::Log;
use super::manager::Manager;
use super::types::{Options, Room};
use crate::fehler::Fehler;
use crate::store::Stamp;

/// Go: `metrics` (metrics.go), die Spielmetrik des Raumlaufs; RU24. Ohne Sammler ist `Room::met` `None`.
#[derive(Debug, Clone, Default)]
pub struct Metrics;

impl Metrics {
    /// Go: `metrics.join`.
    pub fn join(&mut self, _id: &str) {
        unimplemented!("RU24: metrics join")
    }
}

/// Aus `actions.go` und `logging.go` (RU21.4).
impl Room {
    /// Go: `Room.Tick` (actions.go): rechnet einen Tick; `false`: der Raum ist geschlossen.
    pub fn tick(&mut self, _env: &mut Env) -> bool {
        unimplemented!("RU21.4: tick")
    }

    /// Go: `Room.sweep` (actions.go): prüft die Fristen nach Wanduhr; `(changed, remove)`.
    pub fn sweep(&mut self, _env: &mut Env, _now: Stamp) -> (bool, bool) {
        unimplemented!("RU21.4: sweep")
    }

    /// Go: `noteTick` (logging.go): merkt langsame Ticks vor und meldet sie gesammelt.
    pub fn note_tick(&mut self, _env: &mut Env, _took_nanos: i64) {
        unimplemented!("RU21.4: note_tick")
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
