//! Ereignisse der Balancing-Seite (Port von `app_balance_events.go`, B-662): Statt abzufragen lädt die Seite Läufe und
//! Berichte nur bei diesen Ereignissen neu. Das Register (`crate::simrun::notify`) meldet Läufe gedrosselt und Berichte
//! nach einem fertigen Lauf oder einer gesicherten Baseline; Bewerten und Löschen melden die Methoden selbst.

use std::path::Path;
use std::sync::Arc;

use serde_json::{Value, json};

use super::{App, emit_to, lock};
use crate::simrun::{Notify, Registry};

/// Ein Lauf hat Zustand, Phase oder Fortschritt geändert; Nutzlast `simrun::Info`.
pub const EV_BALANCE_RUNS: &str = "balance:runs";
/// Berichte oder Baselines haben sich geändert; ohne Nutzlast.
pub const EV_BALANCE_REPORTS: &str = "balance:reports";

impl App {
    /// Das Register der Läufe mit den Rückrufen für die Ereignisse der Seite. Die Rückrufe hängen sich bei jedem Aufruf
    /// der Seite ein (`set_notify` ersetzt, das ist billig): vor dem ersten Laden hört keine Oberfläche zu. Gemeldet
    /// wird nur, was die Seite zeigt: Läufe und Berichte der Repo-Wurzel. Wartet auf `startup`.
    pub(super) fn sims(&self) -> Result<Arc<Registry>, String> {
        self.wait();
        let registry = lock(&self.running)
            .as_ref()
            .map(|r| r.server.sims())
            .ok_or_else(|| "Studio ist beendet".to_owned())?;
        let (forward, bus, root) = (self.forward.clone(), self.bus(), self.root.clone());
        let on_run = Arc::new(move |changed: &Path, info: crate::simrun::Info| {
            if changed == root {
                let data = serde_json::to_value(info).unwrap_or(Value::Null);
                emit_to(forward.as_ref(), &bus, EV_BALANCE_RUNS, data);
            }
        });
        let (forward, bus, root) = (self.forward.clone(), self.bus(), self.root.clone());
        let on_reports = Arc::new(move |changed: &Path| {
            if changed == root {
                emit_to(forward.as_ref(), &bus, EV_BALANCE_REPORTS, json!(null));
            }
        });
        registry.set_notify(Notify {
            run: Some(on_run),
            reports: Some(on_reports),
        });
        Ok(registry)
    }

    /// Meldet geänderte Berichte und reicht das Ergebnis durch; auch nach einem Fehler, denn ein Sammel-Löschen kann
    /// halb gelungen sein (die Liste lädt dann einmal zu viel, nie zu wenig).
    pub(super) fn reports_done(&self, result: Result<(), String>) -> Result<(), String> {
        self.emit(EV_BALANCE_REPORTS, Value::Null);
        result
    }
}
