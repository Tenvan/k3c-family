//! Hilfen der Tests: Wegwerf-Ordner und feste Uhr. Geschrieben wird nur im Repo (`.work/`, ignoriert), nie in `%TEMP%`.
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use super::stamp::Stamp;

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Ein leerer Ordner unter `<Repo>/.work/room-tests/`, der beim Ende des Tests gelöscht wird (Go `t.TempDir`).
pub struct TempDir(PathBuf);

impl TempDir {
    pub fn new() -> TempDir {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.work/room-tests");
        let dir = root.join(format!("{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("Testordner anlegen");
        TempDir(dir)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Uhr, die bei jedem Aufruf eine Sekunde später liefert, beginnend nach dem 30.9.2026 12:00:00 UTC (Go `clock`).
pub fn clock() -> impl FnMut() -> Stamp {
    let mut t = Stamp::from_civil(2026, 9, 30, 12, 0, 0, 0);
    move || {
        t = t.add_secs(1);
        t
    }
}

/// Ein Spielstand mit der Nummer `n` (Go `save`).
pub fn save(n: i64) -> Vec<u8> {
    format!(r#"{{"version":1,"campaignId":"a","n":{n}}}"#).into_bytes()
}
