//! Teile und Zeiten eines Controllers (aus `controller.go`): die Test-Nähte und die Vorgaben aus B-067.

use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use chrono::{DateTime, Local};
use k3c_kit::console::{DEFAULT_CAPACITY, Store};
use k3c_kit::logger::Logger;
use tokio::sync::watch;

use super::config::Service;
use super::status::Status;
use super::{runtime, system};

/// Ende eines Prozesses mit Fehler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WaitError {
    pub code: i64,
    pub message: String,
}

/// Ergebnis des Wartens auf einen Prozess.
pub type Exit = Result<(), WaitError>;

/// Ein gestarteter Dienst-Prozess.
pub struct Spawned {
    pub pid: u32,
    /// Beendet den ganzen Prozessbaum.
    pub kill: Box<dyn Fn() -> Result<(), String> + Send + Sync>,
    /// Endet mit dem Prozess, nachdem seine Ausgabe gelesen ist.
    pub wait: Pin<Box<dyn Future<Output = Exit> + Send>>,
}

/// Bekommt jede Ausgabezeile (Strom, Text).
pub type OutFn = Arc<dyn Fn(&str, &str) + Send + Sync>;
/// Startet einen Dienst unter der Repo-Wurzel; wird in einer tokio-Laufzeit gerufen.
pub type Starter = Arc<dyn Fn(&Service, &Path, OutFn) -> Result<Spawned, String> + Send + Sync>;
/// Prüft, ob ein Dienst gesund ist; darf blockieren.
pub type Checker = Arc<dyn Fn(&Service) -> Result<(), String> + Send + Sync>;
/// Wer auf einem Port lauscht: (PID, 0 = unbekannt; ob jemand lauscht); darf blockieren.
pub type Listener = Arc<dyn Fn(u16) -> (u32, bool) + Send + Sync>;
/// Misst einen Prozess; darf blockieren.
pub type Sampler = Arc<dyn Fn(u32) -> Result<system::Metrics, String> + Send + Sync>;
/// Beendet einen übernommenen Prozessbaum (ohne Job).
pub type KillPid = Arc<dyn Fn(u32) -> Result<(), String> + Send + Sync>;
/// PID des Supervisors eines Prozesses, 0 = keiner.
pub type Supervisor = Arc<dyn Fn(u32) -> u32 + Send + Sync>;
/// Uhr.
pub type Clock = Arc<dyn Fn() -> DateTime<Local> + Send + Sync>;

/// Abbruchsignal wie ein Go-Context.
#[derive(Clone, Debug)]
pub struct Cancel(Arc<watch::Sender<bool>>, Arc<AtomicBool>);

impl Default for Cancel {
    fn default() -> Cancel {
        Cancel(Arc::new(watch::Sender::new(false)), Arc::default())
    }
}

impl Cancel {
    pub fn cancel(&self) {
        self.0.send_replace(true);
    }

    /// Abbruch, weil eine Frist abgelaufen ist (Go `context.WithTimeout`).
    pub fn expire(&self) {
        self.1.store(true, Ordering::SeqCst);
        self.cancel();
    }

    /// Text wie Go `ctx.Err()` nach dem Abbruch.
    pub fn reason(&self) -> &'static str {
        if self.1.load(Ordering::SeqCst) {
            "context deadline exceeded"
        } else {
            "context canceled"
        }
    }

    pub fn is_cancelled(&self) -> bool {
        *self.0.borrow()
    }

    /// Wartet auf [`Cancel::cancel`].
    pub async fn cancelled(&self) {
        let mut rx = self.0.subscribe();
        // Der Sender lebt in `self`, der Fehlerfall kommt nicht vor.
        let _ = rx.wait_for(|cancelled| *cancelled).await;
    }
}

/// Teile und Zeiten eines Controllers. `on_change` darf den Controller nicht aufrufen (läuft teils unter der
/// Befehlssperre).
#[derive(Clone)]
pub struct Options {
    /// Repo-Wurzel; leer: keine Ausgabe-Datei.
    pub root: PathBuf,
    pub console: Arc<Store>,
    /// Vor jeder Konsolen-Quelle, z. B. `<worktree>/`.
    pub prefix: String,
    pub log: Logger,
    pub on_change: Arc<dyn Fn(&Status) + Send + Sync>,
    pub start: Starter,
    pub check: Checker,
    pub listen: Listener,
    pub sample: Sampler,
    pub kill_pid: KillPid,
    pub supervisor: Supervisor,
    pub now: Clock,
    /// Bis gesund.
    pub start_timeout: Duration,
    /// Zwischen zwei Prüfungen beim Start.
    pub start_poll: Duration,
    /// Zwischen zwei Prüfungen im Lauf.
    pub watch_every: Duration,
    /// Fehlschläge in Folge.
    pub fail_limit: u32,
    /// Neustarts je `restart_window`.
    pub restart_limit: usize,
    pub restart_window: Duration,
    /// Warten auf das Ende nach dem Beenden.
    pub stop_timeout: Duration,
    /// Zwischen zwei Prüfungen der beobachteten Dateien.
    pub files_every: Duration,
    /// Ruhe, bevor eine Änderung zum Neustart führt.
    pub files_quiet: Duration,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            root: PathBuf::new(),
            console: Arc::new(Store::new(DEFAULT_CAPACITY, None)),
            prefix: String::new(),
            log: Logger::discard(),
            on_change: Arc::new(|_| {}),
            start: Arc::new(runtime::proc_starter),
            check: Arc::new(runtime::health_check),
            listen: Arc::new(system::port_listener),
            sample: system::new_sampler(),
            kill_pid: Arc::new(|pid| k3c_kit::proc::kill_tree(pid).map_err(|e| e.to_string())),
            supervisor: Arc::new(system::supervisor_of),
            now: Arc::new(Local::now),
            start_timeout: Duration::from_secs(60),
            start_poll: Duration::from_secs(1),
            watch_every: Duration::from_secs(10),
            fail_limit: 3,
            restart_limit: 3,
            restart_window: Duration::from_secs(600),
            stop_timeout: Duration::from_secs(10),
            files_every: Duration::from_secs(1),
            files_quiet: Duration::from_millis(500),
        }
    }
}

/// Dauer wie Go `time.Duration.String()` (ab 1 ms; kleinere Anteile nennt keine Vorgabe).
pub(crate) fn go_duration(d: Duration) -> String {
    let ms = d.as_millis();
    if ms == 0 {
        return "0s".to_owned();
    }
    if ms < 1000 {
        return format!("{ms}ms");
    }
    let (hours, minutes) = (ms / 3_600_000, ms / 60_000 % 60);
    let (seconds, rest) = (ms / 1000 % 60, ms % 1000);
    let mut out = String::new();
    if hours > 0 {
        out = format!("{hours}h");
    }
    if hours > 0 || minutes > 0 {
        out.push_str(&format!("{minutes}m"));
    }
    out.push_str(&seconds.to_string());
    if rest > 0 {
        out.push('.');
        out.push_str(format!("{rest:03}").trim_end_matches('0'));
    }
    out.push('s');
    out
}
