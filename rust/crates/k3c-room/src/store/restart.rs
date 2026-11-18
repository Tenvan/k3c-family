//! Gemerkte Räume für einen geplanten Neustart (WZ25.1, B-508): eine Datei im Spielstand-Ordner, die kein Spielstand
//! ist (Endung `.state`, nicht `.json`, deshalb nicht in der Spielstandliste). Port von `engine/store/restart.go`.
use std::io::ErrorKind;

use super::saves::Saves;
use super::stamp::Stamp;
use crate::fehler::Fehler;

const RESTART_ROOMS_FILE: &str = "restart-rooms.state";

/// Höchstalter der Merkdatei in Nanosekunden: Herunterfahren und Neustart dauern Sekunden (2 Minuten).
pub const REMEMBERED_ROOMS_MAX_AGE: i128 = 2 * 60 * 1_000_000_000;

impl Saves {
    /// Schreibt die gemerkten Räume (atomar).
    pub fn remember_rooms(&mut self, data: &[u8]) -> Result<(), Fehler> {
        let path = self.dir.join(RESTART_ROOMS_FILE);
        self.fs.write_atomic(&path, data)?;
        Ok(())
    }

    /// Liest die gemerkten Räume und löscht die Datei; ohne Datei `Fehler::NotFound`.
    pub fn take_remembered_rooms(&mut self) -> Result<Vec<u8>, Fehler> {
        let path = self.dir.join(RESTART_ROOMS_FILE);
        let data = match std::fs::read(&path) {
            Ok(d) => d,
            Err(e) if e.kind() == ErrorKind::NotFound => return Err(Fehler::NotFound),
            Err(e) => return Err(e.into()),
        };
        // Eine alte Datei stammt von einem Neustart, der nicht stattfand (Supervisor gestoppt, Build gescheitert):
        // verwerfen, ein späterer normaler Start öffnet keine alten Räume.
        let modified = std::fs::metadata(&path)
            .and_then(|m| m.modified())
            .ok()
            .and_then(Stamp::from_modified);
        let stale = modified.is_some_and(|m| self.now().since_nanos(m) > REMEMBERED_ROOMS_MAX_AGE);
        std::fs::remove_file(&path)?;
        if stale {
            return Err(Fehler::NotFound);
        }
        Ok(data)
    }
}
