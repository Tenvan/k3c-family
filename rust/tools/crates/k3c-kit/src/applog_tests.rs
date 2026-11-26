//! Port von `workkit/applog/applog_test.go`. Die Umgebungsvariable setzen die Tests nicht (`set_var` ist in Edition
//! 2024 `unsafe` und liefe quer zu parallelen Tests); sie prüfen `levels_from` und `open_with` mit festen Stufen.

use std::path::Path;

use serde_json::{Map, Value, json};

use super::*;
use crate::testutil::tmp_dir;

/// Liest die JSON-Zeilen einer Log-Datei.
fn entries(path: &Path) -> Vec<Map<String, Value>> {
    let text = std::fs::read_to_string(path).unwrap();
    text.lines()
        .map(|l| serde_json::from_str(l).unwrap_or_else(|e| panic!("keine JSON-Zeile: {l:?} ({e})")))
        .collect()
}

#[test]
fn schreibt_json_und_spiegelt() {
    let tmp = tmp_dir();
    let dir = tmp.path().join("logs");
    let store = Arc::new(Store::new(0, None));
    let (l, err) = open(&dir, Arc::clone(&store));
    assert!(err.is_none(), "{err:?}");
    l.logger()
        .with(&[("ns", json!("check"))])
        .warn("lauf beendet", &[("target", json!("task:test")), ("exit", json!(1))]);
    l.close().unwrap();
    let es = entries(&dir.join("k3c-workbench.jsonl"));
    assert_eq!(es.len(), 1, "Einträge: {es:?}");
    let e = &es[0];
    assert!(
        e["level"] == "WARN"
            && e["ns"] == "check"
            && e["msg"] == "lauf beendet"
            && e["target"] == "task:test"
            && e["exit"] == 1,
        "Eintrag: {e:?}"
    );
    let lines = store.tail(SOURCE, 0).unwrap_or_default();
    assert!(
        lines.len() == 1
            && lines[0].stream == "log"
            && lines[0].text.contains("WARN")
            && lines[0].text.contains("[check]"),
        "Spiegel: {lines:?}"
    );
}

#[test]
fn ordner_nicht_beschreibbar() {
    let tmp = tmp_dir();
    let file = tmp.path().join("keinordner");
    std::fs::write(&file, b"").unwrap();
    let store = Arc::new(Store::new(0, None));
    let (l, err) = open(&file.join("logs"), Arc::clone(&store));
    assert!(err.is_some(), "kein Fehler bei nicht anlegbarem Ordner");
    l.logger().info("geht trotzdem", &[]);
    let lines = store.tail(SOURCE, 0).unwrap_or_default();
    assert_eq!(lines.len(), 1, "Spiegel ohne Datei: {lines:?}");
    l.close().unwrap();
}

#[test]
fn parse_level_liest_die_stufen() {
    let cases = [
        ("debug", Some(Level::Debug)),
        ("INFO", Some(Level::Info)),
        (" warn ", Some(Level::Warn)),
        ("error", Some(Level::Error)),
        ("", None),
        ("laut", None),
    ];
    for (input, want) in cases {
        assert_eq!(parse_level(input), want, "parse_level({input:?})");
    }
}

#[test]
fn levels_aus_umgebung() {
    assert_eq!(levels_from(Some("")), (Level::Debug, Level::Info), "Standard");
    assert_eq!(levels_from(None), (Level::Debug, Level::Info), "ohne Variable");
    assert_eq!(levels_from(Some("warn")), (Level::Warn, Level::Warn), "warn");
}

#[test]
fn level_filtert_datei() {
    let tmp = tmp_dir();
    let (l, _) = open_with(
        tmp.path(),
        SOURCE,
        Arc::new(Store::new(0, None)),
        levels_from(Some("warn")),
    );
    l.logger().info("leise", &[]);
    l.logger().warn("laut", &[]);
    let _ = l.close();
    let es = entries(&tmp.path().join("k3c-workbench.jsonl"));
    assert!(es.len() == 1 && es[0]["msg"] == "laut", "Einträge: {es:?}");
}

#[test]
fn rotate_benennt_grosse_dateien_um() {
    let tmp = tmp_dir();
    let path = tmp.path().join("a.jsonl");
    let one = tmp.path().join("a.jsonl.1");
    for (name, size, rotated) in [("klein", 5usize, false), ("gross", 50, true)] {
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(&one);
        std::fs::write(&one, b"alt").unwrap();
        std::fs::write(&path, vec![0u8; size]).unwrap();
        rotate(&path, 10).unwrap();
        let new_gone = !path.exists();
        let one_len = std::fs::read(&one).map(|b| b.len()).unwrap_or(0);
        assert!(
            rotated == new_gone && rotated == (one_len == size),
            "{name}: neu weg={new_gone}, .1={one_len} Bytes"
        );
    }
    assert!(rotate(&tmp.path().join("fehlt.jsonl"), 10).is_ok(), "fehlende Datei");
}

#[test]
fn json_zeile_wie_slog() {
    let t = chrono::Local::now();
    let r = Record {
        time: t,
        level: Level::Info,
        msg: "✅ a\"b\n".into(),
        attrs: vec![
            ("ms".into(), json!(74.092)),
            ("n".into(), json!(3)),
            ("big".into(), json!(1e21)),
            ("ok".into(), json!(true)),
            (String::new(), Value::Null),
            ("html".into(), json!("<&>\u{1}\u{2028}")),
        ],
    };
    let want = format!(
        "{{\"time\":\"{}\",\"level\":\"INFO\",\"msg\":\"✅ a\\\"b\\n\",\"ms\":74.092,\"n\":3,\"big\":1e+21,\"ok\":true,\
         \"html\":\"<&>\\u0001\\u2028\"}}\n",
        time_nanos(&t)
    );
    assert_eq!(json_line(&r), want);
    assert_eq!(go_json_float(1e-7), "1e-7");
    assert_eq!(go_json_float(2.0), "2");
}
