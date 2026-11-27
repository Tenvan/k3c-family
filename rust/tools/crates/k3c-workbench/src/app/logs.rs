//! Quellenleiste und Konsole (Port von `Sources` und `ConsoleTail` aus `app_logs.go`); die Log-Abfragen der Seite Logs
//! folgen mit RU69.5.

use std::collections::BTreeSet;
use std::path::Path;

use k3c_kit::console::Line;
use serde::Serialize;

use super::{App, lock};
use crate::mcp::{CheckState, log_sources};
use crate::services::Status;

/// Arten einer Quelle.
const KIND_SERVICE: &str = "service";
const KIND_RUN: &str = "run";
const KIND_LOG: &str = "log";
/// Konsolen-Quelle ohne Dienst, Lauf oder Log-Datei.
const KIND_CONSOLE: &str = "console";

/// Eintrag der Quellenleiste (`Source` in `types.ts`). Der Name ist zugleich die Konsolen-Quelle (Lauf: `check:<ziel>`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Source {
    pub name: String,
    pub kind: String,
    pub state: String,
    pub detail: String,
}

impl Source {
    fn new(name: String, kind: &str, state: &str, detail: String) -> Source {
        Source {
            name,
            kind: kind.to_owned(),
            state: state.to_owned(),
            detail,
        }
    }
}

pub(super) fn service_source(st: &Status) -> Source {
    let mut detail = format!("Port {}", st.port);
    if st.pid > 0 {
        detail.push_str(&format!(" · PID {}", st.pid));
    }
    if !st.last_error.is_empty() {
        detail.push_str(&format!(" · {}", st.last_error));
    }
    Source::new(st.name.clone(), KIND_SERVICE, st.state.as_str(), detail)
}

/// Millisekunden deutsch als Sekunden: 12400 → „12,4 s“.
fn seconds(ms: f64) -> String {
    format!("{:.1} s", ms / 1000.0).replacen('.', ",", 1)
}

fn check_source(st: &CheckState) -> Source {
    let at = st.at.format("%H:%M:%S").to_string();
    let (state, detail) = if st.running {
        ("running", format!("läuft seit {at}"))
    } else if !st.error.is_empty() {
        ("failed", format!("nicht gestartet: {}", st.error))
    } else if st.timed_out {
        ("timeout", format!("Zeitlimit nach {} · {at}", seconds(st.ms)))
    } else if st.exit != 0 {
        ("failed", format!("Exit {} · {} · {at}", st.exit, seconds(st.ms)))
    } else {
        ("ok", format!("Exit 0 · {} · {at}", seconds(st.ms)))
    };
    Source::new(format!("check:{}", st.name), KIND_RUN, state, detail)
}

fn log_source(root: &Path, name: &str) -> Source {
    let size = std::fs::metadata(root.join("logs").join(format!("{name}.jsonl"))).map_or(0, |m| m.len());
    if size == 0 {
        return Source::new(name.to_owned(), KIND_LOG, "empty", "noch keine Einträge".to_owned());
    }
    let detail = format!("logs/{name}.jsonl · {} KB", size.div_ceil(1024));
    Source::new(name.to_owned(), KIND_LOG, "entries", detail)
}

impl App {
    /// Alle Quellen: Dienste in der Reihenfolge der Konfiguration, dann Log-Dateien, dann Läufe, dann übrige
    /// Konsolen-Quellen; jeder Name nur einmal.
    pub fn sources(&self) -> Vec<Source> {
        self.wait();
        let (services, checks, console) = {
            let running = lock(&self.running);
            let Some(r) = running.as_ref() else {
                return Vec::new();
            };
            let services = r.services.as_ref().map_or_else(|_| Vec::new(), |c| c.statuses());
            (services, r.server.checks(), r.store.sources())
        };
        let mut all: Vec<Source> = services.iter().map(service_source).collect();
        all.extend(log_sources(&self.root).iter().map(|n| log_source(&self.root, n)));
        all.extend(checks.iter().map(check_source));
        all.extend(
            console
                .into_iter()
                .map(|n| Source::new(n, KIND_CONSOLE, "", String::new())),
        );
        let mut seen = BTreeSet::new();
        all.retain(|s| seen.insert(s.name.clone()));
        all
    }

    /// Der ganze Puffer einer Quelle, ANSI-Farben eingeschlossen. Eine bekannte Quelle ohne Ausgabe liefert eine leere
    /// Liste, eine unbekannte einen Fehler.
    pub fn console_tail(&self, source: &str) -> Result<Vec<Line>, String> {
        self.wait();
        let lines = lock(&self.running).as_ref().and_then(|r| r.store.tail(source, 0));
        if let Some(lines) = lines {
            return Ok(lines);
        }
        let names: Vec<String> = self.sources().into_iter().map(|s| s.name).collect();
        if names.iter().any(|n| n == source) {
            return Ok(Vec::new());
        }
        Err(format!("unbekannte Quelle {source:?}; gültig: {}", names.join(", ")))
    }
}

#[cfg(test)]
#[path = "logs_tests.rs"]
mod tests;
