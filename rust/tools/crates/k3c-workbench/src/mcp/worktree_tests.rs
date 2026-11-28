//! Port von `TestWorktreeDiensteMitEigenenPorts` und `TestWorktreeAlteBasisMeidetWurzelPorts`, dazu die Checkout-Regel
//! der schreibenden Tools. Die Ports liegen abseits der Dienste des Rechners; kein Dienst wird gestartet.

use std::path::Path;
use std::sync::Arc;

use k3c_kit::console::Store;
use serde_json::json;

use super::services_tests::{Stubs, controller, service};
use super::tasks_tests::FakeTasks;
use super::test_client::Client;
use super::test_helpers::write;
use super::workspace_tests::repo_with_worktree;
use super::{Config, Server, TaskHost};
use crate::testutil::tmp_dir;

const SERVICES: &str = r#"[
    {"name":"Vite","command":["x"],"port":47173,"health":"http","portEnv":"K3C_VITE_PORT"},
    {"name":"Spielserver","command":["x"],"port":47080,"health":"http","portEnv":"K3C_HTTP_PORT"}]"#;

fn services_json(worktree: &Path, text: &str) {
    write(
        &worktree.join("tools").join("k3c-workbench").join("services.json"),
        text,
    );
}

async fn started(cfg: Config) -> Server {
    let server = Server::new(cfg);
    server.start().await.unwrap();
    server
}

async fn from(server: &Server, dir: &Path) -> Client {
    Client::connect_from(server.url(), Some(dir.display().to_string())).await
}

async fn end(server: Server) {
    server.stop_worktree_services().await;
    server.stop().await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn dienste_mit_eigenen_ports() {
    let tmp = tmp_dir();
    let (main, wt, _) = repo_with_worktree(tmp.path());
    services_json(&wt, SERVICES);
    // Versatz 10 ist belegt → 20.
    let server = started(Config {
        root: main.clone(),
        port_busy: Some(Arc::new(|port| port == 47090)),
        ..Config::default()
    })
    .await;
    let mut worktree = from(&server, &wt).await;
    let (text, is_error) = worktree.call_text("svc_status", json!({})).await;
    assert!(!is_error, "{text}");
    assert!(text.contains("Vite · gestoppt · Port 47193"), "{text}");
    assert!(text.contains("Spielserver · gestoppt · Port 47100"), "{text}");
    // Ein schreibendes Tool nennt den Checkout in der letzten Zeile.
    let (text, is_error) = worktree.call_text("svc_stop_all", json!({})).await;
    let last = text.lines().last().unwrap();
    assert!(!is_error && last.starts_with("Checkout: Worktree wt1 ("), "{text}");
    let (text, _) = from(&server, &main).await.call_text("svc_status", json!({})).await;
    assert!(text.contains("keine Dienste"), "{text}");
    end(server).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn alte_basis_meidet_wurzel_ports() {
    let tmp = tmp_dir();
    let (main, wt, _) = repo_with_worktree(tmp.path());
    services_json(
        &wt,
        r#"[{"name":"Vite","command":["x"],"port":47173,"health":"http","portEnv":"K3C_VITE_PORT"}]"#,
    );
    let store = Arc::new(Store::new(0, None));
    let root = controller(&Arc::new(Stubs::default()), &store, vec![service("Vite", 47183)]);
    let server = started(Config {
        root: main,
        services: Some(Ok(root)),
        port_busy: Some(Arc::new(|_| false)),
        ..Config::default()
    })
    .await;
    let (text, is_error) = from(&server, &wt).await.call_text("svc_status", json!({})).await;
    assert!(!is_error && text.contains("Vite · gestoppt · Port 47193"), "{text}");
    end(server).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn ohne_services_json_und_ohne_freien_versatz() {
    let tmp = tmp_dir();
    let (main, wt, _) = repo_with_worktree(tmp.path());
    let server = started(Config {
        root: main,
        port_busy: Some(Arc::new(|_| true)),
        ..Config::default()
    })
    .await;
    let mut client = from(&server, &wt).await;
    let (text, is_error) = client.call_text("svc_status", json!({})).await;
    assert!(
        is_error && text.starts_with("keine Dienste im Worktree wt1: "),
        "{text}"
    );
    services_json(&wt, SERVICES);
    let (text, is_error) = client.call_text("svc_status", json!({})).await;
    assert!(
        is_error && text.starts_with("kein freier Port-Versatz für die Dienste (50 Versuche)"),
        "{text}"
    );
    end(server).await;
}

/// AC-01: `task_*` lehnt einen Worktree mit dem Text aus Go ab, ohne den Task-Host zu rufen.
#[tokio::test(flavor = "multi_thread")]
async fn task_tools_nur_in_der_wurzel() {
    let tmp = tmp_dir();
    let (main, wt, _) = repo_with_worktree(tmp.path());
    let host = Arc::new(FakeTasks::default());
    let server = started(Config {
        root: main,
        tasks: Some(Arc::clone(&host) as Arc<dyn TaskHost>),
        ..Config::default()
    })
    .await;
    let mut client = from(&server, &wt).await;
    let want = "task_* bedient nur die Repo-Wurzel, nicht den Worktree wt1; im Worktree check_run nutzen";
    for (tool, arguments) in [
        ("task_list", json!({})),
        ("task_start", json!({"task": "check"})),
        ("task_stop", json!({"task": "check"})),
        ("task_status", json!({})),
        ("task_output", json!({"task": "check"})),
    ] {
        let (text, is_error) = client.call_text(tool, arguments).await;
        assert!(is_error && text.starts_with(want), "{tool}: {text}");
        let writes = matches!(tool, "task_start" | "task_stop");
        let last = text.lines().last().unwrap();
        assert_eq!(last.starts_with("Checkout: Worktree wt1 ("), writes, "{text}");
    }
    assert!(host.started().is_empty());
    end(server).await;
}
