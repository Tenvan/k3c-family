//! MCP-Server auf `rmcp`: Streamable HTTP nur an 127.0.0.1 unter `/mcp` (Port von `mcpkit/server.go`). Die Tools
//! bringen die Apps als `ToolRouter` mit; der HTTP-Teil lässt sich neu starten, ohne dass Katalog und Zähler verloren
//! gehen. Den Ablauf je Aufruf beschreibt `observe.rs`.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use chrono::TimeDelta;
use rmcp::handler::server::router::prompt::PromptRouter;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::Extensions;
use rmcp::transport::streamable_http_server::session::SessionManager;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use serde_json::json;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use super::chunked::force_chunked;
use super::service::{Core, KitService};
use super::{Call, CallHook, Snapshot, TargetFn, Workspace, same_dir};
use crate::logger::Logger;
use crate::usage::Tracker;

/// Grenze für das Warten auf offene Verbindungen (Streams der Clients) beim Stoppen.
const STOP_TIMEOUT: Duration = Duration::from_secs(2);
/// Sessions ohne Anfrage (Agent ohne Abmelden beendet) werden geschlossen; sonst zählten sie für immer als Clients.
const SESSION_TIMEOUT: Duration = Duration::from_secs(30 * 60);

/// Ja/Nein zu einem Tool-Namen.
pub type ToolPredicate = Arc<dyn Fn(&str) -> bool + Send + Sync>;
/// Misst einen Aufruf (Name `tool:<name>`); die zurückgegebene Funktion beendet die Messung.
pub type TrackFn = Arc<dyn Fn(&str) -> Box<dyn FnOnce() + Send> + Send + Sync>;
/// Hängt weitere Routen an den HTTP-Server; `/mcp` ist belegt.
pub type MountFn = Arc<dyn Fn(axum::Router) -> axum::Router + Send + Sync>;
/// Läuft in [`Server::stop`] vor dem Schließen des HTTP-Servers.
pub type StopFn = Arc<dyn Fn() + Send + Sync>;

/// Beschreibung eines Servers. Port 0 wählt einen freien Port (Tests). Alle Funktionen sind optional.
#[derive(Clone)]
pub struct Options {
    /// Name des Servers für die Clients, z. B. k3c-workbench.
    pub name: String,
    pub version: String,
    /// Markdown, das jeder Client beim Verbinden bekommt.
    pub instructions: String,
    /// Repo-Wurzel; leer = unbekannt.
    pub root: PathBuf,
    pub port: u16,
    /// Umgebungsvariable des Ports, für die Meldung bei belegtem Port.
    pub env_port: String,
    /// Eigenes Log; Vorgabe verwirft alles.
    pub log: Logger,
    /// Aufruf beginnt.
    pub on_start: Option<CallHook>,
    /// Aufruf beendet.
    pub on_call: Option<CallHook>,
    pub usage: Option<Arc<Tracker>>,
    /// Tools mit dem Argument `checkout` im Schema (B-388).
    pub takes_checkout: Option<ToolPredicate>,
    /// Schreibende Tools; ihre Antwort endet mit der Zeile `Checkout: …` (B-275).
    pub write_tool: Option<ToolPredicate>,
    /// Liest das Ziel eines Aufrufs aus den rohen Argumenten ins Aufruf-Log.
    pub target: Option<TargetFn>,
    pub track: Option<TrackFn>,
    pub mount: Option<MountFn>,
    pub before_stop: Option<StopFn>,
    /// Reihenfolge der Tools in den Zählern. Leer = alphabetisch; Namen ohne Route zählen nicht, Routen ohne Namen
    /// folgen alphabetisch.
    pub catalog: Vec<String>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            name: String::new(),
            version: String::new(),
            instructions: String::new(),
            root: PathBuf::new(),
            port: 0,
            env_port: String::new(),
            log: Logger::discard(),
            on_start: None,
            on_call: None,
            usage: None,
            takes_checkout: None,
            write_tool: None,
            target: None,
            track: None,
            mount: None,
            before_stop: None,
            catalog: Vec::new(),
        }
    }
}

/// Liest den Port aus dem Wert der Umgebungsvariable `env`; leer heißt `def`.
pub fn resolve_port(env: &str, raw: &str, def: u16) -> Result<u16, String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(def);
    }
    match raw.parse::<u16>() {
        Ok(port) if port > 0 => Ok(port),
        _ => Err(format!("{env}={raw:?} ist keine gültige Portzahl (1–65535)")),
    }
}

/// Der laufende HTTP-Teil.
struct Running {
    addr: SocketAddr,
    sessions: Arc<LocalSessionManager>,
    /// Beendet die Streams von `rmcp`.
    cancel: Box<dyn FnOnce() + Send>,
    shutdown: oneshot::Sender<()>,
    task: JoinHandle<()>,
}

/// MCP-Server samt HTTP-Server, der ihn ausliefert. [`Server::start`] braucht eine Tokio-Laufzeit.
pub struct Server<A: Clone + Send + Sync + 'static> {
    core: Arc<Core<A>>,
    /// Nie über ein `await` halten.
    running: Mutex<Option<Running>>,
}

impl<A: Clone + Send + Sync + 'static> Server<A> {
    /// Baut den Server; gestartet wird er mit [`Server::start`].
    pub fn new(opts: Options, app: A, tools: ToolRouter<A>, prompts: Option<PromptRouter<A>>) -> Arc<Server<A>> {
        Arc::new(Server {
            core: Arc::new(Core::new(opts, app, tools, prompts)),
            running: Mutex::new(None),
        })
    }

    fn running(&self) -> MutexGuard<'_, Option<Running>> {
        self.running.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn busy(&self, error: &std::io::Error) -> String {
        let opts = &self.core.opts;
        format!(
            "port {} nicht verfügbar ({error}); einen anderen Port über {} setzen",
            opts.port, opts.env_port
        )
    }

    /// Öffnet den HTTP-Server an 127.0.0.1 unter `/mcp`.
    pub async fn start(&self) -> Result<(), String> {
        let mut running = self.running();
        if running.is_some() {
            return Err("server läuft bereits".to_owned());
        }
        let listener = std::net::TcpListener::bind(("127.0.0.1", self.core.opts.port)).map_err(|e| self.busy(&e))?;
        listener.set_nonblocking(true).map_err(|e| self.busy(&e))?;
        let listener = TcpListener::from_std(listener).map_err(|e| self.busy(&e))?;
        let addr = listener.local_addr().map_err(|e| self.busy(&e))?;
        let mut manager = LocalSessionManager::default();
        manager.session_config.keep_alive = Some(SESSION_TIMEOUT);
        let sessions = Arc::new(manager);
        let (router, cancel) = self.router(&sessions);
        let (shutdown, stopped) = oneshot::channel::<()>();
        let log = self.core.opts.log.clone();
        let task = tokio::spawn(async move {
            let signal = async {
                // Auch ein fallengelassener Sender beendet den Server.
                let _ = stopped.await;
            };
            if let Err(e) = axum::serve(listener, router).with_graceful_shutdown(signal).await {
                let attrs = [("ns", json!("mcp")), ("error", json!(e.to_string()))];
                log.error("💥 server beendet", &attrs);
            }
        });
        *running = Some(Running {
            addr,
            sessions,
            cancel,
            shutdown,
            task,
        });
        Ok(())
    }

    /// Routen eines Starts und die Funktion, die ihre Streams abbricht.
    fn router(&self, sessions: &Arc<LocalSessionManager>) -> (axum::Router, Box<dyn FnOnce() + Send>) {
        let config = StreamableHttpServerConfig::default();
        let token = config.cancellation_token.clone();
        let core = Arc::clone(&self.core);
        let manager = Arc::clone(sessions);
        let service = StreamableHttpService::new(
            move || Ok(KitService::new(Arc::clone(&core), Arc::clone(&manager))),
            Arc::clone(sessions),
            config,
        );
        let mut router = axum::Router::new()
            .route_service("/mcp", service)
            .layer(axum::middleware::from_fn(force_chunked));
        if let Some(mount) = &self.core.opts.mount {
            router = mount(router);
        }
        (router, Box::new(move || token.cancel()))
    }

    /// Schließt den HTTP-Server (Annahme bis 2 s Frist) und alle Sessions, damit sie nach einem Neustart nicht weiter als
    /// Clients zählen. Abweichung von Go: Schon offene Verbindungen (Streams von `mount`-Routen) trennt das nicht hart.
    pub async fn stop(&self) -> Result<(), String> {
        if let Some(before_stop) = &self.core.opts.before_stop {
            before_stop();
        }
        let Some(mut running) = self.running().take() else {
            return Ok(());
        };
        (running.cancel)();
        // Fehlt der Empfänger, ist der Server schon beendet.
        let _ = running.shutdown.send(());
        if tokio::time::timeout(STOP_TIMEOUT, &mut running.task).await.is_err() {
            running.task.abort();
        }
        let ids: Vec<_> = running.sessions.sessions.read().await.keys().cloned().collect();
        let mut result = Ok(());
        for id in ids {
            if let Err(e) = running.sessions.close_session(&id).await {
                result = Err(e.to_string());
            }
        }
        result
    }

    /// Startet den HTTP-Teil neu; Clients verbinden sich danach neu.
    pub async fn restart(&self) -> Result<(), String> {
        self.stop().await?;
        let log = &self.core.opts.log;
        if let Err(e) = self.start().await {
            log.error(
                "💥 neustart fehlgeschlagen",
                &[("ns", json!("mcp")), ("error", json!(e))],
            );
            return Err(e);
        }
        log.info(
            "🔁 server neu gestartet",
            &[("ns", json!("mcp")), ("url", json!(self.url()))],
        );
        Ok(())
    }

    /// Adresse für `.mcp.json`, leer solange der Server nicht läuft.
    pub fn url(&self) -> String {
        match self.running().as_ref() {
            Some(running) => format!("http://{}/mcp", running.addr),
            None => String::new(),
        }
    }

    /// Zahl der offenen Sessions.
    pub async fn clients(&self) -> usize {
        let sessions = self.running().as_ref().map(|r| Arc::clone(&r.sessions));
        match sessions {
            Some(manager) => manager.sessions.read().await.len(),
            None => 0,
        }
    }

    /// Zähler; die Zahl der Clients wird dabei frisch gezählt.
    pub async fn stats(&self) -> Snapshot {
        self.core.stats.observe_clients(self.clients().await);
        self.core.stats.snapshot()
    }

    /// Aufruf-Log, laufende Aufrufe eingeschlossen, neueste zuerst.
    pub fn calls(&self) -> Vec<Call> {
        self.core.stats.calls()
    }

    /// Laufzeit seit [`Server::new`].
    pub fn uptime(&self) -> TimeDelta {
        self.core.stats.uptime()
    }

    /// Repo-Wurzel des Servers.
    pub fn root(&self) -> &Path {
        &self.core.opts.root
    }

    /// Checkout des laufenden Aufrufs aus dessen Erweiterungen; ohne Zuordnung die Repo-Wurzel.
    pub fn workspace(&self, extensions: &Extensions) -> Workspace {
        extensions.get::<Workspace>().cloned().unwrap_or_else(|| Workspace {
            root: self.core.opts.root.clone(),
            name: String::new(),
        })
    }

    /// Ob der Aufruf im Checkout dieses Servers läuft (B-924, B-925). Tools, die Code und Daten des eigenen Prozesses
    /// rechnen, lehnen sonst ab.
    pub fn own_checkout(&self, workspace: &Workspace) -> bool {
        let own = &self.core.opts.root;
        workspace.root.as_os_str().is_empty() || own.as_os_str().is_empty() || same_dir(&workspace.root, own)
    }
}

#[cfg(test)]
#[path = "server_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "server_tests_checkout.rs"]
mod tests_checkout;
