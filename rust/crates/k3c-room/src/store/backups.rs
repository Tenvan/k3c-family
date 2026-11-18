//! Rotierende Sicherungen je Spielstand (B-028). Port von `engine/store/backups.go`.
use std::io::ErrorKind;
use std::path::Path;

use super::saves::{Saves, is_slot};
use super::stamp::Stamp;
use crate::fehler::Fehler;

/// Zahl der Sicherungen je Spielstand (B-028, Entscheidung 🧑: 5).
pub const BACKUP_KEEP: usize = 5;

/// Eine Sicherung eines Spielstands; `saved_at` fehlt, wenn der Dateiname keinen Zeitstempel trägt.
#[derive(Debug, Clone, PartialEq)]
pub struct Backup {
    pub name: String,
    pub saved_at: Option<Stamp>,
    pub size: i64,
}

fn exists(path: &Path) -> bool {
    std::fs::metadata(path).is_ok()
}

impl Saves {
    /// Legt den bisherigen Stand eines Slots als Sicherung ab und behält die `BACKUP_KEEP` neuesten.
    pub fn rotate(&mut self, slot: &str, prev: &[u8]) -> Result<(), Fehler> {
        if !is_slot(slot) {
            return Err(Fehler::Slot);
        }
        let dir = self.backup_dir(slot);
        let mut name = format!("{}.json", self.now().format_backup());
        let mut i = 1;
        while exists(&dir.join(&name)) {
            name = format!("{}-{i}.json", self.now().format_backup());
            i += 1;
        }
        self.fs.write_atomic(&dir.join(&name), prev)?;
        let Ok(list) = self.list_backups(slot) else {
            return Ok(()); // Aufräumen ist Kür: der neue Stand wird trotzdem geschrieben
        };
        for b in list.iter().skip(BACKUP_KEEP) {
            let _ = std::fs::remove_file(dir.join(&b.name)); // beim nächsten Speichern erneut versucht
        }
        Ok(())
    }

    /// Listet die Sicherungen eines Slots, neueste zuerst.
    pub fn backups(&self, slot: &str) -> Result<Vec<Backup>, Fehler> {
        self.list_backups(slot)
    }

    fn list_backups(&self, slot: &str) -> Result<Vec<Backup>, Fehler> {
        if !is_slot(slot) {
            return Err(Fehler::Slot);
        }
        let entries = match std::fs::read_dir(self.backup_dir(slot)) {
            Ok(e) => e,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(e.into()),
        };
        let mut out = Vec::new();
        for entry in entries.flatten() {
            let Ok(info) = entry.metadata() else { continue };
            let name = entry.file_name().to_string_lossy().into_owned();
            if info.is_dir() || !name.ends_with(".json") || name.starts_with('.') {
                continue;
            }
            let stem = name.trim_end_matches(".json");
            let first = stem.split('-').next().unwrap_or_default();
            let size = i64::try_from(info.len()).unwrap_or(i64::MAX);
            out.push(Backup {
                saved_at: Stamp::parse_backup(first),
                name,
                size,
            });
        }
        // Absteigend nach Name; Namen sind je Ordner eindeutig, es gibt keinen Gleichstand.
        out.sort_by(|a, b| b.name.cmp(&a.name));
        Ok(out)
    }

    /// Macht eine Sicherung zum aktuellen Stand; der bisherige wird dabei selbst gesichert. Der Name wird nur
    /// angenommen, wenn er in der Liste des Slots steht (nie als Pfad).
    pub fn restore(&mut self, slot: &str, name: &str) -> Result<(), Fehler> {
        let list = self.list_backups(slot)?;
        let Some(b) = list.iter().find(|b| b.name == name) else {
            return Err(Fehler::NotFound);
        };
        let data = std::fs::read(self.backup_dir(slot).join(&b.name))?;
        self.store(slot, &data)?;
        Ok(())
    }
}
