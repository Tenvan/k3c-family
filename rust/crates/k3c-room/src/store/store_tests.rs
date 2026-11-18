//! Tests aus `store_test.go` (Spielstände, Berichte, Löschen, Purge) und die Spielstand-Dateien `testdata/saves`.
use std::path::{Path, PathBuf};
use std::time::Duration;

use super::reports::{MAX_REPORT_BYTES, Reports};
use super::saves::{MAX_SAVE_BYTES, Saves, parse_save};
use super::stamp::Stamp;
use super::testutil::{TempDir, clock};
use crate::fehler::Fehler;

fn saves(dir: &TempDir) -> Saves {
    let mut s = Saves::new(dir.path(), clock());
    s.fs.pause = Duration::ZERO;
    s
}

const T0: i64 = 1_800_000_000;

fn set_mtime(path: &Path, secs: i64) {
    let f = std::fs::OpenOptions::new().write(true).open(path).expect("Datei");
    let t = std::time::UNIX_EPOCH + Duration::from_secs(u64::try_from(secs).expect("positiv"));
    f.set_modified(t).expect("Zeit setzen");
}

#[test]
fn saves_rundlauf_und_pruefung() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    assert_eq!(s.load("autosave"), Err(Fehler::NotFound), "leer");
    assert_eq!(
        s.store("autosave", br#"{"version":1,"campaignId":"ci"}"#),
        Ok(String::new())
    );
    let data = s.load("autosave").expect("Load");
    assert!(String::from_utf8_lossy(&data).contains(r#""campaignId":"ci""#));
    for bad in [
        "kaputt",
        "null",
        "[]",
        r#"{"version":1}"#,
        r#"{"campaignId":"x","version":"1"}"#,
        r#"{"campaignId":1,"version":1}"#,
    ] {
        assert_eq!(s.store("autosave", bad.as_bytes()), Err(Fehler::Invalid), "{bad}");
    }
    let long = "a".repeat(33);
    for slot in ["", "Auto", "../x", "..\\x", "a/b", long.as_str()] {
        assert_eq!(
            s.store(slot, br#"{"version":1,"campaignId":"ci"}"#),
            Err(Fehler::Slot),
            "Slot {slot:?}"
        );
    }
    let big = format!(r#"{{"version":1,"campaignId":"{}"}}"#, "x".repeat(MAX_SAVE_BYTES));
    assert_eq!(s.store("autosave", big.as_bytes()), Err(Fehler::TooLarge));
}

// Wie server/saves.mjs: überschreibt ein Stand ein anderes Spiel, wird das alte vorher gesichert.
#[test]
fn saves_sichern_bei_anderem_spiel() {
    let dir = TempDir::new();
    let mut s = Saves::new(dir.path(), || Stamp::new(1_700_000_000, 0));
    let _ = s.store(
        "autosave",
        br#"{"version":1,"campaignId":"a","savedAt":"2026-09-30T12:00:00.000Z"}"#,
    );
    assert_eq!(
        s.store("autosave", br#"{"version":1,"campaignId":"a"}"#),
        Ok(String::new()),
        "gleiches Spiel gesichert"
    );
    let _ = s.store("autosave", br#"{"version":1,"campaignId":"a","savedAt":"../../evil"}"#);
    let b = s
        .store("autosave", br#"{"version":1,"campaignId":"b"}"#)
        .expect("Sicherung");
    assert_eq!(b, "autosave-----evil.json", "savedAt darf kein Pfad werden");
    assert!(dir.path().join(&b).exists());
    let _ = s.store("slot2", br#"{"version":1,"campaignId":"a"}"#);
    assert_eq!(
        s.store("slot2", br#"{"version":1,"campaignId":"b"}"#),
        Ok("slot2-1700000000000.json".to_string())
    );
}

// Ein unterbrochenes Schreiben (hier: Ziel ist ein Ordner) lässt keine halbe Datei und keinen Rest zurück.
#[test]
fn write_atomic_laesst_bei_fehler_nichts_zurueck() {
    let dir = TempDir::new();
    let target = dir.path().join("x.json");
    std::fs::create_dir(&target).expect("Ordner");
    let mut s = saves(&dir);
    assert!(s.fs.write_atomic(&target, b"{}").is_err(), "kein Fehler");
    assert_eq!(std::fs::read_dir(dir.path()).expect("Ordner").count(), 1, "Reste");
}

#[test]
fn reports() {
    let dir = TempDir::new();
    let mut r = Reports::new(dir.path(), || Stamp::from_civil(2026, 9, 30, 12, 0, 0, 0));
    assert_eq!(r.store(b"kaputt", "1.2.3.4"), Err(Fehler::Invalid));
    let a = r.store(br#"{"ci":true}"#, "1.2.3.4").expect("a");
    assert_eq!(
        a.file_name().and_then(|n| n.to_str()),
        Some("gamepad-2026-09-30T12-00-00-000Z.json")
    );
    let b = r.store(br#"{"ci":2}"#, "1.2.3.4").expect("b");
    assert_eq!(
        b.file_name().and_then(|n| n.to_str()),
        Some("gamepad-2026-09-30T12-00-00-000Z-1.json"),
        "zweiter in derselben ms"
    );
    let got: serde_json::Value = serde_json::from_slice(&std::fs::read(&a).expect("lesen")).expect("JSON");
    assert_eq!(got["ci"], true);
    assert_eq!(got["remoteAddress"], "1.2.3.4");
    assert!(!got["receivedAt"].is_null());
    let big = format!(r#"{{"x":"{}"}}"#, "x".repeat(MAX_REPORT_BYTES));
    assert_eq!(r.store(big.as_bytes(), ""), Err(Fehler::TooLarge));
}

/// Legt Spielstände an; das zweite Speichern erzeugt je eine Sicherung.
fn fill(s: &mut Saves, slots: &[&str]) {
    for slot in slots {
        for _ in 0..2 {
            s.store(slot, br#"{"version":1,"campaignId":"ci"}"#).expect("fill");
        }
    }
}

#[test]
fn saves_purge() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    fill(&mut s, &["test-alt", "test-neu", "familie"]);
    for (slot, secs) in [
        ("test-alt", T0 - 48 * 3600),
        ("familie", T0 - 48 * 3600),
        ("test-neu", T0),
    ] {
        set_mtime(&dir.path().join(format!("{slot}.json")), secs);
    }
    assert_eq!(s.purge("test-", Stamp::new(T0 - 24 * 3600, 0)), Ok(1));
    for (slot, want) in [("test-alt", false), ("test-neu", true), ("familie", true)] {
        assert_eq!(s.load(slot).is_ok(), want, "{slot} vorhanden");
    }
    assert!(!s.backup_dir("test-alt").exists(), "Sicherungen von test-alt bleiben");
    assert_eq!(s.purge("", Stamp::new(T0, 0)), Err(Fehler::Slot), "leeres Präfix");
}

#[test]
fn saves_delete() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    fill(&mut s, &["test-x", "familie"]);
    s.delete("test-x").expect("delete");
    assert_eq!(s.delete("test-x"), Ok(()), "zweites Löschen");
    assert_eq!(s.delete("../x"), Err(Fehler::Slot), "ungültiger Name");
    assert_eq!(s.load("test-x"), Err(Fehler::NotFound), "test-x noch da");
    assert!(s.load("familie").is_ok(), "familie weg");
    assert_eq!(s.count(), 1);
}

// AC-01: Alle Spielstand-Dateien unter testdata/saves (Version 1 bis 9) lassen sich lesen und werden vom Kopf-Test
// angenommen; die Liste nennt Version und Welt (ab Version 8).
#[test]
fn testdata_saves_v1_bis_v9_laden() {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../testdata/saves");
    for v in 1..=9_i64 {
        let s = Saves::new(root.join(format!("v{v}")), || Stamp::new(T0, 0));
        let data = s.load("familie").expect("laden");
        let h = parse_save(&data).expect("Kopf");
        assert_eq!((h.campaign_id.as_str(), h.version), ("familie", v as f64), "v{v}");
        let list = s.list();
        assert_eq!(list.len(), 1, "v{v}");
        assert_eq!((list[0].version, list[0].error.is_empty()), (v, true), "v{v}");
        if v >= 8 {
            assert_eq!(list[0].world, "standard", "v{v}");
        }
        assert_eq!(s.count(), 1);
    }
}
