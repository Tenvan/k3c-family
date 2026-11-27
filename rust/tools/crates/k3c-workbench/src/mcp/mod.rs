//! MCP-Server von k3c-workbench (Port von `mcpsrv/server.go`) auf dem Gerüst `k3c_kit::mcp` (HTTP an 127.0.0.1, Zähler
//! und Aufruf-Log, Checkout je Aufruf). Den Katalog führt nur `tools.rs`.

mod checkout_policy;
mod notify;
mod stats_target;
mod status;
mod tools;

#[cfg(test)]
mod checkout_tests;
#[cfg(test)]
mod roundtrip_tests;
#[cfg(test)]
mod rules_tests;
#[cfg(test)]
mod stats_tests;
#[cfg(test)]
pub(crate) mod test_client;
#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod workspace_tests;

use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use k3c_kit::logger::Logger;
use k3c_kit::mcp::{Call, CallHook, Options, Server as KitServer, Snapshot};
use k3c_kit::usage::Tracker;
use rmcp::handler::server::router::tool::ToolRouter;

use crate::ports;
pub use notify::{Notice, NotifyHook};
use tools::Workbench;

/// Name des Servers für die Clients (Tool-Präfix `mcp__k3c-workbench__`).
pub const NAME: &str = "k3c-workbench";
/// Text, den jeder Client beim Verbinden bekommt (Markdown; Byte-Kopie der Go-Datei).
pub const INSTRUCTIONS: &str = include_str!("instructions.md");

/// Beschreibung eines Servers. Port 0 wählt einen freien Port (Tests). Alle Funktionen sind optional.
#[derive(Clone)]
pub struct Config {
    /// Repo-Wurzel.
    pub root: PathBuf,
    pub port: u16,
    pub version: String,
    pub log: Logger,
    pub usage: Option<Arc<Tracker>>,
    /// Aufruf beginnt.
    pub on_start: Option<CallHook>,
    /// Aufruf beendet.
    pub on_call: Option<CallHook>,
    /// Hinweis an den Nutzer in der Oberfläche (`notify_ui`).
    pub on_notify: Option<NotifyHook>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            root: PathBuf::new(),
            port: 0,
            version: String::new(),
            log: Logger::discard(),
            usage: None,
            on_start: None,
            on_call: None,
            on_notify: None,
        }
    }
}

/// MCP-Server der Workbench.
pub struct Server {
    kit: Arc<KitServer<Workbench>>,
}

impl Server {
    /// Baut den Server mit allen Tools aus dem Katalog; gestartet wird er mit [`Server::start`].
    pub fn new(cfg: Config) -> Server {
        Server::with_router(cfg, tools::router())
    }

    /// Wie [`Server::new`] mit eigenem Router (Tests hängen weitere Tools an).
    fn with_router(cfg: Config, router: ToolRouter<Workbench>) -> Server {
        let own = Arc::new(OnceLock::new());
        let app = Workbench {
            server: Arc::clone(&own),
            usage: cfg.usage.clone(),
            on_notify: cfg.on_notify,
        };
        let options = Options {
            name: NAME.to_owned(),
            version: cfg.version,
            instructions: INSTRUCTIONS.to_owned(),
            root: cfg.root,
            port: cfg.port,
            env_port: ports::ENV_MCP_PORT.to_owned(),
            log: cfg.log,
            on_start: cfg.on_start,
            on_call: cfg.on_call,
            usage: cfg.usage,
            takes_checkout: Some(Arc::new(checkout_policy::takes_checkout)),
            write_tool: Some(Arc::new(checkout_policy::write_tool)),
            target: Some(Arc::new(stats_target::target_of)),
            catalog: tools::CATALOG.iter().map(|name| (*name).to_owned()).collect(),
            ..Options::default()
        };
        let kit = KitServer::new(options, app, router, None);
        // Frisch angelegt, also noch leer.
        let _ = own.set(Arc::downgrade(&kit));
        Server { kit }
    }

    /// Öffnet den HTTP-Server an 127.0.0.1 unter `/mcp`.
    pub async fn start(&self) -> Result<(), String> {
        self.kit.start().await
    }

    /// Schließt HTTP-Server und Sessions.
    pub async fn stop(&self) -> Result<(), String> {
        self.kit.stop().await
    }

    /// Startet den HTTP-Teil neu; Clients verbinden sich danach neu.
    pub async fn restart(&self) -> Result<(), String> {
        self.kit.restart().await
    }

    /// Adresse für `.mcp.json`, leer solange der Server nicht läuft.
    pub fn url(&self) -> String {
        self.kit.url()
    }

    /// Zähler; die Zahl der Clients wird dabei frisch gezählt.
    pub async fn stats(&self) -> Snapshot {
        self.kit.stats().await
    }

    /// Aufruf-Log, laufende Aufrufe eingeschlossen, neueste zuerst.
    pub fn calls(&self) -> Vec<Call> {
        self.kit.calls()
    }
}
