//! Tests aus `rename_test.go`. Die Wiederholungs-Tests gelten wie in Go nur unter Windows.
use std::io;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use super::atomic::{Fs, RENAME_TRIES};
use super::testutil::TempDir;

fn schnell() -> Fs {
    let mut fs = Fs::default();
    fs.pause = Duration::ZERO;
    fs
}

fn zaehler() -> (Arc<AtomicU32>, Arc<AtomicU32>) {
    let a = Arc::new(AtomicU32::new(0));
    (a.clone(), a)
}

// B-187: Ein kurz verweigertes Ersetzen wird wiederholt (nur unter Windows), andere Fehler nicht.
#[cfg(windows)]
#[test]
fn rename_wiederholt_verweigertes_ersetzen() {
    let (calls, c) = zaehler();
    let mut fs = schnell();
    fs.rename = Box::new(move |_, _| {
        if c.fetch_add(1, Ordering::SeqCst) + 1 < 3 {
            return Err(io::Error::from_raw_os_error(5));
        }
        Ok(())
    });
    assert!(fs.rename_file(Path::new("a"), Path::new("b")).is_ok());
    assert_eq!(calls.load(Ordering::SeqCst), 3, "erwartet gelungen im dritten");
}

#[cfg(windows)]
#[test]
fn rename_gibt_dauerhaften_fehler_nach_den_versuchen_zurueck() {
    let (calls, c) = zaehler();
    let mut fs = schnell();
    fs.rename = Box::new(move |_, _| {
        c.fetch_add(1, Ordering::SeqCst);
        Err(io::Error::from_raw_os_error(32))
    });
    let err = fs
        .rename_file(Path::new("a"), Path::new("b"))
        .expect_err("Fehler erwartet");
    assert_eq!(err.raw_os_error(), Some(32));
    assert_eq!(calls.load(Ordering::SeqCst), RENAME_TRIES);
}

#[test]
fn rename_andere_fehler_sofort() {
    let (calls, c) = zaehler();
    let mut fs = schnell();
    fs.rename = Box::new(move |_, _| {
        c.fetch_add(1, Ordering::SeqCst);
        Err(io::Error::from(io::ErrorKind::NotFound))
    });
    assert!(fs.rename_file(Path::new("a"), Path::new("b")).is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 1, "erwartet ein Versuch");
}

#[test]
fn rename_ersetzt_vorhandene_datei() {
    let dir = TempDir::new();
    let (a, b) = (dir.path().join("a"), dir.path().join("b"));
    std::fs::write(&a, "neu").expect("a");
    std::fs::write(&b, "alt").expect("b");
    let mut fs = schnell();
    fs.rename_file(&a, &b).expect("umbenennen");
    assert_eq!(std::fs::read_to_string(&b).expect("lesen"), "neu");
}

// ST1.2 (B-187/AC-01): Eine gesperrte Zieldatei lässt write_atomic wiederholen (⏳ im Log); bleibt sie gesperrt,
// meldet write_atomic den Fehler (❌), der alte Stand bleibt Byte für Byte und keine temporäre Datei liegt herum.
#[cfg(windows)]
#[test]
fn write_atomic_gesperrte_zieldatei() {
    let dir = TempDir::new();
    let path = dir.path().join("autosave.json");
    std::fs::write(&path, "alt").expect("alt");
    let locked = Arc::new(AtomicU32::new(0)); // so viele Versuche ist die Datei noch gesperrt
    let l = locked.clone();
    let mut fs = schnell();
    fs.rename = Box::new(move |a, b| {
        if l.load(Ordering::SeqCst) > 0 {
            l.fetch_sub(1, Ordering::SeqCst);
            return Err(io::Error::from_raw_os_error(32));
        }
        std::fs::rename(a, b)
    });

    locked.store(RENAME_TRIES, Ordering::SeqCst);
    let err = fs.write_atomic(&path, b"neu").expect_err("dauerhaft gesperrt");
    assert_eq!(err.raw_os_error(), Some(32));
    assert_eq!(
        std::fs::read_to_string(&path).expect("lesen"),
        "alt",
        "alter Stand verändert"
    );
    let tmp = std::fs::read_dir(dir.path())
        .expect("Ordner")
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().starts_with(".tmp-"))
        .count();
    assert_eq!(tmp, 0, "temporäre Dateien liegen herum");
    let log = fs.take_log().join("\n");
    assert!(log.contains('⏳') && log.contains('❌'), "Log ohne ⏳ und ❌:\n{log}");

    locked.store(2, Ordering::SeqCst);
    fs.write_atomic(&path, b"neu").expect("kurz gesperrt");
    assert_eq!(
        std::fs::read_to_string(&path).expect("lesen"),
        "neu",
        "neuer Stand fehlt"
    );
}

#[test]
fn write_atomic_legt_ordner_an_und_ersetzt() {
    let dir = TempDir::new();
    let path = dir.path().join("a").join("b.json");
    let mut fs = schnell();
    fs.write_atomic(&path, b"eins").expect("neu");
    fs.write_atomic(&path, b"zwei").expect("ersetzen");
    assert_eq!(std::fs::read(&path).expect("lesen"), b"zwei");
    assert_eq!(
        std::fs::read_dir(path.parent().expect("Ordner"))
            .expect("Ordner")
            .count(),
        1
    );
}

#[test]
fn read_retry_gibt_fehler_ohne_wiederholung_zurueck() {
    let (calls, c) = zaehler();
    let mut fs = schnell();
    fs.read = Box::new(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
        Err(io::Error::from(io::ErrorKind::InvalidInput))
    });
    assert!(fs.read_retry(Path::new("x")).is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
