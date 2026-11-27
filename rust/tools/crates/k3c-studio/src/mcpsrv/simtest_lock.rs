//! Sim-Sperre im Werkzeug (Port von `simtest_lock.go`, B-788, V15): Simulationen und Balancing sind bis auf Widerruf
//! gesperrt (Beschluss 🧑 2026-11-11). `sim_test start` und `sim_run` lehnen ab, solange das Studio nicht mit
//! `K3C_SIM_ENTSPERRT=1` läuft.

use std::ffi::OsStr;

/// Der Schalter, der die Sperre aufhebt (Umgebung des Studios, Wert 1).
pub(super) const SIM_UNLOCK_ENV: &str = "K3C_SIM_ENTSPERRT";

/// Ob der Wert des Schalters die Sperre aufhebt.
pub(super) fn unlocked_by(value: Option<&OsStr>) -> bool {
    value.is_some_and(|v| v == "1")
}

/// Ob die Umgebung des Studios die Sperre aufhebt.
pub(super) fn env_unlocked() -> bool {
    unlocked_by(std::env::var_os(SIM_UNLOCK_ENV).as_deref())
}

/// Die Ablehnung für `tool`, solange die Sperre gilt.
pub(super) fn lock_err(unlocked: bool, tool: &str) -> Result<(), String> {
    if unlocked {
        return Ok(());
    }
    Err(format!(
        "{tool} abgelehnt: Simulationen und Balancing sind bis auf Widerruf gesperrt (Beschluss 🧑 2026-11-11, \
         docs/arbeitsweise.md › Sim, Balance und Gen); kein Lauf gestartet. Aufheben nur durch 🧑: Studio mit {SIM_UNLOCK_ENV}=1 starten"
    ))
}
