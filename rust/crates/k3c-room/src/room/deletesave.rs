//! Spielstand löschen (B-506). Port von `deletesave.go`.
use super::env::Store;
use super::log::Log;
use super::manager::{Manager, valid_save_name};
use crate::fehler::Fehler;

impl<S: Store, L: Log> Manager<S, L> {
    /// Führt `del` aus (Löschen des Spielstands `name`), solange kein Raum ihn offen hat. Ist er in einem Raum offen, auch
    /// ohne verbundenes Gerät, `SaveInUse`: Der Raum würde ihn sonst beim nächsten Speichern neu schreiben. `del` läuft,
    /// bevor der Manager etwas anderes tut (Gos Sperre des Managers: hier gibt es keinen zweiten Zugriff).
    pub fn delete_save(&mut self, name: &str, del: impl FnOnce() -> Result<(), Fehler>) -> Result<(), Fehler> {
        if !valid_save_name(name) {
            return Err(Fehler::BadRequest);
        }
        if let Some(code) = self.rooms.iter().find(|(_, r)| r.name == name).map(|(c, _)| c.clone()) {
            self.log.info(
                "🚫 Spielstand löschen abgelehnt, Raum offen",
                &[("ns", &"room"), ("save", &name), ("room", &code)],
            );
            return Err(Fehler::SaveInUse);
        }
        del()
    }
}
