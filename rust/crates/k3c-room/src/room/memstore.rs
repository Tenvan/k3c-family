//! Spielstand-Speicher der Raum-Tests (Go: `memStore` aus `memstore_test.go`, dazu `countStore` aus `save_test.go` und
//! `panicStore` aus `saver_test.go`): im Speicher, zählt Aufrufe, kann scheitern oder einmal paniken.
#![allow(clippy::panic)] // Testhilfe: `panic_once` stellt Gos `panicStore` nach

use std::collections::BTreeMap;

use super::env::Store;
use crate::fehler::Fehler;

/// Spielstand-Speicher im Speicher (Go `memStore`).
#[derive(Debug, Default)]
pub struct MemStore {
    pub data: BTreeMap<String, Vec<u8>>,
    /// Gelungene Speicherungen (Go `memStore.saves`).
    pub saves: usize,
    pub deleted: Vec<String>,
    /// Alle Aufrufe von `store`, auch gescheiterte (Go `countStore.n`).
    pub calls: usize,
    /// Gesetzt: `store` scheitert mit diesem Fehler (Go `countStore.fail`).
    pub fail: Option<Fehler>,
    /// Gesetzt: das nächste `store` panikt (Go `panicStore`), danach wieder normal.
    pub panic_once: bool,
}

impl Store for MemStore {
    fn load(&mut self, name: &str) -> Result<Vec<u8>, Fehler> {
        self.data.get(name).cloned().ok_or(Fehler::NotFound)
    }

    fn store(&mut self, name: &str, data: &[u8]) -> Result<String, Fehler> {
        self.calls += 1;
        if self.panic_once {
            self.panic_once = false;
            panic!("Platte weg");
        }
        if let Some(e) = &self.fail {
            return Err(e.clone());
        }
        self.data.insert(name.to_string(), data.to_vec());
        self.saves += 1;
        Ok(String::new())
    }

    fn delete(&mut self, name: &str) -> Result<(), Fehler> {
        self.data.remove(name);
        self.deleted.push(name.to_string());
        Ok(())
    }
}
