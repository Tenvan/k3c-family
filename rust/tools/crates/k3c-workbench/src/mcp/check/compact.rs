//! Verdichtung der Ausgabe (Port von `check_compact.go`): grün eine Zeile, sonst Kopfzeile und höchstens die
//! Fehlerzeilen.

use std::sync::LazyLock;

use regex::Regex;

use super::super::status::format_ms;
use super::run::RunResult;
use super::targets::{CheckTarget, Kind};

/// Begrenzt die Fehlerzeilen einer Antwort.
pub(super) const MAX_ERROR_LINES: usize = 60;
/// Letzte Zeilen, wenn ein roter Lauf keine bekannte Fehlerzeile hat.
pub(super) const FALLBACK_LINES: usize = 20;

static ANSI: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"\x1b\[[0-9;?]*[ -/]*[@-~]").ok());

/// Muster für Fehlerzeilen je Werkzeug, abgelesen an echten Läufen in diesem Repo (`tests/data/check/`).
const PATTERNS: &[(Kind, &[&str])] = &[
    (
        Kind::Vitest,
        &[
            r"^\s*FAIL\s",
            r"^\s*[A-Za-z]*Error: ",
            r"❯ \S+:\d+:\d+",
            r"^\s*Test Files\s",
            r"^\s*Tests\s+\d",
        ],
    ),
    (Kind::Tsc, &[r"^\S+\(\d+,\d+\): error TS\d+"]),
    (Kind::Oxlint, &[r"^\S+:\d+:\d+: error "]),
    (
        Kind::GoTest,
        &[
            r"^--- FAIL",
            r"^\s+\S+\.go:\d+: ",
            r"^FAIL\s",
            r"^panic: ",
            r"^\S+\.go:\d+:\d+: ",
        ],
    ),
    (Kind::Golangci, &[r"^\S+\.go:\d+:\d+: .*\(\w+\)$", r"^\d+ issues?:"]),
    (
        Kind::Vite,
        &[
            r"^error during build",
            r"^Build failed with",
            r"^\[plugin ",
            r"^Error: ",
        ],
    ),
];

static COMPILED: LazyLock<Vec<(Kind, Vec<Regex>)>> = LazyLock::new(|| {
    PATTERNS
        .iter()
        .map(|(kind, list)| (*kind, list.iter().filter_map(|p| Regex::new(p).ok()).collect()))
        .collect()
});

/// Ob ein Muster eines der Werkzeuge die Zeile erkennt.
pub(super) fn matches_any(line: &str, kinds: &[Kind]) -> bool {
    COMPILED
        .iter()
        .filter(|(kind, _)| kinds.contains(kind))
        .any(|(_, list)| list.iter().any(|re| re.is_match(line)))
}

/// Ohne ANSI-Farben und Leerzeilen.
fn clean_lines(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|l| {
            let plain = ANSI
                .as_ref()
                .map_or_else(|| l.as_str().into(), |re| re.replace_all(l, ""));
            let line = plain.trim_end_matches([' ', '\t']);
            (!line.trim().is_empty()).then(|| line.to_owned())
        })
        .collect()
}

/// Antwort von `check_run`.
pub(super) fn report(target: &CheckTarget, res: &RunResult, output: &[String]) -> String {
    let head = if res.timed_out {
        format!(
            "{} · Zeitlimit {} überschritten · {}",
            target.name,
            format_ms(target.timeout.as_millis() as f64),
            format_ms(res.ms())
        )
    } else {
        format!("{} · exit {} · {}", target.name, res.exit, format_ms(res.ms()))
    };
    if res.exit == 0 && !res.timed_out {
        return head;
    }
    let lines = clean_lines(output);
    let mut found: Vec<String> = lines
        .iter()
        .filter(|l| matches_any(l, target.kinds))
        .map(|l| l.trim().to_owned())
        .collect();
    if found.is_empty() {
        found = lines[lines.len().saturating_sub(FALLBACK_LINES)..].to_vec();
    }
    if found.len() > MAX_ERROR_LINES {
        let more = found.len() - MAX_ERROR_LINES;
        found.truncate(MAX_ERROR_LINES);
        found.push(format!("… {more} weitere"));
    }
    found.insert(0, head);
    found.join("\n")
}

#[cfg(test)]
pub(super) fn all_patterns_compile() -> bool {
    PATTERNS.len() == COMPILED.len()
        && PATTERNS
            .iter()
            .zip(COMPILED.iter())
            .all(|(p, c)| p.1.len() == c.1.len())
}
