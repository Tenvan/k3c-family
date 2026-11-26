//! Betrieb ohne Fenster (Port von `headless.go` ohne `-dev`): MCP-Server und Oberflächen-API auf
//! `127.0.0.1:<K3C_STUDIO_RS_UI_API_PORT>`, die Oberfläche läuft im Browser unter Vite.

use std::path::PathBuf;
use std::sync::Arc;

use k3c_kit::uikit;
use serde_json::json;

use super::{App, rpc};
use crate::ports;

/// Vorgabe des UI-Ports: ohne eigenen Wert folgt er dem MCP-Port (Versatz der Worktree-Dienste).
fn ui_default(mcp_port: u16) -> u16 {
    if mcp_port == 0 {
        return ports::UI_API_PORT;
    }
    let offset = i32::from(mcp_port) - i32::from(ports::MCP_PORT);
    u16::try_from(i32::from(ports::UI_API_PORT) + offset).unwrap_or(ports::UI_API_PORT)
}

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
    let raw = std::env::var(ports::UI_API_PORT_ENV).unwrap_or_default();
    let ui_port = uikit::resolve_port(ports::UI_API_PORT_ENV, &raw, ui_default(port))?;
    let listener = uikit::listen(ports::UI_API_PORT_ENV, ui_port).await?;
    let app = Arc::new(App::new(root, port, None));
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
        "🚀 k3c-studio ohne Fenster: MCP {} · Oberflächen-API {url}",
        app.mcp_url()
    );
    let result = uikit::serve(listener, rpc::api(&app).router(), interrupted()).await;
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

#[cfg(test)]
mod tests {
    use super::ui_default;

    #[test]
    fn ui_port_folgt_dem_mcp_port() {
        assert_eq!(ui_default(5281), 8293);
        assert_eq!(ui_default(5291), 8303);
        assert_eq!(ui_default(0), 8293);
        assert_eq!(ui_default(65535), 8293);
    }
}
