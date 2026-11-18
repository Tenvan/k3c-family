//! Port von `watchedStore` aus `native/k3c-game/local/saves.go` (RU27.3 liefert den Rest der Datei: `List`, `Delete`,
//! `WorldList`). Hier steht nur, was `Session` zum Anlegen eines Managers braucht: der Spielstand-Speicher des Raums, der
//! das Ergebnis jeder Speicherung merkt, damit die Session daraus Cues macht (`CueSaved`, `CueSaveFailed`).
//!
//! Abweichung von Go: Der Manager besitzt seinen Store, die Session liest die Ergebnisse über ein geteiltes
//! `Arc<Mutex<SaveResults>>` (Go: `store`-Zeiger mit `sync.Mutex`). Gespeichert wird in `k3c-room` synchron im Takt, es
//! gibt keine Schreib-Goroutine.

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use k3c_room::fehler::Fehler;
use k3c_room::room::Store;
use k3c_room::store::Saves;

/// Die Ergebnisse der Speicherungen seit dem Anlegen (`None` = gespeichert, `Some(text)` = Fehler).
#[derive(Debug, Default)]
pub struct SaveResults {
    results: Vec<Option<String>>,
    stored: usize,
}

impl SaveResults {
    /// Zahl aller Ergebnisse seit dem Anlegen.
    pub fn count(&self) -> usize {
        self.stored
    }

    /// Liefert die Ergebnisse seit dem letzten Aufruf und leert sie.
    pub fn take(&mut self) -> Vec<Option<String>> {
        std::mem::take(&mut self.results)
    }
}

/// Geteilte Ergebnisse von Session und Store.
pub type SharedResults = Arc<Mutex<SaveResults>>;

/// Sperrt `m`; eine vergiftete Sperre wird übernommen (die Daten sind nur Zähler und Texte).
pub fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

/// `room::Store` über `Saves`, der das Ergebnis jeder Speicherung merkt.
pub struct WatchedStore {
    saves: Saves,
    shared: SharedResults,
}

impl WatchedStore {
    pub fn new(saves: Saves, shared: SharedResults) -> WatchedStore {
        WatchedStore { saves, shared }
    }
}

impl Store for WatchedStore {
    fn load(&mut self, name: &str) -> Result<Vec<u8>, Fehler> {
        self.saves.load(name)
    }

    fn store(&mut self, name: &str, data: &[u8]) -> Result<String, Fehler> {
        let res = self.saves.store(name, data);
        let mut r = lock(&self.shared);
        r.results.push(res.as_ref().err().map(ToString::to_string));
        r.stored += 1;
        res
    }

    fn delete(&mut self, name: &str) -> Result<(), Fehler> {
        self.saves.delete(name)
    }
}
