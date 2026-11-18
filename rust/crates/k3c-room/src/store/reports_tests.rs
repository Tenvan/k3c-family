//! Tests aus `reports_test.go`.
use std::path::Path;

use super::reports::{MAX_REPORTS, Reports};
use super::stamp::Stamp;
use super::testutil::TempDir;
use crate::fehler::Fehler;

fn start() -> Stamp {
    Stamp::from_civil(2026, 1, 1, 0, 0, 0, 0)
}

/// Dateiname `<prefix><start + i Minuten>.json` wie in Go (`2006-01-02T15-04-05-000Z`).
fn name(prefix: &str, i: usize) -> String {
    let t = start().add_secs(i64::try_from(i).expect("klein") * 60);
    format!("{prefix}{}.json", t.format_report().replace([':', '.'], "-"))
}

fn exists(dir: &Path, name: &str) -> bool {
    dir.join(name).exists()
}

fn glob_count(dir: &Path, prefix: &str) -> usize {
    std::fs::read_dir(dir)
        .expect("Ordner")
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().starts_with(prefix))
        .count()
}

// B-142/AC-01, AC-02: Bei Obergrenze + 20 Berichten löscht store die ältesten (20 plus Platz für den neuen), saves/
// daneben bleibt Byte für Byte gleich.
#[test]
fn reports_rotieren_aelteste_zuerst() {
    let root = TempDir::new();
    let dir = root.path().join("reports");
    let saves = root.path().join("saves");
    std::fs::create_dir_all(&dir).expect("reports");
    std::fs::create_dir_all(saves.join("backups").join("autosave")).expect("saves");
    let save_files = [
        (saves.join("autosave.json"), r#"{"version":3}"#),
        (
            saves.join("backups").join("autosave").join("gamepad-1.json"),
            r#"{"alt":true}"#,
        ),
    ];
    for (path, data) in &save_files {
        std::fs::write(path, data).expect("schreiben");
    }
    for i in 0..MAX_REPORTS + 20 {
        std::fs::write(dir.join(name("gamepad-", i)), format!(r#"{{"i":{i}}}"#)).expect("Bericht");
    }

    let mut r = Reports::new(&dir, || start().add_secs(1000 * 60));
    r.store(br#"{"neu":true}"#, "1.2.3.4").expect("store");

    assert_eq!(r.count(), MAX_REPORTS);
    for i in 0..MAX_REPORTS + 20 {
        assert_eq!(!exists(&dir, &name("gamepad-", i)), i <= 20, "Bericht {i} gelöscht");
    }
    for (path, want) in &save_files {
        assert_eq!(
            std::fs::read_to_string(path).expect("lesen"),
            *want,
            "{path:?} verändert"
        );
    }
    assert!(r.take_log().iter().any(|l| l.starts_with("🧹")));
}

// S2.3 (B-150): store_session legt session-<zeit>.json an, überschreibt keinen Report derselben Millisekunde und
// lehnt Nicht-Objekte ab.
#[test]
fn store_session_legt_neue_datei_an() {
    let dir = TempDir::new();
    let mut r = Reports::new(dir.path(), || Stamp::from_civil(2026, 10, 4, 20, 0, 0, 0));
    let a = r.store_session(br#"{"schema":1}"#).expect("a");
    let b = r.store_session(br#"{"schema":1,"b":true}"#).expect("b");
    assert_eq!(
        a.file_name().and_then(|n| n.to_str()),
        Some("session-2026-10-04T20-00-00-000Z.json")
    );
    assert_eq!(
        b.file_name().and_then(|n| n.to_str()),
        Some("session-2026-10-04T20-00-00-000Z-1.json")
    );
    assert_eq!(
        std::fs::read_to_string(&a).expect("lesen"),
        r#"{"schema":1}"#,
        "erster Report überschrieben"
    );
    assert_eq!(r.store_session(b"[1]"), Err(Fehler::Invalid), "Liste angenommen");
    assert_eq!(r.store_session(b"null"), Err(Fehler::Invalid));
    assert_eq!(r.store_session(b"kaputt"), Err(Fehler::Invalid));
}

// ST1.2 (B-272/AC-01): store_session begrenzt session-*.json auf MAX_REPORTS, die neuesten bleiben; Gamepad-Berichte
// daneben bleiben unberührt, und store verdrängt umgekehrt keine Sessions.
#[test]
fn session_reports_rotieren_ohne_gamepad_berichte() {
    let dir = TempDir::new();
    for i in 0..MAX_REPORTS + 5 {
        std::fs::write(dir.path().join(name("session-", i)), "{}").expect("Session");
    }
    for i in 0..3 {
        std::fs::write(dir.path().join(name("gamepad-", i)), "{}").expect("Gamepad");
    }
    let mut r = Reports::new(dir.path(), || start().add_secs(1000 * 60));
    r.store_session(br#"{"schema":1}"#).expect("store_session");
    assert_eq!(glob_count(dir.path(), "session-"), MAX_REPORTS);
    for i in 0..MAX_REPORTS + 5 {
        assert_eq!(
            !exists(dir.path(), &name("session-", i)),
            i <= 5,
            "Session {i} gelöscht"
        );
    }
    assert_eq!(r.count(), 3, "Gamepad-Berichte");
    r.store(b"{}", "1.2.3.4").expect("store");
    assert_eq!(
        glob_count(dir.path(), "session-"),
        MAX_REPORTS,
        "store hat Sessions verdrängt"
    );
}

// Ergänzung zu Go (Nachbehandlung der Ausgabe): receivedAt, remoteAddress, Einrückung, sortierte Schlüssel und das
// HTML-Escape von Go (`<`, `>`, `&`, U+2028, U+2029) im Bericht.
#[test]
fn bericht_schreibt_wie_go() {
    let dir = TempDir::new();
    let mut r = Reports::new(dir.path(), || Stamp::from_civil(2024, 2, 29, 23, 59, 59, 123_456_789));
    let path = r
        .store("{\"z\":1,\"a\":\"<script>&\u{2028}\"}".as_bytes(), "::1")
        .expect("store");
    assert_eq!(
        path.file_name().and_then(|n| n.to_str()),
        Some("gamepad-2024-02-29T23-59-59-123Z.json")
    );
    let text = std::fs::read_to_string(&path).expect("lesen");
    let want = "{\n  \"a\": \"\\u003cscript\\u003e\\u0026\\u2028\",\n  \"receivedAt\": \"2024-02-29T23:59:59.123Z\",\n  \"remoteAddress\": \"::1\",\n  \"z\": 1\n}";
    assert_eq!(text, want);
    assert_eq!(r.store(&vec![b' '; 256 * 1024 + 1], "x"), Err(Fehler::TooLarge));
    assert_eq!(r.store(b"[]", "x"), Err(Fehler::Invalid));
}
