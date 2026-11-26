//! Tauri-Commands des Spikes: Gegenstück zu den Wails-Bindings (`window.go.main.App.*`).

use crate::log;
use crate::state::{Info, Shared};
use serde::Serialize;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

static PINGS: AtomicU64 = AtomicU64::new(0);

/// Verwalteter Zustand im Tauri-App-Handle.
pub struct AppState(pub Arc<Shared>);

/// Nutzlast von `spike:pong`.
#[derive(Clone, Debug, Serialize)]
pub struct Pong {
    pub msg: String,
    pub n: u64,
}

/// `info` wie Go `App.Info`: Version und MCP-Zustand.
#[tauri::command]
pub fn info(state: tauri::State<'_, AppState>) -> Info {
    let info = state.0.info();
    log::line(&format!("✅ Command info aufgerufen → {}", info.version));
    info
}

/// Command mit Argument und Event als Antwort: gibt das Echo zurück und sendet `spike:pong`.
#[tauri::command]
pub fn spike_ping(msg: String, state: tauri::State<'_, AppState>) -> String {
    let n = PINGS.fetch_add(1, Ordering::SeqCst) + 1;
    log::line(&format!("✅ Command spike_ping({msg:?}) Nr. {n}"));
    let pong = Pong { msg: msg.clone(), n };
    state.0.emit("spike:pong", serde_json::to_value(pong).unwrap_or_default());
    format!("pong: {msg}")
}

/// Meldung des Frontends nach dem ersten Rendern (Startzeit).
#[tauri::command]
pub fn ui_ready(perf_ms: f64, href: String) {
    log::line(&format!("🚦 UI bereit: {href} · performance.now {perf_ms:.0} ms · seit Prozessstart {} ms", log::since_start_ms()));
}

/// Quittung des Frontends: ein Event von Rust ist im JavaScript angekommen.
#[tauri::command]
pub fn ui_event_ack(event: String, payload: serde_json::Value) {
    log::line(&format!("✅ UI hat Event empfangen: {event} {payload}"));
}
