//! Repo-Wurzel, Stand des Slot-Registers, Schreibregel und Beobachtung.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use sha2::{Digest, Sha256};

use crate::error::{Error, Result};
use crate::register::{self, Choice, REGISTER_PATH, Slot};

/// Sucht ab `dir` aufwärts die Repo-Wurzel: der Ordner mit der Datei `rust/Cargo.toml` und dem Ordner `data/`. Gleiche
/// Regel wie `k3c_kit::mcp::find_root` (eigene Kopie: `k3c-assetkit` hängt nicht an `k3c-kit`).
pub fn find_root(dir: &Path) -> Result<PathBuf> {
    let mut dir = dir.to_path_buf();
    loop {
        if is_repo_root(&dir) {
            return Ok(dir);
        }
        let parent = match dir.parent() {
            Some(p) if p.as_os_str().is_empty() => Some(PathBuf::from(".")),
            p => p.map(Path::to_path_buf),
        };
        match parent {
            Some(p) if p != dir => dir = p,
            _ => {
                return Err(Error::Msg(format!(
                    "repo-wurzel nicht gefunden: kein Ordner mit rust/Cargo.toml und data/ über {}",
                    dir.display()
                )));
            }
        }
    }
}

/// Beides zusammen: das Spielpaket `dist/k3c-game` hat nur `data/`.
fn is_repo_root(dir: &Path) -> bool {
    dir.join("rust").join("Cargo.toml").is_file() && dir.join("data").is_dir()
}

/// Stand des Slot-Registers: SHA-256 des Datei-Inhalts (hex).
pub fn stamp(root: &Path) -> Result<String> {
    let bytes = fs::read(root.join(REGISTER_PATH))?;
    Ok(Sha256::digest(&bytes).iter().map(|b| format!("{b:02x}")).collect())
}

/// [`crate::set_choices`] mit Schreibregel: gespeichert wird nur, wenn das Register noch den Stand `stand` hat, sonst
/// [`Error::Stale`] ohne Schreibzugriff. Liefert den Slot und den neuen Stand.
///
/// Die Sperre reiht nur Schreiber dieses Prozesses; zwischen Stand und Speichern kann ein anderer Prozess noch
/// schreiben, [`watch`] meldet einen solchen Fremdschreiber trotzdem.
pub fn set_choices_if(root: &Path, id: &str, choices: &[Choice], stand: &str) -> Result<(Slot, String)> {
    let _guard = register::lock();
    if stamp(root)? != stand {
        return Err(Error::Stale);
    }
    let slot = register::set_choices_locked(root, id, choices)?;
    Ok((slot, stamp(root)?))
}

/// Laufende Beobachtung; [`WatchHandle::stop`] oder das Fallenlassen beenden sie.
#[derive(Debug)]
#[must_use = "beim Fallenlassen endet die Beobachtung"]
pub struct WatchHandle {
    stopped: Arc<AtomicBool>,
    thread: thread::Thread,
}

impl WatchHandle {
    /// Beendet den Thread; ein gerade laufender `on_change` läuft noch zu Ende.
    pub fn stop(&self) {
        self.stopped.store(true, Ordering::SeqCst);
        self.thread.unpark();
    }
}

impl Drop for WatchHandle {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Größe und Änderungszeit; `None` bei fehlender Datei.
type FileInfo = Option<(u64, Option<SystemTime>)>;

fn file_info(path: &Path) -> FileInfo {
    fs::metadata(path).ok().map(|m| (m.len(), m.modified().ok()))
}

/// Prüft im Takt `every`, ob sich das Slot-Register geändert hat, und ruft dann `on_change`. Größe und Änderungszeit
/// sind der schnelle Vorfilter, erst der Hash entscheidet; eine fehlende Datei gilt als eigener Stand. Der Stand beim
/// Aufruf ist der Ausgangsstand.
pub fn watch(root: &Path, every: Duration, on_change: impl Fn() + Send + 'static) -> WatchHandle {
    let root = root.to_path_buf();
    let path = root.join(REGISTER_PATH);
    let mut last_info = file_info(&path);
    let mut last_hash = stamp(&root).ok();
    let stopped = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stopped);
    let worker = thread::spawn(move || {
        while wait(every, &flag) {
            let info = file_info(&path);
            if info == last_info {
                continue;
            }
            let hash = stamp(&root).ok();
            if hash.is_none() && info.is_some() {
                continue; // gesperrt (Rename eines anderen Schreibers): im nächsten Takt neu versuchen
            }
            last_info = info;
            if hash != last_hash {
                last_hash = hash;
                on_change();
            }
        }
    });
    WatchHandle {
        stopped,
        thread: worker.thread().clone(),
    }
}

/// Schläft einen Takt; `false`, sobald die Beobachtung beendet ist.
fn wait(every: Duration, stopped: &AtomicBool) -> bool {
    let start = Instant::now();
    while !stopped.load(Ordering::SeqCst) {
        let Some(rest) = every.checked_sub(start.elapsed()).filter(|r| !r.is_zero()) else {
            return true;
        };
        thread::park_timeout(rest);
    }
    false
}

#[cfg(test)]
#[path = "stamp_tests.rs"]
mod tests;
