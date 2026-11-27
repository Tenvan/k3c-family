//! Betrieb ohne Fenster (Port von `runHeadless` aus `headless.go`): MCP-Server und Oberflächen-API auf
//! `127.0.0.1:<K3C_RS_DEV_UI_API_PORT>`, die Oberfläche läuft im Browser unter Vite.

use std::path::PathBuf;
use std::sync::Arc;

use k3c_kit::uikit;
use serde_json::json;

use super::{App, config_path};
use crate::{ports, rpc};

/// Wartet auf Strg+C oder Strg+Untbr; ohne Konsole läuft der Prozess, bis er beendet wird.
async fn interrupted() {
    #[cfg(windows)]
    if let Ok(mut stop) = tokio::signal::windows::ctrl_break() {
        tokio::select! {
            () = ctrl_c() => {}
            _ = stop.recv() => {}
        }
        return;
    }
    ctrl_c().await;
}

async fn ctrl_c() {
    if tokio::signal::ctrl_c().await.is_err() {
        std::future::pending::<()>().await;
    }
}

async fn serve(root: PathBuf, port: u16) -> Result<(), String> {
    let raw = std::env::var(ports::ENV_UI_API_PORT).unwrap_or_default();
    let ui_port = uikit::resolve_port(ports::ENV_UI_API_PORT, &raw, ports::UI_API_PORT)?;
    let listener = uikit::listen(ports::ENV_UI_API_PORT, ui_port).await?;
    let usage = config_path(&root, ports::USAGE_FILE_HEADLESS);
    let app = Arc::new(App::new(root, port, None).with_usage_file(usage));
    app.startup().await;
    let page = uikit::page_url(&listener);
    let url = format!("{page}{}", uikit::PREFIX.trim_start_matches('/'));
    let log = app.logger();
    log.info(
        "🚀 Oberflächen-API bereit",
        &[("ns", json!("main")), ("url", json!(url)), ("seite", json!(page))],
    );
    // Konsole zusätzlich: wer von Hand startet, sieht die Adressen.
    println!(
        "🚀 k3c-workbench ohne Fenster: MCP {} · Oberflächen-API {url}",
        app.mcp_url()
    );
    let result = uikit::serve(listener, rpc::uiapi::router(&app), interrupted()).await;
    if let Err(e) = &result {
        log.error(
            "💥 Oberflächen-API beendet",
            &[("ns", json!("main")), ("error", json!(e.to_string()))],
        );
    }
    app.shutdown().await;
    result.map_err(|e| e.to_string())
}

/// Läuft bis Strg+C.
pub fn run(root: PathBuf, port: u16) -> Result<(), String> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| e.to_string())?;
    runtime.block_on(serve(root, port))
}
