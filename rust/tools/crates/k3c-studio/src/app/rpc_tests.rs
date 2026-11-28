//! Port von `uiapi_test.go`: Verdrahtung der Aufruf-Tabelle. Schutz, RPC-Mechanik und Ereignisstrom prüft
//! `k3c_kit::uikit`.

use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

use k3c_kit::uikit::{self, HEADER, PREFIX};
use serde_json::{Value, json};

use super::*;
use crate::app::VERSION;

fn app() -> Arc<App> {
    let app = App::new("unbenutzt".into(), 0, None);
    app.mark_ready();
    Arc::new(app)
}

/// Startet die UI-API auf einem freien Port.
async fn serve(app: &Arc<App>) -> String {
    let listener = uikit::listen("TEST_PORT", 0).await.unwrap();
    let url = uikit::page_url(&listener);
    let router = api(app).router();
    tokio::spawn(uikit::serve(listener, router, std::future::pending()));
    url
}

async fn post(base: &str, method: &str, body: &str) -> (u16, Value) {
    let url = format!("{base}{}rpc/{method}", PREFIX.trim_start_matches('/'));
    let response = reqwest::Client::new()
        .post(url)
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
    let app = app();
    let base = serve(&app).await;
    let (code, out) = post(&base, "Info", "[]").await;
    assert_eq!((code, &out["result"]["version"]), (200, &json!(VERSION)), "{out}");
    // Bindings der Workbench (WZ63) gibt es hier nicht.
    for method in ["Services", "PlanningData", "TicketCreate"] {
        let (code, out) = post(&base, method, "[]").await;
        assert_eq!(code, 404, "{method}: {out}");
    }
    // Falsche Argumente: 400 wie Go `decodeArgs`.
    let (code, out) = post(&base, "Info", "[1]").await;
    assert_eq!(code, 400, "{out}");
    assert_eq!(out["error"], json!("erwartet 0 Argumente, bekommen 1"));
}

#[test]
fn command_und_ui_api_nutzen_eine_tabelle() {
    let app = app();
    assert_eq!(call(&app, "Info", &[]).unwrap()["version"], json!(VERSION));
    assert_eq!(call(&app, "Nichts", &[]), Err("Unbekannte Methode Nichts".to_owned()));
    // `methods` ist eine HashMap: die Reihenfolge zählt nicht.
    let served: HashSet<String> = api(&app).methods.into_keys().collect();
    let table: HashSet<String> = METHODS.iter().map(|(name, _)| (*name).to_owned()).collect();
    assert_eq!(served, table);
}

/// Die Aufruf-Tabelle hat genau die Methoden der Liste `tests/data/ui-methoden.txt`.
#[test]
fn tabelle_wie_liste() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/ui-methoden.txt");
    let text = std::fs::read_to_string(path).unwrap();
    let wanted: HashSet<String> = text.lines().map(str::to_owned).collect();
    let table: HashSet<String> = METHODS.iter().map(|(name, _)| (*name).to_owned()).collect();
    assert_eq!(table.len(), METHODS.len(), "Methode doppelt in der Tabelle");
    assert_eq!(table, wanted);
}
