//! Reiter Log und Fehler der Seite Logs (Port von `app_logview.go`, B-064): feste Limits, Fehler über die letzten 24 h
//! wie `logs_errors`.

use std::path::PathBuf;

use chrono::{Local, TimeDelta};
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::App;
use crate::logs::{Entry, Group, LEVELS, Query, digest, scan};
use crate::mcp::log_path;

const LOG_LIMITS: [i64; 3] = [100, 200, 500];
const ERRORS_WINDOW_HOURS: i64 = 24;

/// Filter des Reiters Log; ein leeres `min_level` heißt alle.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct LogQuery {
    pub min_level: String,
    pub ns: String,
    pub pattern: String,
    pub limit: i64,
}

/// Einträge des Reiters Log, neueste zuerst (die Oberfläche dreht sie um).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogView {
    pub entries: Vec<Entry>,
    pub bytes_read: u64,
    pub budget_hit: bool,
    pub skipped: usize,
    /// Datei gibt es (noch) nicht.
    pub missing: bool,
}

/// Gruppen des Reiters Fehler (verdichtet), häufigste zuerst.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorsView {
    pub groups: Vec<Group>,
    pub entries: usize,
    pub bytes_read: u64,
    pub budget_hit: bool,
    pub missing: bool,
}

fn check_level(level: &str, allowed: &[&str]) -> Result<(), String> {
    if allowed.contains(&level) {
        return Ok(());
    }
    // Go schreibt die Liste mit `%v`: durch Leerzeichen getrennt, in eckigen Klammern.
    Err(format!("unbekanntes Level {level:?}; gültig: [{}]", allowed.join(" ")))
}

impl App {
    /// Prüft die Quelle gegen die Liste (nur so wird aus einem Namen ein Pfad) und ob die Datei fehlt.
    fn log_file(&self, source: &str) -> Result<(PathBuf, bool), String> {
        self.wait();
        let path = log_path(&self.root, &self.root, source)?;
        let missing = std::fs::metadata(&path).is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound);
        Ok((path, missing))
    }

    /// Gefilterte Einträge einer Log-Datei (`LogsQuery`).
    pub fn logs_query(&self, source: &str, input: &LogQuery) -> Result<LogView, String> {
        let (path, missing) = self.log_file(source)?;
        if !LOG_LIMITS.contains(&input.limit) {
            let valid = LOG_LIMITS.map(|l| l.to_string()).join(" ");
            return Err(format!("limit {}; gültig: [{valid}]", input.limit));
        }
        let mut levels = vec![""];
        levels.extend(LEVELS);
        check_level(&input.min_level, &levels)?;
        let mut query = Query {
            min_level: input.min_level.clone(),
            ns: input.ns.clone(),
            limit: usize::try_from(input.limit).unwrap_or_default(),
            ..Query::default()
        };
        if !input.pattern.is_empty() {
            let compiled =
                Regex::new(&input.pattern).map_err(|e| format!("ungültige Suche, kein regulärer Ausdruck: {e}"))?;
            query.pattern = Some(compiled);
        }
        let res = scan(&path, &query).map_err(|e| e.to_string())?;
        Ok(LogView {
            entries: res.entries,
            bytes_read: res.bytes_read,
            budget_hit: res.budget_hit,
            skipped: res.skipped,
            missing,
        })
    }

    /// Warnungen oder Fehler der letzten 24 h zu Gruppen zusammengefasst (`LogsErrors`).
    pub fn logs_errors(&self, source: &str, level: &str) -> Result<ErrorsView, String> {
        let (path, missing) = self.log_file(source)?;
        check_level(level, &["WARN", "ERROR"])?;
        let query = Query {
            min_level: level.to_owned(),
            since: Local::now()
                .fixed_offset()
                .checked_sub_signed(TimeDelta::hours(ERRORS_WINDOW_HOURS)),
            ..Query::default()
        };
        let res = scan(&path, &query).map_err(|e| e.to_string())?;
        Ok(ErrorsView {
            groups: digest(&res.entries),
            entries: res.entries.len(),
            bytes_read: res.bytes_read,
            budget_hit: res.budget_hit,
            missing,
        })
    }
}

#[cfg(test)]
#[path = "logview_tests.rs"]
mod tests;
