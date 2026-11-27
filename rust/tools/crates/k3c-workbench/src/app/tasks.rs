//! Seite Tasks (Port von `app_tasks.go`): Katalog und Läufe. [`TaskCenter`] teilen sich die Methoden der Oberfläche
//! und die Tools `task_*`; seine Methoden blockieren und dürfen nur aus einem blockierenden Faden gerufen werden.

use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use chrono::{DateTime, Local, SecondsFormat};
use k3c_kit::console::Store;
use k3c_kit::logger::Logger;
use serde::Serialize;
use serde_json::json;
use tokio::runtime::Handle;

use super::events::{Events, TASK_STATE};
use super::{App, lock};
use crate::mcp::TaskHost;
use crate::services::Controller;
use crate::taskcat::{self, Namespace, Task};
use crate::taskrun::{self, Run, Runner};

/// Fängt nur ein hängendes `task` ab; ein Katalog braucht unter einer Sekunde.
const LOAD_TIMEOUT: Duration = Duration::from_secs(10);
/// Begrenzt einen Stopp.
pub(crate) const STOP_TIMEOUT: Duration = Duration::from_secs(15);

/// Katalog der Tasks-Seite. `error` ist ein Feld, keine Ablehnung: nach einem Ladefehler bleibt der zuletzt bekannte
/// Baum sichtbar.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskCatalog {
    pub namespaces: Vec<Namespace>,
    pub count: usize,
    /// RFC3339, leer solange nie geladen.
    pub loaded_at: String,
    pub error: String,
}

/// Liest den Katalog; darf blockieren.
pub(crate) type Loader = Arc<dyn Fn() -> Result<Vec<Task>, String> + Send + Sync>;

#[derive(Default)]
struct Catalog {
    loaded: bool,
    tasks: Vec<Task>,
    loaded_at: Option<DateTime<Local>>,
    error: Option<String>,
}

/// Teile eines [`TaskCenter`].
pub(crate) struct Setup {
    pub runtime: Handle,
    pub log: Logger,
    /// Dienste, die vor einem Task enden (B-1214).
    pub services: Option<Arc<Controller>>,
    pub loader: Loader,
    pub runner: Runner,
}

/// Katalog und Läufe eines laufenden Backends.
pub(crate) struct TaskCenter {
    setup: Setup,
    catalog: Mutex<Catalog>,
}

/// Konsolen-Quelle eines Laufs.
fn source(name: &str) -> String {
    format!("task:{name}")
}

/// Runner, dessen Ausgabe als Quelle `task:<name>` im Puffer liegt und dessen Wechsel als `task:state` gehen.
pub(crate) fn runner_options(root: &Path, store: &Arc<Store>, events: &Events) -> taskrun::Options {
    let (lines, reset, events) = (Arc::clone(store), Arc::clone(store), events.clone());
    taskrun::Options {
        root: root.to_path_buf(),
        out: Arc::new(move |name: &str, stream: &str, text: &str| lines.add(&source(name), stream, text)),
        reset: Arc::new(move |name: &str| reset.reset(&source(name))),
        on_change: Arc::new(move |run: &Run| events.emit_value(TASK_STATE, run)),
        ..taskrun::Options::default()
    }
}

impl TaskCenter {
    pub(crate) fn new(setup: Setup) -> TaskCenter {
        TaskCenter {
            setup,
            catalog: Mutex::default(),
        }
    }

    /// Center mit echtem `task` in `root`.
    pub(crate) fn open(
        root: &Path,
        store: &Arc<Store>,
        events: &Events,
        log: &Logger,
        services: Option<Arc<Controller>>,
    ) -> TaskCenter {
        let runtime = Handle::current();
        let (handle, dir) = (runtime.clone(), root.to_path_buf());
        TaskCenter::new(Setup {
            runtime,
            log: log.clone(),
            services,
            loader: Arc::new(move || handle.block_on(taskcat::load(&dir, LOAD_TIMEOUT))),
            runner: Runner::new(runner_options(root, store, events)),
        })
    }

    fn catalog_state(&self) -> MutexGuard<'_, Catalog> {
        self.catalog.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn load(&self, c: &mut Catalog) {
        c.loaded = true;
        match (self.setup.loader)() {
            Ok(tasks) => {
                let count = tasks.len();
                (c.tasks, c.loaded_at, c.error) = (tasks, Some(Local::now()), None);
                let attrs = [("ns", json!("tasks")), ("anzahl", json!(count))];
                self.setup.log.info("📂 Task-Katalog geladen", &attrs);
            }
            // Der alte Stand bleibt stehen.
            Err(e) => {
                let message = format!("💥 Task-Katalog nicht ladbar: {e}");
                self.setup.log.warn(&message, &[("ns", json!("tasks"))]);
                c.error = Some(e);
            }
        }
    }

    fn loaded(&self, reload: bool) -> MutexGuard<'_, Catalog> {
        let mut c = self.catalog_state();
        if reload || !c.loaded {
            self.load(&mut c);
        }
        c
    }

    /// Der Katalog, beim ersten Aufruf oder mit `reload` frisch gelesen.
    pub(crate) fn catalog(&self, reload: bool) -> TaskCatalog {
        let c = self.loaded(reload);
        TaskCatalog {
            namespaces: taskcat::group(&c.tasks),
            count: c.tasks.len(),
            loaded_at: c
                .loaded_at
                .map_or_else(String::new, |at| at.to_rfc3339_opts(SecondsFormat::Secs, true)),
            error: c.error.clone().unwrap_or_default(),
        }
    }

    pub(crate) async fn stop_all(&self) -> Result<(), String> {
        self.setup.runner.stop_all(STOP_TIMEOUT).await
    }
}

impl TaskHost for TaskCenter {
    /// Für `task_list`; nach einem Ladefehler der Fehler.
    fn task_list(&self) -> Result<Vec<Task>, String> {
        let c = self.loaded(false);
        match &c.error {
            Some(e) => Err(e.clone()),
            None => Ok(c.tasks.clone()),
        }
    }

    /// Ein Name außerhalb des Katalogs wird vor dem Start abgelehnt, sonst stünde ein Lauf in der Liste, den `task`
    /// selbst erst nach dem Prozessstart verwirft.
    fn task_start(&self, name: &str, args: Vec<String>) -> Result<Run, String> {
        if !self.loaded(false).tasks.iter().any(|t| t.name == name) {
            return Err(format!(
                "unbekannter Task {name:?} (steht nicht im Katalog, task --list-all)"
            ));
        }
        let _runtime = self.setup.runtime.enter();
        if let Some(services) = &self.setup.services {
            // B-1214: laufendes Spiel aus dem Paket vor dem Neubau beenden (Hinweis im Log).
            self.setup.runtime.block_on(services.stop_for_task(name));
        }
        self.setup.runner.start(name, args)
    }

    fn task_stop(&self, name: &str) -> Result<Run, String> {
        self.setup.runtime.block_on(self.setup.runner.stop(name, STOP_TIMEOUT))
    }

    fn task_runs(&self) -> Vec<Run> {
        self.setup.runner.all()
    }
}

impl App {
    fn task_center(&self) -> Result<Arc<TaskCenter>, String> {
        self.wait();
        let running = lock(&self.running);
        running
            .as_ref()
            .map(|r| Arc::clone(&r.tasks))
            .ok_or_else(|| super::services::STOPPED.to_owned())
    }

    fn catalog(&self, reload: bool) -> TaskCatalog {
        match self.task_center() {
            Ok(center) => center.catalog(reload),
            Err(error) => TaskCatalog {
                error,
                ..TaskCatalog::default()
            },
        }
    }

    /// Der Katalog; beim ersten Aufruf geladen.
    pub fn tasks(&self) -> TaskCatalog {
        self.catalog(false)
    }

    /// Liest den Katalog neu; eine Taskfile-Änderung wirkt erst damit, kein Datei-Watcher.
    pub fn tasks_reload(&self) -> TaskCatalog {
        self.catalog(true)
    }

    /// Startet `task <name> [-- args...]`.
    pub fn task_start(&self, name: &str, args: Vec<String>) -> Result<Run, String> {
        self.task_center()?.task_start(name, args)
    }

    /// Beendet den Prozessbaum eines laufenden Tasks.
    pub fn task_stop(&self, name: &str) -> Result<Run, String> {
        self.task_center()?.task_stop(name)
    }

    /// Der aktuelle oder letzte Lauf je Task.
    pub fn task_runs(&self) -> Vec<Run> {
        self.task_center().map_or_else(|_| Vec::new(), |c| c.task_runs())
    }
}

#[cfg(test)]
#[path = "tasks_tests.rs"]
mod tests;
