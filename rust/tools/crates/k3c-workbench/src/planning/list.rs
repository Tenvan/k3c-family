//! `plan_list`: Sprints (mit Sessions), Tickets und Projekte als knappe Zeilen.

use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use super::planning::{Data, Sprint, Ticket, is_ticket_file, load, parse_ticket};
use super::priority::sprint_key;
use super::projects::Project;
use super::store::{docs_path, entries, read_text};
use super::tables::first_of;
use super::ticketid::ticket_cmp;

/// Filter für [`list`]; leere Felder filtern nicht.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, schemars::JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct Filter {
    /// ticket, sprint, projekt oder leer für Sprints und Tickets
    pub kind: String,
    /// Status, z. B. aktiv, geplant, offen, eingeplant, erledigt
    pub status: String,
    /// Domäne: REG, SIM, SRV, CLI, PLAT, INF, DEV, PAP
    pub domain: String,
    /// Sprint-ID: nur dieser Sprint und seine Tickets; – oder ungeplant: Tickets ohne Sprint
    #[schemars(
        description = "Sprint-ID: nur dieser Sprint und seine Tickets; – oder ungeplant: Tickets ohne Sprint (wie der \
            Schnellfilter Ungeplant der Workbench)"
    )]
    pub sprint: String,
    /// Projekt-Kürzel: nur dieses Projekt, seine Sprints und Tickets
    pub projekt: String,
    /// auch erledigte und verworfene Tickets aus backlog/archiv
    pub archive: bool,
}

/// Liefert Sprints (mit Sessions) und Tickets als knappe Zeilen.
pub fn list(root: &Path, f: &Filter) -> Result<String, String> {
    let mut data = load(root)?;
    let all = data.tickets.clone(); // nicht archivierte Tickets: Bezug der Kennzahl-Zeile
    if f.archive {
        data.tickets.extend(archived(root));
    }
    let out = match f.kind.as_str() {
        "projekt" => project_lines(&data, f),
        "sprint" => sprint_list(&mut data, f),
        "ticket" => ticket_list(&data, f, &all, true),
        _ => {
            let mut lines = sprint_list(&mut data, f);
            lines.extend(ticket_list(&data, f, &all, false));
            lines
        }
    };
    if out.is_empty() {
        return Ok("keine Treffer".to_owned());
    }
    Ok(out.join("\n"))
}

/// Leerer Filter oder gleich ohne Rücksicht auf Groß/klein.
pub(super) fn matches(want: &str, have: &str) -> bool {
    want.is_empty() || want.to_lowercase() == have.to_lowercase()
}

/// „–“ oder „ungeplant“ als Sprint-Filter meint Tickets ohne Sprint, wie der Schnellfilter der Planungsseite.
pub(super) fn unplanned(want: &str) -> bool {
    want == "–" || want == "-" || want.to_lowercase() == "ungeplant"
}

/// Vergleicht den Sprint eines Tickets mit dem Filter.
fn matches_sprint(want: &str, have: &str) -> bool {
    if unplanned(want) {
        return matches!(have, "" | "–" | "-");
    }
    matches(want, have)
}

fn sprint_list(data: &mut Data, f: &Filter) -> Vec<String> {
    sort_sprints(&mut data.sprints, &data.projects);
    data.sprints
        .iter()
        .filter(|s| {
            matches(&f.status, &s.status)
                && matches(&f.domain, &s.domain)
                && !unplanned(&f.sprint)
                && matches(&f.sprint, &s.id)
                && matches(&f.projekt, &s.project)
        })
        .flat_map(sprint_lines)
        .collect()
}

/// Kennzahl-Zeile vor den Tickets: gezeigt von allen nicht archivierten, dazu offen, eingeplant und ungeplant (wie die
/// Schnellfilter der Oberfläche; ungeplant = ohne Sprint, unabhängig vom Status).
fn counts_line(shown: usize, all: &[Ticket]) -> String {
    let count = |wanted: &dyn Fn(&Ticket) -> bool| all.iter().filter(|t| wanted(t)).count();
    format!(
        "Tickets: {shown} von {} · {} offen · {} eingeplant · {} ungeplant",
        all.len(),
        count(&|t| t.status == "offen"),
        count(&|t| t.status == "eingeplant"),
        count(&|t| matches_sprint("ungeplant", &t.sprint)),
    )
}

/// Die Kennzahl-Zeile steht vorn, ohne `always` nur, wenn Tickets passen (sonst „keine Treffer“).
fn ticket_list(data: &Data, f: &Filter, all: &[Ticket], always: bool) -> Vec<String> {
    let mut out: Vec<String> = data
        .tickets
        .iter()
        .filter(|t| {
            matches(&f.status, &t.status)
                && matches(&f.domain, &t.domain)
                && matches_sprint(&f.sprint, &t.sprint)
                && matches(&f.projekt, &t.project)
        })
        .map(ticket_line)
        .collect();
    if !out.is_empty() || always {
        out.insert(0, counts_line(out.len(), all));
    }
    out
}

fn ticket_line(t: &Ticket) -> String {
    format!(
        "{} {} {} {} {} {} {} · {}{}",
        t.nr,
        t.domain,
        t.typ,
        t.prio,
        t.env,
        t.status,
        t.sprint,
        t.title,
        rel_suffix(t)
    )
}

/// Gesetzte Relationen hinter dem Titel: „→ B-1“ bei Duplikaten, „ersetzt B-2, B-3“.
fn rel_suffix(t: &Ticket) -> String {
    let mut out = String::new();
    if !t.duplicate.is_empty() {
        out = format!(" → {}", t.duplicate);
    }
    if !t.replaces.is_empty() {
        out = format!("{out} · ersetzt {}", t.replaces.join(", "));
    }
    out
}

/// Je Projekt `Rang Kürzel Status Sprints erledigt/gesamt · Titel` und die Sprints in Reihenfolge.
fn project_lines(data: &Data, f: &Filter) -> Vec<String> {
    let status: HashMap<&str, &str> = data
        .sprints
        .iter()
        .map(|s| (s.id.as_str(), s.status.as_str()))
        .collect();
    let state = |id: &str| status.get(id).copied().unwrap_or_default();
    let mut out = Vec::new();
    for p in &data.projects {
        if !matches(&f.status, &p.status) || !matches(&f.projekt, &p.id) {
            continue;
        }
        let done = p.sprints.iter().filter(|id| state(id) == "erledigt").count();
        let total = p.sprints.len();
        out.push(format!(
            "{} {} {} Sprints {done}/{total} · {}",
            p.rang, p.id, p.status, p.title
        ));
        if total > 0 {
            let names: Vec<String> = p
                .sprints
                .iter()
                .map(|id| format!("{id} {}", first_of(&[state(id), "fehlt"])))
                .collect();
            out.push(format!("  {}", names.join(" · ")));
        }
    }
    out
}

fn sprint_lines(s: &Sprint) -> Vec<String> {
    let done = s
        .sessions
        .iter()
        .filter(|x| x.status == "fertig" || x.status == "verworfen")
        .count();
    let mut out = vec![format!(
        "{} {} {} Reife {} Spec {} · {} · Sessions {done}/{}",
        s.id,
        s.domain,
        s.status,
        s.reife,
        s.spec,
        s.title,
        s.sessions.len()
    )];
    if s.status == "erledigt" {
        return out;
    }
    out.extend(s.sessions.iter().map(|x| {
        let line = format!(
            "  {} {} {} {} {} {} {}",
            x.nr, x.domain, x.typ, x.agent, x.env, x.status, x.titel
        );
        line.trim_end_matches(' ').to_owned()
    }));
    out
}

pub fn archived(root: &Path) -> Vec<Ticket> {
    let mut out: Vec<Ticket> = entries(&docs_path(root, "backlog/archiv"))
        .iter()
        .filter(|e| is_ticket_file(&e.name))
        .filter_map(|e| read_text(root, &format!("backlog/archiv/{}", e.name)).ok())
        .map(|(text, _)| parse_ticket(&text))
        .collect();
    out.sort_by(|a, b| ticket_cmp(&a.nr, &b.nr));
    out
}

/// Ordnet je Status (aktiv, geplant, erledigt) nach dem Projektplatz, sonst bleibt die Fahrplan-Reihenfolge.
pub fn sort_sprints(sprints: &mut [Sprint], projects: &[Project]) {
    let place = |status: &str| match status {
        "geplant" => 1,
        "erledigt" => 2,
        _ => 0,
    };
    let key = sprint_key(projects);
    sprints.sort_by_cached_key(|s| (place(&s.status), key(s)));
}
