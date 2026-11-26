//! Stand eines Laufs für `sim_test` und die Seite (Port von `info.go`, JSON der Methode `BalanceRuns`).

use chrono::{DateTime, Local};
use serde::{Serialize, Serializer};

use super::run::{CANCELED, DONE, FAILED, RUNNING, WAITING};
use super::spec::Spec;
use crate::balance::report_types::nil_empty;

/// Stand eines Laufs.
#[derive(Clone, Debug, Serialize)]
pub struct Info {
    pub id: String,
    pub label: String,
    /// „Repo-Wurzel“ oder der Name des Worktrees.
    #[serde(rename = "where")]
    pub place: String,
    pub state: String,
    pub phase: String,
    pub done: i64,
    pub total: i64,
    #[serde(serialize_with = "go_time")]
    pub started: DateTime<Local>,
    #[serde(serialize_with = "go_time_opt", skip_serializing_if = "Option::is_none")]
    pub ended: Option<DateTime<Local>>,
    pub verdict: String,
    /// Pfad relativ zum Checkout, leer bis der Bericht steht.
    pub report: String,
    /// Ohne Zeilen `null` wie Go.
    #[serde(serialize_with = "nil_empty")]
    pub lines: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<Spec>,
}

/// Zeitpunkt wie Go `time.Time` in JSON (RFC 3339, Nachkommastellen ohne Nullen am Ende, UTC als `Z`).
pub(super) fn go_time<S: Serializer>(t: &DateTime<Local>, s: S) -> Result<S::Ok, S::Error> {
    let nanos = format!(".{:09}", t.timestamp_subsec_nanos());
    let fraction = nanos.trim_end_matches('0').trim_end_matches('.');
    let zone = if t.offset().local_minus_utc() == 0 {
        "Z".to_owned()
    } else {
        t.format("%:z").to_string()
    };
    s.serialize_str(&format!("{}{fraction}{zone}", t.format("%Y-%m-%dT%H:%M:%S")))
}

fn go_time_opt<S: Serializer>(t: &Option<DateTime<Local>>, s: S) -> Result<S::Ok, S::Error> {
    match t {
        Some(t) => go_time(t, s),
        None => s.serialize_none(),
    }
}

/// Dauer auf ganze Sekunden gerundet, geschrieben wie Go `time.Duration` („12s“, „1m5s“, „1h0m0s“).
fn go_seconds(millis: i64) -> String {
    let sign = if millis <= -500 { "-" } else { "" };
    let total = millis.unsigned_abs().saturating_add(500) / 1000;
    let (h, m, s) = (total / 3600, total / 60 % 60, total % 60);
    match (h, m) {
        (0, 0) => format!("{sign}{s}s"),
        (0, _) => format!("{sign}{m}m{s}s"),
        _ => format!("{sign}{h}h{m}m{s}s"),
    }
}

impl Info {
    /// Kopfzeile eines Laufs, z. B. „▶ run-1 · offline·balance · läuft · 12s · Repo-Wurzel · rechnen“.
    pub fn head(&self, now: DateTime<Local>) -> String {
        let end = self.ended.unwrap_or(now);
        let icon = match self.state.as_str() {
            WAITING => "⏳",
            RUNNING => "▶",
            DONE => "✅",
            CANCELED => "■",
            FAILED => "❌",
            _ => "",
        };
        let age = go_seconds(end.signed_duration_since(self.started).num_milliseconds());
        let mut line = format!("{icon} {} · {} · {} · {age}", self.id, self.label, self.state);
        let mut add = |part: &str| {
            line.push_str(" · ");
            line.push_str(part);
        };
        add(&self.place);
        if self.state == RUNNING {
            add(&self.phase);
        }
        if !self.verdict.is_empty() {
            add(&self.verdict);
        }
        line
    }

    /// Kopfzeile, Kennzahlen, Bericht; höchstens `max_lines` Zeilen.
    pub fn status(&self, now: DateTime<Local>, max_lines: usize) -> String {
        let mut out = vec![self.head(now)];
        let mut room = max_lines.saturating_sub(1);
        if !self.report.is_empty() {
            room = room.saturating_sub(1);
        }
        if self.lines.len() > room {
            let keep = room.saturating_sub(1);
            out.extend(self.lines.iter().take(keep).cloned());
            out.push(format!("… {} weitere im Bericht", self.lines.len() - keep));
        } else {
            out.extend(self.lines.iter().cloned());
        }
        if !self.report.is_empty() {
            out.push(format!("Bericht: {}", self.report));
        }
        out.join("\n")
    }
}
