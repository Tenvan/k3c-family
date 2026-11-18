//! Tests aus `discard_test.go` (BD5.1, B-506/B-533).
use std::time::Duration;

use super::saves::Saves;
use super::stamp::Stamp;
use super::testutil::TempDir;
use crate::fehler::Fehler;

// Discard nimmt den Stand aus der Liste, legt ihn als Sicherung ab und lässt ältere Sicherungen stehen.
#[test]
fn saves_discard_behaelt_sicherungen() {
    let dir = TempDir::new();
    let mut t = Stamp::from_civil(2026, 10, 22, 12, 0, 0, 0);
    let mut s = Saves::new(dir.path(), move || {
        t = t.add_secs(1);
        t
    });
    s.fs.pause = Duration::ZERO;
    for day in ["1", "2"] {
        let data = format!(r#"{{"version":8,"campaignId":"nord","day":{day}}}"#);
        s.store("nord", data.as_bytes()).expect("store");
    }
    s.discard("nord").expect("discard");
    assert!(!dir.path().join("nord.json").exists(), "Stand liegt noch");
    assert!(s.list().is_empty(), "Liste: {:?}", s.list());
    let list = s.backups("nord").expect("Sicherungen");
    assert_eq!(list.len(), 2);
    // Der gelöschte Stand ist die neueste Sicherung und lässt sich wiederherstellen.
    s.restore("nord", &list[0].name).expect("restore");
    assert_eq!(
        s.load("nord").expect("load"),
        br#"{"version":8,"campaignId":"nord","day":2}"#
    );
    assert_eq!(s.discard("fehlt"), Err(Fehler::NotFound), "fehlender Stand");
    assert_eq!(s.discard("../x"), Err(Fehler::Slot), "ungültiger Name");
}

// B-506: `list` nennt die Welt des Stands (Version 8), ältere Stände ohne Feld bleiben ohne world.
#[test]
fn saves_list_nennt_welt() {
    let dir = TempDir::new();
    let mut s = Saves::new(dir.path(), || Stamp::new(0, 0));
    s.fs.pause = Duration::ZERO;
    for (name, data) in [
        (
            "nord",
            r#"{"version":8,"campaignId":"nord","savedAt":"2026-10-22T10:00:00Z","world":"nordmeer"}"#,
        ),
        (
            "alt",
            r#"{"version":4,"campaignId":"alt","savedAt":"2026-10-01T10:00:00Z"}"#,
        ),
    ] {
        s.store(name, data.as_bytes()).expect("store");
    }
    let got = s.list();
    assert_eq!(got.len(), 2);
    assert_eq!((got[0].world.as_str(), got[1].world.as_str()), ("", "nordmeer"));
}
