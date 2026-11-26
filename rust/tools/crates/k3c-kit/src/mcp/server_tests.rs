//! Server-Tests über echtes Streamable HTTP. In Go testen Workbench und Studio das Verhalten; die erwarteten Texte
//! stammen vom laufenden Go-Server (go-sdk 1.8.0, jsonschema-go 0.4.3).

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rmcp::handler::server::common::Extension;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use serde::Deserialize;
use serde_json::{Value, json};

use super::*;
use crate::mcp::{PANIC_TEXT, Progress};
use crate::testutil::tmp_dir;
use crate::usage::Tracker;

#[derive(Clone)]
pub(super) struct App;

#[derive(Deserialize, schemars::JsonSchema)]
pub(super) struct Echo {
    text: String,
}

#[tool_router]
impl App {
    #[tool(description = "Echo.", annotations(read_only_hint = true))]
    async fn echo(&self, Parameters(args): Parameters<Echo>) -> String {
        args.text
    }

    #[tool(description = "Schreibt.")]
    async fn schreib(&self, Parameters(args): Parameters<Echo>, Extension(ws): Extension<Workspace>) -> String {
        format!("{} in {}", args.text, ws.short())
    }

    #[tool(description = "Panik.")]
    async fn panik(&self) -> String {
        panic!("kaputt {}", 7)
    }

    #[tool(description = "Fortschritt.")]
    async fn fortschritt(&self, Extension(progress): Extension<Progress>) -> String {
        progress.report("läuft", Duration::from_millis(1500));
        // Die Meldung geht in einer eigenen Task raus, die Antwort soll ihr nicht zuvorkommen.
        tokio::time::sleep(Duration::from_millis(200)).await;
        format!("aktiv {}", progress.is_active())
    }
}

/// Ordner, der für `find_root` wie ein Checkout aussieht.
pub(super) fn checkout_dir(dir: &Path) {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(dir.join("go.mod"), "module k3c\n").unwrap();
}

pub(super) fn options(root: &Path) -> Options {
    Options {
        name: "test".to_owned(),
        version: "1".to_owned(),
        instructions: "# Anleitung".to_owned(),
        root: root.to_path_buf(),
        env_port: "K3C_TEST_PORT".to_owned(),
        takes_checkout: Some(Arc::new(|tool| tool == "schreib")),
        write_tool: Some(Arc::new(|tool| tool == "schreib")),
        ..Options::default()
    }
}

pub(super) async fn started(options: Options) -> Arc<Server<App>> {
    let server = Server::new(options, App, App::tool_router(), None);
    server.start().await.unwrap();
    server
}

/// MCP-Client mit einer Session.
pub(super) struct Client {
    http: reqwest::Client,
    url: String,
    session: Option<String>,
    /// Kopf `X-K3C-Root`.
    pub(super) root: Option<String>,
    id: u64,
}

impl Client {
    pub(super) fn new(url: String) -> Client {
        Client {
            http: reqwest::Client::new(),
            url,
            session: None,
            root: None,
            id: 0,
        }
    }

    async fn post(&self, body: &Value) -> reqwest::Response {
        let mut request = self
            .http
            .post(&self.url)
            .header("Accept", "application/json, text/event-stream")
            .header("MCP-Protocol-Version", "2025-06-18");
        if let Some(session) = &self.session {
            request = request.header("Mcp-Session-Id", session);
        }
        if let Some(root) = &self.root {
            request = request.header("X-K3C-Root", root);
        }
        let send = request.json(body).timeout(Duration::from_secs(10)).send();
        send.await.unwrap()
    }

    /// `initialize` und `notifications/initialized`; liefert das Ergebnis und den Status der Benachrichtigung.
    pub(super) async fn initialize(&mut self) -> (Value, u16) {
        let params = json!({"protocolVersion": "2025-06-18", "capabilities": {},
            "clientInfo": {"name": "t", "version": "0"}});
        let (answer, _) = self.rpc("initialize", params).await;
        let note = json!({"jsonrpc": "2.0", "method": "notifications/initialized"});
        let status = self.post(&note).await.status().as_u16();
        (answer, status)
    }

    /// Anfrage; liefert die Antwort und alle Nachrichten, die davor im Stream kamen.
    pub(super) async fn rpc(&mut self, method: &str, params: Value) -> (Value, Vec<Value>) {
        self.id += 1;
        let body = json!({"jsonrpc": "2.0", "id": self.id, "method": method, "params": params});
        let response = self.post(&body).await;
        if let Some(session) = response.headers().get("mcp-session-id") {
            self.session = Some(session.to_str().unwrap().to_owned());
        }
        let text = response.text().await.unwrap();
        let mut before = Vec::new();
        for line in text.lines() {
            let data = line.strip_prefix("data:").unwrap_or(line).trim();
            let Ok(message) = serde_json::from_str::<Value>(data) else {
                continue;
            };
            if message["id"] == json!(self.id) {
                return (message, before);
            }
            before.push(message);
        }
        panic!("keine Antwort auf {method}: {text}")
    }

    /// `tools/call`; liefert die Antwort.
    pub(super) async fn call(&mut self, tool: &str, arguments: Value) -> Value {
        let params = json!({"name": tool, "arguments": arguments});
        self.rpc("tools/call", params).await.0
    }
}

/// Texte eines Ergebnisses und ob es ein Fehler-Ergebnis ist.
pub(super) fn result(answer: &Value) -> (Vec<String>, bool) {
    let content = answer["result"]["content"]
        .as_array()
        .unwrap_or_else(|| panic!("kein Ergebnis: {answer}"));
    let texts = content.iter().map(|c| c["text"].as_str().unwrap().to_owned());
    (texts.collect(), answer["result"]["isError"] == json!(true))
}

#[tokio::test(flavor = "multi_thread")]
async fn initialize_liste_und_aufruf() {
    let root = tmp_dir();
    checkout_dir(root.path());
    let server = started(options(root.path())).await;
    assert!(server.url().starts_with("http://127.0.0.1:") && server.url().ends_with("/mcp"));
    assert_eq!(server.root(), root.path());
    let mut client = Client::new(server.url());
    let (init, status) = client.initialize().await;
    assert_eq!(init["result"]["serverInfo"]["name"], "test", "{init}");
    assert_eq!(init["result"]["instructions"], "# Anleitung", "{init}");
    assert!(init["result"]["capabilities"]["tools"].is_object(), "{init}");
    assert!(init["result"]["capabilities"]["prompts"].is_null(), "{init}");
    assert_eq!(status, 202);
    assert_eq!(server.clients().await, 1);

    let (list, _) = client.rpc("tools/list", json!({})).await;
    let tools = list["result"]["tools"].as_array().unwrap();
    let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(names, ["echo", "fortschritt", "panik", "schreib"]);
    assert!(tools[3]["inputSchema"]["properties"]["checkout"].is_object(), "{list}");
    assert!(tools[0]["inputSchema"]["properties"]["checkout"].is_null(), "{list}");
    let (prompts, _) = client.rpc("prompts/list", json!({})).await;
    assert_eq!(prompts["result"]["prompts"], json!([]), "{prompts}");

    let answer = client.call("echo", json!({"text": "hallo"})).await;
    assert_eq!(result(&answer), (vec!["hallo".to_owned()], false));
    assert!(answer["result"]["isError"].is_null(), "{answer}");
    server.stop().await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn ablehnungen_wie_im_go_sdk_und_panik() {
    let root = tmp_dir();
    checkout_dir(root.path());
    let server = started(options(root.path())).await;
    let mut client = Client::new(server.url());
    client.initialize().await;

    let (texts, is_error) = result(&client.call("echo", json!({"text": "a", "txt": "b"})).await);
    assert!(is_error);
    assert_eq!(
        texts,
        [
            r#"validating "arguments": validating root: unexpected additional properties ["txt"]"#,
            "gültige Parameter: text"
        ]
    );
    let (texts, _) = result(&client.call("panik", json!({"x": 1})).await);
    assert_eq!(texts[1], "gültige Parameter: keine");

    // Falscher Typ: wie in Go ein Fehler-Ergebnis, kein Protokollfehler (der Text kommt von rmcp).
    let (texts, is_error) = result(&client.call("echo", json!({"text": 5})).await);
    assert!(
        is_error && texts[0].starts_with("failed to deserialize parameters:"),
        "{texts:?}"
    );

    let answer = client.call("nichtda", json!({})).await;
    assert_eq!(answer["error"]["code"], -32602, "{answer}");
    assert_eq!(answer["error"]["message"], r#"unknown tool "nichtda""#, "{answer}");

    let (texts, is_error) = result(&client.call("panik", json!({})).await);
    assert_eq!((texts, is_error), (vec![format!("{PANIC_TEXT}: kaputt 7")], true));
    let answer = client.call("echo", json!({"text": "noch da"})).await;
    assert_eq!(result(&answer), (vec!["noch da".to_owned()], false));
    server.stop().await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn zaehler_aufruf_log_nutzung_und_messung() {
    let root = tmp_dir();
    checkout_dir(root.path());
    let usage = Arc::new(Tracker::new(Arc::new(chrono::Local::now)));
    let tracked = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&tracked);
    let mut opts = options(root.path());
    opts.usage = Some(Arc::clone(&usage));
    opts.catalog = ["schreib", "gibt-es-nicht", "echo"].map(str::to_owned).to_vec();
    opts.track = Some(Arc::new(move |name: &str| {
        let (seen, name) = (Arc::clone(&seen), name.to_owned());
        Box::new(move || seen.lock().unwrap().push(name))
    }));
    let server = started(opts).await;
    let mut client = Client::new(server.url());
    client.initialize().await;
    client.call("echo", json!({"text": "eins"})).await;
    client.call("panik", json!({})).await;
    client.call("nichtda", json!({})).await;

    let stats = server.stats().await;
    let names: Vec<&str> = stats.tools.iter().map(|t| t.name.as_str()).collect();
    assert_eq!(names, ["schreib", "echo", "fortschritt", "panik"]);
    assert_eq!((stats.total_calls, stats.errors, stats.in_flight), (3, 2, 0));
    assert_eq!((stats.clients, stats.peak_clients), (1, 1));
    assert_eq!(stats.tools[1].calls, 1);
    let calls = server.calls();
    let log: Vec<(&str, bool)> = calls.iter().map(|c| (c.tool.as_str(), c.ok)).collect();
    assert_eq!(log, [("nichtda", false), ("panik", false), ("echo", true)]);
    assert_eq!(calls[2].args, r#"{"text":"eins"}"#);
    assert!(server.uptime().num_milliseconds() >= 0);

    let session = usage.snapshot().session;
    let mut used: Vec<&str> = session.tools.iter().map(|t| t.name.as_str()).collect();
    used.sort_unstable();
    assert_eq!(used, ["echo", "panik"]);
    assert_eq!(*tracked.lock().unwrap(), ["tool:echo", "tool:panik", "tool:nichtda"]);
    server.stop().await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn fortschritt_nur_mit_token() {
    let root = tmp_dir();
    checkout_dir(root.path());
    let server = started(options(root.path())).await;
    let mut client = Client::new(server.url());
    client.initialize().await;
    let params = json!({"name": "fortschritt", "arguments": {}, "_meta": {"progressToken": "p1"}});
    let (answer, before) = client.rpc("tools/call", params).await;
    assert_eq!(result(&answer).0, ["aktiv true"]);
    let note = before.iter().find(|m| m["method"] == "notifications/progress");
    let note = &note.unwrap_or_else(|| panic!("keine Meldung: {before:?}"))["params"];
    assert_eq!(note["progressToken"], "p1");
    assert_eq!(note["message"], "läuft");
    assert_eq!(note["progress"], json!(1.5));
    let answer = client.call("fortschritt", json!({})).await;
    assert_eq!(result(&answer).0, ["aktiv false"]);
    server.stop().await.unwrap();
}

#[test]
fn eigener_melder_fuer_tests() {
    let got = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&got);
    let progress = Progress::from_fn(Arc::new(move |msg: &str, elapsed| {
        seen.lock().unwrap().push((msg.to_owned(), elapsed));
    }));
    progress.report("a", Duration::from_secs(1));
    assert!(progress.is_active() && !Progress::none().is_active());
    assert_eq!(*got.lock().unwrap(), [("a".to_owned(), Duration::from_secs(1))]);
}

#[tokio::test(flavor = "multi_thread")]
async fn neustart_stopp_und_belegter_port() {
    let root = tmp_dir();
    checkout_dir(root.path());
    let stops = Arc::new(Mutex::new(0));
    let counted = Arc::clone(&stops);
    let mut opts = options(root.path());
    opts.before_stop = Some(Arc::new(move || *counted.lock().unwrap() += 1));
    opts.mount = Some(Arc::new(|router: axum::Router| {
        router.route("/ping", axum::routing::get(|| async { "pong" }))
    }));
    let server = started(opts).await;
    assert_eq!(server.start().await.unwrap_err(), "server läuft bereits");
    let mut client = Client::new(server.url());
    client.initialize().await;
    client.call("echo", json!({"text": "a"})).await;
    let ping = server.url().replace("/mcp", "/ping");
    assert_eq!(reqwest::get(ping).await.unwrap().text().await.unwrap(), "pong");

    let port = server
        .url()
        .trim_end_matches("/mcp")
        .rsplit(':')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let mut taken = options(root.path());
    taken.port = port;
    let error = Server::new(taken, App, ToolRouter::new(), None)
        .start()
        .await
        .unwrap_err();
    assert!(error.starts_with(&format!("port {port} nicht verfügbar (")), "{error}");
    assert!(
        error.ends_with("); einen anderen Port über K3C_TEST_PORT setzen"),
        "{error}"
    );

    server.restart().await.unwrap();
    assert_eq!(server.stats().await.total_calls, 1);
    assert_eq!(server.clients().await, 0);
    let mut client = Client::new(server.url());
    client.initialize().await;
    client.call("echo", json!({"text": "b"})).await;
    assert_eq!(server.stats().await.total_calls, 2);
    assert_eq!(server.clients().await, 1);

    server.stop().await.unwrap();
    assert_eq!(server.url(), "");
    assert_eq!(server.clients().await, 0);
    server.stop().await.unwrap();
    assert_eq!(*stops.lock().unwrap(), 3);
}

#[test]
fn port_aus_der_umgebung() {
    assert_eq!(resolve_port("K3C_X", "  ", 5280), Ok(5280));
    assert_eq!(resolve_port("K3C_X", " 6000 ", 5280), Ok(6000));
    for raw in ["0", "65536", "-1", "abc"] {
        let expected = format!("K3C_X=\"{raw}\" ist keine gültige Portzahl (1–65535)");
        assert_eq!(resolve_port("K3C_X", raw, 5280), Err(expected));
    }
}

#[test]
fn hilfen_fuer_tools() {
    let annotations = serde_json::to_value(crate::mcp::read_only()).unwrap();
    assert_eq!(
        annotations,
        json!({"readOnlyHint": true, "idempotentHint": true, "openWorldHint": false})
    );
    let image = serde_json::to_value(crate::mcp::image_result(b"png", "Bild")).unwrap();
    assert_eq!(
        image["content"],
        json!([{"type": "image", "data": "cG5n", "mimeType": "image/png"}, {"type": "text", "text": "Bild"}])
    );
    assert!(image["isError"].is_null(), "{image}");
}
