//! Command `rpc` und Ereignis `mcp:state` ohne Bildschirm über die Tauri-Mock-Laufzeit (Muster Spike).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Duration;

use k3c_studio::app::window::{AppState, app_for, builder};
use serde_json::{Value, json};
use tauri::test::{INVOKE_KEY, MockRuntime, get_ipc_response, mock_builder, mock_context, noop_assets};
use tauri::{Listener, Manager, WebviewWindow};

#[path = "support/tmp.rs"]
mod tmp;

fn invoke(window: &WebviewWindow<MockRuntime>, cmd: &str, body: Value) -> Result<Value, Value> {
    let request = tauri::webview::InvokeRequest {
        cmd: cmd.into(),
        callback: tauri::ipc::CallbackFn(0),
        error: tauri::ipc::CallbackFn(1),
        url: "http://tauri.localhost".parse().unwrap(),
        body: tauri::ipc::InvokeBody::from(body),
        headers: Default::default(),
        invoke_key: INVOKE_KEY.to_string(),
    };
    get_ipc_response(window, request).map(|body| body.deserialize().unwrap())
}

#[test]
fn command_rpc_antwortet_und_sendet_mcp_state() {
    let root = tmp::tmp_dir();
    let app = builder(mock_builder()).build(mock_context(noop_assets())).unwrap();
    let (tx, rx) = channel();
    app.listen("mcp:state", move |event| {
        let _ = tx.send(event.payload().to_string());
    });
    let studio = app_for(app.handle(), root.path().to_path_buf(), 0).with_usage_file(root.path().join("usage.json"));
    let studio = Arc::new(studio);
    app.manage(AppState(Arc::clone(&studio)));
    // Der Server lebt auf der Laufzeit von Tauri.
    tauri::async_runtime::block_on(studio.startup());
    let event: Value = serde_json::from_str(&rx.recv_timeout(Duration::from_secs(5)).unwrap()).unwrap();
    assert_eq!(event["listening"], json!(true), "{event}");

    let window = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();
    let info = invoke(&window, "rpc", json!({ "method": "Info", "args": [] })).unwrap();
    assert_eq!(info["version"], json!("0.1.0"), "{info}");
    assert_eq!(info["mcp"], event, "{info}");
    assert_eq!(
        invoke(&window, "rpc", json!({ "method": "Nichts", "args": [] })),
        Err(json!("Unbekannte Methode Nichts"))
    );
    // Abgelehnt, ohne etwas zu öffnen.
    let refused = invoke(
        &window,
        "open_url",
        json!({ "url": "file:///C:/Windows/System32/calc.exe" }),
    );
    assert_eq!(refused, Err(json!("Adresse abgelehnt: nur http und https")));
    tauri::async_runtime::block_on(studio.shutdown());
}
