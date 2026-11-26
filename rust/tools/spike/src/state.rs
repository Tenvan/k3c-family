//! Gemeinsamer Zustand von Fenster-Commands und MCP-Server (wie `Info`/`McpState` der Go-Workbench).

use serde::Serialize;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// Stand dieses Builds; beim Hot-Reload-Test wird er geändert und muss im Log erscheinen.
pub const BUILD_TAG: &str = "spike-2";

/// Nutzlast von `mcp:state` und Teil von `info` (Frontend-Typ `McpState`, zusätzliche Felder stören nicht).
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct McpState {
    pub addr: String,
    pub listening: bool,
    pub error: String,
    pub calls: u64,
}

/// Antwort des Commands `info` (Frontend-Typ `Info`).
#[derive(Clone, Debug, Serialize)]
pub struct Info {
    pub version: String,
    pub mcp: McpState,
}

/// Senke für Ereignisse an die Oberfläche: im Fenster `AppHandle::emit`, im Test eine Liste.
pub type UiSink = Arc<dyn Fn(&str, serde_json::Value) + Send + Sync>;

/// Zustand, den Commands und MCP-Tools teilen.
pub struct Shared {
    pub addr: Mutex<String>,
    pub listening: AtomicBool,
    pub error: Mutex<String>,
    pub calls: AtomicU64,
    pub ui: UiSink,
}

impl Shared {
    pub fn new(ui: UiSink) -> Arc<Self> {
        Arc::new(Self {
            addr: Mutex::new(String::new()),
            listening: AtomicBool::new(false),
            error: Mutex::new(String::new()),
            calls: AtomicU64::new(0),
            ui,
        })
    }

    pub fn mcp_state(&self) -> McpState {
        McpState {
            addr: self.addr.lock().map(|a| a.clone()).unwrap_or_default(),
            listening: self.listening.load(Ordering::SeqCst),
            error: self.error.lock().map(|e| e.clone()).unwrap_or_default(),
            calls: self.calls.load(Ordering::SeqCst),
        }
    }

    pub fn info(&self) -> Info {
        Info { version: format!("{} ({})", env!("CARGO_PKG_VERSION"), BUILD_TAG), mcp: self.mcp_state() }
    }

    /// Ereignis an die Oberfläche senden und protokollieren.
    pub fn emit(&self, event: &str, payload: serde_json::Value) {
        crate::log::line(&format!("📨 Event an UI: {event} {payload}"));
        (self.ui)(event, payload);
    }

    pub fn emit_mcp_state(&self) {
        let st = self.mcp_state();
        self.emit("mcp:state", serde_json::to_value(st).unwrap_or_default());
    }
}
