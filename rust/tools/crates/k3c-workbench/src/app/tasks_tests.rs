//! Die fünf Commands der Seite Tasks über die Aufruf-Tabelle, mit gestelltem Katalog und Skript statt `task`.

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use serde_json::{Value, json};
use tokio::runtime::Handle;

use super::*;
use crate::app::events::Emit;
use crate::rpc::call;
use crate::services::testutil::wait_for;
use crate::testutil::tmp_dir;

type Seen = Arc<Mutex<Vec<(String, Value)>>>;

fn script() -> Vec<String> {
    let head: [&str; 2] = if cfg!(windows) { ["cmd", "/c"] } else { ["sh", "-c"] };
    head.iter().chain([&"echo hallo"]).map(|s| (*s).to_owned()).collect()
}

/// App, deren Task-Seite einen gestellten Katalog (mit `fail` ein Ladefehler) und ein Skript statt `task` nutzt.
async fn task_app(root: &Path, fail: &Arc<AtomicBool>) -> (Arc<App>, Seen) {
    let seen = Seen::default();
    let sink = Arc::clone(&seen);
    let forward: Emit = Arc::new(move |name: &str, data: Value| {
        sink.lock().unwrap().push((name.to_owned(), data));
    });
    let app = App::new(root.to_path_buf(), 0, Some(forward)).with_usage_file(root.join("usage.json"));
    let app = Arc::new(app);
    app.startup().await;
    let fail = Arc::clone(fail);
    let store = Arc::clone(&lock(&app.running).as_ref().unwrap().store);
    let command = script();
    let options = taskrun::Options {
        argv: Arc::new(move |_| command.clone()),
        ..runner_options(root, &store, &app.events)
    };
    let center = TaskCenter::new(Setup {
        runtime: Handle::current(),
        log: app.logger(),
        services: None,
        loader: Arc::new(move || {
            if fail.load(Ordering::SeqCst) {
                return Err("Taskfile kaputt".to_owned());
            }
            Ok(vec![Task {
                name: "check".to_owned(),
                namespace: taskcat::WORKSPACE.to_owned(),
                ..Task::default()
            }])
        }),
        runner: Runner::new(options),
    });
    lock(&app.running).as_mut().unwrap().tasks = Arc::new(center);
    (app, seen)
}

async fn rpc(app: &Arc<App>, method: &'static str, args: Value) -> Result<Value, String> {
    let app = Arc::clone(app);
    let args = args.as_array().cloned().unwrap_or_default();
    tokio::task::spawn_blocking(move || call(&app, method, &args).map_err(|e| e.0))
        .await
        .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn fuenf_commands() {
    let tmp = tmp_dir();
    let fail = Arc::new(AtomicBool::new(false));
    let (app, seen) = task_app(tmp.path(), &fail).await;
    assert_eq!(rpc(&app, "TaskRuns", json!([])).await.unwrap(), json!([]));
    let catalog = rpc(&app, "Tasks", json!([])).await.unwrap();
    assert_eq!(catalog["count"], 1);
    assert_eq!(catalog["namespaces"][0]["name"], "Workspace");
    assert_eq!(catalog["namespaces"][0]["tasks"][0]["name"], "check");
    assert_eq!(catalog["error"], "");
    let loaded = catalog["loadedAt"].as_str().unwrap().to_owned();
    assert!(chrono::DateTime::parse_from_rfc3339(&loaded).is_ok(), "{loaded}");
    // Nach einem Ladefehler bleibt der alte Baum.
    fail.store(true, Ordering::SeqCst);
    assert_eq!(rpc(&app, "Tasks", json!([])).await.unwrap()["error"], "");
    let catalog = rpc(&app, "TasksReload", json!([])).await.unwrap();
    assert_eq!(catalog["error"], "Taskfile kaputt");
    assert_eq!(
        (catalog["count"].as_u64(), catalog["loadedAt"].as_str()),
        (Some(1), Some(loaded.as_str()))
    );
    let error = rpc(&app, "TaskStart", json!(["nein", null])).await.unwrap_err();
    assert_eq!(
        error,
        "unbekannter Task \"nein\" (steht nicht im Katalog, task --list-all)"
    );
    let error = rpc(&app, "TaskStart", json!(["check", ["a&b"]])).await.unwrap_err();
    assert!(error.contains("nicht erlaubte Zeichen"), "{error}");
    let started = rpc(&app, "TaskStart", json!(["check", []])).await.unwrap();
    assert_eq!(started["state"], "running");
    assert_eq!(started["endedAt"], "0001-01-01T00:00:00Z");
    wait_for("Ende", || app.task_runs()[0].state.as_str() != "running").await;
    let runs = rpc(&app, "TaskRuns", json!([])).await.unwrap();
    assert_eq!(runs[0]["state"], "succeeded");
    let tail = rpc(&app, "ConsoleTail", json!(["task:check"])).await.unwrap();
    assert_eq!(tail[0]["text"], "hallo");
    let error = rpc(&app, "TaskStop", json!(["check"])).await.unwrap_err();
    assert_eq!(error, "task läuft nicht: check");
    wait_for("task:state", || {
        let events = seen.lock().unwrap();
        events.iter().filter(|(name, _)| name == "task:state").count() == 2
    })
    .await;
    app.shutdown().await;
    assert_eq!(rpc(&app, "TaskRuns", json!([])).await.unwrap(), json!([]));
    assert_eq!(rpc(&app, "Tasks", json!([])).await.unwrap()["error"], "backend beendet");
    assert_eq!(rpc(&app, "Sources", json!([])).await.unwrap(), json!([]));
}
