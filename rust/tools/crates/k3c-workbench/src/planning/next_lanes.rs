//! Lanes und Kollisionen von `plan_next`, dazu die Session-Felder, die `Set` prüft und nachträgt (Go
//! `next_lanes.go`).
//!
//! Kollision wie bei den Arbeitswellen (B-783, docs/arbeitsweise.md › Parallelität und Wellen): derselbe Sprint,
//! gleiche Sperre oder überlappende Erlaubte Dateien. Übergang für Sessions ohne Dateimenge: Sperre je Domäne und Lane.

use std::sync::LazyLock;

use regex::Regex;

use super::edit::head_end;
use super::next::{Candidate, session_head};
use super::planning::{Session, field_line};
use super::relations::{FIELD_DUP, FIELD_REPLACE};
use super::store::quoted;
use super::tables::{first_of, join_lines, split_lines};
use super::waves_spec::{area_overlap, shared_lock};

/// „Parallel möglich“: Sessions in Arbeit, Auswahl und Vorschläge zusammen höchstens vier Agenten (B-839).
const MAX_PARALLEL: usize = 3;

/// Werte des Session-Felds `Modell`; ohne Feld oder `–` gilt sonnet.
pub const MODELS: [&str; 5] = ["–", "haiku", "sonnet", "opus", "fable"];

/// Ein Kopf-Feld der Session; leer, wenn es fehlt oder mit `–` beginnt.
fn session_value(x: &Session, name: &str) -> String {
    session_head(x)
        .remove(name)
        .filter(|v| !v.starts_with('–'))
        .unwrap_or_default()
}

/// Das Feld `Lane` einer Session; leer, wenn es fehlt oder `–` ist.
pub(super) fn lane_of(x: &Session) -> String {
    session_value(x, "Lane")
}

/// Das Feld `Modell` einer Session; ohne Wert gilt sonnet.
fn model_of(x: &Session) -> String {
    let model = session_value(x, "Modell");
    first_of(&[&model, "sonnet (Standard)"]).to_owned()
}

/// `Domäne` oder `Domäne/Lane`.
fn lock_key(x: &Session) -> String {
    let lane = lane_of(x);
    if lane.is_empty() {
        x.domain.clone()
    } else {
        format!("{}/{lane}", x.domain)
    }
}

/// Eine Session mit ihrem Sprint, ihrer Dateimenge (Erlaubte Dateien) und den Sperren ihres Sprints (B-783).
#[derive(Debug, Clone, Default)]
pub(super) struct Scoped {
    pub session: Session,
    pub sprint: String,
    pub area: Vec<String>,
    pub locks: Vec<String>,
}

/// Warum `x` neben den Sessions in Arbeit nicht laufen darf; `None` = frei.
pub(super) fn busy(running: &[Scoped], x: &Scoped) -> Option<String> {
    running
        .iter()
        .find_map(|r| Some(format!("{} gesperrt ({} in Arbeit)", clash(r, x)?, r.session.nr)))
}

/// Die Kollisionsprüfung der Arbeitswellen (wie `plan_waves`). Sessions desselben Sprints laufen nie parallel (B-838).
/// Fehlt einer der beiden die Dateimenge, gilt die alte Regel nach Domäne und Lane.
fn clash(a: &Scoped, b: &Scoped) -> Option<String> {
    if !a.sprint.is_empty() && a.sprint == b.sprint {
        return Some(format!("Sprint {}", a.sprint));
    }
    if let Some(lock) = shared_lock(&a.locks, &b.locks).filter(|l| !l.is_empty()) {
        return Some(format!("Sperre {lock}"));
    }
    if !a.area.is_empty() && !b.area.is_empty() {
        return area_overlap(&a.area, &b.area).map(|pair| format!("Bereich {pair}"));
    }
    if !conflict(&a.session, &b.session) {
        return None;
    }
    let lane = lane_of(&b.session);
    if !lane.is_empty() && lane == lane_of(&a.session) {
        return Some(format!("Domäne {}, Lane {lane}", b.session.domain));
    }
    Some(format!("Domäne {}", b.session.domain))
}

/// Zwei Sessions ohne Dateimenge dürfen nicht gleichzeitig laufen (gleiche Domäne und gleiche oder fehlende Lane).
fn conflict(a: &Session, b: &Session) -> bool {
    let (la, lb) = (lane_of(a), lane_of(b));
    a.domain == b.domain && (la.is_empty() || lb.is_empty() || la == lb)
}

/// Weitere freie Sessions, die weder mit `pick` noch untereinander kollidieren: höchstens [`MAX_PARALLEL`] abzüglich
/// der Sessions in Arbeit (B-839).
pub(super) fn parallel<'a, 'd>(
    candidates: &'a [Candidate<'d>],
    pick: &Candidate<'d>,
    running: usize,
) -> Vec<&'a Candidate<'d>> {
    let limit = MAX_PARALLEL.saturating_sub(running);
    let mut chosen = vec![&pick.scope];
    let mut out = Vec::new();
    for c in candidates {
        if c.why.is_some() || out.len() >= limit {
            continue;
        }
        if chosen.iter().all(|s| clash(s, &c.scope).is_none()) {
            chosen.push(&c.scope);
            out.push(c);
        }
    }
    out
}

pub(super) fn parallel_lines(candidates: &[&Candidate]) -> Vec<String> {
    if candidates.is_empty() {
        return vec!["keine (sonst ist keine Session frei, die nicht kollidiert)".to_owned()];
    }
    candidates
        .iter()
        .map(|c| {
            format!(
                "- {} · {} · {} (Sprint {}, {}) · Modell: {}",
                lock_key(c.session),
                c.session.nr,
                c.session.titel,
                c.sprint.id,
                first_of(&[&c.session.env, "?"]),
                model_of(c.session)
            )
        })
        .collect()
}

/// Die Kopfzeile mit Domäne, Lane (nur wenn gesetzt), Umgebung und Modell.
pub(super) fn domain_line(x: &Session) -> String {
    let mut out = format!("Domäne: {}", x.domain);
    let lane = lane_of(x);
    if !lane.is_empty() {
        out = format!("{out} · Lane: {lane}");
    }
    format!(
        "{out} · Umgebung: {} · Modell: {}",
        first_of(&[&x.env, "?"]),
        model_of(x)
    )
}

static LANE: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^(–|[a-z0-9-]+)$").ok());

/// Das Session-Feld `Lane`: `–` oder Kurzname aus a-z, 0-9 und `-`.
pub fn check_lane(value: &str) -> Result<(), String> {
    if LANE.as_ref().is_some_and(|re| re.is_match(value)) {
        return Ok(());
    }
    Err(format!(
        "feld Lane = {} ungültig (– oder Kurzname aus a-z, 0-9, -)",
        quoted(value)
    ))
}

/// Felder, die in älteren Dateien fehlen dürfen, und die Felder, hinter die sie beim Setzen kommen.
fn optional_after(kind: &str, name: &str) -> &'static [&'static str] {
    match (kind, name) {
        ("session", "Lane") => &["Domäne"],
        ("session", "Modell") => &["Lane", "Domäne"],
        ("ticket", FIELD_DUP) => &["Freigabe", "Status"],
        ("ticket", FIELD_REPLACE) => &[FIELD_DUP, "Freigabe", "Status"],
        _ => &[],
    }
}

/// Fügt ein fehlendes optionales Feld hinter dem ersten vorhandenen Vorgänger ein; sonst `None`.
pub fn insert_field(kind: &str, text: &str, name: &str, value: &str) -> Option<String> {
    let mut lines = split_lines(text);
    let end = head_end(&lines);
    let at = optional_after(kind, name).iter().find_map(|previous| {
        lines[..end]
            .iter()
            .position(|l| field_line(l).is_some_and(|(found, _)| found == *previous))
    })?;
    lines.insert(at + 1, format!("- **{name}:** {value}"));
    Some(join_lines(&lines))
}
