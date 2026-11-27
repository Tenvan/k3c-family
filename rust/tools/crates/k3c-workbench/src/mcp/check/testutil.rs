//! Hilfen der `check_run`-Tests: gestellte Läufe statt echter Prozesse (Go `fakeServer`), ein Riegel, an dem ein Lauf
//! wartet, und ein mitschreibendes Log.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

use k3c_kit::console::Store;
use k3c_kit::logger::{Level, Logger, Record, Sink};
use k3c_kit::mcp::{Progress, Workspace};
use tempfile::TempDir;

use super::run::{RunResult, RunSpec};
use super::targets::DEV_FRONTEND_DEPS;
use super::{CheckHook, CheckIn, Checks, Hooks};
use crate::testutil::tmp_dir;

/// Hält einen gestellten Lauf an. Die Frist verhindert, dass ein roter Test die Laufzeit hängen lässt.
#[derive(Clone, Default)]
pub(super) struct Gate(Arc<(Mutex<bool>, Condvar)>);

impl Gate {
    pub(super) fn wait(&self) {
        let open = self.0.0.lock().unwrap();
        drop(
            self.0
                .1
                .wait_timeout_while(open, Duration::from_secs(20), |open| !*open),
        );
    }

    pub(super) fn open(&self) {
        *self.0.0.lock().unwrap() = true;
        self.0.1.notify_all();
    }
}

/// Log-Einträge als Zeilen `LEVEL meldung schlüssel=wert …`.
#[derive(Clone, Default)]
pub(super) struct Records(Arc<Mutex<Vec<String>>>);

impl Records {
    pub(super) fn logger(&self) -> Logger {
        Logger::new(vec![Box::new(self.clone())])
    }

    pub(super) fn text(&self) -> String {
        self.0.lock().unwrap().join("\n")
    }
}

impl Sink for Records {
    fn enabled(&self, _level: Level) -> bool {
        true
    }

    fn handle(&self, r: &Record) -> std::io::Result<()> {
        let attrs: Vec<String> = r.attrs.iter().map(|(k, v)| format!("{k}={v}")).collect();
        let line = format!("{} {} {}", r.level, r.msg, attrs.join(" "));
        self.0.lock().unwrap().push(line);
        Ok(())
    }
}

/// Hooks mit gestelltem Lauf (darf blockieren und in Panik geraten) und fester Task-Version; `started` zählt, wie oft
/// wirklich gestartet worden wäre.
pub(super) fn hooks(run: impl Fn(RunSpec) -> RunResult + Send + Sync + 'static) -> (Hooks, Arc<AtomicUsize>) {
    let run = Arc::new(run);
    let started = Arc::new(AtomicUsize::new(0));
    let count = Arc::clone(&started);
    let hooks = Hooks {
        run: Arc::new(move |spec| {
            count.fetch_add(1, Ordering::SeqCst);
            let run = Arc::clone(&run);
            Box::pin(async move {
                tokio::task::spawn_blocking(move || run(spec))
                    .await
                    .unwrap_or_else(|e| std::panic::resume_unwind(e.into_panic()))
            })
        }),
        // Unabhängig vom installierten Task.
        task_version: Arc::new(|| Box::pin(async { Ok("3.54.0".to_owned()) })),
        ..Hooks::default()
    };
    (hooks, started)
}

pub(super) struct Fake {
    pub checks: Arc<Checks>,
    pub ws: Workspace,
    pub started: Arc<AtomicUsize>,
    pub console: Arc<Store>,
    _root: TempDir,
}

impl Fake {
    pub(super) fn with(hooks: Hooks, started: Arc<AtomicUsize>, on_check: Option<CheckHook>) -> Fake {
        let root = tmp_dir();
        let console = Arc::new(Store::new(0, None));
        let checks = Checks::with_hooks(Logger::discard(), Arc::clone(&console), on_check, hooks);
        Fake {
            checks: Arc::new(checks),
            ws: Workspace {
                root: root.path().to_path_buf(),
                name: String::new(),
            },
            started,
            console,
            _root: root,
        }
    }

    pub(super) fn new(run: impl Fn(RunSpec) -> RunResult + Send + Sync + 'static) -> Fake {
        let (hooks, started) = hooks(run);
        Fake::with(hooks, started, None)
    }

    pub(super) fn started(&self) -> usize {
        self.started.load(Ordering::SeqCst)
    }

    pub(super) async fn wait_started(&self, n: usize) {
        for _ in 0..2000 {
            if self.started() >= n {
                return;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        panic!("Lauf {n} beginnt nicht");
    }

    /// Legt die Frontend-Abhängigkeiten an, ohne die die Workbench-Ziele nicht starten.
    pub(super) fn add_frontend_deps(&self) {
        std::fs::create_dir_all(self.ws.root.join(DEV_FRONTEND_DEPS)).unwrap();
    }

    async fn call(&self, input: CheckIn, progress: &Progress) -> Result<String, String> {
        self.checks.check_run(&self.ws, &input, progress).await
    }

    pub(super) async fn run(&self, target: &str, pattern: &str) -> Result<String, String> {
        let input = CheckIn {
            target: target.to_owned(),
            pattern: pattern.to_owned(),
            run: String::new(),
        };
        self.call(input, &Progress::none()).await
    }

    pub(super) async fn run_with(&self, target: &str, progress: &Progress) -> Result<String, String> {
        let input = CheckIn {
            target: target.to_owned(),
            ..CheckIn::default()
        };
        self.call(input, progress).await
    }

    /// `check_run {run}`.
    pub(super) async fn resume(&self, id: &str, progress: &Progress) -> Result<String, String> {
        let input = CheckIn {
            run: id.to_owned(),
            ..CheckIn::default()
        };
        self.call(input, progress).await
    }
}

pub(super) fn run_id(text: &str) -> String {
    let rest = text.split("Lauf-ID ").nth(1).expect("Lauf-ID");
    rest.chars().take_while(char::is_ascii_alphanumeric).collect()
}
