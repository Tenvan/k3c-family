//! Was die Oberfläche beim Laden abfragt (Methode `Info`, Port aus `app.go`).

use serde::Serialize;

use super::{App, VERSION, lock};

/// Zustand des MCP-Servers für das Badge der Kopfzeile.
#[derive(Clone, Debug, Default, Serialize)]
pub struct McpState {
    pub addr: String,
    pub listening: bool,
    pub error: String,
}

/// Version und MCP-Zustand.
#[derive(Clone, Debug, Serialize)]
pub struct Info {
    pub version: String,
    pub mcp: McpState,
}

impl App {
    /// Version und MCP-Zustand; wartet auf `startup`.
    pub fn info(&self) -> Info {
        self.wait();
        Info {
            version: VERSION.to_owned(),
            mcp: lock(&self.mcp).clone(),
        }
    }
}
