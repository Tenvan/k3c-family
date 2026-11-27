//! Port von `logs_test.go`.

use std::io::Write;
use std::path::{Path, PathBuf};

use regex::Regex;

use super::digest::FINGERPRINT_MAX;
use super::reader::scan_blocks;
use super::*;
use crate::testutil::tmp_dir;

/// Sieben gültige Einträge (09:00 bis 09:06) und eine kaputte Zeile.
fn sample() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/logs/k3c-workbench.jsonl")
}

fn msgs(entries: &[Entry]) -> String {
    entries.iter().map(|e| e.msg.as_str()).collect::<Vec<_>>().join("|")
}

/// `TestScanRueckwaertsUeberBlockgrenzen`.
#[test]
fn scan_rueckwaerts_ueber_blockgrenzen() {
    for size in [7, 64, 1 << 20] {
        let res = scan_blocks(&sample(), &Query::default(), size).unwrap();
        assert_eq!(
            (res.entries.len(), res.skipped, res.budget_hit),
            (7, 1, false),
            "{size}"
        );
        let first = &res.entries[0];
        assert_eq!(first.msg, "server gestoppt", "Block {size}");
        assert_eq!(first.time.format("%M").to_string(), "06", "Block {size}");
    }
}

/// `TestScanFilter`.
#[test]
fn scan_filter() {
    let x = "check_run: Ziel \"task:x\" unbekannt";
    let y = "check_run: Ziel \"go:y\" unbekannt";
    let cases = [
        (
            Query {
                min_level: "WARN".into(),
                ..Query::default()
            },
            format!("server gestoppt|{x}|{y}"),
        ),
        (
            Query {
                ns: "check".into(),
                ..Query::default()
            },
            "lauf beendet".to_owned(),
        ),
        (
            Query {
                pattern: Some(Regex::new("^check_run").unwrap()),
                ..Query::default()
            },
            format!("{x}|{y}"),
        ),
        (
            Query {
                limit: 2,
                ..Query::default()
            },
            format!("server gestoppt|{x}"),
        ),
        (
            Query {
                since: parse_rfc3339("2026-09-30T09:05:00Z"),
                ..Query::default()
            },
            format!("server gestoppt|{x}"),
        ),
    ];
    for (query, want) in cases {
        let res = scan(&sample(), &query).unwrap();
        assert_eq!(msgs(&res.entries), want, "{query:?}");
    }
}

/// `TestScanBudgetUndFehlendeDatei`.
#[test]
fn scan_budget_und_fehlende_datei() {
    let query = Query {
        budget: 64,
        ..Query::default()
    };
    let res = scan_blocks(&sample(), &query, 64).unwrap();
    assert!(
        res.budget_hit && res.bytes_read == 64 && res.entries.len() != 7,
        "{res:?}"
    );
    let tmp = tmp_dir();
    let res = scan(&tmp.path().join("fehlt.jsonl"), &Query::default()).unwrap();
    assert!(res.entries.is_empty(), "{res:?}");
}

/// `TestReadSinceMitCursor`.
#[test]
fn read_since_mit_cursor() {
    let tmp = tmp_dir();
    let path = tmp.path().join("a.jsonl");
    let line = |i: u32| format!("{{\"time\":\"2026-09-30T10:00:{i:02}Z\",\"level\":\"INFO\",\"msg\":\"m{i}\"}}\n");
    let partial = r#"{"time":"2026-09-30T10:00:03Z","lev"#;
    std::fs::write(&path, format!("{}{}{partial}", line(1), line(2))).unwrap();
    let res = read_since(&path, 0, 0, 0).unwrap();
    assert_eq!((msgs(&res.entries).as_str(), res.truncated), ("m1|m2", false));
    let mut file = std::fs::OpenOptions::new().append(true).open(&path).unwrap();
    let rest = r#"el":"INFO","msg":"m3"}"#;
    write!(file, "{rest}\n{}", line(4)).unwrap();
    drop(file);
    let res = read_since(&path, res.cursor, 1, 0).unwrap();
    assert_eq!((msgs(&res.entries).as_str(), res.truncated), ("m4", true), "Limit");
    std::fs::write(&path, line(9)).unwrap();
    let res = read_since(&path, res.cursor, 0, 0).unwrap();
    assert_eq!(
        (msgs(&res.entries).as_str(), res.truncated),
        ("m9", true),
        "geschrumpft"
    );
    let res = read_since(&path, 0, 0, 10).unwrap();
    assert_eq!((res.entries.len(), res.truncated, res.cursor), (0, true, 0), "Budget");
}

/// `TestFingerprintReihenfolge`.
#[test]
fn fingerprint_reihenfolge() {
    let long = "x".repeat(300);
    let cut = "x".repeat(FINGERPRINT_MAX);
    let cases = [
        (
            "Anfrage 3f2a9c1e-0dfd-28b2-eb78-0c47ed6f37b9 nach 12837 ms",
            "Anfrage <id> nach <n> ms",
        ),
        (r"Datei C:\WORKSPACE\k3c\saves\a1.json fehlt", "Datei <path> fehlt"),
        (r#"Ziel "task:x" unbekannt"#, "Ziel <s> unbekannt"),
        ("Versuch 4 von 5, 2,5 s", "Versuch <n> von <n>, <n> s"),
        (long.as_str(), cut.as_str()),
    ];
    for (input, want) in cases {
        assert_eq!(fingerprint(input), want, "{input}");
    }
}

/// `TestDigestGruppen`.
#[test]
fn digest_gruppen() {
    let query = Query {
        min_level: "WARN".into(),
        ..Query::default()
    };
    let groups = digest(&scan(&sample(), &query).unwrap().entries);
    assert_eq!(groups.len(), 2, "{groups:?}");
    let json = serde_json::to_value(&groups[0]).unwrap();
    let want = serde_json::json!({
        "ns": "mcp", "fingerprint": "check_run: Ziel <s> unbekannt", "level": "ERROR", "count": 2,
        "first": "2026-09-30T09:04:00Z", "last": "2026-09-30T09:05:00Z",
        "example": "check_run: Ziel \"task:x\" unbekannt"
    });
    assert_eq!(json, want);
    assert_eq!((level_rank("TRACE"), level_rank("warn")), (-1, 2));
}

/// `TestScanLiestZeileDesGoServers` (B-066): eine Zeile, wie sie `slog.NewJSONHandler` schreibt.
#[test]
fn scan_liest_zeile_von_slog() {
    let line = r#"{"time":"2026-10-01T16:01:11.453+02:00","level":"ERROR","msg":"Spielstand nicht gespeichert","ns":"room","room":"FAMILIE","err":"disk voll"}"#;
    let tmp = tmp_dir();
    let path = tmp.path().join("k3c-server.jsonl");
    std::fs::write(&path, format!("{line}\n")).unwrap();
    let res = scan(&path, &Query::default()).unwrap();
    assert_eq!(res.entries.len(), 1, "{res:?}");
    let entry = &res.entries[0];
    assert_eq!(
        (entry.level.as_str(), entry.ns.as_str(), entry.msg.as_str()),
        ("ERROR", "room", "Spielstand nicht gespeichert")
    );
    assert_eq!(entry.data.as_ref().unwrap()["room"], "FAMILIE");
    let json = serde_json::to_value(entry).unwrap();
    assert_eq!(json["time"], "2026-10-01T16:01:11.453+02:00");
    assert_eq!(json["checkout"], "");
}
