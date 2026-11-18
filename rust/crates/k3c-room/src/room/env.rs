//! Die Umgebung eines Raum-Aufrufs (`Env`): Uhr, Store, Log und Snapshot gehören dem Manager und kommen als `&mut Env`
//! in jede Raum-Methode, die sie braucht (Gos Rückverweis `r.m` entfällt). `Env` ist ein kurzlebiger Verleih ohne
//! `Rc`/`RefCell`: der Manager baut ihn je Aufruf aus seinen Feldern.
use k3c_data::Data;
use k3c_sim::state::World;

use super::log::Log;
use crate::fehler::Fehler;
use crate::store::{Saves, Stamp};

/// Die Spielstände (Go `room.Store`, `engine/store.Saves`). `load` liefert `Fehler::NotFound`, wenn es keinen gibt.
pub trait Store {
    fn load(&mut self, name: &str) -> Result<Vec<u8>, Fehler>;
    /// Schreibt den Stand; die Antwort ist der Name einer Sicherung eines anderen Spiels (sonst leer).
    fn store(&mut self, name: &str, data: &[u8]) -> Result<String, Fehler>;
    /// Entfernt einen Spielstand samt Sicherungen; ein fehlender ist kein Fehler.
    fn delete(&mut self, name: &str) -> Result<(), Fehler>;
}

impl Store for Saves {
    fn load(&mut self, name: &str) -> Result<Vec<u8>, Fehler> {
        Saves::load(self, name)
    }
    fn store(&mut self, name: &str, data: &[u8]) -> Result<String, Fehler> {
        Saves::store(self, name, data)
    }
    fn delete(&mut self, name: &str) -> Result<(), Fehler> {
        Saves::delete(self, name)
    }
}

/// Baut den Zustand einer Stufe für `Peer::state` (Go `Manager.Snapshot`): Stufe, Zeitraffer, Pause.
pub type SnapshotFn = fn(&World, i64, bool) -> serde_json::Value;

/// Verleih von Uhr, Store, Log und Snapshot an den Raum (der Manager bleibt ihr Besitzer).
pub struct Env<'a> {
    /// Die Uhr des Manager zu Beginn des Aufrufs (Wanduhr, nur für Fristen und Stempel, nicht für den Spielverlauf).
    pub now: Stamp,
    /// Spiel-Daten des Managers (Schwierigkeitsgrade für `step_island`).
    pub data: &'a Data,
    pub store: &'a mut dyn Store,
    pub log: &'a mut dyn Log,
    /// Dev-Mode (`K3C_DEV`): `timescale` erscheint dann im Zustand.
    pub dev: bool,
    /// `None`: der Zustand ist die Welt selbst als JSON (Tests); sonst der Aufbau des Aufrufers.
    pub snapshot: Option<SnapshotFn>,
}
