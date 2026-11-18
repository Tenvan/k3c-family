//! Vorgezogen aus RU24: die kleinen Teile von Speichern (`saver.go`) und Beobachtern (`observer.go`), ohne die
//! `Room::tick` nicht laufen kann. Sie rechnen wie Go; RU24.2 und RU24.4 ergänzen den Rest (Hintergrund-Schreiber,
//! Beobachter) und bewegen sie in ihre Dateien. Pause und Zeitraffer stehen seit RU24.1 in `pause.rs` und `dev.rs`.
use super::env::Env;
use super::types::{Room, SAVE_TICKS};

impl Room {
    /// Beobachter mit Verbindung (`observer`, B-352).
    pub fn watching(&self) -> usize {
        self.devices.values().filter(|d| d.connected && d.observer).count()
    }

    /// Zählt einen Tick für das Speichern; im Takt `SAVE_TICKS`, bei Stufenwechsel und bei Tagesanbruch wird gespeichert.
    /// Vorläufig synchron (`save`), RU24 ersetzt es durch den Hintergrund-Schreiber.
    pub fn autosave(&mut self, env: &mut Env, travelled: bool, dawn: bool) {
        self.save_due += 1;
        if travelled || dawn || self.save_due >= SAVE_TICKS {
            self.save(env);
        }
    }

    /// Sichert den Spielstand (Go `save`): Zähler zurück, nach Game Over nichts. Vorläufig synchron, RU24 schreibt im
    /// Hintergrund.
    pub fn save(&mut self, env: &mut Env) {
        self.save_due = 0;
        self.save_sync(env);
    }

    /// Schickt Zustände; ein „saved“-Ereignis nach einer Hintergrund-Speicherung kommt mit RU24 (ohne Schreiber ist nie
    /// etwas „geschrieben“).
    pub fn push_states_saved(&mut self, env: &mut Env) -> bool {
        self.push_states(env)
    }
}
