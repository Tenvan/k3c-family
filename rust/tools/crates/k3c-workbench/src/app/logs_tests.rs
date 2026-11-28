//! Port von `app_logs_test.go` (ohne die Log-Abfragen, RU69.5).

use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::{Local, TimeZone};
use serde_json::{Value, json};

use super::*;
use crate::app::events::{CONSOLE_LINE, Emit};
use crate::ports::LOG_NAME;
use crate::rpc::call;
use crate::services::State;
use crate::testutil::tmp_dir;

/// App auf einem leeren Repo (ohne `services.json`) mit einem leeren Log `server`; zählt die `console:line`.
async fn started_app(root: &Path) -> (Arc<App>, Arc<AtomicU32>) {
    std::fs::create_dir_all(root.join("logs")).unwrap();
    std::fs::write(root.join("logs").join("server.jsonl"), "").unwrap();
    let lines = Arc::new(AtomicU32::new(0));
    let seen = Arc::clone(&lines);
    let forward: Emit = Arc::new(move |name: &str, data: Value| {
        if name == CONSOLE_LINE && data.as_array().is_some_and(|l| !l.is_empty()) {
            seen.fetch_add(1, Ordering::SeqCst);
        }
    });
    let app = App::new(root.to_path_buf(), 0, Some(forward)).with_usage_file(root.join("usage.json"));
    let app = Arc::new(app);
    app.startup().await;
    (app, lines)
}

async fn rpc(app: &Arc<App>, method: &'static str, args: Value) -> Result<Value, String> {
    let app = Arc::clone(app);
    let args = args.as_array().cloned().unwrap_or_default();
    tokio::task::spawn_blocking(move || call(&app, method, &args).map_err(|e| e.0))
        .await
        .unwrap()
}

/// `TestSourcesUndConsoleTail`.
#[tokio::test(flavor = "multi_thread")]
async fn sources_und_console_tail() {
    let tmp = tmp_dir();
    let (app, _) = started_app(tmp.path()).await;
    let sources = rpc(&app, "Sources", json!([])).await.unwrap();
    let find = |name: &str| {
        let list = sources.as_array().unwrap();
        list.iter().find(|s| s["name"] == name).cloned().unwrap()
    };
    let own = find(LOG_NAME);
    assert_eq!(
        (own["kind"].as_str(), own["state"].as_str()),
        (Some("log"), Some("entries"))
    );
    let empty = find("server");
    assert_eq!(
        (empty["kind"].as_str(), empty["state"].as_str()),
        (Some("log"), Some("empty"))
    );
    assert_eq!(empty["detail"], "noch keine Einträge");
    assert_eq!(sources.as_array().unwrap().len(), 2, "{sources}");
    let own = rpc(&app, "ConsoleTail", json!([LOG_NAME])).await.unwrap();
    let started = own.as_array().unwrap().iter().any(|l| {
        l["text"]
            .as_str()
            .unwrap_or_default()
            .contains("k3c-workbench gestartet")
    });
    assert!(started, "{own}");
    assert_eq!(rpc(&app, "ConsoleTail", json!(["server"])).await.unwrap(), json!([]));
    let error = rpc(&app, "ConsoleTail", json!(["../geheim"])).await.unwrap_err();
    assert!(error.contains(&format!("gültig: {LOG_NAME}, server")), "{error}");
    app.shutdown().await;
}

/// `TestConsoleLineEreignis`: der Spiegel des eigenen Logs kommt als `console:line` an die Oberfläche.
#[tokio::test(flavor = "multi_thread")]
async fn console_line_ereignis() {
    let tmp = tmp_dir();
    let (app, lines) = started_app(tmp.path()).await;
    for _ in 0..400 {
        if lines.load(Ordering::SeqCst) > 0 {
            break;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    assert!(lines.load(Ordering::SeqCst) > 0);
    app.shutdown().await;
}

/// Ein Zustandswechsel eines Dienstes meldet auch seine Quelle (`source:state`). Der Dienst aus der eigenen
/// `services.json` hat keinen Port und ein Programm, das es nicht gibt: es startet kein Prozess.
#[tokio::test(flavor = "multi_thread")]
async fn dienstwechsel_meldet_die_quelle() {
    let tmp = tmp_dir();
    let dir = tmp.path().join("tools").join("k3c-workbench");
    std::fs::create_dir_all(&dir).unwrap();
    let services = r#"[{"name":"Spiel","command":["k3c-gibt-es-nicht"],"health":"process","manual":true}]"#;
    std::fs::write(dir.join("services.json"), services).unwrap();
    let seen = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&seen);
    let forward: Emit = Arc::new(move |name: &str, data: Value| {
        if name == "source:state" {
            sink.lock().unwrap().push(data);
        }
    });
    let app = App::new(tmp.path().to_path_buf(), 0, Some(forward)).with_usage_file(tmp.path().join("usage.json"));
    let app = Arc::new(app);
    app.startup().await;
    assert!(rpc(&app, "ServiceStart", json!(["Spiel"])).await.is_err());
    let last = seen.lock().unwrap().last().cloned().unwrap();
    assert_eq!(
        (&last["name"], &last["kind"], &last["state"]),
        (&json!("Spiel"), &json!("service"), &json!("fehlgeschlagen"))
    );
    app.shutdown().await;
}

fn check_state() -> CheckState {
    CheckState {
        name: "task:test".to_owned(),
        running: false,
        at: Local.with_ymd_and_hms(2026, 9, 30, 8, 3, 15).unwrap(),
        ms: 0.0,
        exit: 0,
        timed_out: false,
        error: String::new(),
    }
}

/// Die Fälle aus `TestSourceZustaende`: Lauf, erwarteter Zustand und Zusatz.
fn check_cases() -> Vec<(CheckState, &'static str, &'static str)> {
    let state = check_state();
    vec![
        (
            CheckState {
                running: true,
                ..state.clone()
            },
            "running",
            "läuft seit 08:03:15",
        ),
        (
            CheckState {
                ms: 12400.0,
                ..state.clone()
            },
            "ok",
            "Exit 0 · 12,4 s · 08:03:15",
        ),
        (
            CheckState {
                exit: 1,
                ms: 800.0,
                ..state.clone()
            },
            "failed",
            "Exit 1 · 0,8 s · 08:03:15",
        ),
        (
            CheckState {
                timed_out: true,
                ms: 300_000.0,
                ..state.clone()
            },
            "timeout",
            "Zeitlimit nach 300,0 s · 08:03:15",
        ),
        (
            CheckState {
                error: "exec: task nicht gefunden".to_owned(),
                ..state
            },
            "failed",
            "nicht gestartet: exec: task nicht gefunden",
        ),
    ]
}

/// `TestSourceZustaende`.
#[test]
fn source_zustaende() {
    for (st, want_state, want_detail) in check_cases() {
        let want = Source::new("check:task:test".to_owned(), "run", want_state, want_detail.to_owned());
        assert_eq!(check_source(&st), want);
    }
    let service = Status {
        name: "Vite".to_owned(),
        port: 5173,
        pid: 41232,
        state: State::Running,
        ..Status::default()
    };
    let want = Source::new(
        "Vite".to_owned(),
        "service",
        "läuft",
        "Port 5173 · PID 41232".to_owned(),
    );
    assert_eq!(service_source(&service), want);
}
