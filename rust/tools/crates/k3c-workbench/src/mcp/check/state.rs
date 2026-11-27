//! Zustand der Prüfziele für Quellenleiste, `console_tail` und `workbench_status` (Port von `check_state.go`; die
//! Ausgabe steht in der Konsolen-Quelle `check:<name>`).

use std::collections::BTreeMap;

use chrono::{DateTime, Local};
use k3c_kit::mcp::Workspace;

use super::super::status::format_ms;
use super::Checks;
use super::run::RunResult;
use super::targets::TARGETS;

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

impl CheckState {
    pub(super) fn running(name: &str, at: DateTime<Local>) -> CheckState {
        CheckState {
            name: name.to_owned(),
            running: true,
            at,
            ms: 0.0,
            exit: 0,
            timed_out: false,
            error: String::new(),
        }
    }

    pub(super) fn finished(name: &str, res: &RunResult) -> CheckState {
        CheckState {
            running: false,
            ms: res.ms(),
            exit: res.exit,
            timed_out: res.timed_out,
            error: res.err.clone().unwrap_or_default(),
            ..CheckState::running(name, res.at.unwrap_or_else(Local::now))
        }
    }
}

impl Checks {
    pub(super) fn notify(&self, state: &CheckState) {
        if let Some(on_check) = &self.on_check {
            on_check(state);
        }
    }

    /// Alle Ziele, die gerade laufen oder schon einmal gelaufen sind, nach Namen sortiert.
    pub(in crate::mcp) fn all(&self) -> Vec<CheckState> {
        let runs = self.runs();
        let mut out: BTreeMap<&str, CheckState> = runs
            .last
            .iter()
            .map(|(name, res)| (name.as_str(), CheckState::finished(name, res)))
            .collect();
        out.extend(
            runs.running
                .iter()
                .map(|(name, at)| (name.as_str(), CheckState::running(name, *at))),
        );
        out.into_values().collect()
    }

    /// Beginn des laufenden oder sonst letzten Laufs (B-855).
    pub(in crate::mcp) fn start_of(&self, key: &str) -> Option<DateTime<Local>> {
        let runs = self.runs();
        runs.running
            .get(key)
            .copied()
            .or_else(|| runs.last.get(key).and_then(|r| r.at))
    }

    /// Letzter Lauf je Ziel in Katalog-Reihenfolge für `workbench_status`.
    pub(in crate::mcp) fn run_lines(&self, ws: &Workspace) -> Vec<String> {
        let runs = self.runs();
        let mut out = Vec::new();
        for target in TARGETS {
            let Some(res) = runs.last.get(&ws.source(target.name)) else {
                continue;
            };
            let state = if res.timed_out {
                "Zeitlimit".to_owned()
            } else {
                format!("exit {}", res.exit)
            };
            out.push(format!(
                "Lauf {} · {state} · {} · {}",
                target.name,
                format_ms(res.ms()),
                res.at.unwrap_or_else(Local::now).format("%H:%M:%S")
            ));
        }
        if out.is_empty() {
            out.push("Läufe: noch keine".to_owned());
        }
        out
    }

    /// Trägt einen Stand von außen ein (Tests der Konsole).
    #[cfg(test)]
    pub(in crate::mcp) fn record(&self, state: CheckState) {
        let mut runs = self.runs();
        if state.running {
            runs.running.insert(state.name, state.at);
            return;
        }
        let res = RunResult {
            exit: state.exit,
            timed_out: state.timed_out,
            at: Some(state.at),
            ..RunResult::default()
        };
        runs.end(&state.name, res);
    }
}
