//! Umbenennen mit Wiederholung: unter Windows hält ein Virenscanner oder der Suchindex eine frisch geschriebene Datei
//! kurz offen, das Umbenennen meldet dann „Zugriff verweigert“. Ein paar kurze Wiederholungen genügen.

use std::fs;
use std::io;
use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::error::Result;
use crate::fsutil;

const RENAME_TRIES: u32 = 5;
const RENAME_PAUSE: Duration = Duration::from_millis(40);

/// Zugriffssperre: `PermissionDenied` oder unter Windows die Fehler 5 (Zugriff) und 32 (Freigabeverletzung).
fn is_locked(e: &io::Error) -> bool {
    e.kind() == io::ErrorKind::PermissionDenied || (cfg!(windows) && matches!(e.raw_os_error(), Some(5 | 32)))
}

/// Ersetzt `to` durch `from`, wiederholt bei einer Zugriffssperre; scheitert es, wird `from` gelöscht, damit keine
/// Temp-Datei liegen bleibt.
pub fn rename(from: &Path, to: &Path) -> Result<()> {
    let mut result = fs::rename(from, to);
    for attempt in 1..=RENAME_TRIES {
        match &result {
            Err(e) if is_locked(e) => thread::sleep(RENAME_PAUSE * attempt),
            _ => break,
        }
        if attempt < RENAME_TRIES {
            result = fs::rename(from, to);
        }
    }
    if result.is_err() {
        let _ = fsutil::remove(from);
    }
    Ok(result?)
}
