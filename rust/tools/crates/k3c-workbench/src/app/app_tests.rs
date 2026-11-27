//! Port von `app_test.go` und `app_mcp_test.go`, dazu der Weg `notify_ui` → `ui:notify` (AC-01).

use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};

use super::events::{Emit, MCP_CALL, MCP_START, MCP_STATE, NOTIFY};
use super::*;
use crate::mcp::test_client::Client;
use crate::rpc::call;
use crate::testutil::tmp_dir;

type Seen = Arc<Mutex<Vec<(String, Value)>>>;

/// App in einem Temp-Ordner, deren Ereignisse gesammelt werden.
fn app(root: &Path, port: u16) -> (Arc<App>, Seen) {
    let seen = Seen::default();
    let sink = Arc::clone(&seen);
    let forward: Emit = Arc::new(move |name: &str, data: Value| sink.lock().unwrap().push((name.to_owned(), data)));
    let app = App::new(root.to_path_buf(), port, Some(forward)).with_usage_file(root.join("usage.json"));
    (Arc::new(app), seen)
}

/// Ein Aufruf der Oberfläche, wie das Fenster ihn macht (blockierender Thread).
async fn ui(app: &Arc<App>, method: &'static str) -> Value {
    let app = Arc::clone(app);
    tokio::task::spawn_blocking(move || call(&app, method, &[]).unwrap())
        .await
        .unwrap()
}

/// Ein belegter Port hält das Fenster nicht auf: Info und Ereignis nennen den Grund.
#[tokio::test(flavor = "multi_thread")]
async fn startup_port_belegt() {
    let root = tmp_dir();
    let busy = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let (app, seen) = app(root.path(), busy.local_addr().unwrap().port());
    app.startup().await;
    let info = app.info();
    assert!(!info.mcp.listening, "{info:?}");
    assert!(info.mcp.error.contains("nicht verfügbar"), "{info:?}");
    let names: Vec<String> = seen.lock().unwrap().iter().map(|(n, _)| n.clone()).collect();
    assert_eq!(names, [MCP_STATE]);
    app.shutdown().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn startup_lauscht_und_schreibt_das_eigene_log() {
    let root = tmp_dir();
    let (app, _) = app(root.path(), 0);
    app.startup().await;
    let state = app.info().mcp;
    assert!(state.listening && state.error.is_empty(), "{state:?}");
    app.shutdown().await;
    let log = root.path().join("logs").join(format!("{}.jsonl", ports::LOG_NAME));
    let text = std::fs::read_to_string(log).unwrap();
    assert!(
        text.contains("🚀 k3c-workbench gestartet") && text.contains("🛑"),
        "{text}"
    );
}

/// Aus `TestBindingWartetAufStartup` (mit `Info`; `Sources` kommt mit RU69.2).
#[tokio::test(flavor = "multi_thread")]
async fn aufruf_wartet_auf_startup() {
    let root = tmp_dir();
    let (app, _) = app(root.path(), 0);
    let early = Arc::clone(&app);
    let info = tokio::task::spawn_blocking(move || early.info());
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(!info.is_finished(), "Info vor startup");
    app.startup().await;
    assert!(info.await.unwrap().mcp.listening);
    app.shutdown().await;
}

/// Aus `TestMcpBindings`.
#[tokio::test(flavor = "multi_thread")]
async fn mcp_methoden() {
    let root = tmp_dir();
    let (app, _) = app(root.path(), 0);
    app.startup().await;
    let overview = ui(&app, "McpOverview").await;
    assert_eq!(overview["mcp"]["listening"], json!(true), "{overview}");
    assert_eq!(overview["stats"]["totalCalls"], json!(0), "{overview}");
    assert!(overview["stats"]["startedAt"].as_str().is_some_and(|s| !s.is_empty()));
    assert!(!overview["stats"]["tools"].as_array().unwrap().is_empty());
    let text = ui(&app, "McpInstructions").await;
    let text = text.as_str().unwrap();
    assert!(text.starts_with("# k3c-workbench") && text.contains("`check_run`"));
    assert_eq!(ui(&app, "McpCalls").await, json!([]));
    let usage = ui(&app, "McpUsage").await;
    let rules = json!({"outlierFactor": 2.0, "outlierFloorMs": 1000.0, "baselineCalls": 8, "percentileErrorPct": 12});
    assert_eq!(usage["rules"], rules);
    for key in ["session", "allTime", "minutes"] {
        assert!(usage.get(key).is_some(), "McpUsage ohne {key}: {usage}");
    }
    app.shutdown().await;
}

/// Aus `TestMcpRestart`.
#[tokio::test(flavor = "multi_thread")]
async fn mcp_neustart() {
    let root = tmp_dir();
    let (app, _) = app(root.path(), 0);
    app.startup().await;
    let state = ui(&app, "McpRestart").await;
    assert_eq!(
        (&state["listening"], &state["error"]),
        (&json!(true), &json!("")),
        "{state}"
    );
    assert_eq!(ui(&app, "McpOverview").await["mcp"]["listening"], json!(true));
    Client::connect(app.mcp_url()).await;
    app.shutdown().await;
}

/// Aus `TestMcpEreignisse` und AC-01: ein Aufruf kommt als `mcp:start` und `mcp:call` an, `notify_ui` als `ui:notify`.
#[tokio::test(flavor = "multi_thread")]
async fn mcp_ereignisse_und_hinweis() {
    let root = tmp_dir();
    let (app, seen) = app(root.path(), 0);
    app.startup().await;
    let mut client = Client::connect(app.mcp_url()).await;
    let (status, is_error) = client.call_text("workbench_status", json!({})).await;
    assert!(!is_error && status.starts_with("k3c-workbench · http://127.0.0.1:"));
    let names: Vec<String> = seen.lock().unwrap().iter().map(|(n, _)| n.clone()).collect();
    assert_eq!(names, [MCP_STATE, MCP_START, MCP_CALL]);
    let notice = json!({"level": "success", "title": "Fertig", "text": "alles grün"});
    client.call_text("notify_ui", notice.clone()).await;
    let got = seen.lock().unwrap().clone();
    assert!(got.iter().any(|(n, d)| n == NOTIFY && *d == notice), "{got:?}");
    let calls = ui(&app, "McpCalls").await;
    assert_eq!(calls[0]["tool"], json!("notify_ui"), "{calls}");
    app.shutdown().await;
}
