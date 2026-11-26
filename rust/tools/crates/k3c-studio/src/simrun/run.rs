//! Ein Lauf des Registers (aus Go `registry.go`): feste Angaben, veränderlicher Stand und das Abbruch-Token
//! (Gegenstück zu `context.CancelFunc`).

use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant, SystemTime};

use super::info::Info;
use super::spec::Spec;

/// Zustände eines Laufs.
pub const WAITING: &str = "wartet";
pub const RUNNING: &str = "läuft";
pub const DONE: &str = "fertig";
pub const CANCELED: &str = "abgebrochen";
pub const FAILED: &str = "fehler";

/// Rechnet einen Lauf und meldet sein Ende mit [`super::Registry::finish`].
pub type Runner = Arc<dyn Fn(CancelToken, Arc<Run>) + Send + Sync>;

pub(super) fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Abbruchsignal eines Laufs; alle Klone teilen den Zustand.
#[derive(Clone, Default)]
pub struct CancelToken(Arc<(Mutex<bool>, Condvar)>);

impl CancelToken {
    pub fn new() -> CancelToken {
        CancelToken::default()
    }

    pub fn cancel(&self) {
        *lock(&self.0.0) = true;
        self.0.1.notify_all();
    }

    pub fn is_canceled(&self) -> bool {
        *lock(&self.0.0)
    }

    /// Wartet höchstens `timeout` auf den Abbruch; `true`, wenn abgebrochen ist.
    pub fn wait_timeout(&self, timeout: Duration) -> bool {
        let (guard, _) = self
            .0
            .1
            .wait_timeout_while(lock(&self.0.0), timeout, |canceled| !*canceled)
            .unwrap_or_else(PoisonError::into_inner);
        *guard
    }
}

/// Ein Lauf. `id`, `root`, `place`, `label` und `spec` ändern sich nicht; alles andere liegt in [`RunState`] und wird
/// nur unter der Sperre des Registers geändert.
pub struct Run {
    id: String,
    root: PathBuf,
    /// „Repo-Wurzel“ oder der Name des Worktrees (Go `where`).
    place: String,
    label: String,
    spec: Option<Spec>,
    state: Mutex<RunState>,
}

/// Veränderlicher Stand eines Laufs.
pub(super) struct RunState {
    pub started: SystemTime,
    pub ended: Option<SystemTime>,
    pub state: &'static str,
    pub phase: String,
    pub verdict: String,
    pub done: i64,
    pub total: i64,
    pub lines: Vec<String>,
    pub report: String,
    /// Runner arbeitet und hält einen Platz.
    pub active: bool,
    /// Platz 0 … `MAX_PER_ROOT`-1, solange `active` (eigene EXE je Platz).
    pub slot: usize,
    pub cancel: Option<CancelToken>,
    /// Bis zum Start gehalten; danach leer (der Runner hält das Register).
    pub runner: Option<Runner>,
    /// Letzte Meldung an `Notify::run` (notify.rs).
    pub noted: Option<Instant>,
    /// Eine Meldung ist angesetzt.
    pub note_pending: bool,
}

impl Run {
    pub(super) fn new(
        id: String,
        root: &Path,
        place: &str,
        label: &str,
        spec: Option<Spec>,
        runner: Option<Runner>,
    ) -> Run {
        let state = RunState {
            started: SystemTime::now(),
            ended: None,
            state: WAITING,
            phase: WAITING.to_owned(),
            verdict: String::new(),
            done: 0,
            total: 0,
            lines: Vec::new(),
            report: String::new(),
            active: false,
            slot: 0,
            cancel: None,
            runner,
            noted: None,
            note_pending: false,
        };
        Run {
            id,
            root: root.to_path_buf(),
            place: place.to_owned(),
            label: label.to_owned(),
            spec,
            state: Mutex::new(state),
        }
    }

    pub(super) fn state(&self) -> MutexGuard<'_, RunState> {
        lock(&self.state)
    }

    /// Platz des laufenden Laufs im Checkout.
    pub fn slot(&self) -> usize {
        self.state().slot
    }

    /// Lauf-ID, z. B. `run-3`.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Checkout des Laufs.
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn spec(&self) -> Option<&Spec> {
        self.spec.as_ref()
    }

    /// Beginn des Laufs (fest, sobald der Runner läuft).
    pub fn started(&self) -> SystemTime {
        self.state().started
    }

    /// Mit Fenster: der Lauf läuft allein (RU40.3).
    pub(super) fn head(&self) -> bool {
        self.spec.as_ref().is_some_and(|s| s.head)
    }

    pub(super) fn info(&self) -> Info {
        let s = self.state();
        Info {
            id: self.id.clone(),
            label: self.label.clone(),
            place: self.place.clone(),
            state: s.state.to_owned(),
            phase: s.phase.clone(),
            done: s.done,
            total: s.total,
            started: s.started.into(),
            ended: s.ended.map(Into::into),
            verdict: s.verdict.clone(),
            report: s.report.clone(),
            lines: s.lines.clone(),
            spec: self.spec.clone(),
        }
    }
}
