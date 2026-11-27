//! Command `rpc` und Ereignis `mcp:state` ohne Bildschirm über die Tauri-Mock-Laufzeit (Muster Spike).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Duration;

use k3c_workbench::app::window::{AppState, app_for, builder};
use serde_json::{Value, json};
use tauri::test::{INVOKE_KEY, MockRuntime, get_ipc_response, mock_builder, mock_context, noop_assets};
use tauri::{Listener, Manager, WebviewWindow};

/// Temporäre Ordner liegen im Repo: unter `K3C_TMP`, sonst `.work/tmp`.
fn tmp_dir() -> tempfile::TempDir {
    let base = std::env::var_os("K3C_TMP").filter(|v| !v.is_empty()).map_or_else(
        || PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../.work/tmp")),
        PathBuf::from,
    );
    std::fs::create_dir_all(&base).unwrap();
    tempfile::Builder::new()
        .prefix("k3c-workbench-")
        .tempdir_in(base)
        .unwrap()
}

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
fn rpc_info_antwortet() {
    let root = tmp_dir();
    let app = builder(mock_builder()).build(mock_context(noop_assets())).unwrap();
    let (tx, rx) = channel();
    app.listen("mcp:state", move |event| {
        let _ = tx.send(event.payload().to_string());
    });
    let workbench = app_for(app.handle(), root.path().to_path_buf(), 0).with_usage_file(root.path().join("usage.json"));
    let workbench = Arc::new(workbench);
    app.manage(AppState(Arc::clone(&workbench)));
    // Der Server lebt auf der Laufzeit von Tauri.
    tauri::async_runtime::block_on(workbench.startup());
    let event: Value = serde_json::from_str(&rx.recv_timeout(Duration::from_secs(5)).unwrap()).unwrap();
    assert_eq!(event["listening"], json!(true), "{event}");

    let window = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();
    let rpc = |method: &str| invoke(&window, "rpc", json!({ "method": method, "args": [] }));
    let info = rpc("Info").unwrap();
    assert_eq!(info, json!({ "version": "0.1.0", "mcp": event }));
    // Eine Methode, die den Server in seiner Laufzeit fragt.
    assert_eq!(rpc("McpOverview").unwrap()["mcp"], event);
    assert_eq!(rpc("Nichts"), Err(json!("Unbekannte Methode Nichts")));
    // Abgelehnt, ohne etwas zu öffnen.
    let refused = invoke(
        &window,
        "open_url",
        json!({ "url": "file:///C:/Windows/System32/calc.exe" }),
    );
    assert_eq!(refused, Err(json!("Adresse abgelehnt: nur http und https")));
    tauri::async_runtime::block_on(workbench.shutdown());
}
