//! Belege ohne Bildschirm: Command + Event über die Tauri-Mock-Laufzeit, MCP-Tools über echtes Streamable HTTP.

use spike_lib::commands::AppState;
use spike_lib::state::Shared;
use spike_lib::{builder, mcp, shared_for};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::Listener;
use tauri::Manager;
use tauri::test::{INVOKE_KEY, get_ipc_response, mock_builder, mock_context, noop_assets};

#[test]
fn command_spike_ping_antwortet_und_sendet_event() {
    let app = builder(mock_builder()).build(mock_context(noop_assets())).expect("app");
    app.manage(AppState(shared_for(app.handle())));
    let (tx, rx) = std::sync::mpsc::channel();
    app.listen("spike:pong", move |e| {
        let _ = tx.send(e.payload().to_string());
    });
    let w = tauri::WebviewWindowBuilder::new(&app, "main", Default::default()).build().expect("window");
    let req = tauri::webview::InvokeRequest {
        cmd: "spike_ping".into(),
        callback: tauri::ipc::CallbackFn(0),
        error: tauri::ipc::CallbackFn(1),
        url: "http://tauri.localhost".parse().unwrap(),
        body: tauri::ipc::InvokeBody::from(serde_json::json!({ "msg": "test" })),
        headers: Default::default(),
        invoke_key: INVOKE_KEY.to_string(),
    };
    let res = get_ipc_response(&w, req).expect("ipc").deserialize::<String>().expect("string");
    assert_eq!(res, "pong: test");
    let ev = rx.recv_timeout(Duration::from_secs(2)).expect("event spike:pong");
    assert!(ev.contains("\"msg\":\"test\"") && ev.contains("\"n\":"), "{ev}");
}

/// POST an /mcp; liest den SSE- oder JSON-Körper, bis die Antwort mit `id` da ist.
async fn rpc(c: &reqwest::Client, url: &str, sid: &mut Option<String>, body: serde_json::Value) -> serde_json::Value {
    let id = body.get("id").cloned();
    let mut req = c
        .post(url)
        .header("Accept", "application/json, text/event-stream")
        .header("Content-Type", "application/json")
        .header("MCP-Protocol-Version", "2025-06-18");
    if let Some(s) = sid.as_ref() {
        req = req.header("Mcp-Session-Id", s);
    }
    let mut resp = req.json(&body).send().await.expect("send");
    assert!(resp.status().is_success(), "status {}", resp.status());
    if let Some(s) = resp.headers().get("mcp-session-id") {
        *sid = Some(s.to_str().unwrap().to_string());
    }
    let Some(id) = id else { return serde_json::Value::Null };
    let mut buf = String::new();
    loop {
        let chunk = tokio::time::timeout(Duration::from_secs(5), resp.chunk()).await.expect("timeout").expect("chunk");
        let Some(chunk) = chunk else { break };
        buf.push_str(&String::from_utf8_lossy(&chunk));
        for line in buf.lines() {
            let json = line.strip_prefix("data:").unwrap_or(line).trim();
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(json) {
                if v.get("id") == Some(&id) {
                    return v;
                }
            }
        }
    }
    panic!("keine Antwort für id {id}: {buf}")
}

#[tokio::test(flavor = "multi_thread")]
async fn mcp_streamable_http_tools_list_und_call() {
    let events = Arc::new(Mutex::new(Vec::<(String, String)>::new()));
    let ev = events.clone();
    let shared = Shared::new(Arc::new(move |e: &str, p: serde_json::Value| ev.lock().unwrap().push((e.into(), p.to_string()))));
    let listener = mcp::bind(&shared, 0).await.expect("bind");
    let url = shared.mcp_state().addr;
    tokio::spawn(mcp::serve(shared.clone(), listener));
    let c = reqwest::Client::new();
    let mut sid = None;
    let init = serde_json::json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{
        "protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"spike-test","version":"0"}}});
    let r = rpc(&c, &url, &mut sid, init).await;
    assert_eq!(r["result"]["serverInfo"]["name"], "k3c-spike-tauri", "{r}");
    rpc(&c, &url, &mut sid, serde_json::json!({"jsonrpc":"2.0","method":"notifications/initialized"})).await;
    let list = rpc(&c, &url, &mut sid, serde_json::json!({"jsonrpc":"2.0","id":2,"method":"tools/list"})).await;
    let names: Vec<_> = list["result"]["tools"].as_array().unwrap().iter().map(|t| t["name"].as_str().unwrap().to_string()).collect();
    assert!(names.contains(&"workbench_status".into()) && names.contains(&"notify_ui".into()), "{names:?}");
    let call = serde_json::json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"workbench_status","arguments":{}}});
    let r = rpc(&c, &url, &mut sid, call).await;
    assert!(r["result"]["content"][0]["text"].as_str().unwrap().contains("Aufrufe: 1"), "{r}");
    let call = serde_json::json!({"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"notify_ui","arguments":{"level":"success","title":"Hallo aus MCP"}}});
    rpc(&c, &url, &mut sid, call).await;
    let got = events.lock().unwrap().clone();
    assert!(got.iter().any(|(e, p)| e == "ui:notify" && p.contains("Hallo aus MCP")), "{got:?}");
    assert!(got.iter().any(|(e, p)| e == "mcp:state" && p.contains("\"calls\":2")), "{got:?}");
}
