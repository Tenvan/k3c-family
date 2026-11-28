//! Zustand eines Dienstes für MCP und Oberfläche (aus `controller.go`; JSON wie `ServiceStatus` in `types.ts`).

use std::fmt;

use chrono::{DateTime, Local};
use k3c_kit::logger::time_nanos;
use serde::{Serialize, Serializer};

use super::config::Service;

/// Zustände eines Dienstes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum State {
    #[default]
    Stopped,
    Starting,
    Running,
    Adopted,
    Stopping,
    Failed,
}

impl State {
    pub fn as_str(self) -> &'static str {
        match self {
            State::Stopped => "gestoppt",
            State::Starting => "startet",
            State::Running => "läuft",
            State::Adopted => "übernommen",
            State::Stopping => "stoppt",
            State::Failed => "fehlgeschlagen",
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for State {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

/// Zeit wie Go `time.Time` als JSON; keine Zeit ist die Null-Zeit von Go.
pub(crate) fn go_time<S: Serializer>(t: &Option<DateTime<Local>>, s: S) -> Result<S::Ok, S::Error> {
    match t {
        Some(t) => s.serialize_str(&time_nanos(t)),
        None => s.serialize_str("0001-01-01T00:00:00Z"),
    }
}

/// Zustand eines Dienstes.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub name: String,
    #[serde(rename = "description")]
    pub desc: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub port: i64,
    pub health: String,
    /// Name unter `logs/`; leer: kein Log.
    pub log: String,
    pub state: State,
    pub pid: u32,
    #[serde(serialize_with = "go_time")]
    pub started_at: Option<DateTime<Local>>,
    pub restarts: u32,
    pub last_error: String,
    /// Letzter Neustart wegen einer Dateiänderung (Watch-Modus).
    pub change_file: String,
    #[serde(serialize_with = "go_time")]
    pub change_at: Option<DateTime<Local>>,
    /// Prozent, alle 2 s gemessen.
    pub cpu: f64,
    /// RSS in Bytes.
    pub memory: u64,
    /// Zählt je Dienst jede Änderung; Rückrufe kommen ungeordnet an.
    pub seq: u64,
}

impl Status {
    pub(crate) fn new(s: &Service) -> Status {
        let mut st = Status {
            name: s.name.clone(),
            ..Status::default()
        };
        st.apply(s);
        st
    }

    /// Übernimmt die Felder der Konfiguration.
    pub(crate) fn apply(&mut self, s: &Service) {
        self.desc.clone_from(&s.description);
        self.tags.clone_from(&s.tags);
        self.port = s.port;
        self.health = s.health_url();
        self.log.clone_from(&s.log);
    }

    /// Kein Prozess mehr: Zustand `state` mit dem Grund `reason`, ohne PID und Messwerte.
    pub(crate) fn rest(&mut self, state: State, reason: &str) {
        self.state = state;
        reason.clone_into(&mut self.last_error);
        self.pid = 0;
        self.cpu = 0.0;
        self.memory = 0;
    }
}
