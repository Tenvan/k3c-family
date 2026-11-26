//! MCP-Server des Spikes über das offizielle Rust-SDK `rmcp` (Streamable HTTP, Pfad `/mcp`, nur Loopback).
//! Zwei Beispiel-Tools wie in der Go-Workbench: `workbench_status` (lesend) und `notify_ui` (Brücke MCP → Oberfläche).

use crate::log;
use crate::state::{BUILD_TAG, Shared};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use rmcp::{ErrorData, ServerHandler, schemars, tool, tool_handler, tool_router};
use serde::Deserialize;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::net::TcpListener;

/// Vorgabe-Port des Spikes (die echte Workbench nutzt 5180, das Studio 5181).
pub const DEFAULT_PORT: u16 = 5280;

#[derive(Clone)]
pub struct SpikeMcp {
    shared: Arc<Shared>,
}

/// Argumente von `notify_ui` (wie in der Go-Workbench).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct NotifyArgs {
    /// info, success, warn oder error
    pub level: String,
    /// Überschrift des Hinweises
    pub title: String,
    /// optionaler Text
    pub text: Option<String>,
}

impl SpikeMcp {
    fn count(&self, tool: &str) {
        let n = self.shared.calls.fetch_add(1, Ordering::SeqCst) + 1;
        log::line(&format!("📨 MCP-Tool {tool} aufgerufen (Aufruf {n})"));
        self.shared.emit_mcp_state();
    }
}

#[tool_router]
impl SpikeMcp {
    #[tool(description = "Zustand des Spikes: Adresse, Laufzeit, Aufrufe, Build. Nutze es bei: Verbindungstest. Statt: Prozessliste.")]
    async fn workbench_status(&self) -> String {
        self.count("workbench_status");
        let st = self.shared.mcp_state();
        format!(
            "k3c-spike-tauri (Tauri 2, rmcp)\nAdresse: {}\nLaufzeit: {} ms\nAufrufe: {}\nBuild: {BUILD_TAG}",
            st.addr,
            log::since_start_ms(),
            st.calls
        )
    }

    #[tool(description = "Hinweis an den Nutzer in der Oberfläche (info, success, warn, error). Nutze es bei: lange Arbeit fertig. Statt: nichts.")]
    async fn notify_ui(&self, Parameters(args): Parameters<NotifyArgs>) -> Result<String, ErrorData> {
        if !matches!(args.level.as_str(), "info" | "success" | "warn" | "error") {
            return Err(ErrorData::invalid_params(format!("level {:?} ungültig: info, success, warn, error", args.level), None));
        }
        self.count("notify_ui");
        let payload = serde_json::json!({ "level": args.level, "title": args.title, "text": args.text });
        self.shared.emit("ui:notify", payload);
        Ok(format!("Hinweis gesendet: {}", args.title))
    }
}

#[tool_handler(name = "k3c-spike-tauri", instructions = "Spike RU65.1: Tauri 2 + rmcp. Tools: workbench_status, notify_ui.")]
impl ServerHandler for SpikeMcp {}

/// Port binden (0 = frei wählen) und den Zustand setzen; Fehler landen im Zustand wie in Go.
pub async fn bind(shared: &Shared, port: u16) -> std::io::Result<TcpListener> {
    match TcpListener::bind(("127.0.0.1", port)).await {
        Ok(l) => {
            let addr = l.local_addr()?;
            *shared.addr.lock().expect("addr") = format!("http://{addr}/mcp");
            shared.listening.store(true, Ordering::SeqCst);
            log::line(&format!("🔌 MCP lauscht auf http://{addr}/mcp"));
            Ok(l)
        }
        Err(e) => {
            *shared.error.lock().expect("error") = e.to_string();
            log::line(&format!("❌ MCP-Port {port} belegt: {e}"));
            Err(e)
        }
    }
}

/// Streamable-HTTP-Dienst auf dem gebundenen Port bedienen (läuft bis zum Prozessende).
pub async fn serve(shared: Arc<Shared>, listener: TcpListener) -> std::io::Result<()> {
    let s = shared.clone();
    let service = StreamableHttpService::new(
        move || Ok(SpikeMcp { shared: s.clone() }),
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default(),
    );
    shared.emit_mcp_state();
    let router = axum::Router::new().nest_service("/mcp", service);
    axum::serve(listener, router).await
}
