//! Stand der Prüfläufe, soweit Konsole und Quellenleiste ihn brauchen (aus `check_state.go` und `check_run.go`). Die
//! Läufe selbst bringt `check_run` (RU69.4); er trägt sie hier ein.

use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard, PoisonError};

use chrono::{DateTime, Local};

/// Laufender oder letzter Lauf eines Ziels.
#[derive(Clone, Debug)]
pub struct CheckState {
    /// Schlüssel wie die Konsolen-Quelle ohne `check:`, z. B. `<worktree>/task:test`.
    pub name: String,
    pub running: bool,
    /// Beginn.
    pub at: DateTime<Local>,
    pub ms: f64,
    pub exit: i64,
    pub timed_out: bool,
    /// Grund, wenn der Lauf nicht starten konnte.
    pub error: String,
}

#[derive(Default)]
pub(super) struct Checks(Mutex<BTreeMap<String, CheckState>>);

impl Checks {
    fn states(&self) -> MutexGuard<'_, BTreeMap<String, CheckState>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub(super) fn record(&self, state: CheckState) {
        self.states().insert(state.name.clone(), state);
    }

    /// Beginn des laufenden oder sonst letzten Laufs (B-855).
    pub(super) fn start_of(&self, key: &str) -> Option<DateTime<Local>> {
        self.states().get(key).map(|s| s.at)
    }

    pub(super) fn all(&self) -> Vec<CheckState> {
        self.states().values().cloned().collect()
    }
}
