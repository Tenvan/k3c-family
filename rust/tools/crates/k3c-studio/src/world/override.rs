//! Werte-Overrides schreiben (Port von `override.go`). RU70.3 zieht nur den Schreibweg vor (`writeMu`, `given`,
//! `writeAtomic`), den Katalog und Bearbeiten brauchen; die Overrides selbst folgen in RU70.4.

use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

use k3c_kit::logger::Logger;

use super::rename::rename_replace;

/// Reiht schreibende Aufrufe: zwei gleichzeitige Änderungen derselben Datei verlieren keinen Wert.
static WRITE_MU: Mutex<()> = Mutex::new(());

pub(super) fn write_lock() -> MutexGuard<'static, ()> {
    WRITE_MU.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Der Wert ist gesetzt (nicht leer, nicht `null`).
pub(super) fn given(value: &str) -> bool {
    let text = value.trim();
    !text.is_empty() && text != "null"
}

/// Neue Datei `.tmp-world-*` im Ordner von `path` (das Präfix ignoriert der Datei-Watch).
pub(super) fn temp_file(path: &Path) -> io::Result<(PathBuf, File)> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    loop {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        let name = format!(
            ".tmp-world-{}-{}-{}",
            std::process::id(),
            now.as_nanos(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
        let tmp = dir.join(name);
        match File::create_new(&tmp) {
            Ok(file) => return Ok((tmp, file)),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
}

/// Schreibt über eine Temp-Datei im selben Ordner, damit nie eine halbe Datei liegt; die Rechte bleiben.
pub(super) fn write_atomic(path: &Path, content: &[u8], log: &Logger) -> Result<(), String> {
    let (tmp, mut file) = temp_file(path).map_err(|e| e.to_string())?;
    if let Ok(meta) = std::fs::metadata(path) {
        let _ = file.set_permissions(meta.permissions());
    }
    let written = file.write_all(content);
    drop(file);
    let result = written
        .map_err(|e| format!("schreiben: {e}"))
        .and_then(|()| rename_replace(&tmp, path, log).map_err(|e| e.to_string()));
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
}
