//! Tests aus `restart_test.go`.
use std::time::{Duration, UNIX_EPOCH};

use super::restart::REMEMBERED_ROOMS_MAX_AGE;
use super::saves::Saves;
use super::stamp::Stamp;
use super::testutil::TempDir;
use crate::fehler::Fehler;

const T0: i64 = 1_800_000_000;

fn saves(dir: &TempDir) -> Saves {
    Saves::new(dir.path(), || Stamp::new(T0, 0))
}

fn setze_zeit(dir: &TempDir, secs_vor_jetzt: i64) {
    let f = std::fs::OpenOptions::new()
        .write(true)
        .open(dir.path().join("restart-rooms.state"))
        .expect("Datei");
    let secs = u64::try_from(T0 - secs_vor_jetzt).expect("positiv");
    f.set_modified(UNIX_EPOCH + Duration::from_secs(secs))
        .expect("Zeit setzen");
}

// WZ25.1 (B-508): Die Merkdatei ist atomar, wird beim Lesen gelöscht und gilt nur kurz.
#[test]
fn gemerkte_raeume() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    assert_eq!(s.take_remembered_rooms(), Err(Fehler::NotFound), "ohne Datei");
    let inhalt = br#"[{"code":"ABCD","name":"familie"}]"#;
    s.remember_rooms(inhalt).expect("merken");
    // "Merkdatei steht nicht in der Spielstandliste" (Go: `s.List()`) folgt mit `Saves::list` in RU21.2.
    setze_zeit(&dir, 1);
    assert_eq!(s.take_remembered_rooms().as_deref(), Ok(&inhalt[..]));
    assert!(
        !dir.path().join("restart-rooms.state").exists(),
        "Datei bleibt nach dem Lesen liegen"
    );
}

#[test]
fn alte_merkdatei_wird_verworfen() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    s.remember_rooms(b"[]").expect("merken");
    let alter = i64::try_from(2 * REMEMBERED_ROOMS_MAX_AGE / 1_000_000_000).expect("klein");
    setze_zeit(&dir, alter);
    assert_eq!(s.take_remembered_rooms(), Err(Fehler::NotFound), "alte Merkdatei");
    assert!(
        !dir.path().join("restart-rooms.state").exists(),
        "alte Datei bleibt liegen"
    );
}
