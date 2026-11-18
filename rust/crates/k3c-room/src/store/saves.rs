//! Spielstände unter `dir`, je Slot eine Datei `<slot>.json`. Port von `engine/store/saves.go`.
//!
//! Gos `sync.Mutex` entfällt: kein `Mutex` und keine Threads im Crate; wer mehrere Threads braucht, sperrt selbst (RU27).
//! Der Store speichert und liest die Bytes unverändert; `parse_save` prüft nur `campaignId`, `version` und `savedAt`.
//! Die Versionen 1 bis 8 migriert die Sim (RU10), nicht der Store.
use std::io::ErrorKind;
use std::path::PathBuf;

use serde_json::Value;

use super::atomic::Fs;
use super::stamp::Stamp;
use crate::fehler::Fehler;

/// Obergrenze eines Spielstands (wie `server/saves.mjs`).
pub const MAX_SAVE_BYTES: usize = 1 << 20;

/// Die Spielstände eines Ordners. Die Uhr gibt der Aufrufer vor (Pflichtfeld, kein `SystemTime` im Crate).
pub struct Saves {
    pub dir: PathBuf,
    pub now: Box<dyn FnMut() -> Stamp>,
    pub fs: Fs,
}

/// Slot-Name: `^[a-z0-9-]{1,32}$` (von Hand, ohne `regex`).
pub fn is_slot(slot: &str) -> bool {
    let n = slot.len();
    (1..=32).contains(&n)
        && slot
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
}

/// Die Felder, die jeder Spielstand haben muss.
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub campaign_id: String,
    pub saved_at: String,
    pub version: f64,
}

/// Prüft: ein JSON-Objekt mit `campaignId` (Text) und `version` (Zahl).
pub fn parse_save(data: &[u8]) -> Result<Header, Fehler> {
    let Ok(Value::Object(m)) = serde_json::from_slice::<Value>(data) else {
        return Err(Fehler::Invalid);
    };
    let (Some(Value::String(id)), Some(Value::Number(version))) = (m.get("campaignId"), m.get("version")) else {
        return Err(Fehler::Invalid);
    };
    let saved_at = match m.get("savedAt") {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.as_f64().map(|f| f.to_string()).unwrap_or_default(),
        _ => String::new(),
    };
    Ok(Header {
        campaign_id: id.clone(),
        saved_at,
        version: version.as_f64().unwrap_or_default(),
    })
}

/// Macht aus `savedAt` einen sicheren Dateinamen-Teil: `:` und `.` werden `-`, alles außer `A-Za-z0-9-` fällt weg.
fn clean_stamp(saved_at: &str) -> String {
    saved_at
        .replace([':', '.'], "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect()
}

impl Saves {
    pub fn new(dir: impl Into<PathBuf>, now: impl FnMut() -> Stamp + 'static) -> Saves {
        Saves {
            dir: dir.into(),
            now: Box::new(now),
            fs: Fs::default(),
        }
    }

    pub fn backup_dir(&self, slot: &str) -> PathBuf {
        self.dir.join("backups").join(slot)
    }

    pub(super) fn now(&mut self) -> Stamp {
        (self.now)()
    }

    fn path(&self, slot: &str) -> Result<PathBuf, Fehler> {
        if !is_slot(slot) {
            return Err(Fehler::Slot);
        }
        Ok(self.dir.join(format!("{slot}.json")))
    }

    /// Liest einen Spielstand; ohne Datei `Fehler::NotFound`.
    pub fn load(&self, slot: &str) -> Result<Vec<u8>, Fehler> {
        match std::fs::read(self.path(slot)?) {
            Err(e) if e.kind() == ErrorKind::NotFound => Err(Fehler::NotFound),
            r => Ok(r?),
        }
    }

    /// Schreibt einen Spielstand. Gehört der bisherige Stand des Slots zu einem anderen Spiel (andere `campaignId`) oder
    /// einer anderen Version, wird er vorher als `<slot>-<savedAt>.json` gesichert (wie `server/saves.mjs`); die Antwort
    /// ist dann dessen Dateiname, sonst leer. Sonst wandert der bisherige Stand in die rotierenden Sicherungen (B-028).
    pub fn store(&mut self, slot: &str, data: &[u8]) -> Result<String, Fehler> {
        let path = self.path(slot)?;
        if data.len() > MAX_SAVE_BYTES {
            return Err(Fehler::TooLarge);
        }
        let next = parse_save(data)?;
        let prev = match self.fs.read_retry(&path) {
            Err(e) if e.kind() == ErrorKind::NotFound => {
                self.fs.write_atomic(&path, data)?;
                return Ok(String::new());
            }
            // nie ohne Sicherung überschreiben: ein gesperrtes Lesen ist kein fehlender Stand (B-274)
            r => r?,
        };
        let backup = self.backup_prev(slot, &prev, &next)?;
        self.fs.write_atomic(&path, data)?;
        Ok(backup)
    }

    /// Sichert den bisherigen Stand `prev`: anderes Spiel oder andere Version unter eigenem Namen
    /// (`<slot>-<savedAt>.json`, Rückgabe), sonst in die rotierenden Sicherungen.
    fn backup_prev(&mut self, slot: &str, prev: &[u8], next: &Header) -> Result<String, Fehler> {
        let old = match parse_save(prev) {
            Ok(old) if old.campaign_id != next.campaign_id || old.version != next.version => old,
            _ => {
                self.rotate(slot, prev)?;
                return Ok(String::new());
            }
        };
        let backup = format!("{slot}-{}.json", self.stamp(&old.saved_at));
        std::fs::rename(self.path(slot)?, self.dir.join(&backup))?;
        Ok(backup)
    }

    /// Entfernt einen Spielstand und seine rotierenden Sicherungen. Gibt es ihn nicht, ist das kein Fehler.
    /// (Sicherungen eines anderen Spiels unter demselben Namen, `<slot>-<savedAt>.json`, bleiben liegen.)
    pub fn delete(&self, slot: &str) -> Result<(), Fehler> {
        let path = self.path(slot)?;
        match std::fs::remove_file(path) {
            Err(e) if e.kind() != ErrorKind::NotFound => return Err(e.into()),
            _ => {}
        }
        match std::fs::remove_dir_all(self.backup_dir(slot)) {
            Err(e) if e.kind() != ErrorKind::NotFound => Err(e.into()),
            _ => Ok(()),
        }
    }

    /// Nimmt einen Spielstand aus der Liste, ohne Daten zu verlieren (B-506, B-533): Er wandert in seine rotierenden
    /// Sicherungen (wiederherstellbar mit `restore`), ältere Sicherungen bleiben. Ohne Stand `Fehler::NotFound`.
    pub fn discard(&mut self, slot: &str) -> Result<(), Fehler> {
        let path = self.path(slot)?;
        let data = match std::fs::read(&path) {
            Err(e) if e.kind() == ErrorKind::NotFound => return Err(Fehler::NotFound),
            r => r?,
        };
        self.rotate(slot, &data)?;
        std::fs::remove_file(path)?;
        Ok(())
    }

    /// Löscht Spielstände, deren Name mit `prefix` beginnt und die seit `before` nicht mehr geschrieben wurden (Testläufe,
    /// B-086). Liefert die Zahl der gelöschten. Andere Spielstände bleiben, auch bei leerem `prefix`: der muss mindestens
    /// ein Zeichen haben.
    pub fn purge(&self, prefix: &str, before: Stamp) -> Result<usize, Fehler> {
        if prefix.is_empty() {
            return Err(Fehler::Slot);
        }
        let Ok(entries) = std::fs::read_dir(&self.dir) else {
            return Ok(0);
        };
        let mut slots: Vec<String> = entries
            .flatten()
            .filter_map(|e| {
                e.file_name()
                    .to_string_lossy()
                    .strip_suffix(".json")
                    .map(str::to_string)
            })
            .filter(|slot| slot.starts_with(prefix) && is_slot(slot))
            .collect();
        slots.sort_unstable();
        let mut n = 0;
        for slot in slots {
            let modified = std::fs::metadata(self.dir.join(format!("{slot}.json")))
                .and_then(|m| m.modified())
                .ok()
                .and_then(Stamp::from_modified);
            if modified.is_some_and(|m| m < before) {
                self.delete(&slot)?;
                n += 1;
            }
        }
        Ok(n)
    }

    /// Zahl der Spielstände (ohne Sicherungen).
    pub fn count(&self) -> usize {
        let Ok(entries) = std::fs::read_dir(&self.dir) else {
            return 0;
        };
        entries
            .flatten()
            .filter(|e| {
                let name = e.file_name().to_string_lossy().into_owned();
                name.strip_suffix(".json").is_some_and(is_slot)
            })
            .count()
    }

    /// Macht aus `savedAt` einen sicheren Dateinamen-Teil (nie ein Pfad); ohne `savedAt` die aktuelle Zeit in ms.
    fn stamp(&mut self, saved_at: &str) -> String {
        let clean = clean_stamp(saved_at);
        if clean.is_empty() || clean.len() > 40 {
            return self.now().unix_millis().to_string();
        }
        clean
    }
}
