//! Port von `uiapi_test.go`: Verdrahtung der Aufruf-Tabelle. Schutz, RPC-Mechanik und Ereignisstrom prüft
//! `k3c_kit::uikit`; die Liste gegen `GoApp` prüft `tests/ui_methoden.rs`.

use std::sync::Arc;
use std::time::Duration;

use k3c_kit::uikit::{self, HEADER, PREFIX};
use serde_json::{Value, json};

use super::*;
use crate::app::events::{MCP_STATE, PLANNING};
use crate::app::{McpState, VERSION};
use crate::rpc::{RpcError, call};

/// App ohne `startup`: genug für `Info` und den Bus.
fn app() -> Arc<App> {
    let app = App::new("unbenutzt".into(), 1, None);
    app.mark_ready();
    Arc::new(app)
}

/// Startet die UI-API auf einem freien Port.
async fn serve(app: &Arc<App>) -> String {
    let listener = uikit::listen("TEST_PORT", 0).await.unwrap();
    let url = uikit::page_url(&listener);
    tokio::spawn(uikit::serve(listener, router(app), std::future::pending()));
    format!("{url}{}", PREFIX.trim_start_matches('/'))
}

async fn post(base: &str, method: &str, body: &str) -> (u16, Value) {
    let response = reqwest::Client::new()
        .post(format!("{base}rpc/{method}"))
        .header(HEADER, "1")
        .body(body.to_owned())
        .send()
        .await
        .unwrap();
    let status = response.status().as_u16();
    let text = response.text().await.unwrap();
    (status, serde_json::from_str(&text).unwrap_or_default())
}

/// Aus `TestUIRPC`.
#[tokio::test(flavor = "multi_thread")]
async fn ui_rpc() {
    let base = serve(&app()).await;
    let (code, out) = post(&base, "Info", "[]").await;
    assert_eq!((code, &out["result"]["version"]), (200, &json!(VERSION)), "{out}");
    // MCP-intern (TaskList) und Bindings des Studios gibt es hier nicht.
    for method in ["TaskList", "WorldList", "AssetsList", "HubPlan", "BalanceRuns"] {
        let (code, out) = post(&base, method, "[]").await;
        assert_eq!(code, 404, "{method}: {out}");
    }
    // Falsche Argumente: 400 wie Go `decodeArgs`.
    let (code, out) = post(&base, "Info", "[1]").await;
    assert_eq!((code, &out["error"]), (400, &json!("erwartet 0 Argumente, bekommen 1")));
    // Fehler einer Methode: 422 (die App läuft hier nicht).
    let (code, out) = post(&base, "McpCalls", "[]").await;
    assert_eq!((code, &out["error"]), (422, &json!("backend beendet")));
}

#[test]
fn command_und_ui_api_nutzen_eine_tabelle() {
    let app = app();
    assert_eq!(call(&app, "Info", &[]).unwrap()["version"], json!(VERSION));
    assert_eq!(
        call(&app, "Nichts", &[]),
        Err(RpcError("Unbekannte Methode Nichts".to_owned()))
    );
    let mut served: Vec<String> = api(&app).methods.into_keys().collect();
    served.sort();
    let mut table: Vec<String> = METHODS.iter().map(|(name, _)| (*name).to_owned()).collect();
    table.sort();
    assert_eq!(served, table);
}

/// Aus `TestUIEvents`: Ereignisse erreichen offene Ströme der HTTP-Oberfläche.
#[tokio::test(flavor = "multi_thread")]
async fn ui_events() {
    let app = app();
    let base = serve(&app).await;
    let mut response = reqwest::get(format!("{base}events")).await.unwrap();
    let mut text = String::new();
    read_until(&mut response, &mut text, ": offen").await;
    let state = McpState {
        addr: "127.0.0.1:5190".to_owned(),
        listening: true,
        error: String::new(),
    };
    app.emit(MCP_STATE, Some(serde_json::to_value(&state).unwrap()));
    app.emit(PLANNING, None);
    read_until(&mut response, &mut text, "data: null").await;
    let lines: Vec<&str> = text.lines().filter(|l| !l.is_empty() && !l.starts_with(':')).collect();
    // Die Schlüssel stehen alphabetisch, nicht in Go-Reihenfolge; für JSON.parse gleich.
    let data: Value = serde_json::from_str(lines[1].strip_prefix("data: ").unwrap()).unwrap();
    assert_eq!(data, serde_json::to_value(&state).unwrap());
    assert_eq!(
        [lines[0], lines[2], lines[3]],
        ["event: mcp:state", "event: planning:changed", "data: null"]
    );
}

async fn read_until(response: &mut reqwest::Response, text: &mut String, needle: &str) {
    while !text.contains(needle) {
        let chunk = tokio::time::timeout(Duration::from_secs(5), response.chunk())
            .await
            .expect("Frist")
            .unwrap()
            .expect("Strom zu Ende");
        text.push_str(&String::from_utf8_lossy(&chunk));
    }
}
