//! Ereignisse der Balancing-Seite (Port von `notify.go`, B-662): Das Register meldet Änderungen an Läufen (Zustand,
//! Phase, Fortschritt, Zwischenstand) und an Berichten über Rückrufe; das Studio macht daraus `balance:runs` und
//! `balance:reports`. Ein Lauf meldet sich höchstens einmal je [`NOTIFY_EVERY`]; die letzte Änderung kommt immer an
//! (nachlaufend).

use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use super::info::Info;
use super::registry::{Inner, Registry};
use super::run::Run;

/// Mindestabstand zweier Meldungen eines Laufs (höchstens etwa 2 je Sekunde).
pub const NOTIFY_EVERY: Duration = Duration::from_millis(500);

/// Ein Lauf des Checkouts hat sich geändert (auch: wartender entfernt).
pub type OnRun = Arc<dyn Fn(&Path, Info) + Send + Sync>;
/// Berichte oder Baselines des Checkouts haben sich geändert.
pub type OnReports = Arc<dyn Fn(&Path) + Send + Sync>;

/// Rückrufe des Registers; sie laufen in eigenem Thread, nie unter der Sperre des Registers.
#[derive(Clone, Default)]
pub struct Notify {
    pub run: Option<OnRun>,
    pub reports: Option<OnReports>,
}

impl Registry {
    /// Setzt die Rückrufe und ersetzt frühere.
    pub fn set_notify(&self, notify: Notify) {
        self.lock().notify = notify;
    }

    /// Meldet eine Änderung von `run`, gedrosselt auf [`NOTIFY_EVERY`].
    pub(super) fn changed(&self, g: &Inner, run: &Arc<Run>) {
        if g.notify.run.is_none() {
            return;
        }
        let wait = {
            let mut s = run.state();
            if s.note_pending {
                return;
            }
            s.note_pending = true;
            s.noted
                .map_or(Duration::ZERO, |at| NOTIFY_EVERY.saturating_sub(at.elapsed()))
        };
        let (me, run) = (self.me.upgrade(), Arc::clone(run));
        thread::spawn(move || {
            let Some(me) = me else { return };
            thread::sleep(wait);
            let (call, info) = {
                let g = me.lock();
                {
                    let mut s = run.state();
                    s.note_pending = false;
                    s.noted = Some(Instant::now());
                }
                (g.notify.run.clone(), run.info())
            };
            if let Some(call) = call {
                call(run.root(), info);
            }
        });
    }

    /// Meldet geänderte Berichte des Checkouts.
    pub(super) fn reports_changed(g: &Inner, root: &Path) {
        if let Some(call) = g.notify.reports.clone() {
            let root = root.to_path_buf();
            thread::spawn(move || call(&root));
        }
    }
}

#[cfg(test)]
#[path = "notify_tests.rs"]
mod tests;
