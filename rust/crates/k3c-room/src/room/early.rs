//! Vorgezogen aus RU24: der kleine Teil der Beobachter (`observer.go`), ohne den der Raum nicht zählen kann. RU24.4
//! ergänzt den Rest und bewegt ihn in seine Datei. Speichern steht seit RU24.2 in `saver.rs`, Pause und Zeitraffer seit
//! RU24.1 in `pause.rs` und `dev.rs`.
use super::types::Room;

impl Room {
    /// Beobachter mit Verbindung (`observer`, B-352).
    pub fn watching(&self) -> usize {
        self.devices.values().filter(|d| d.connected && d.observer).count()
    }
}
