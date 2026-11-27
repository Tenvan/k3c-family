//! Rücknahme eines gescheiterten Entfernens in `ChangeSet::apply` (B-867): Datei oder Ordner wird vorher vollständig
//! gelesen (Planungsdateien sind klein) und bei einem Fehler byte-gleich wiederhergestellt.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Ein Schritt zum Zurücknehmen.
pub type Undo = Box<dyn FnOnce() -> io::Result<()>>;

/// Pfad → Inhalt, Ordner mit `None`; Eltern sortieren vor ihren Kindern.
type Snapshot = BTreeMap<PathBuf, Option<Vec<u8>>>;

fn backup_tree(path: &Path, out: &mut Snapshot) -> io::Result<()> {
    if !fs::symlink_metadata(path)?.is_dir() {
        out.insert(path.to_owned(), Some(fs::read(path)?));
        return Ok(());
    }
    out.insert(path.to_owned(), None);
    for entry in fs::read_dir(path)? {
        backup_tree(&entry?.path(), out)?;
    }
    Ok(())
}

/// Stellt den Stand wieder her: fehlende Ordner anlegen, fehlende oder geänderte Dateien schreiben.
fn restore_tree(snapshot: &Snapshot) -> io::Result<()> {
    for (path, content) in snapshot {
        let Some(bytes) = content else {
            fs::create_dir_all(path)?;
            continue;
        };
        if fs::read(path).is_ok_and(|current| &current == bytes) {
            continue;
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, bytes)?;
    }
    Ok(())
}

fn remove_all(path: &Path) -> io::Result<()> {
    if fs::symlink_metadata(path)?.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

fn nothing() -> Undo {
    Box::new(|| Ok(()))
}

/// Sichert `path` und entfernt es; der erste Wert stellt den Stand wieder her (fehlt `path`, ist nichts zu tun).
/// Scheitert das Entfernen, kommt das Zurücknehmen mit dem Fehler, weil `path` teilweise weg sein kann.
pub fn remove_with_backup(path: &Path) -> (Undo, io::Result<()>) {
    let mut snapshot = Snapshot::new();
    match backup_tree(path, &mut snapshot) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => (nothing(), Ok(())),
        Err(e) => (nothing(), Err(e)),
        Ok(()) => {
            let removed = remove_all(path);
            (Box::new(move || restore_tree(&snapshot)), removed)
        }
    }
}
