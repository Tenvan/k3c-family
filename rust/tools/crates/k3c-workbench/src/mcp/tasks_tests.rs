//! Port von `tools_tasks_test.go` mit gestelltem Task-Host: kein Test startet einen echten Task.

use std::sync::{Arc, Mutex};

use chrono::Local;
use k3c_kit::console::Store;
use serde_json::json;

use super::test_client::Client;
use super::{Config, Server, TaskHost};
use crate::taskcat::Task;
use crate::taskrun::{Run, State};

fn run(name: &str, state: State) -> Run {
    Run {
        name: name.to_owned(),
        state,
        pid: 0,
        args: Vec::new(),
        started_at: Some(Local::now()),
        ended_at: None,
        exit_code: 0,
        duration_ms: 0,
        reason: String::new(),
    }
}

fn task(name: &str, namespace: &str, desc: &str) -> Task {
    Task {
        name: name.to_owned(),
        namespace: namespace.to_owned(),
        desc: desc.to_owned(),
        ..Task::default()
    }
}

#[derive(Default)]
pub(super) struct FakeTasks {
    started: Mutex<Vec<String>>,
}

impl FakeTasks {
    pub(super) fn started(&self) -> Vec<String> {
        self.started.lock().unwrap().clone()
    }
}

impl TaskHost for FakeTasks {
    fn task_list(&self) -> Result<Vec<Task>, String> {
        Ok(vec![
            task("check", "Workspace", "Lint und Tests"),
            task("golden:update", "golden", "Golden neu"),
        ])
    }

    fn task_start(&self, name: &str, args: Vec<String>) -> Result<Run, String> {
        self.started.lock().unwrap().push(format!("{name} {}", args.join(" ")));
        Ok(Run {
            pid: 7,
            args,
            ..run(name, State::Running)
        })
    }

    fn task_stop(&self, name: &str) -> Result<Run, String> {
        Err(format!("task läuft nicht: {name}"))
    }

    fn task_runs(&self) -> Vec<Run> {
        vec![Run {
            duration_ms: 4200,
            ..run("check", State::Succeeded)
        }]
    }
}

/// `TestTaskTools`.
#[tokio::test(flavor = "multi_thread")]
async fn task_tools() {
    let host = Arc::new(FakeTasks::default());
    let store = Arc::new(Store::new(0, None));
    let server = Server::new(Config {
        console: Some(Arc::clone(&store)),
        tasks: Some(Arc::clone(&host) as Arc<dyn TaskHost>),
        ..Config::default()
    });
    server.start().await.unwrap();
    let mut client = Client::connect(server.url()).await;
    let (text, _) = client.call_text("task_list", json!({})).await;
    assert_eq!(
        text,
        "Workspace\n  check · Lint und Tests\ngolden\n  golden:update · Golden neu"
    );
    let (text, _) = client.call_text("task_list", json!({"namespace": "GOLDEN"})).await;
    assert_eq!(text, "golden\n  golden:update · Golden neu");
    let (text, _) = client.call_text("task_list", json!({"query": "nichts"})).await;
    assert_eq!(text, "keine Tasks gefunden");
    // Kein Freigabe-Schloss: jeder Task aus dem Katalog startet.
    let arguments = json!({"task": "golden:update"});
    let (text, is_error) = client.call_text("task_start", arguments).await;
    assert!(!is_error && text.starts_with("golden:update · running"), "{text}");
    let arguments = json!({"task": "check", "args": "a;b"});
    let (text, is_error) = client.call_text("task_start", arguments).await;
    assert!(
        is_error && text.contains("nicht erlaubte Zeichen"),
        "Metazeichen: {text}"
    );
    assert_eq!(host.started(), ["golden:update "]);
    let arguments = json!({"task": "check", "args": "-run Foo"});
    let (text, is_error) = client.call_text("task_start", arguments).await;
    assert!(
        !is_error && text.starts_with("check · running · PID 7 · seit "),
        "{text}"
    );
    assert!(
        text.contains(" · -- -run Foo · Ausgabe: task_output check\nCheckout: Repo-Wurzel "),
        "{text}"
    );
    assert_eq!(host.started()[1], "check -run Foo");
    let (text, _) = client.call_text("task_status", json!({})).await;
    assert_eq!(text, "check · succeeded · Exit 0 · 4,2 s");
    let (text, _) = client.call_text("task_status", json!({"task": "x"})).await;
    assert_eq!(text, "keine Läufe in dieser Sitzung");
    let (text, is_error) = client.call_text("task_stop", json!({"task": "check"})).await;
    assert!(is_error && text.starts_with("task läuft nicht: check"), "{text}");
    store.add("task:check", "stdout", "fertig");
    let (text, _) = client.call_text("task_output", json!({"task": "check"})).await;
    assert_eq!(text, "task:check · letzte 1 Zeilen (bis Nr. 1)\nfertig");
    server.stop().await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn ohne_tasks_seite() {
    let server = Server::new(Config::default());
    server.start().await.unwrap();
    let mut client = Client::connect(server.url()).await;
    let (text, is_error) = client.call_text("task_list", json!({})).await;
    assert!(is_error);
    assert_eq!(text, "keine Tasks: k3c-workbench läuft ohne Tasks-Seite");
    server.stop().await.unwrap();
}
