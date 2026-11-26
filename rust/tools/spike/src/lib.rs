//! RU65.1 · Spike Tauri 2: Workbench-Frontend, Rust-Commands und -Events, MCP-Server über `rmcp`.

pub mod commands;
pub mod log;
pub mod mcp;
pub mod state;

use state::Shared;
use std::sync::Arc;
use tauri::{Emitter, Manager, Runtime, WebviewUrl, WebviewWindowBuilder};

/// Commands registrieren; generisch, damit der Test die Mock-Laufzeit nutzen kann.
pub fn builder<R: Runtime>(b: tauri::Builder<R>) -> tauri::Builder<R> {
    b.invoke_handler(tauri::generate_handler![
        commands::info,
        commands::spike_ping,
        commands::ui_ready,
        commands::ui_event_ack
    ])
}

/// Zustand anlegen: Ereignisse gehen per `AppHandle::emit` an alle Fenster.
pub fn shared_for<R: Runtime>(app: &tauri::AppHandle<R>) -> Arc<Shared> {
    let handle = app.clone();
    Shared::new(Arc::new(move |event: &str, payload: serde_json::Value| {
        if let Err(e) = handle.emit(event, payload) {
            log::line(&format!("❌ emit {event}: {e}"));
        }
    }))
}

/// Fenster mit eigenem WebView2-Datenordner im Spike (nicht `%LOCALAPPDATA%`); optional CDP für Belege ohne Bildschirm.
fn open_window<R: Runtime>(app: &tauri::App<R>) -> tauri::Result<()> {
    let data = std::env::var_os("SPIKE_DATA_DIR").map(Into::into).unwrap_or_else(|| log::spike_dir().join("webview-data"));
    let mut w = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("K3C Workbench · Tauri-Spike")
        .inner_size(1400.0, 900.0)
        .data_directory(data);
    if let Ok(port) = std::env::var("SPIKE_CDP_PORT") {
        let args = format!("--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection --remote-debugging-port={port}");
        w = w.additional_browser_args(&args);
        log::line(&format!("🐛 CDP auf 127.0.0.1:{port}"));
    }
    w.build()?;
    log::line("🚀 Fenster erzeugt");
    Ok(())
}

/// Einstieg der EXE.
pub fn run() {
    log::init();
    log::line(&format!("🚀 Spike startet, Build {}", state::BUILD_TAG));
    builder(tauri::Builder::default())
        .setup(|app| {
            let shared = shared_for(app.handle());
            app.manage(commands::AppState(shared.clone()));
            open_window(app)?;
            let port = std::env::var("SPIKE_MCP_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(mcp::DEFAULT_PORT);
            tauri::async_runtime::spawn(async move {
                if let Ok(listener) = mcp::bind(&shared, port).await {
                    if let Err(e) = mcp::serve(shared, listener).await {
                        log::line(&format!("💥 MCP beendet: {e}"));
                    }
                } else {
                    shared.emit_mcp_state();
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("💥 Tauri-Laufzeit");
}
