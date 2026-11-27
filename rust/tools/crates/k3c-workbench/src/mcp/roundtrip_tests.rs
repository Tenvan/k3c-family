//! Port von `roundtrip_test.go` (ohne Planung → RU69.12 und `gh_status` → RU69.7; `TestResolvePort` in `ports.rs`).

use serde_json::json;

use super::test_client::Client;
use super::test_helpers::started;
use super::tools::CATALOG;
use super::{Config, INSTRUCTIONS, Server};

/// Server nur mit den echten Tools.
async fn plain() -> (Server, Client) {
    let server = Server::new(Config::default());
    server.start().await.unwrap();
    let client = Client::connect(server.url()).await;
    (server, client)
}

/// Aus `TestAlleToolsMitBeschreibungUndAnnotations`.
#[tokio::test(flavor = "multi_thread")]
async fn alle_tools_mit_beschreibung_und_annotations() {
    let (server, mut client) = plain().await;
    let tools = client.tools().await;
    for tool in &tools {
        assert!(!tool["description"].as_str().unwrap_or_default().is_empty(), "{tool}");
        assert!(tool["annotations"].is_object(), "{tool}");
    }
    let listed: Vec<String> = tools.iter().map(|t| t["name"].as_str().unwrap().to_owned()).collect();
    let mut catalog: Vec<String> = server.stats().await.tools.into_iter().map(|t| t.name).collect();
    catalog.sort();
    assert_eq!(listed, catalog);
    let mut registered: Vec<&str> = CATALOG.to_vec();
    registered.sort_unstable();
    assert_eq!(listed, registered);
    assert_eq!(client.init["serverInfo"]["name"], json!("k3c-workbench"));
    assert_eq!(client.init["instructions"], json!(INSTRUCTIONS));
    server.stop().await.unwrap();
}

/// Aus `TestKeineStudioToolsInDerWorkbench`: Weltdesign und Balancing führt nur k3c-studio.
#[tokio::test(flavor = "multi_thread")]
async fn keine_studio_tools_in_der_workbench() {
    let balance = [
        "sim_test",
        "sim_run",
        "replay_run",
        "reports_list",
        "report_read",
        "saves_list",
    ];
    let prefixes = ["level_generate", "world_", "assets_", "hub_"];
    let (server, mut client) = plain().await;
    for tool in client.tools().await {
        let name = tool["name"].as_str().unwrap().to_owned();
        let studio = prefixes.iter().any(|p| name.starts_with(p)) || balance.contains(&name.as_str());
        assert!(!studio, "Studio-Tool {name} im Katalog der Workbench");
    }
    for name in prefixes.iter().chain(&balance) {
        assert!(
            !INSTRUCTIONS.contains(&format!("`{name}")),
            "instructions.md nennt noch {name}"
        );
    }
    server.stop().await.unwrap();
}

/// Aus `TestUnbekannterParameterNenntGueltige`.
#[tokio::test(flavor = "multi_thread")]
async fn unbekannter_parameter_nennt_gueltige() {
    let server = started(Config::default()).await;
    let mut client = Client::connect(server.url()).await;
    let (text, is_error) = client.call_text("echo", json!({"txt": "x"})).await;
    assert!(is_error && text.contains("gültige Parameter: text, times"), "{text}");
    let (text, _) = client.call_text("workbench_status", json!({"foo": 1})).await;
    assert!(text.contains("gültige Parameter: keine"), "{text}");
    server.stop().await.unwrap();
}

/// Aus `TestPanikWirdZumFehler`.
#[tokio::test(flavor = "multi_thread")]
async fn panik_wird_zum_fehler() {
    let server = started(Config::default()).await;
    let mut client = Client::connect(server.url()).await;
    let (text, is_error) = client.call_text("boom", json!({})).await;
    assert!(is_error && text.contains(k3c_kit::mcp::PANIC_TEXT), "{text}");
    let (text, is_error) = client.call_text("workbench_status", json!({})).await;
    assert!(!is_error, "Server antwortet nach Panik nicht mehr: {text}");
    let stats = server.stats().await;
    assert_eq!((stats.errors, stats.total_calls), (1, 2));
    server.stop().await.unwrap();
}

/// Aus `TestHTTPNurAnLocalhost`.
#[tokio::test(flavor = "multi_thread")]
async fn http_nur_an_localhost() {
    let (server, mut client) = plain().await;
    let url = server.url();
    assert!(url.starts_with("http://127.0.0.1:") && url.ends_with("/mcp"), "{url}");
    let foreign = reqwest::Client::new()
        .post(&url)
        .header("Host", "example.org")
        .header("Accept", "application/json, text/event-stream")
        .header("Content-Type", "application/json")
        .body("{}")
        .send()
        .await
        .unwrap();
    assert_eq!(foreign.status().as_u16(), 403, "fremder Host");
    let (text, is_error) = client.call_text("workbench_status", json!({})).await;
    assert!(!is_error && text.contains("Clients 1"), "{text}");
    server.restart().await.unwrap();
    let stats = server.stats().await;
    assert_eq!((stats.clients, stats.peak_clients), (0, 1), "alte Session zählt weiter");
    server.stop().await.unwrap();
}
