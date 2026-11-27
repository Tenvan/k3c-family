//! Robustes Ersetzen einer Datei (Port von `rename.go`).

use std::io;
use std::path::Path;
use std::time::Duration;

use k3c_kit::logger::Logger;
use serde_json::json;

/// Windows: `ERROR_ACCESS_DENIED` und `ERROR_SHARING_VIOLATION`, wenn ein Fremdprozess (Virenscanner, Indexer) die
/// Datei kurz hält.
const ERROR_ACCESS_DENIED: i32 = 5;
const ERROR_SHARING_VIOLATION: i32 = 32;

/// Wiederholungen beim Ersetzen einer kurz gesperrten Datei (B-187).
const RENAME_TRIES: u32 = 6;
const RENAME_PAUSE: Duration = Duration::from_millis(20);

/// Benennt `old` nach `new` um und ersetzt eine vorhandene Datei. Unter Windows wird ein kurz verweigertes Ersetzen
/// begrenzt wiederholt (⏳ im Log, nach dem letzten Versuch ❌); andere Fehler melden sofort.
pub(super) fn rename_replace(old: &Path, new: &Path, log: &Logger) -> io::Result<()> {
    rename_replace_with(old, new, log, |a, b| std::fs::rename(a, b))
}

/// Wie [`rename_replace`]; `rename` ist die Naht für den Test.
fn rename_replace_with(
    old: &Path,
    new: &Path,
    log: &Logger,
    mut rename: impl FnMut(&Path, &Path) -> io::Result<()>,
) -> io::Result<()> {
    let name = new.file_name().map(|n| n.to_string_lossy().into_owned());
    let mut result = rename(old, new);
    let mut tries = 1;
    while tries < RENAME_TRIES {
        let Some(e) = result.as_ref().err().filter(|e| retryable(e)) else {
            break;
        };
        let attrs = [
            ("ns", json!("world")),
            ("datei", json!(name)),
            ("versuch", json!(tries + 1)),
            ("err", json!(e.to_string())),
        ];
        log.warn("⏳ Datei gesperrt, neuer Versuch", &attrs);
        std::thread::sleep(RENAME_PAUSE);
        result = rename(old, new);
        tries += 1;
    }
    if let Err(e) = &result
        && tries > 1
    {
        let attrs = [
            ("ns", json!("world")),
            ("datei", json!(name)),
            ("versuche", json!(tries)),
            ("err", json!(e.to_string())),
        ];
        log.error("❌ Datei bleibt gesperrt, nicht ersetzt", &attrs);
    }
    result
}

fn retryable(error: &io::Error) -> bool {
    cfg!(windows)
        && matches!(
            error.raw_os_error(),
            Some(ERROR_ACCESS_DENIED | ERROR_SHARING_VIOLATION)
        )
}

#[cfg(test)]
#[path = "rename_tests.rs"]
mod tests;
