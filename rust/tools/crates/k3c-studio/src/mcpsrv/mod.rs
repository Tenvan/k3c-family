//! MCP-Server von k3c-studio (Port von `mcpsrv/server.go`) auf dem Gerüst `k3c_kit::mcp` (HTTP an 127.0.0.1, Zähler und
//! Aufruf-Log, Checkout je Aufruf). Den Katalog führt nur `tools.rs`.

mod checkout_policy;
mod kit;
mod tools;
mod tools_gamedata;

use std::path::PathBuf;
use std::sync::Arc;

use crate::ports;
pub use kit::{Call, HEADER_ROOT, Snapshot, find_root};
use kit::{KitServer, Logger, Options, Tracker, resolve_port_env};
pub use tools::Studio;

/// Name des Servers für die Clients (Tool-Präfix `mcp__k3c-studio__`).
pub const NAME: &str = "k3c-studio";
/// Text, den jeder Client beim Verbinden bekommt (Markdown; Byte-Kopie der Go-Datei).
pub const INSTRUCTIONS: &str = include_str!("instructions.md");

/// Liest den Port aus dem Wert der Umgebungsvariable; leer heißt [`ports::MCP_PORT`].
pub fn resolve_port(raw: &str) -> Result<u16, String> {
    resolve_port_env(ports::MCP_PORT_ENV, raw, ports::MCP_PORT)
}

/// Beschreibung eines Servers. Port 0 wählt einen freien Port (Tests).
pub struct Config {
    /// Repo-Wurzel.
    pub root: PathBuf,
    pub port: u16,
    pub version: String,
    pub log: Logger,
    pub usage: Option<Arc<Tracker>>,
}

/// MCP-Server des Studios.
pub struct Server {
    kit: Arc<KitServer<Studio>>,
}

impl Server {
    /// Baut den Server mit allen Tools aus dem Katalog; gestartet wird er mit [`Server::start`].
    pub fn new(cfg: Config) -> Server {
        let options = Options {
            name: NAME.to_owned(),
            version: cfg.version,
            instructions: INSTRUCTIONS.to_owned(),
            root: cfg.root,
            port: cfg.port,
            env_port: ports::MCP_PORT_ENV.to_owned(),
            log: cfg.log,
            usage: cfg.usage,
            takes_checkout: Some(Arc::new(checkout_policy::takes_checkout)),
            write_tool: Some(Arc::new(checkout_policy::write_tool)),
            catalog: tools::CATALOG.iter().map(|name| (*name).to_owned()).collect(),
            ..Options::default()
        };
        Server {
            kit: KitServer::new(options, Studio, tools::router(), None),
        }
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// Die Kopie der Anleitung bleibt gleich der Go-Datei, solange es sie gibt.
    #[test]
    fn instructions_wie_go() {
        let go =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../tools/k3c-studio/internal/mcpsrv/instructions.md");
        if !go.exists() {
            eprintln!("übersprungen: {} fehlt", go.display());
            return;
        }
        assert!(
            std::fs::read(go).unwrap() == super::INSTRUCTIONS.as_bytes(),
            "instructions.md weicht ab"
        );
    }

    #[test]
    fn port_aus_der_umgebung() {
        assert_eq!(super::resolve_port(""), Ok(5281));
        assert_eq!(super::resolve_port("5291"), Ok(5291));
        assert!(super::resolve_port("0").is_err());
    }
}
