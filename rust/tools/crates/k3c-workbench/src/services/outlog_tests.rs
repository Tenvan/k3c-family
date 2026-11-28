//! Port von `outlog_test.go`.

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use chrono::Local;
use k3c_kit::applog::MAX_FILE_SIZE;
use serde_json::Value;

use super::options::Clock;
use super::outlog::{OutSink, out_level};
use super::testutil::{Captured, Fake, options, svc, wait_for};
use super::*;
use crate::testutil::tmp_dir;

#[test]
fn level_aus_dem_text() {
    for (text, want) in [
        ("Build ERROR in x", "ERROR"),
        ("Fehler beim Laden", "ERROR"),
        ("fatal: nope", "ERROR"),
        ("Warnung: warn", "WARN"),
        ("ready in 300 ms", "INFO"),
        ("", "INFO"),
    ] {
        assert_eq!(out_level(text), want, "{text}");
    }
}

pub(crate) fn read_out(root: &Path, name: &str) -> Vec<Value> {
    let path = root.join("logs").join(format!("{name}.jsonl"));
    let text = std::fs::read_to_string(path).unwrap_or_default();
    text.lines().map(|l| serde_json::from_str(l).unwrap()).collect()
}

/// Die Ausgabe eines Dienstes landet als JSON-Zeile in `logs/<name klein>.jsonl`, mit Strom und Level.
#[tokio::test(flavor = "multi_thread")]
async fn dienst_ausgabe_in_datei() {
    let root = tmp_dir();
    let log = Captured::default();
    let mut opts = options(&Fake::new(true));
    opts.root = root.path().to_path_buf();
    opts.log = log.logger();
    let c = Controller::new(vec![svc("Vite", false)], opts);
    c.start(&Cancel::default(), "Vite").await.unwrap();
    c.stop("Vite", false).await.unwrap();
    wait_for("Prozessende geloggt", || log.text().contains("prozess beendet")).await;
    let entries = read_out(root.path(), "vite");
    assert_eq!(entries.len(), 1, "{entries:?}");
    let entry = &entries[0];
    for (key, want) in [
        ("ns", "out"),
        ("level", "INFO"),
        ("msg", "hallo von Vite"),
        ("stream", "stdout"),
    ] {
        assert_eq!(entry[key], want, "{entry}");
    }
    assert!(chrono::DateTime::parse_from_rfc3339(entry["time"].as_str().unwrap()).is_ok());
    let text = log.text();
    for want in [
        "dienst Vite: start",
        "command=x",
        "port=1",
        "exit=0",
        "durch=k3c-workbench",
    ] {
        assert!(text.contains(want), "Log ohne {want}:\n{text}");
    }
}

/// Hat der Dienst ein eigenes Log, schreibt die Workbench keine Ausgabe-Datei.
#[tokio::test(flavor = "multi_thread")]
async fn eigenes_log_ohne_ausgabe_datei() {
    let root = tmp_dir();
    let mut opts = options(&Fake::new(true));
    opts.root = root.path().to_path_buf();
    let service = Service {
        log: "k3c-server".to_owned(),
        ..svc("Spielserver", false)
    };
    let c = Controller::new(vec![service], opts);
    c.start(&Cancel::default(), "Spielserver").await.unwrap();
    c.stop_all().await;
    assert!(!root.path().join("logs").exists());
}

#[test]
fn ansi_rotation_und_fehler() {
    let root = tmp_dir();
    let dir = root.path().join("logs");
    std::fs::create_dir_all(&dir).unwrap();
    let big = usize::try_from(MAX_FILE_SIZE).unwrap() + 1;
    std::fs::write(dir.join("x.jsonl"), vec![0; big]).unwrap();
    let now: Clock = Arc::new(Local::now);
    let sink = OutSink::open(root.path(), "X", &k3c_kit::logger::Logger::discard(), &now);
    sink.write("stderr", "\x1b[31mWARN\x1b[0m rot <a&b>");
    sink.close();
    let entries = read_out(root.path(), "x");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["msg"], "WARN rot <a&b>");
    assert_eq!(entries[0]["level"], "WARN");
    assert_eq!(entries[0]["stream"], "stderr");
    // Byte-Form wie Go: Felder nach Namen, <, > und & maskiert.
    let raw = std::fs::read_to_string(dir.join("x.jsonl")).unwrap();
    let msg = ["WARN rot ", "003ca", "0026b", "003e"].join("\\u");
    let head = format!(r#"{{"level":"WARN","msg":"{msg}","ns":"out","stream":"stderr","time":""#);
    assert!(raw.starts_with(&head), "{raw}");
    assert!(std::fs::metadata(dir.join("x.jsonl.1")).unwrap().len() > MAX_FILE_SIZE);
    // Ordner nicht anlegbar: nur eine Warnung, kein Absturz.
    let file = root.path().join("datei");
    std::fs::write(&file, "").unwrap();
    let log = Captured::default();
    let bad = OutSink::open(&file, "Y", &log.logger(), &now);
    bad.write("stdout", "egal");
    bad.close();
    assert!(log.text().contains("Ausgabe-Log nicht geöffnet"), "{}", log.text());
}

/// Wechsel der Gesundheitsprüfung im Lauf stehen mit Grund im Log.
#[tokio::test(flavor = "multi_thread")]
async fn gesundheitswechsel_geloggt() {
    let log = Captured::default();
    let f = Fake::new(true);
    let mut opts = options(&f);
    opts.log = log.logger();
    opts.fail_limit = 100_000;
    let c = Controller::new(vec![svc("Vite", false)], opts);
    c.start(&Cancel::default(), "Vite").await.unwrap();
    f.healthy.store(false, Ordering::SeqCst);
    wait_for("ungesund", || {
        let text = log.text();
        text.contains("ungesund") && text.contains("antwortet nicht")
    })
    .await;
    f.healthy.store(true, Ordering::SeqCst);
    wait_for("wieder gesund", || log.text().contains("wieder gesund")).await;
    c.stop_all().await;
}
