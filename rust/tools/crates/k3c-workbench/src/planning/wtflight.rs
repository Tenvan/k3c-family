//! Bündelt die Aufrufe des Worktree-Abgleichs (Go `wtflight.go`).

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex, PoisonError};
use std::time::Instant;

use super::planning::Data;
use super::wtdiff::WorktreeDiff;

/// Begrenzt den Cache der Merge-Base-Stände; ein Stand ist ein ganzer docs/-Baum.
pub(super) const MAX_BASES: usize = 8;

/// Merge-Base → Planung dort; ein Commit ändert sich nie, der Eintrag bleibt gültig.
pub(super) type Bases = BTreeMap<String, Data>;

pub(super) type Diffs = Result<Vec<WorktreeDiff>, String>;

pub(super) struct FlightState {
    start: Option<Instant>,
    diffs: Diffs,
    pub(super) bases: Bases,
}

/// Je Wurzel läuft höchstens ein Check; wer währenddessen ankommt, wartet und nimmt den Lauf, der nach seiner
/// Ankunft begann (Änderungen während eines Laufs gehen so nicht verloren, mehr als ein Folgelauf entsteht nie).
pub(super) struct Flight {
    pub(super) state: Mutex<FlightState>,
}

impl Default for Flight {
    fn default() -> Flight {
        Flight {
            state: Mutex::new(FlightState {
                start: None,
                diffs: Ok(Vec::new()),
                bases: Bases::new(),
            }),
        }
    }
}

impl Flight {
    /// Führt `check` aus, außer ein Lauf nach `arrived` liegt schon vor.
    pub(super) fn run(&self, arrived: Instant, check: impl FnOnce(&mut Bases) -> Diffs) -> Diffs {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.start.is_some_and(|start| start > arrived) {
            return state.diffs.clone();
        }
        state.start = Some(Instant::now());
        if state.bases.len() > MAX_BASES {
            state.bases.clear();
        }
        state.diffs = check(&mut state.bases);
        state.diffs.clone()
    }
}

// Nur Zugriff je Schlüssel, nie eine Iteration.
static FLIGHTS: LazyLock<Mutex<HashMap<PathBuf, Arc<Flight>>>> = LazyLock::new(Mutex::default);

pub(super) fn flight_for(root: &Path) -> Arc<Flight> {
    let mut flights = FLIGHTS.lock().unwrap_or_else(PoisonError::into_inner);
    Arc::clone(flights.entry(root.to_owned()).or_default())
}
