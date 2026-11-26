//! Backend des Studios (Port von `app.go`): es hostet den MCP-Server k3c-studio und das eigene Log; die Methoden der
//! Oberfläche stehen in der Aufruf-Tabelle [`rpc`]. Ohne Tauri-Typen, damit Fenster und Headless-Betrieb dasselbe nutzen.

mod assets;
pub mod headless;
mod hub;
pub mod open_url;
pub mod rpc;
mod studio;
pub mod window;
mod world;

use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};

use k3c_kit::applog::{self, Log};
use k3c_kit::console::{self, Store};
use k3c_kit::logger::Logger;
use k3c_kit::uikit::Bus;
use k3c_kit::usage::Tracker;
use serde::Serialize;
use serde_json::{Value, json};

use crate::assetsui::PruefMemo;
use crate::hubplan;
use crate::studioview::Game;
use crate::world::{EngineFactory, Watcher, exe_factory};
use crate::{mcpsrv, ports};

/// Version der App (Info, MCP).
pub const VERSION: &str = "0.1.0";
/// Name der App in der UI-API (das eigene Log heißt [`crate::ports::LOG_NAME`]).
pub const LOG_SOURCE: &str = "k3c-studio";
/// Ereignis: Zustand des MCP-Servers.
pub const EV_MCP_STATE: &str = "mcp:state";
/// Ereignis: Welt- oder Basis-Dateien haben sich geändert.
pub const EV_WORLD: &str = "world:changed";

/// Zusätzliche Senke der Ereignisse (im Fenster `AppHandle::emit`); an den [`Bus`] gehen sie immer.
pub type Emit = Arc<dyn Fn(&str, Value) + Send + Sync>;

/// Zustand des MCP-Servers für das Badge der Kopfzeile.
#[derive(Clone, Debug, Default, Serialize)]
pub struct McpState {
    pub addr: String,
    pub listening: bool,
    pub error: String,
}

/// Was die Oberfläche beim Laden abfragt.
#[derive(Clone, Debug, Serialize)]
pub struct Info {
    pub version: String,
    pub mcp: McpState,
}

/// Was zwischen [`App::startup`] und [`App::shutdown`] lebt.
struct Running {
    store: Arc<Store>,
    log: Log,
    tracker: Arc<Tracker>,
    server: Arc<mcpsrv::Server>,
    /// Welten-Wächter; er endet, wenn er fallen gelassen wird.
    watcher: Watcher,
}

/// Backend des Studios.
pub struct App {
    root: PathBuf,
    port: u16,
    usage_file: PathBuf,
    bus: Arc<Bus>,
    forward: Option<Emit>,
    /// Wird am Ende von `startup` wahr; Aufrufe der Oberfläche können davor kommen.
    ready: (Mutex<bool>, Condvar),
    mcp: Mutex<McpState>,
    running: Mutex<Option<Running>>,
    /// „Im Spiel ansehen“ im Bereich Assets (WB1.4), angelegt beim ersten Aufruf.
    game: Mutex<Option<Arc<Game>>>,
    /// Engine-Seite des Welten-Editors der Oberfläche.
    world_engine: EngineFactory,
    /// Engine-Seite des Gebäude-Planers der Oberfläche.
    hub_engine: hubplan::EngineFactory,
    /// Gemerkte Prüfergebnisse des Asset-Katalogs (B-732).
    asset_memo: PruefMemo,
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

fn emit_to(forward: Option<&Emit>, bus: &Bus, name: &str, data: Value) {
    if let Some(forward) = forward {
        forward(name, data.clone());
    }
    bus.publish(name, vec![data]);
}

/// Datei unter `<%APPDATA%>/k3c/` wie Go `windowstate.ConfigPath`.
fn config_path(root: &Path, name: &str) -> PathBuf {
    let base = std::env::var_os("APPDATA")
        .filter(|v| !v.is_empty())
        .map_or_else(|| root.join(".work").join("config"), PathBuf::from);
    base.join("k3c").join(name)
}

impl App {
    /// App für die Repo-Wurzel `root` mit dem MCP-Port `port`.
    pub fn new(root: PathBuf, port: u16, forward: Option<Emit>) -> App {
        let usage_file = config_path(&root, ports::USAGE_FILE);
        App {
            root,
            port,
            usage_file,
            bus: Arc::new(Bus::new()),
            forward,
            ready: (Mutex::new(false), Condvar::new()),
            mcp: Mutex::new(McpState::default()),
            running: Mutex::new(None),
            game: Mutex::new(None),
            world_engine: exe_factory(),
            hub_engine: hubplan::exe_factory(),
            asset_memo: PruefMemo::default(),
        }
    }

    /// Andere Datei für die Nutzungsstatistik (Tests).
    pub fn with_usage_file(mut self, path: PathBuf) -> App {
        self.usage_file = path;
        self
    }

    /// Andere Engine-Seite des Welten-Editors (Tests).
    #[cfg(test)]
    pub(crate) fn with_world_engine(mut self, engine: EngineFactory) -> App {
        self.world_engine = engine;
        self
    }

    /// Ereignisse für die HTTP-Oberfläche.
    pub fn bus(&self) -> Arc<Bus> {
        Arc::clone(&self.bus)
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

    /// Sendet ein Ereignis an die Oberfläche (Fenster und Ströme der HTTP-Oberfläche).
    pub fn emit(&self, name: &str, data: Value) {
        emit_to(self.forward.as_ref(), &self.bus, name, data);
    }

    /// Öffnet Log und Statistik und startet den MCP-Server. Ein belegter Port hält die App nicht auf: der Grund steht
    /// im Log und im Badge.
    pub async fn startup(&self) {
        let store = Arc::new(Store::new(console::DEFAULT_CAPACITY, None));
        let (log, log_error) = applog::open_named(&self.root.join("logs"), ports::LOG_NAME, Arc::clone(&store));
        if let Some(e) = log_error {
            eprintln!("k3c-studio: eigenes Log nur im Speicher: {e}");
        }
        let logger = log.logger().clone();
        let warn = logger.clone();
        let tracker = Tracker::open(
            &self.usage_file,
            Arc::new(chrono::Local::now),
            Arc::new(move |e: &str| warn.warn(&format!("📄 {e}"), &[("ns", json!("usage"))])),
        );
        // Wie Go vor dem Server.
        let watcher = self.watch_worlds();
        let server = Arc::new(mcpsrv::Server::new(mcpsrv::Config {
            root: self.root.clone(),
            port: self.port,
            version: VERSION.to_owned(),
            log: logger.clone(),
            usage: Some(Arc::clone(&tracker)),
        }));
        let started = server.start().await;
        match &started {
            Ok(()) => logger.info(
                "🚀 k3c-studio gestartet",
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
            watcher,
        });
        self.set_mcp(started.err());
        self.mark_ready();
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

    /// Beendet „Im Spiel ansehen“, Server, Statistik und Log (Fenster und Betrieb ohne Fenster).
    pub async fn shutdown(&self) {
        self.close_studio();
        let taken = lock(&self.running).take();
        let Some(running) = taken else {
            return;
        };
        let logger = running.log.logger();
        logger.info("🛑 k3c-studio beendet", &[("ns", json!("main"))]);
        drop(running.watcher);
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
        self.emit(EV_MCP_STATE, json!(state));
    }

    /// Version und MCP-Zustand (Methode `Info`).
    pub fn info(&self) -> Info {
        self.wait();
        Info {
            version: VERSION.to_owned(),
            mcp: lock(&self.mcp).clone(),
        }
    }
}
