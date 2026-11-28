//! Port von `usage_test.go`: die Nutzungsstatistik bekommt die rohen Argumente.

use std::sync::Arc;

use k3c_kit::usage::Tracker;
use serde_json::json;

use super::Config;
use super::test_client::Client;
use super::test_helpers::started;

#[tokio::test(flavor = "multi_thread")]
async fn nutzungsstatistik_bekommt_rohe_argumente() {
    let tracker = Arc::new(Tracker::new(Arc::new(chrono::Local::now)));
    let server = started(Config {
        usage: Some(Arc::clone(&tracker)),
        ..Config::default()
    })
    .await;
    let mut client = Client::connect(server.url()).await;
    // Unsortiert und lang: aus dem auf 120 Zeichen gekürzten Aufruf-Log ließe sich das nicht mehr normieren.
    let (_, is_error) = client
        .call_text("echo", json!({"times": 0, "text": "x".repeat(300)}))
        .await;
    assert!(!is_error);
    client.call_text("check_run", json!({"target": "task:alles"})).await;
    // Erfundener Name: Protokollfehler, die Zähler sehen ihn, die Statistik nicht.
    let unknown = client
        .answer("tools/call", json!({"name": "gibt_es_nicht", "arguments": {}}))
        .await;
    assert!(unknown["error"].is_object(), "{unknown}");
    let snapshot = tracker.snapshot().session;
    assert_eq!((snapshot.calls, snapshot.errors), (2, 1));
    for tool in &snapshot.tools {
        assert_ne!(tool.name, "gibt_es_nicht", "unbekanntes Tool in der Statistik");
        if tool.name == "echo" {
            assert!(
                tool.args[0].value.starts_with(r#"{"text":"xxx"#),
                "{}",
                tool.args[0].value
            );
        }
        if tool.name == "check_run" {
            assert_eq!(tool.top_errors.len(), 1);
            assert!(tool.top_errors[0].value.starts_with("unbekanntes Ziel"));
        }
    }
    let calls = server.calls();
    assert_eq!(calls[1].target, "task:alles", "Ziel im Aufruf-Log");
    let (status, _) = client.call_text("workbench_status", json!({})).await;
    assert!(status.contains("Sitzung: p95 "), "{status}");
    server.stop().await.unwrap();
}
