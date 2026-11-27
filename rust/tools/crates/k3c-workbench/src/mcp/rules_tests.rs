//! Port von `tools_rules_test.go`.

use std::sync::{Arc, Mutex};

use serde_json::json;

use super::test_client::Client;
use super::{Config, Notice, Server};

/// Workbench-Spec › MCP-Tools: Name `<bereich>_<verb>`, Beschreibung mit „Nutze es bei: … Statt: …“.
#[tokio::test(flavor = "multi_thread")]
async fn tool_regeln_der_spec() {
    let server = Server::new(Config::default());
    server.start().await.unwrap();
    for tool in Client::connect(server.url()).await.tools().await {
        let name = tool["name"].as_str().unwrap();
        let parts: Vec<&str> = name.split('_').collect();
        let lower = |p: &&str| !p.is_empty() && p.bytes().all(|b| b.is_ascii_lowercase());
        assert!(
            parts.len() >= 2 && parts.iter().all(lower),
            "{name}: Name nicht <bereich>_<verb>"
        );
        let description = tool["description"].as_str().unwrap();
        let tail = description.rfind("Nutze es bei: ").map(|at| &description[at..]);
        assert!(tail.is_some_and(|t| t.contains(" Statt: ")), "{name}: {description}");
    }
    server.stop().await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn notify_ui() {
    let got = Arc::new(Mutex::new(Vec::<Notice>::new()));
    let seen = Arc::clone(&got);
    let server = Server::new(Config {
        on_notify: Some(Arc::new(move |n: &Notice| seen.lock().unwrap().push(n.clone()))),
        ..Config::default()
    });
    server.start().await.unwrap();
    let mut client = Client::connect(server.url()).await;
    let ok = client
        .call_text("notify_ui", json!({"level": "warn", "title": "Fertig"}))
        .await;
    assert_eq!(ok, ("angezeigt: Fertig".to_owned(), false));
    assert_eq!(got.lock().unwrap().len(), 1);
    let (text, is_error) = client
        .call_text("notify_ui", json!({"level": "laut", "title": "x"}))
        .await;
    assert!(
        is_error && text.contains("erlaubt info, success, warn, error"),
        "{text}"
    );
    let empty = client
        .call_text("notify_ui", json!({"level": "info", "title": " "}))
        .await;
    assert_eq!(empty, ("title fehlt".to_owned(), true));
    assert_eq!(got.lock().unwrap().len(), 1, "abgelehnt heißt nicht angezeigt");
    server.stop().await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn notify_ui_ohne_oberflaeche() {
    let server = Server::new(Config::default());
    server.start().await.unwrap();
    let mut client = Client::connect(server.url()).await;
    let (text, is_error) = client
        .call_text("notify_ui", json!({"level": "info", "title": "x"}))
        .await;
    assert!(is_error && text.starts_with("keine Oberfläche"), "{text}");
    server.stop().await.unwrap();
}
