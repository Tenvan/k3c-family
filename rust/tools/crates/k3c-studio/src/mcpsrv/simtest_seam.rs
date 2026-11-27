//! Test-Naht der Testläufe (Port von `simtest_seam.go`): In der Produktion rechnet das Register mit der Spiel-EXE; Tests
//! ersetzen seinen `Exec` ([`Registry::set_exec`]), die Sim-Sperre und die Fristen des Wartens.

use std::sync::Arc;
use std::time::Duration;

use k3c_kit::logger::Logger;

use super::simtest_lock::env_unlocked;
use crate::simrun::Registry;

// Warten von sim_test wie check_run der Workbench (B-644): mit progressToken alle `PROGRESS_EVERY` ein Lebenszeichen,
// ohne höchstens `CHECK_WAIT`, knapp unter der Leerlauf-Frist von HTTP-MCP-Clients (5 min).
const CHECK_WAIT: Duration = Duration::from_secs(4 * 60);
const PROGRESS_EVERY: Duration = Duration::from_secs(20);

/// Was `sim_test` und `sim_run` brauchen.
#[derive(Clone)]
pub(super) struct Sim {
    /// Testläufe von `sim_test` und der Balancing-Seite.
    pub(super) registry: Arc<Registry>,
    /// Ob die Sim-Sperre aufgehoben ist; je Aufruf gelesen.
    pub(super) unlocked: fn() -> bool,
    /// Wie lange `sim_test wait` ohne `progressToken` wartet.
    pub(super) check_wait: Duration,
    /// Abstand der Lebenszeichen von `sim_test wait`.
    pub(super) progress_every: Duration,
}

impl Sim {
    pub(super) fn new(log: Logger) -> Sim {
        Sim {
            registry: Registry::new(log),
            unlocked: env_unlocked,
            check_wait: CHECK_WAIT,
            progress_every: PROGRESS_EVERY,
        }
    }
}
