//! Seite Dienste (Port von `app_services.go` und `openServices` aus `main.go`). Die Methoden blockieren und dürfen nur
//! aus einem blockierenden Thread gerufen werden.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use k3c_kit::console::Store;
use k3c_kit::logger::Logger;
use serde::Serialize;
use serde_json::json;
use tokio::runtime::Handle;

use super::events::{Events, SERVICE_STATE};
use super::{App, lock};
use crate::services::{self, Cancel, Controller, LevelCounts, Options, Outcome, Status};

/// So oft misst der Monitor.
const MONITOR_EVERY: Duration = Duration::from_secs(2);
/// Meldung, wenn eine Methode nach dem Beenden kommt.
const STOPPED: &str = "backend beendet";

/// Die Dienste-Seite beim Laden. `error` ist gesetzt, wenn `services.json` nicht geladen wurde.
#[derive(Clone, Debug, Serialize)]
pub struct ServicesView {
    pub services: Vec<Status>,
    pub error: String,
}

/// Dienste eines laufenden Backends: der Controller oder der Grund, warum es keine gibt.
pub(crate) type Services = Result<Arc<Controller>, String>;

/// `services.json` liegt, wo die Go-Workbench sie hat (RU71 verlegt sie); nur lesend.
fn config_file(root: &Path) -> PathBuf {
    root.join("tools").join("k3c-workbench").join("services.json")
}

/// Lädt `services.json`, übernimmt laufende Dienste und misst, bis `cancel` kommt. Eine kaputte Konfiguration hält den
/// Server nicht auf: dann gibt es keine Dienste.
pub(super) async fn open(root: &Path, store: Arc<Store>, log: &Logger, events: &Events, cancel: &Cancel) -> Services {
    let list = services::load(&config_file(root)).inspect_err(|e| {
        log.error(&format!("💥 dienste nicht geladen: {e}"), &[("ns", json!("svc"))]);
    })?;
    let events = events.clone();
    let options = Options {
        root: root.to_path_buf(),
        console: store,
        log: log.clone(),
        on_change: Arc::new(move |st| events.emit_value(SERVICE_STATE, st)),
        ..Options::default()
    };
    let ctl = Controller::new(services::shift(&list, 0), options);
    ctl.adopt().await;
    tokio::spawn(Arc::clone(&ctl).monitor(cancel.clone(), MONITOR_EVERY));
    Ok(ctl)
}

/// Controller samt der Laufzeit, in der seine Befehle laufen, und dem Abbruch beim Beenden.
struct Access {
    ctl: Arc<Controller>,
    runtime: Handle,
    cancel: Cancel,
}

impl App {
    fn access(&self) -> Result<Access, String> {
        self.wait();
        let running = lock(&self.running);
        let running = running.as_ref().ok_or_else(|| STOPPED.to_owned())?;
        Ok(Access {
            ctl: Arc::clone(running.services.as_ref().map_err(Clone::clone)?),
            runtime: running.runtime.clone(),
            cancel: running.cancel.clone(),
        })
    }

    /// Alle Dienste in der Reihenfolge der Konfiguration.
    pub fn services(&self) -> ServicesView {
        match self.access() {
            Ok(a) => ServicesView {
                services: a.ctl.statuses(),
                error: String::new(),
            },
            Err(error) => ServicesView {
                services: Vec::new(),
                error,
            },
        }
    }

    /// Liest `services.json` neu. Läuft ein geänderter Dienst, gilt seine neue Konfiguration erst nach dem nächsten
    /// Start (der Hinweis steht im Log).
    pub fn services_reload(&self) -> Result<ServicesView, String> {
        let a = self.access().map_err(|e| format!("{e}; k3c-workbench neu starten"))?;
        let log = self.logger();
        let list = services::load(&config_file(&self.root)).inspect_err(|e| {
            log.error(&format!("💥 dienste nicht neu geladen: {e}"), &[("ns", json!("svc"))]);
        })?;
        let pending = a.runtime.block_on(a.ctl.reload(services::shift(&list, 0)));
        log.info(
            "🔁 dienste neu geladen",
            &[("ns", json!("svc")), ("erstBeimNaechstenStart", json!(pending))],
        );
        Ok(ServicesView {
            services: a.ctl.statuses(),
            error: String::new(),
        })
    }

    /// Startet einen Dienst und wartet, bis er gesund ist. Den Zustand unterwegs melden Ereignisse.
    pub fn service_start(&self, name: &str) -> Result<Status, String> {
        let a = self.access()?;
        message(a.runtime.block_on(a.ctl.start(&a.cancel, name)))
    }

    /// Stoppt einen Dienst; einen übernommenen nur mit `force` (setzt die Oberfläche erst nach der Bestätigung).
    pub fn service_stop(&self, name: &str, force: bool) -> Result<Status, String> {
        let a = self.access()?;
        message(a.runtime.block_on(a.ctl.stop(name, force)))
    }

    /// Startet einen Dienst neu.
    pub fn service_restart(&self, name: &str) -> Result<Status, String> {
        let a = self.access()?;
        message(a.runtime.block_on(a.ctl.restart(&a.cancel, name)))
    }

    /// Startet alle, die nicht schon laufen, parallel.
    pub fn services_start_all(&self) -> Result<(), String> {
        let a = self.access()?;
        a.runtime.block_on(a.ctl.start_all(&a.cancel))
    }

    /// Stoppt alle eigenen rückwärts; übernommene bleiben.
    pub fn services_stop_all(&self) -> Result<(), String> {
        let a = self.access()?;
        a.runtime.block_on(a.ctl.stop_all());
        Ok(())
    }

    /// Log-Einträge der letzten 60 min je Level.
    pub fn service_log_levels(&self, name: &str) -> Result<LevelCounts, String> {
        let a = self.access()?;
        a.ctl.log_levels(name)?.ok_or_else(|| format!("{name} hat kein Log"))
    }
}

fn message(outcome: Outcome) -> Result<Status, String> {
    outcome.map_err(|e| e.message)
}

#[cfg(test)]
#[path = "services_tests.rs"]
mod tests;
