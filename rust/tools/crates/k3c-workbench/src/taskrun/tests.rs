//! Port von `taskrun_test.go`. Statt `task` startet über die Naht `argv` die Befehlszeile der Plattform; kein Test
//! ruft einen echten Task.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::json;

use super::*;
use crate::services::testutil::wait_for;
use crate::testutil::tmp_dir;

type Lines = Arc<Mutex<Vec<String>>>;

fn shell(script: &str) -> Vec<String> {
    let head: [&str; 2] = if cfg!(windows) { ["cmd", "/c"] } else { ["sh", "-c"] };
    head.iter().chain([&script]).map(|s| (*s).to_owned()).collect()
}

/// Runner, dessen Läufe statt `task` das Skript starten; sammelt Ausgabe und Zustandswechsel.
fn runner(root: &std::path::Path, script: &str) -> (Runner, Lines, Lines) {
    let (out, changes) = (Lines::default(), Lines::default());
    let (lines, states) = (Arc::clone(&out), Arc::clone(&changes));
    let command = shell(script);
    let runner = Runner::new(Options {
        root: root.to_path_buf(),
        out: Arc::new(move |name: &str, stream: &str, text: &str| {
            lines.lock().unwrap().push(format!("{name} {stream} {text}"));
        }),
        on_change: Arc::new(move |run: &Run| states.lock().unwrap().push(run.state.as_str().to_owned())),
        argv: Arc::new(move |_| command.clone()),
        ..Options::default()
    });
    (runner, out, changes)
}

async fn ended(r: &Runner) -> Run {
    wait_for("Ende des Laufs", || {
        r.all().iter().all(|run| run.state != State::Running)
    })
    .await;
    r.all().remove(0)
}

/// `TestStart_ErfolgUndFehler`.
#[tokio::test]
async fn erfolg_und_fehler() {
    let tmp = tmp_dir();
    let (ok, out, changes) = runner(tmp.path(), "echo hallo");
    let started = ok.start("x", Vec::new()).unwrap();
    assert_eq!(started.state, State::Running);
    assert!(started.pid > 0);
    let run = ended(&ok).await;
    assert_eq!(
        (run.state, run.exit_code, run.reason.as_str()),
        (State::Succeeded, 0, "")
    );
    assert_eq!(*out.lock().unwrap(), ["x stdout hallo"]);
    wait_for("zwei Ereignisse", || changes.lock().unwrap().len() == 2).await;
    assert_eq!(*changes.lock().unwrap(), ["running", "succeeded"]);

    let (bad, _, _) = runner(tmp.path(), "exit 2");
    bad.start("x", Vec::new()).unwrap();
    let run = ended(&bad).await;
    assert_eq!((run.state, run.exit_code, run.reason.as_str()), (State::Failed, 2, ""));
}

/// `TestStart_LehntArgumenteMitMetazeichenAb`.
#[tokio::test]
async fn lehnt_metazeichen_ab_und_stoppt_nichts_ohne_lauf() {
    let r = Runner::new(Options::default());
    let error = r.start("x", vec!["a;b".to_owned()]).unwrap_err();
    assert!(error.contains("nicht erlaubte Zeichen"), "{error}");
    assert!(r.all().is_empty());
    let error = r.stop("x", Duration::from_secs(1)).await.unwrap_err();
    assert_eq!(error, "task läuft nicht: x");
}

#[tokio::test]
async fn zweiter_start_stopp_und_stop_all() {
    let tmp = tmp_dir();
    let wait = if cfg!(windows) {
        "ping -n 60 127.0.0.1 >nul"
    } else {
        "sleep 60"
    };
    let (r, _, changes) = runner(tmp.path(), wait);
    let first = r.start("lang", Vec::new()).unwrap();
    let again = r.start("lang", Vec::new()).unwrap_err();
    assert_eq!(again, format!("task läuft bereits: lang (PID {})", first.pid));
    let stopped = r.stop("lang", Duration::from_secs(15)).await.unwrap();
    assert_eq!(stopped.state, State::Cancelled);
    assert!(stopped.ended_at.is_some());
    r.start("lang", Vec::new()).unwrap();
    r.stop_all(Duration::from_secs(15)).await.unwrap();
    assert_eq!(r.all()[0].state, State::Cancelled);
    wait_for("vier Ereignisse", || changes.lock().unwrap().len() == 4).await;
}

#[tokio::test]
async fn startfehler_bleibt_als_lauf() {
    let tmp = tmp_dir();
    let r = Runner::new(Options {
        root: tmp.path().to_path_buf(),
        argv: Arc::new(|_| vec!["k3c-gibt-es-nicht".to_owned()]),
        ..Options::default()
    });
    let error = r.start("x", Vec::new()).unwrap_err();
    let run = r.all().remove(0);
    assert_eq!((run.state, run.reason.as_str()), (State::Failed, error.as_str()));
    let value = serde_json::to_value(&run).unwrap();
    assert_eq!(value["state"], "failed");
    assert_eq!(value["args"], json!([]));
    assert_eq!(value["durationMs"], 0);
    assert_eq!(value["exitCode"], 0);
}

#[test]
fn zeiten_wie_go() {
    let run = Run {
        name: "x".to_owned(),
        state: State::Running,
        pid: 7,
        args: Vec::new(),
        started_at: None,
        ended_at: None,
        exit_code: 0,
        duration_ms: 0,
        reason: String::new(),
    };
    assert_eq!(serde_json::to_value(&run).unwrap()["endedAt"], "0001-01-01T00:00:00Z");
}
