//! Spielstände unter `dir`, je Slot eine Datei `<slot>.json`. Gerüst aus RU21.1 (Struct, `new`, `backup_dir`, `now`);
//! RU21.2 ergänzt `load`, `store`, `delete`, `discard`, `purge`, `count` und die Liste.
//!
//! Gos `sync.Mutex` entfällt: kein `Mutex` und keine Threads im Crate; wer mehrere Threads braucht, sperrt selbst (RU27).
use std::path::PathBuf;

use super::atomic::Fs;
use super::stamp::Stamp;
use crate::fehler::Fehler;

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
}

/// Platzhalter für RU21.2 (Leitfaden § 7): `Saves::store` aus `saves.go`; RU21.2 löscht ihn.
#[allow(clippy::unimplemented)]
impl Saves {
    pub fn store(&mut self, _slot: &str, _data: &[u8]) -> Result<String, Fehler> {
        unimplemented!("RU21.2: store")
    }
}
