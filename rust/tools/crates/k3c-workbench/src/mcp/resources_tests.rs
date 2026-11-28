//! Port von `resources_test.go`. `TestPprofNurFuerLoopbackHosts` entfällt: es gibt kein pprof.

use serde_json::json;

use super::*;
use crate::mcp::test_client::Client;
use crate::mcp::{Config, Server};
use crate::testutil::tmp_dir;

#[tokio::test(flavor = "multi_thread")]
async fn zeigt_puffer_und_vorgaenge() {
    let root = tmp_dir();
    let monitor = Monitor::new(crate::resmon::Config::default());
    let server = Server::new(Config {
        root: root.path().to_path_buf(),
        version: "test".to_owned(),
        resources: Some(Arc::clone(&monitor)),
        ..Config::default()
    });
    server.env.console.add("check:task:test", "stdout", "zeile");
    monitor.track("tool:x")();
    server.start().await.unwrap();
    let mut client = Client::connect(server.url()).await;
    // Der erste Aufruf misst selbst (noch keine Probe), danach zählt der Aufruf als Vorgang.
    let (first, failed) = client.call_text("workbench_resources", json!({})).await;
    assert!(!failed, "{first}");
    let (second, _) = client.call_text("workbench_resources", json!({})).await;
    server.stop().await.unwrap();
    for want in [
        "heap live",
        "Konsole-Zeilen 1",
        "Prüfläufe 0",
        "Prüflauf-Text 0 B",
        "MCP-Sessions 1",
        "tool:x",
    ] {
        assert!(first.contains(want), "Bericht ohne {want:?}:\n{first}");
    }
    assert!(second.contains("tool:workbench_resources"), "{second}");
}

#[tokio::test]
async fn ohne_messgeraet_meldet_es_klar() {
    assert_eq!(report(None).await, Err(OFF.to_owned()));
}
