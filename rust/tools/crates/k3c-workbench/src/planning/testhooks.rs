//! Erzwungene Fehler für die Tests (Go ersetzt dafür `writeFile` und `removeAll`). Je Thread, damit parallel laufende
//! Tests sich nicht stören.

use std::cell::RefCell;
use std::io;
use std::path::{Path, PathBuf};

thread_local! {
    static FAIL_WRITE: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
    static FAIL_REMOVE: RefCell<bool> = const { RefCell::new(false) };
    static WRITTEN: RefCell<Vec<PathBuf>> = const { RefCell::new(Vec::new()) };
}

/// Schreiben nach `path` scheitert in diesem Thread; die übrigen Schreibziele werden mitgeschrieben.
pub(super) fn fail_write(path: PathBuf) {
    FAIL_WRITE.set(Some(path));
    WRITTEN.with_borrow_mut(Vec::clear);
}

/// Schreibziele seit [`fail_write`], in ihrer Reihenfolge.
pub(super) fn written() -> Vec<PathBuf> {
    WRITTEN.with_borrow(Clone::clone)
}

/// Jedes Entfernen entfernt erst einen Teil (die README.md im Ordner bzw. die Datei selbst) und scheitert dann.
pub(super) fn fail_remove() {
    FAIL_REMOVE.set(true);
}

pub(super) fn before_write(path: &Path) -> io::Result<()> {
    if FAIL_WRITE.with_borrow(|fail| fail.as_deref() == Some(path)) {
        return Err(io::Error::other("erzwungener Schreibfehler"));
    }
    WRITTEN.with_borrow_mut(|written| written.push(path.to_owned()));
    Ok(())
}

pub(super) fn before_remove(path: &Path) -> io::Result<()> {
    if !FAIL_REMOVE.with_borrow(|fail| *fail) {
        return Ok(());
    }
    let part = if path.is_dir() {
        path.join("README.md")
    } else {
        path.to_owned()
    };
    // Ein fehlender Teil ändert nichts am erzwungenen Fehler.
    let _ = std::fs::remove_file(part);
    Err(io::Error::other("erzwungener Löschfehler"))
}
