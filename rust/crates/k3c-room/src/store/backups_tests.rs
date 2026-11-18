//! Tests aus `backups_test.go`. Die Rotation wird zusätzlich direkt über `rotate` geprüft. `TestGleichzeitigesSpeichern`
//! entfällt: kein `Mutex` und keine Threads im Crate (RU27).
use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use super::backups::BACKUP_KEEP;
use super::saves::Saves;
use super::stamp::Stamp;
use super::testutil::{TempDir, clock, save};
use crate::fehler::Fehler;

fn saves(dir: &TempDir) -> Saves {
    let mut s = Saves::new(dir.path(), clock());
    s.fs.pause = Duration::ZERO;
    s
}

// B-028/AC-01: nach 7 Sicherungen liegen genau die 5 zuletzt abgelegten Stände vor (2 bis 6 von 1 bis 6).
#[test]
fn rotation_behaelt_fuenf_direkt() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    for i in 1..=7 {
        s.rotate("autosave", &save(i)).expect("rotate");
    }
    let list = s.backups("autosave").expect("Liste");
    assert_eq!(list.len(), BACKUP_KEEP);
    for (i, b) in list.iter().enumerate() {
        // neueste zuerst: 7, 6, 5, 4, 3
        let data = std::fs::read(s.backup_dir("autosave").join(&b.name)).expect("lesen");
        assert_eq!(data, save(7 - i64::try_from(i).expect("klein")), "Sicherung {i}");
        assert!(b.saved_at.is_some() && b.size > 0, "{b:?}");
    }
    let first = list[0].saved_at.expect("Zeit");
    assert_eq!(first, Stamp::from_civil(2026, 9, 30, 12, 0, 7, 0));
    assert_eq!(list[0].name, "20260930T120007.000000000Z.json");
}

// Slots mit gemeinsamem Präfix teilen keine Sicherungen.
#[test]
fn rotation_je_slot_direkt() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    for i in 1..=3 {
        s.rotate("auto", &save(i)).expect("auto");
        s.rotate("auto-save", &save(i)).expect("auto-save");
    }
    assert_eq!(s.backups("auto").expect("a").len(), 3);
    assert_eq!(s.backups("auto-save").expect("b").len(), 3);
}

#[test]
fn gleicher_zeitstempel_bekommt_zaehler() {
    let dir = TempDir::new();
    let mut s = Saves::new(dir.path(), || Stamp::from_civil(2026, 1, 1, 0, 0, 0, 0));
    s.rotate("x", b"1").expect("1");
    s.rotate("x", b"2").expect("2");
    let names: Vec<String> = s.backups("x").expect("Liste").into_iter().map(|b| b.name).collect();
    assert_eq!(
        names,
        ["20260101T000000.000000000Z.json", "20260101T000000.000000000Z-1.json"]
    );
}

#[test]
fn backups_ungueltiger_slot_und_fehlender_ordner() {
    let dir = TempDir::new();
    let s = saves(&dir);
    assert_eq!(s.backups("../x"), Err(Fehler::Slot));
    assert_eq!(s.backups(""), Err(Fehler::Slot));
    assert_eq!(s.backups("neu"), Ok(Vec::new()));
}

#[test]
fn backups_ignoriert_fremde_dateien() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    s.rotate("x", b"1").expect("rotate");
    let d = s.backup_dir("x");
    std::fs::write(d.join(".versteckt.json"), "x").expect("versteckt");
    std::fs::write(d.join("notiz.txt"), "x").expect("txt");
    std::fs::create_dir(d.join("ordner.json")).expect("Ordner");
    std::fs::write(d.join("ohne-zeit.json"), "x").expect("ohne Zeit");
    let list = s.backups("x").expect("Liste");
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].name, "ohne-zeit.json"); // absteigend nach Name: "o" kommt nach "2"
    assert_eq!(list.iter().filter(|b| b.saved_at.is_none()).count(), 1);
}

#[test]
fn restore_prueft_name_und_slot() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    s.rotate("autosave", &save(1)).expect("rotate");
    for bad in ["fehlt.json", "../autosave.json", "..\\autosave.json", ""] {
        assert_eq!(s.restore("autosave", bad), Err(Fehler::NotFound), "{bad:?}");
    }
    assert_eq!(s.restore("../x", "a.json"), Err(Fehler::Slot));
}

// B-028/AC-01: nach 7 Speichervorgängen liegen genau die 5 zuletzt überschriebenen Stände vor (2 bis 6).
#[test]
fn rotation_behaelt_fuenf() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    for i in 1..=7 {
        s.store("autosave", &save(i)).expect("store");
    }
    let list = s.backups("autosave").expect("Backups");
    assert_eq!(list.len(), BACKUP_KEEP);
    for (i, b) in list.iter().enumerate() {
        // neueste zuerst: 6, 5, 4, 3, 2
        let data = std::fs::read(s.backup_dir("autosave").join(&b.name)).expect("lesen");
        assert_eq!(
            data,
            save(6 - i64::try_from(i).expect("klein")),
            "Sicherung {i} ({b:?})"
        );
        assert!(b.saved_at.is_some() && b.size > 0, "{b:?}");
    }
    assert_eq!(s.load("autosave").expect("aktuell"), save(7));
    assert_eq!(s.count(), 1, "Sicherungen zählen nicht mit");
}

// Slots mit gemeinsamem Präfix teilen keine Sicherungen.
#[test]
fn rotation_je_slot() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    for i in 1..=3 {
        s.store("auto", &save(i)).expect("auto");
        s.store("auto-save", &save(i)).expect("auto-save");
    }
    assert_eq!(s.backups("auto").expect("a").len(), 2);
    assert_eq!(s.backups("auto-save").expect("b").len(), 2);
}

// B-028/AC-02: eine Sicherung wird der aktuelle Stand, der bisherige wird selbst gesichert.
#[test]
fn restore() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    for i in 1..=3 {
        s.store("autosave", &save(i)).expect("store");
    }
    let list = s.backups("autosave").expect("Liste");
    let oldest = list[list.len() - 1].name.clone(); // Stand 1
    s.restore("autosave", &oldest).expect("restore");
    assert_eq!(s.load("autosave").expect("load"), save(1), "nach Wiederherstellen");
    let after = s.backups("autosave").expect("Liste");
    let newest = std::fs::read(s.backup_dir("autosave").join(&after[0].name)).expect("lesen");
    assert_eq!(newest, save(3), "bisheriger Stand nicht gesichert");
    for bad in ["fehlt.json", "../autosave.json", "..\\autosave.json", ""] {
        assert_eq!(s.restore("autosave", bad), Err(Fehler::NotFound), "Restore({bad:?})");
    }
    assert_eq!(s.restore("../x", &oldest), Err(Fehler::Slot));
}

// B-274: Scheitert das Lesen des bisherigen Stands (Virenscanner hält die Datei), wird nie ohne Sicherung überschrieben:
// dauerhaft gesperrt -> Fehler und alter Stand bleibt; kurz gesperrt (Windows) -> wiederholt und gesichert.
#[test]
fn store_ueberschreibt_nie_ohne_sicherung() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    s.store("autosave", &save(1)).expect("1");
    s.fs.read = Box::new(|_| Err(io::Error::from(io::ErrorKind::InvalidInput)));
    assert!(
        s.store("autosave", &save(2)).is_err(),
        "Store ohne lesbaren bisherigen Stand gelungen"
    );
    s.fs.read = Box::new(|p| std::fs::read(p));
    assert_eq!(
        s.load("autosave").expect("load"),
        save(1),
        "bisheriger Stand überschrieben"
    );
    #[cfg(windows)]
    {
        let calls = Arc::new(AtomicU32::new(0));
        let c = calls.clone();
        s.fs.read = Box::new(move |p| {
            if c.fetch_add(1, Ordering::SeqCst) + 1 < 3 {
                return Err(io::Error::from_raw_os_error(32));
            }
            std::fs::read(p)
        });
        s.store("autosave", &save(2)).expect("kurz gesperrt");
        let list = s.backups("autosave").expect("Liste");
        let got = std::fs::read(s.backup_dir("autosave").join(&list[0].name)).expect("lesen");
        assert_eq!(got, save(1), "Sicherung");
    }
}
