//! Tools `logs_stats` und `logs_context` (Port von `tools_logstats.go`).

use std::collections::BTreeMap;

use chrono::TimeDelta;
use k3c_kit::mcp::{Workspace, or_dash};
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::logs::{ERRORS_SCAN_LIMIT, clamp_limit, read, scan_all};
use super::logtext::{entry_line, now, parse_since, scan_footer};
use super::tools::Workbench;
use crate::logs::{Entry, Query, parse_rfc3339};

const DEFAULT_CONTEXT_LINES: usize = 10;
/// Spanne vor `ts`, die `logs_context` liest; genug für die Zeilen davor, ohne die ganze Datei.
const CONTEXT_WINDOW_HOURS: i64 = 6;

#[derive(Debug, Deserialize, JsonSchema)]
pub(super) struct StatsIn {
    #[serde(default)]
    #[schemars(description = "Dienst mit Logdatei aus logs_services; leer: alle")]
    service: String,
    #[serde(default)]
    #[schemars(description = "Dauer wie 30m, 24h oder Zeitpunkt RFC 3339; Standard 1h")]
    since: String,
    #[serde(default, rename = "groupBy")]
    #[schemars(description = "level (Standard) oder ns")]
    group_by: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(super) struct ContextIn {
    #[schemars(description = "Dienst mit Logdatei aus logs_services, z. B. k3c-workbench")]
    service: String,
    #[schemars(description = "Zeitpunkt RFC 3339, z. B. aus logs_errors oder logs_query")]
    ts: String,
    #[serde(default)]
    #[schemars(description = "Zeilen davor, Standard 10")]
    before: i64,
    #[serde(default)]
    #[schemars(description = "Zeilen danach, Standard 10")]
    after: i64,
    #[serde(default)]
    #[schemars(description = "höchstens so viele Zeilen insgesamt, Standard 100, höchstens 500")]
    limit: i64,
}

fn context_lines(n: i64) -> usize {
    usize::try_from(n)
        .ok()
        .filter(|&n| n > 0)
        .unwrap_or(DEFAULT_CONTEXT_LINES)
}

/// Zahl je Schlüssel, häufigste zuerst, bei Gleichstand nach Namen.
fn counted(entries: &[Entry], by_ns: bool) -> String {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for e in entries {
        *counts.entry(if by_ns { &e.ns } else { &e.level }).or_default() += 1;
    }
    let mut pairs: Vec<(&str, usize)> = counts.into_iter().collect();
    pairs.sort_by_key(|&(_, count)| std::cmp::Reverse(count));
    if pairs.is_empty() {
        return "noch keine".to_owned();
    }
    let parts: Vec<String> = pairs
        .iter()
        .map(|(key, count)| format!("{} {count}", or_dash(key)))
        .collect();
    parts.join(", ")
}

impl Workbench {
    /// Zahl der Einträge je Level oder Namensraum.
    fn stats_text(&self, ws: &Workspace, input: StatsIn) -> Result<String, String> {
        let or = |value: String, default: &str| if value.is_empty() { default.to_owned() } else { value };
        let group_by = or(input.group_by, "level");
        if group_by != "level" && group_by != "ns" {
            return Err(format!("groupBy {group_by:?} ungültig: erlaubt level oder ns"));
        }
        let since = or(input.since, "1h");
        let query = Query {
            since: parse_since(&since, now())?,
            limit: ERRORS_SCAN_LIMIT,
            ..Query::default()
        };
        let res = scan_all(&self.source_paths(ws, &input.service)?, &query)?;
        let head = format!("{} Einträge seit {since} je {group_by}", res.entries.len());
        Ok(format!(
            "{}\n{}",
            scan_footer(&head, &res),
            counted(&res.entries, group_by == "ns")
        ))
    }

    /// Zeilen um einen Zeitpunkt, älteste zuerst; „>“ markiert die erste Zeile ab `ts`.
    fn context_text(&self, ws: &Workspace, input: &ContextIn) -> Result<String, String> {
        let ts =
            parse_rfc3339(&input.ts).ok_or_else(|| format!("ts {:?} ist kein Zeitpunkt nach RFC 3339", input.ts))?;
        let path = self.source_path(ws, &input.service)?;
        let query = Query {
            since: ts.checked_sub_signed(TimeDelta::hours(CONTEXT_WINDOW_HOURS)),
            ..Query::default()
        };
        let mut entries = read(&path, &query)?.entries;
        entries.reverse(); // älteste zuerst
        let at = entries.partition_point(|e| e.time < ts);
        let from = at.saturating_sub(context_lines(input.before));
        let end = at.saturating_add(context_lines(input.after)).saturating_add(1);
        let to = end.min(entries.len()).min(from + clamp_limit(input.limit));
        let mut out: Vec<String> = (from..to)
            .map(|i| {
                let mark = if i == at { "> " } else { "  " };
                format!("{mark}{}", entry_line(&entries[i]))
            })
            .collect();
        out.push(format!("{} Zeilen um {}", to - from, input.ts));
        Ok(out.join("\n"))
    }
}

#[tool_router(router = logstats_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "Zahl der Einträge je Level oder Namensraum (groupBy) seit since, für einen oder alle Dienste. \
            Nutze es bei: Überblick, ob ein Dienst auffällig viel warnt. Statt: Zeilen zählen mit grep.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn logs_stats(
        &self,
        Parameters(input): Parameters<StatsIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        self.stats_text(&ws, input)
    }

    #[tool(
        description = "Zeilen um einen Zeitpunkt ts (before/after), älteste zuerst, \">\" markiert die erste Zeile ab \
            ts. Nutze es bei: was geschah vor und nach einem Fehler. Statt: Datei an der Stelle öffnen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn logs_context(
        &self,
        Parameters(input): Parameters<ContextIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        self.context_text(&ws, &input)
    }
}
