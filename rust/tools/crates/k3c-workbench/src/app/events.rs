//! Ereignisse an die Oberfläche (Port von `emitAll`, `app.go`/`uiapi.go`): im Fenster per `AppHandle::emit` (Senke
//! [`Emit`]) und immer an den Bus der Oberflächen-API. Weitere Namen kommen mit den Seiten, die sie senden.

use std::sync::Arc;

use k3c_kit::uikit::Bus;
use serde::Serialize;
use serde_json::Value;

/// Zustand des MCP-Servers.
pub const MCP_STATE: &str = "mcp:state";
/// Ein Tool-Aufruf beginnt.
pub const MCP_START: &str = "mcp:start";
/// Ein Tool-Aufruf ist beendet.
pub const MCP_CALL: &str = "mcp:call";
/// Hinweis eines Agenten (`notify_ui`).
pub const NOTIFY: &str = "ui:notify";
/// Planungsdateien haben sich geändert.
pub const PLANNING: &str = "planning:changed";
/// Zustand eines Dienstes.
pub const SERVICE_STATE: &str = "service:state";
/// Zustand eines Task-Laufs.
pub const TASK_STATE: &str = "task:state";
/// Zustand einer Quelle der Logs-Seite.
pub const SOURCE_STATE: &str = "source:state";
/// Neue Konsolenzeilen, gebündelt als Liste.
pub const CONSOLE_LINE: &str = "console:line";

/// Zusätzliche Senke der Ereignisse (im Fenster `AppHandle::emit`).
pub type Emit = Arc<dyn Fn(&str, Value) + Send + Sync>;

/// Verteiler, den auch die Rückrufe des Servers halten.
#[derive(Clone)]
pub struct Events {
    bus: Arc<Bus>,
    forward: Option<Emit>,
}

impl Events {
    pub fn new(bus: Arc<Bus>, forward: Option<Emit>) -> Events {
        Events { bus, forward }
    }

    pub fn bus(&self) -> Arc<Bus> {
        Arc::clone(&self.bus)
    }

    /// Sendet `data`; ohne Daten sieht die Oberfläche `null`.
    pub fn emit(&self, name: &str, data: Option<Value>) {
        if let Some(forward) = &self.forward {
            forward(name, data.clone().unwrap_or(Value::Null));
        }
        self.bus.publish(name, data.into_iter().collect());
    }

    /// [`Events::emit`] mit einem Wert in seiner JSON-Form.
    pub fn emit_value(&self, name: &str, data: &impl Serialize) {
        match serde_json::to_value(data) {
            Ok(value) => self.emit(name, Some(value)),
            // Im Dev-Build steht der Fehler in der Konsole; die eigenen Typen lassen sich immer kodieren.
            Err(e) => eprintln!("❌ ereignis {name}: {e}"),
        }
    }
}
