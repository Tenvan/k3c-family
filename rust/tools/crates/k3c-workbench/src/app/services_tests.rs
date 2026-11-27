//! Port von `app_services_test.go`, dazu der Weg vom Start bis zum Beenden mit einer eigenen `services.json`.

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};

use super::*;
use crate::app::events::{CONSOLE_LINE, Emit};
use crate::rpc::call;
use crate::services::{Options, Service, Spawned, State};
use crate::testutil::tmp_dir;

type Seen = Arc<Mutex<Vec<(String, Value)>>>;

async fn start_app(root: &Path) -> (Arc<App>, Seen) {
    let seen = Seen::default();
    let sink = Arc::clone(&seen);
    let forward: Emit = Arc::new(move |name: &str, data: Value| sink.lock().unwrap().push((name.to_owned(), data)));
    let app = App::new(root.to_path_buf(), 0, Some(forward)).with_usage_file(root.join("usage.json"));
    let app = Arc::new(app);
    app.startup().await;
    (app, seen)
}

/// Ein Aufruf wie aus der Oberfläche (blockierender Thread).
async fn ui<T: Send + 'static>(app: &Arc<App>, f: impl FnOnce(&Arc<App>) -> T + Send + 'static) -> T {
    let app = Arc::clone(app);
    tokio::task::spawn_blocking(move || f(&app)).await.unwrap()
}

fn fake_spawn() -> Spawned {
    let done = Cancel::default();
    let kill = done.clone();
    Spawned {
        pid: 42,
        kill: Box::new(move || {
            kill.cancel();
            Ok(())
        }),
        wait: Box::pin(async move {
            done.cancelled().await;
            Ok(())
        }),
    }
}

/// App mit gestelltem Controller: Vite (ohne Log) startet, Spielserver (Log server) läuft schon und wird übernommen.
async fn service_app(root: &Path) -> (Arc<App>, Seen) {
    let (app, seen) = start_app(root).await;
    let adopted = Arc::new(AtomicBool::new(true));
    let started = Arc::new(AtomicBool::new(false));
    let service = |name: &str, port, log: &str| Service {
        name: name.to_owned(),
        command: vec!["x".to_owned()],
        port,
        health: "http".to_owned(),
        log: log.to_owned(),
        ..Service::default()
    };
    let (running, up, listening, kill) = (
        Arc::clone(&started),
        Arc::clone(&adopted),
        Arc::clone(&adopted),
        Arc::clone(&adopted),
    );
    let events = app.events.clone();
    let options = Options {
        root: root.to_path_buf(),
        on_change: Arc::new(move |st| events.emit_value(SERVICE_STATE, st)),
        start: Arc::new(move |_, _, _| {
            started.store(true, Ordering::SeqCst);
            Ok(fake_spawn())
        }),
        check: Arc::new(move |svc: &Service| {
            let down = (svc.name == "Spielserver" && !up.load(Ordering::SeqCst))
                || (svc.name == "Vite" && !running.load(Ordering::SeqCst));
            if down {
                Err("antwortet nicht".to_owned())
            } else {
                Ok(())
            }
        }),
        listen: Arc::new(move |port| (4711, port == 8080 && listening.load(Ordering::SeqCst))),
        sample: Arc::new(|_| Ok(crate::services::Metrics::default())),
        kill_pid: Arc::new(move |_| {
            kill.store(false, Ordering::SeqCst);
            Ok(())
        }),
        supervisor: Arc::new(|_| 0),
        start_poll: Duration::from_millis(2),
        start_timeout: Duration::from_millis(500),
        watch_every: Duration::from_secs(3600),
        stop_timeout: Duration::from_millis(200),
        ..Options::default()
    };
    let list = vec![service("Vite", 5173, ""), service("Spielserver", 8080, "server")];
    let ctl = Controller::new(list, options);
    ctl.adopt().await;
    lock(&app.running).as_mut().unwrap().services = Ok(ctl);
    (app, seen)
}

#[tokio::test(flavor = "multi_thread")]
async fn services_methoden() {
    let root = tmp_dir();
    let (app, _) = service_app(root.path()).await;
    let view = ui(&app, |a| a.services()).await;
    assert_eq!((view.error.as_str(), view.services.len()), ("", 2));
    assert_eq!(
        (view.services[0].log.as_str(), view.services[1].log.as_str()),
        ("", "server")
    );
    assert_eq!(view.services[1].state, State::Adopted);
    let st = ui(&app, |a| a.service_start("Vite")).await.unwrap();
    assert_eq!(st.state, State::Running);
    ui(&app, |a| a.services_start_all()).await.unwrap();
    app.shutdown().await;
}

/// Ein übernommener Dienst stoppt nur mit force; jede Änderung kommt als `service:state`.
#[tokio::test(flavor = "multi_thread")]
async fn services_stopp_und_ereignisse() {
    let root = tmp_dir();
    let (app, seen) = service_app(root.path()).await;
    ui(&app, |a| a.service_start("Vite")).await.unwrap();
    assert!(ui(&app, |a| a.service_stop("Spielserver", false)).await.is_err());
    let st = ui(&app, |a| a.service_stop("Spielserver", true)).await.unwrap();
    assert_eq!(st.state, State::Stopped);
    ui(&app, |a| a.services_stop_all()).await.unwrap();
    assert_eq!(ui(&app, |a| a.services()).await.services[0].state, State::Stopped);
    let got: Vec<String> = seen
        .lock()
        .unwrap()
        .iter()
        .filter(|(name, _)| name == SERVICE_STATE)
        .map(|(_, d)| format!("{} {}", d["name"].as_str().unwrap(), d["state"].as_str().unwrap()))
        .collect();
    for want in [
        "Spielserver übernommen",
        "Vite startet",
        "Vite läuft",
        "Spielserver gestoppt",
        "Vite stoppt",
    ] {
        assert!(got.iter().any(|g| g == want), "Ereignis {want} fehlt in {got:?}");
    }
    app.shutdown().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn log_levels() {
    let root = tmp_dir();
    let (app, _) = service_app(root.path()).await;
    let error = ui(&app, |a| a.service_log_levels("Vite")).await.unwrap_err();
    assert!(error.contains("kein Log"), "{error}");
    let now = chrono::Utc::now().to_rfc3339();
    let line = |level: &str| format!("{{\"time\":\"{now}\",\"level\":\"{level}\",\"msg\":\"x\",\"ns\":\"http\"}}\n");
    let dir = root.path().join("logs");
    std::fs::create_dir_all(&dir).unwrap();
    let text = line("INFO") + &line("WARN") + &line("ERROR") + &line("INFO");
    std::fs::write(dir.join("server.jsonl"), text).unwrap();
    let args = [json!("Spielserver")];
    let got = ui(&app, move |a| call(a, "ServiceLogLevels", &args).unwrap()).await;
    let want = json!({"counts": {"ERROR": 1, "INFO": 2, "WARN": 1}, "total": 4, "budgetHit": false});
    assert_eq!(got, want);
    app.shutdown().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn ohne_konfiguration() {
    let root = tmp_dir();
    let (app, _) = start_app(root.path()).await;
    let view = ui(&app, |a| call(a, "Services", &[]).unwrap()).await;
    assert_eq!(view["services"], json!([]));
    assert!(view["error"].as_str().unwrap().contains("services.json"), "{view}");
    let error = ui(&app, |a| a.service_start("Vite")).await.unwrap_err();
    assert!(error.contains("services.json"), "{error}");
    let error = ui(&app, |a| a.services_reload()).await.unwrap_err();
    assert!(error.ends_with("; k3c-workbench neu starten"), "{error}");
    app.shutdown().await;
    assert_eq!(ui(&app, |a| a.services()).await.error, "backend beendet");
}

fn write_config(root: &Path, text: &str) {
    let dir = root.join("tools").join("k3c-workbench");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("services.json"), text).unwrap();
}

/// Vom Start über die Aufruf-Tabelle bis zum Beenden mit einem echten Attrappen-Prozess (health process, also ohne
/// Übernahme und ohne Port).
#[tokio::test(flavor = "multi_thread")]
async fn eigene_services_json_mit_echtem_prozess() {
    let root = tmp_dir();
    let exe = serde_json::to_string(&std::env::current_exe().unwrap()).unwrap();
    let args = r#""services::testutil::helper_process", "--exact", "--nocapture""#;
    let entry = format!(
        r#"{{"name":"Helfer","command":[{exe},{args}],"cwd":".","health":"process","env":{{"K3C_HELPER":"listen"}}}}"#
    );
    write_config(root.path(), &format!("[{entry}]"));
    let (app, seen) = start_app(root.path()).await;
    let view = ui(&app, |a| call(a, "Services", &[]).unwrap()).await;
    assert_eq!(view["error"], "", "{view}");
    assert_eq!(view["services"][0]["state"], "gestoppt");
    assert_eq!(view["services"][0]["startedAt"], "0001-01-01T00:00:00Z");
    let error = ui(&app, |a| call(a, "ServiceStop", &[json!("Helfer")])).await;
    assert_eq!(error.unwrap_err().0, "erwartet 2 Argumente, bekommen 1");
    let st = ui(&app, |a| call(a, "ServiceStart", &[json!("Helfer")]).unwrap()).await;
    assert_eq!(st["state"], "läuft", "{st}");
    crate::services::testutil::wait_for("console:line der Ausgabe", || {
        seen.lock().unwrap().iter().any(|(name, lines)| {
            name == CONSOLE_LINE
                && lines.as_array().is_some_and(|l| {
                    l.iter().any(|line| {
                        line["source"] == "Helfer" && line["text"].as_str().is_some_and(|t| t.contains("helfer bereit"))
                    })
                })
        })
    })
    .await;
    let st = ui(&app, |a| call(a, "ServiceRestart", &[json!("Helfer")]).unwrap()).await;
    assert_eq!(st["state"], "läuft", "{st}");
    write_config(
        root.path(),
        &format!("[{entry},{}]", entry.replace("Helfer", "Zweiter")),
    );
    let view = ui(&app, |a| call(a, "ServicesReload", &[]).unwrap()).await;
    assert_eq!(view["services"][1]["name"], "Zweiter", "{view}");
    assert_eq!(
        ui(&app, |a| call(a, "ServicesStartAll", &[]).unwrap()).await,
        Value::Null
    );
    assert_eq!(
        ui(&app, |a| call(a, "ServicesStopAll", &[]).unwrap()).await,
        Value::Null
    );
    ui(&app, |a| a.service_start("Helfer")).await.unwrap();
    // Beenden stoppt den eigenen Dienst.
    app.shutdown().await;
    let last = seen
        .lock()
        .unwrap()
        .iter()
        .rev()
        .find(|(name, d)| name == SERVICE_STATE && d["name"] == "Helfer")
        .map(|(_, d)| d["state"].clone());
    assert_eq!(last, Some(json!("gestoppt")));
}
