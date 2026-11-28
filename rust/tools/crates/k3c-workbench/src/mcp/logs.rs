//! Tools `logs_services`, `logs_errors`, `logs_query` und `logs_since` (Port von `tools_logs.go`) und die Prüfung der
//! Log-Quellen, die auch die Oberfläche nutzt. `logs_stats` und `logs_context` stehen in `logstats.rs`.

use std::path::{Path, PathBuf};

use chrono::Local;
use k3c_kit::mcp::{Workspace, clip};
use regex::Regex;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::logtext::{entry_line, now, parse_since, scan_footer, window};
use super::services::format_bytes;
use super::status::log_sources;
use super::tools::Workbench;
use crate::logs::{DEFAULT_BUDGET, Query, ScanResult, digest, read_since, scan};
use crate::ports;

const DEFAULT_QUERY_LIMIT: usize = 100;
const MAX_QUERY_LIMIT: usize = 500;
/// Begrenzt die Einträge je Datei, die `logs_errors` verdichtet.
pub(super) const ERRORS_SCAN_LIMIT: usize = 5000;

/// Prüft eine Quelle gegen die Liste des Checkouts `root`; nur so wird aus einem Namen ein Pfad. Das eigene Log
/// ([`ports::LOG_NAME`]) schreibt die Workbench immer in die Repo-Wurzel `repo`, auch für Aufrufe aus einem Worktree.
pub(crate) fn log_path(root: &Path, repo: &Path, source: &str) -> Result<PathBuf, String> {
    let root = if source == ports::LOG_NAME { repo } else { root };
    let names = log_sources(root);
    let valid = !source.is_empty()
        && source
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'));
    if valid && names.iter().any(|n| n == source) {
        return Ok(root.join("logs").join(format!("{source}.jsonl")));
    }
    Err(format!(
        "unbekannte Log-Quelle {source:?}; gültig: {}",
        names.join(", ")
    ))
}

pub(super) fn clamp_limit(n: i64) -> usize {
    usize::try_from(n)
        .ok()
        .filter(|&n| n > 0)
        .map_or(DEFAULT_QUERY_LIMIT, |n| n.min(MAX_QUERY_LIMIT))
}

pub(super) fn read(path: &Path, query: &Query) -> Result<ScanResult, String> {
    scan(path, query).map_err(|e| e.to_string())
}

/// Liest mehrere Dateien mit derselben Abfrage und fasst die Ergebnisse zusammen (neueste zuerst je Datei).
pub(super) fn scan_all(paths: &[PathBuf], query: &Query) -> Result<ScanResult, String> {
    let mut all = ScanResult::default();
    for path in paths {
        let res = read(path, query)?;
        all.entries.extend(res.entries);
        all.bytes_read += res.bytes_read;
        all.budget_hit |= res.budget_hit;
        all.skipped += res.skipped;
    }
    Ok(all)
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(super) struct QueryIn {
    #[schemars(description = "Dienst mit Logdatei aus logs_services, z. B. k3c-workbench")]
    service: String,
    #[serde(default)]
    #[schemars(description = "Mindest-Level: DEBUG, INFO, WARN oder ERROR")]
    level: String,
    #[serde(default)]
    #[schemars(description = "nur dieser Bereich, z. B. mcp")]
    ns: String,
    #[serde(default)]
    #[schemars(description = "regulärer Ausdruck auf die Meldung")]
    pattern: String,
    #[serde(default)]
    #[schemars(description = "Dauer wie 30m, 24h, 7d oder Zeitpunkt RFC 3339")]
    since: String,
    #[serde(default)]
    #[schemars(description = "höchstens so viele Einträge, Standard 100, höchstens 500")]
    limit: i64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(super) struct ErrorsIn {
    #[serde(default)]
    #[schemars(description = "Dienst mit Logdatei aus logs_services; leer: alle")]
    service: String,
    #[serde(default)]
    #[schemars(description = "Mindest-Level, Standard WARN")]
    level: String,
    #[serde(default)]
    #[schemars(description = "Standard 24h")]
    since: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(super) struct SinceIn {
    #[schemars(description = "Dienst mit Logdatei aus logs_services, z. B. k3c-workbench")]
    service: String,
    #[serde(default)]
    #[schemars(description = "Byte-Cursor aus der letzten Antwort, 0 = Anfang")]
    cursor: i64,
    #[serde(default)]
    #[schemars(description = "höchstens so viele Einträge, Standard 100, höchstens 500")]
    limit: i64,
}

impl Workbench {
    pub(super) fn source_path(&self, ws: &Workspace, source: &str) -> Result<PathBuf, String> {
        log_path(&ws.root, &self.env.root, source)
    }

    /// Die Datei eines Dienstes oder, ohne Namen, aller Log-Quellen des Checkouts.
    pub(super) fn source_paths(&self, ws: &Workspace, source: &str) -> Result<Vec<PathBuf>, String> {
        let names = if source.is_empty() {
            log_sources(&ws.root)
        } else {
            vec![source.to_owned()]
        };
        names.iter().map(|n| self.source_path(ws, n)).collect()
    }

    fn log_file_line(&self, ws: &Workspace, name: &str) -> String {
        let found = self
            .source_path(ws, name)
            .ok()
            .and_then(|path| std::fs::metadata(&path).ok().map(|meta| (path, meta.len())));
        let Some((path, size)) = found else {
            return format!("Log {name} · noch keine Einträge");
        };
        let last = Query {
            limit: 1,
            ..Query::default()
        };
        let size = format_bytes(size);
        match scan(&path, &last).ok().and_then(|res| res.entries.into_iter().next()) {
            Some(e) => {
                let at = e.time.with_timezone(&Local).format("%d.%m. %H:%M:%S");
                format!("Log {name} · {size} · zuletzt {at} {}", e.level)
            }
            None => format!("Log {name} · {size} · noch keine Einträge"),
        }
    }

    /// Log-Dateien und Konsolen-Quellen des eigenen Checkouts.
    fn sources_text(&self, ws: &Workspace) -> String {
        let mut out: Vec<String> = log_sources(&ws.root)
            .iter()
            .map(|name| self.log_file_line(ws, name))
            .collect();
        let console = &self.env.console;
        for name in console.sources() {
            if let Some(own) = ws.own_source(&name) {
                let lines = console.tail(&name, 0).map_or(0, |l| l.len());
                out.push(format!("Konsole {own} · {lines} Zeilen"));
            }
        }
        out.join("\n")
    }

    /// Gefilterte Einträge, neueste zuerst.
    fn query_text(&self, ws: &Workspace, input: QueryIn) -> Result<String, String> {
        let path = self.source_path(ws, &input.service)?;
        let mut query = Query {
            min_level: input.level,
            ns: input.ns,
            limit: clamp_limit(input.limit),
            since: parse_since(&input.since, now())?,
            ..Query::default()
        };
        if !input.pattern.is_empty() {
            let pattern = &input.pattern;
            let compiled = Regex::new(pattern)
                .map_err(|e| format!("pattern {pattern:?} ist kein gültiger regulärer Ausdruck: {e}"))?;
            query.pattern = Some(compiled);
        }
        let res = read(&path, &query)?;
        let mut out: Vec<String> = res.entries.iter().map(entry_line).collect();
        out.push(scan_footer(&format!("{} Einträge", res.entries.len()), &res));
        Ok(out.join("\n"))
    }

    /// Warnungen und Fehler, gleichartige zu je einer Zeile verdichtet.
    fn errors_text(&self, ws: &Workspace, input: ErrorsIn) -> Result<String, String> {
        let paths = self.source_paths(ws, &input.service)?;
        let or = |value: String, default: &str| if value.is_empty() { default.to_owned() } else { value };
        let (level, since) = (or(input.level, "WARN"), or(input.since, "24h"));
        let query = Query {
            min_level: level.clone(),
            limit: ERRORS_SCAN_LIMIT,
            since: parse_since(&since, now())?,
            ..Query::default()
        };
        let res = scan_all(&paths, &query)?;
        let groups = digest(&res.entries);
        let mut out: Vec<String> = groups
            .iter()
            .map(|g| {
                let (count, example) = (g.count, clip(&g.example));
                format!("{count}× {} {} {example} · {}", g.level, g.ns, window(g.first, g.last))
            })
            .collect();
        let head = format!(
            "{} Gruppen aus {} Einträgen ab {level} seit {since}",
            groups.len(),
            res.entries.len()
        );
        out.push(scan_footer(&head, &res));
        Ok(out.join("\n"))
    }

    /// Neue Einträge ab einem Byte-Cursor, älteste zuerst, und der nächste Cursor.
    fn since_text(&self, ws: &Workspace, input: &SinceIn) -> Result<String, String> {
        let path = self.source_path(ws, &input.service)?;
        let res =
            read_since(&path, input.cursor, clamp_limit(input.limit), DEFAULT_BUDGET).map_err(|e| e.to_string())?;
        let mut out: Vec<String> = res.entries.iter().map(entry_line).collect();
        let mut foot = format!("{} neue Einträge · cursor={}", res.entries.len(), res.cursor);
        if res.truncated {
            foot.push_str(" · gekürzt (Datei neu begonnen oder mehr als das Limit)");
        }
        out.push(foot);
        Ok(out.join("\n"))
    }
}

#[tool_router(router = logs_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "Dienste mit Logdatei (logs/*.jsonl mit Größe und letztem Eintrag) und Konsolen-Quellen. \
            Nutze es bei: welcher Name gilt für service. Statt: ls logs/.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn logs_services(&self, Extension(ws): Extension<Workspace>) -> String {
        self.sources_text(&ws)
    }

    #[tool(
        description = "Warnungen und Fehler, gleichartige Meldungen zu je einer Zeile verdichtet (Standard: ab WARN, \
            24h, alle Dienste). Nutze es bei: erste Fehlersuche. Statt: Logdatei lesen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn logs_errors(
        &self,
        Parameters(input): Parameters<ErrorsIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        self.errors_text(&ws, input)
    }

    #[tool(
        description = "Logzeilen eines Dienstes, neueste zuerst, gefiltert nach level, ns, pattern (Regex auf die \
            Meldung) und since. Nutze es bei: gezielte Suche. Statt: Datei lesen oder grep.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn logs_query(
        &self,
        Parameters(input): Parameters<QueryIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        self.query_text(&ws, input)
    }

    #[tool(
        description = "Neue Zeilen eines Dienstes ab einem Byte-Cursor, älteste zuerst, mit dem Cursor für den \
            nächsten Aufruf. Nutze es bei: Tailing während eines Tests. Statt: tail -f.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn logs_since(
        &self,
        Parameters(input): Parameters<SinceIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        self.since_text(&ws, &input)
    }
}
