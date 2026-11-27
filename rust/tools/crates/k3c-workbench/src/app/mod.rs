//! Backend der Workbench (Port von `app.go`): es hostet den MCP-Server und das eigene Log; die Methoden der Oberfläche
//! stehen in der Aufruf-Tabelle [`crate::rpc`]. Ohne Tauri-Typen, damit Fenster und Betrieb ohne Fenster es teilen.

pub mod events;
pub mod headless;
mod info;
mod mcp_page;
pub mod open_url;
pub mod window;

#[cfg(test)]
mod app_tests;
#[cfg(test)]
mod winicon_tests;

use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};

use k3c_kit::applog::{self, Log};
use k3c_kit::console::{self, Store};
use k3c_kit::logger::Logger;
use k3c_kit::uikit::Bus;
use k3c_kit::usage::Tracker;
use serde_json::json;
use tokio::runtime::Handle;

use crate::{mcp, ports};
use events::{Emit, Events};
pub use info::{Info, McpState};
pub use mcp_page::{McpOverview, McpUsageView, UsageRules};

/// Version der App (Info, MCP).
pub const VERSION: &str = "0.1.0";
/// Name der App in der UI-API (das eigene Log heißt [`ports::LOG_NAME`]).
pub const APP_NAME: &str = "k3c-workbench";

/// Was zwischen [`App::startup`] und [`App::shutdown`] lebt.
struct Running {
    store: Arc<Store>,
    log: Log,
    tracker: Arc<Tracker>,
    server: Arc<mcp::Server>,
    /// Laufzeit des MCP-Servers; die blockierenden Methoden der Oberfläche rufen ihn darin.
    runtime: Handle,
}

/// Backend der Workbench.
pub struct App {
    root: PathBuf,
    port: u16,
    usage_file: PathBuf,
    events: Events,
    /// Wird am Ende von `startup` wahr; Aufrufe der Oberfläche können davor kommen.
    ready: (Mutex<bool>, Condvar),
    mcp: Mutex<McpState>,
    running: Mutex<Option<Running>>,
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Datei unter `<%APPDATA%>/k3c/` wie Go `windowstate.ConfigPath`.
pub fn config_path(root: &Path, name: &str) -> PathBuf {
    let base = std::env::var_os("APPDATA")
        .filter(|v| !v.is_empty())
        .map_or_else(|| root.join(".work").join("config"), PathBuf::from);
    base.join("k3c").join(name)
}

impl App {
    /// App für die Repo-Wurzel `root` mit dem MCP-Port `port`; `forward` bekommt die Ereignisse zusätzlich zum Bus.
    pub fn new(root: PathBuf, port: u16, forward: Option<Emit>) -> App {
        let usage_file = config_path(&root, ports::USAGE_FILE);
        App {
            root,
            port,
            usage_file,
            events: Events::new(Arc::new(Bus::new()), forward),
            ready: (Mutex::new(false), Condvar::new()),
            mcp: Mutex::new(McpState::default()),
            running: Mutex::new(None),
        }
    }

    /// Andere Datei für die Nutzungsstatistik (ohne Fenster, Tests).
    pub fn with_usage_file(mut self, path: PathBuf) -> App {
        self.usage_file = path;
        self
    }

    /// Ereignisse für die HTTP-Oberfläche.
    pub fn bus(&self) -> Arc<Bus> {
        self.events.bus()
    }

    /// Sendet ein Ereignis an die Oberfläche (Fenster und Ströme der HTTP-Oberfläche).
    pub fn emit(&self, name: &str, data: Option<serde_json::Value>) {
        self.events.emit(name, data);
    }

    /// Das eigene Log; vor `startup` und nach `shutdown` verwirft es alles.
    pub fn logger(&self) -> Logger {
        lock(&self.running)
            .as_ref()
            .map_or_else(Logger::discard, |r| r.log.logger().clone())
    }

    /// Adresse des laufenden MCP-Servers, sonst leer.
    pub fn mcp_url(&self) -> String {
        lock(&self.running)
            .as_ref()
            .map_or_else(String::new, |r| r.server.url())
    }

    /// Misst einen Aufruf der Oberfläche; bis zum Ressourcen-Monitor (RU69.15) ohne Wirkung.
    pub fn track(&self, _name: &str) -> Box<dyn FnOnce() + Send> {
        Box::new(|| {})
    }

    /// Öffnet Log und Statistik und startet den MCP-Server. Ein belegter Port hält die App nicht auf: der Grund steht
    /// im Log und im Badge.
    pub async fn startup(&self) {
        let store = Arc::new(Store::new(console::DEFAULT_CAPACITY, None));
        let (log, log_error) = applog::open_named(&self.root.join("logs"), ports::LOG_NAME, Arc::clone(&store));
        if let Some(e) = log_error {
            eprintln!("💥 k3c-workbench: eigenes Log nur im Speicher: {e}");
        }
        let logger = log.logger().clone();
        let tracker = self.open_tracker(&logger);
        let server = Arc::new(mcp::Server::new(self.server_config(&logger, &tracker)));
        let started = server.start().await;
        match &started {
            Ok(()) => logger.info(
                "🚀 k3c-workbench gestartet",
                &[
                    ("ns", json!("main")),
                    ("url", json!(server.url())),
                    ("version", json!(VERSION)),
                ],
            ),
            Err(e) => logger.error("💥 start fehlgeschlagen", &[("ns", json!("main")), ("error", json!(e))]),
        }
        *lock(&self.running) = Some(Running {
            store,
            log,
            tracker,
            server,
            runtime: Handle::current(),
        });
        self.set_mcp(started.err());
        self.mark_ready();
    }

    fn open_tracker(&self, logger: &Logger) -> Arc<Tracker> {
        let warn = logger.clone();
        Tracker::open(
            &self.usage_file,
            Arc::new(chrono::Local::now),
            Arc::new(move |e: &str| warn.warn(&format!("📄 {e}"), &[("ns", json!("usage"))])),
        )
    }

    fn server_config(&self, logger: &Logger, tracker: &Arc<Tracker>) -> mcp::Config {
        let (start, call, notify) = (self.events.clone(), self.events.clone(), self.events.clone());
        let log = logger.clone();
        mcp::Config {
            root: self.root.clone(),
            port: self.port,
            version: VERSION.to_owned(),
            log: logger.clone(),
            usage: Some(Arc::clone(tracker)),
            on_start: Some(Arc::new(move |c| start.emit_value(events::MCP_START, c))),
            on_call: Some(Arc::new(move |c| call.emit_value(events::MCP_CALL, c))),
            on_notify: Some(Arc::new(move |n| {
                let message = format!("📨 Hinweis an die Oberfläche: {}", n.title);
                log.info(&message, &[("ns", json!("mcp")), ("level", json!(n.level))]);
                notify.emit_value(events::NOTIFY, n);
            })),
        }
    }

    /// Gibt wartende Aufrufe frei.
    pub fn mark_ready(&self) {
        *lock(&self.ready.0) = true;
        self.ready.1.notify_all();
    }

    /// Blockiert, bis `startup` fertig ist.
    fn wait(&self) {
        let guard = lock(&self.ready.0);
        drop(self.ready.1.wait_while(guard, |ready| !*ready));
    }

    /// Beendet Server, Statistik und Log (Fenster und Betrieb ohne Fenster).
    pub async fn shutdown(&self) {
        let taken = lock(&self.running).take();
        let Some(running) = taken else {
            return;
        };
        let logger = running.log.logger();
        logger.info("🛑 k3c-workbench beendet", &[("ns", json!("main"))]);
        if let Err(e) = running.server.stop().await {
            logger.warn(
                "❌ server nicht sauber beendet",
                &[("ns", json!("mcp")), ("error", json!(e))],
            );
        }
        // Ein Fehler steht schon im Log (Rückruf).
        let _ = running.tracker.flush();
        let _ = running.log.close();
        running.store.close();
    }

    fn set_mcp(&self, error: Option<String>) {
        let state = McpState {
            addr: format!("127.0.0.1:{}", self.port),
            listening: error.is_none(),
            error: error.unwrap_or_default(),
        };
        *lock(&self.mcp) = state.clone();
        self.events.emit_value(events::MCP_STATE, &state);
    }
}
