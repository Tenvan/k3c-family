//! Nächste Session (B-518, Go `next.go`): `plan_next` wählt aus den lokalen Dateien wie docs/arbeitsweise.md ›
//! Autonomer Ablauf, Schritt 1: aktive Projekte nach Rang (ABN zuletzt) → ihre aktiven Sprints in der Reihenfolge der
//! Projekt-Tabelle → erste Session mit Status offen, Agent autonom, erledigten Abhängigkeiten und nicht gesperrter
//! Domäne. Der Stand der Sprint-Branches kommt aus den lokalen Refs origin/sprint/<präfix> (`next_origin.rs`, B-519).

use std::collections::HashMap;
use std::ops::Range;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use serde::Deserialize;

use super::edit::{DOMAINS, head_fields};
use super::next_lanes::{Scoped, busy, domain_line, parallel, parallel_lines};
use super::next_origin::{origin_overlay, sprint_prefix};
use super::planning::{Data, Session, Sprint, is_session_file, load};
use super::projects::Project;
use super::store::{docs_path, quoted, read_text};
use super::store_resolve::{find_entry, find_sprint};
use super::tables::{first_of, split_lines};
use super::waves_spec::{PlanSpec, allowed_files, parse_plan_spec};

/// Eingabe von [`next`]; leer = alle Domänen.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct NextFilter {
    /// nur Sessions dieser Domäne: REG, SIM, SRV, CLI, PLAT, INF, DEV, PAP
    pub domain: String,
}

// Muster wie Go: `\b` und `\d` meinen dort nur ASCII.
pub(super) static TICKET_REF: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"(?-u:\b)B-[0-9]+(?-u:\b)").ok());
static CRITERION: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^- \*\*(AC-[0-9]+)\*\*").ok());
static CRITERION_ID: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"(?-u:\b)AC-[0-9]+(?-u:\b)").ok());

/// So viele Gründe je Projekt, wenn keine Session frei ist.
const MAX_REASONS: usize = 3;

/// Eine offene Session eines aktiven Sprints; `why` fehlt, wenn sie frei ist.
pub(super) struct Candidate<'a> {
    pub project: &'a Project,
    pub sprint: &'a Sprint,
    pub session: &'a Session,
    pub scope: Scoped,
    pub why: Option<String>,
}

/// Ein aktives Projekt mit seinen aktiven Sprints und deren offenen Sessions (Bereich in der Kandidatenliste).
struct ProjectNext<'a> {
    project: &'a Project,
    active: Vec<&'a str>,
    candidates: Range<usize>,
}

/// Was die Prüfung je Session braucht.
struct NextState<'a> {
    root: &'a Path,
    /// Session-ID → Status aus der Session-Tabelle, alle Sprints
    status: HashMap<&'a str, &'a str>,
    /// Ticket-Nr → Status (Backlog ohne Archiv)
    tickets: HashMap<&'a str, &'a str>,
    /// Sessions in Arbeit aktiver Sprints mit Dateimenge und Sperren
    running: Vec<Scoped>,
    /// Sprint-ID → README von origin/sprint/<präfix> (B-519), sonst lokal
    readmes: HashMap<String, String>,
    /// Sprint-ID → Planungsangaben der README (Sperre, Bereich)
    specs: HashMap<&'a str, PlanSpec>,
}

/// Die nächste Session als verdichteter Text: Kopf, Session-Datei, ihre Kriterien aus der Sprint-README und parallel
/// mögliche Sessions (höchstens drei); ohne freie Session der Grund je Projekt.
pub fn next(root: &Path, f: &NextFilter) -> Result<String, String> {
    let want = f.domain.trim().to_uppercase();
    if !want.is_empty() && !DOMAINS.contains(&want.as_str()) {
        return Err(format!(
            "unbekannte Domäne {} ({})",
            quoted(&f.domain),
            DOMAINS.join(", ")
        ));
    }
    let mut data = load(root)?;
    let (notes, readmes) = origin_overlay(root, &mut data);
    let state = NextState::new(root, &data, readmes);
    let (plan, candidates) = state.plan(&data);
    let pick = candidates
        .iter()
        .find(|c| c.why.is_none() && (want.is_empty() || c.session.domain == want));
    let text = match pick {
        Some(pick) => state.pick_text(pick, &parallel(&candidates, pick, state.agents()))?,
        None => none_text(&plan, &candidates, &want),
    };
    Ok(with_notes(text, &notes))
}

/// Hängt die Herkunft des Stands je aktivem Sprint (origin oder lokal) an die Antwort.
fn with_notes(text: String, notes: &[String]) -> String {
    if notes.is_empty() {
        return text;
    }
    format!("{text}\n\n── Stand der Sprints ──\n{}", notes.join("\n"))
}

impl<'a> NextState<'a> {
    fn new(root: &'a Path, data: &'a Data, readmes: HashMap<String, String>) -> NextState<'a> {
        let mut state = NextState {
            root,
            status: HashMap::new(),
            tickets: data
                .tickets
                .iter()
                .map(|t| (t.nr.as_str(), t.status.as_str()))
                .collect(),
            running: Vec::new(),
            readmes,
            specs: HashMap::new(),
        };
        for sprint in &data.sprints {
            if sprint.status == "aktiv" {
                let spec = parse_plan_spec(&state.readme(&sprint.id));
                state.specs.insert(&sprint.id, spec);
            }
            for x in &sprint.sessions {
                state.status.insert(&x.nr, &x.status);
                // Auch ohne Domäne: Dateien und Sperren zählen (B-839).
                if sprint.status == "aktiv" && x.status == "in Arbeit" {
                    let scope = state.scope(sprint, x);
                    state.running.push(scope);
                }
            }
        }
        state
    }

    /// Die Sessions in Arbeit, die einen Agenten belegen; eine Mensch-Session sperrt mit Dateien und Sperren, belegt
    /// aber keinen der vier Plätze (B-839).
    fn agents(&self) -> usize {
        self.running.iter().filter(|r| r.session.agent == "autonom").count()
    }

    /// Je aktivem Projekt (Reihenfolge der Projekte) die aktiven Sprints seiner Tabelle, dazu deren offene Sessions in
    /// Tabellen-Reihenfolge.
    fn plan(&self, data: &'a Data) -> (Vec<ProjectNext<'a>>, Vec<Candidate<'a>>) {
        let active: HashMap<&str, &Sprint> = data
            .sprints
            .iter()
            .filter(|s| s.status == "aktiv")
            .map(|s| (s.id.as_str(), s))
            .collect();
        let (mut projects, mut candidates) = (Vec::new(), Vec::new());
        for project in data.projects.iter().filter(|p| p.status == "aktiv") {
            let from = candidates.len();
            let mut ids = Vec::new();
            for sprint in project.sprints.iter().filter_map(|id| active.get(id.as_str())) {
                ids.push(sprint.id.as_str());
                candidates.extend(self.open(project, sprint));
            }
            projects.push(ProjectNext {
                project,
                active: ids,
                candidates: from..candidates.len(),
            });
        }
        (projects, candidates)
    }

    /// Die Sessions mit Status offen eines Sprints, jede mit dem Grund, warum sie nicht dran ist.
    fn open(&self, project: &'a Project, sprint: &'a Sprint) -> impl Iterator<Item = Candidate<'a>> {
        sprint.sessions.iter().filter(|x| x.status == "offen").map(move |x| {
            let scope = self.scope(sprint, x);
            Candidate {
                project,
                sprint,
                session: x,
                why: self.blocked(&scope),
                scope,
            }
        })
    }

    /// Dateimenge aus „Erlaubte Dateien“ (sonst Bereich der Sprint-README) und Sperren des Sprints.
    fn scope(&self, sprint: &Sprint, x: &Session) -> Scoped {
        let spec = self.specs.get(sprint.id.as_str());
        let mut area = allowed_files(&x.text);
        if area.is_empty() {
            area = spec.map_or_else(Vec::new, |s| s.area.clone());
        }
        Scoped {
            session: x.clone(),
            sprint: sprint.id.clone(),
            area,
            locks: spec.map_or_else(Vec::new, |s| s.locks.clone()),
        }
    }

    /// Die Sprint-README von origin (B-519), sonst die lokale; leer, wenn keine lesbar ist.
    fn readme(&self, id: &str) -> String {
        if let Some(text) = self.readmes.get(id) {
            return text.clone();
        }
        find_sprint(self.root, id)
            .and_then(|found| read_text(self.root, &found.rel).ok())
            .map(|(text, _)| text)
            .unwrap_or_default()
    }

    /// Warum eine offene Session nicht dran ist; `None` = frei.
    fn blocked(&self, scope: &Scoped) -> Option<String> {
        let x = &scope.session;
        if x.agent != "autonom" {
            return Some(format!("Agent {}", first_of(&[&x.agent, "?"])));
        }
        if !is_session_file(&x.file) {
            return Some("ohne Datei".to_owned());
        }
        let open = self.open_deps(x);
        if !open.is_empty() {
            return Some(format!("wartet auf {}", open.join(", ")));
        }
        busy(&self.running, scope)
    }

    /// Die unerledigten Einträge aus „Abhängig von“: Sessions nicht fertig oder verworfen, Tickets weder erledigt noch
    /// verworfen noch im Archiv. Unbekannte IDs gelten als offen (Status „fehlt“).
    fn open_deps(&self, x: &Session) -> Vec<String> {
        let mut open = Vec::new();
        for id in &x.deps {
            let status = first_of(&[self.status.get(id.as_str()).copied().unwrap_or_default(), "fehlt"]);
            if status != "fertig" && status != "verworfen" {
                open.push(format!("{id} ({status})"));
            }
        }
        let head = session_head(x);
        let wanted = head.get("Abhängig von").map_or("", String::as_str);
        for nr in ticket_refs(&TICKET_REF, wanted) {
            let status = self.ticket_status(&nr);
            if status != "erledigt" && status != "verworfen" {
                open.push(format!("{nr} ({status})"));
            }
        }
        open
    }

    /// Status aus dem Backlog; im Archiv liegen nur erledigte und verworfene Tickets.
    fn ticket_status(&self, nr: &str) -> &str {
        if let Some(status) = self.tickets.get(nr) {
            return status;
        }
        let archive = docs_path(self.root, "backlog/archiv");
        if find_entry(&archive, &format!("{nr}-"), false).is_some() {
            "erledigt"
        } else {
            "fehlt"
        }
    }

    /// Kopf, Session-Datei, Kriterien aus der Sprint-README und die parallel möglichen Sessions.
    fn pick_text(&self, c: &Candidate, more: &[&Candidate]) -> Result<String, String> {
        let found =
            find_sprint(self.root, &c.sprint.id).ok_or_else(|| format!("sprint {} nicht gefunden", c.sprint.id))?;
        let readme = match self.readmes.get(&c.sprint.id) {
            Some(text) => text.clone(),
            None => read_text(self.root, &found.rel)?.0,
        };
        let x = c.session;
        let head = session_head(x);
        let wanted = first_of(&[head.get("Kriterien").map_or("", String::as_str), "–"]);
        let mut out = vec![
            format!("Nächste Session: {} · {}", x.nr, x.titel),
            format!("Pfad: docs/{}/{}", found.dir, x.file),
            format!(
                "Sprint: {} · {} (Projekt {}, Rang {})",
                c.sprint.id, c.sprint.title, c.project.id, c.project.rang
            ),
            domain_line(x),
            format!("Branch: sprint/{}", sprint_prefix(c.sprint)), // wie `origin_overlay` (B-839)
            format!("Kriterien: {wanted}"),
            "Hinweis: Stand je Sprint (origin oder lokal) siehe unten; Branches, die lokal noch nicht bekannt sind, \
             prüft der Agent mit git fetch --prune."
                .to_owned(),
            String::new(),
            "── Session-Datei ──".to_owned(),
            x.text.replace("\r\n", "\n").trim_end_matches('\n').to_owned(),
            String::new(),
            "── Kriterien aus der Sprint-README ──".to_owned(),
        ];
        let lines = criteria(&readme, wanted);
        if lines.is_empty() {
            out.push("keine".to_owned());
        } else {
            out.extend(lines);
        }
        out.extend([String::new(), "── Parallel möglich ──".to_owned()]);
        out.extend(parallel_lines(more));
        Ok(out.join("\n"))
    }
}

/// Die Kopf-Felder einer Session-Datei.
pub(super) fn session_head(x: &Session) -> HashMap<String, String> {
    head_fields(&x.text.replace("\r\n", "\n"))
}

pub(super) fn ticket_refs(re: &LazyLock<Option<Regex>>, text: &str) -> Vec<String> {
    re.as_ref().map_or_else(Vec::new, |re| {
        re.find_iter(text).map(|m| m.as_str().to_owned()).collect()
    })
}

/// Aus `## Akzeptanzkriterien` die Einträge, deren ID `want` nennt; „alle“ liefert alle. Eingerückte Folgezeilen
/// gehören zum Eintrag davor.
fn criteria(readme: &str, want: &str) -> Vec<String> {
    let ids = ticket_refs(&CRITERION_ID, want);
    let all = want.to_lowercase().contains("alle");
    let lines = split_lines(readme);
    let Some(start) = lines.iter().position(|l| l == "## Akzeptanzkriterien") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut keep = false;
    for line in lines.into_iter().skip(start + 1) {
        if line.starts_with("## ") {
            break;
        }
        if let Some(found) = CRITERION.as_ref().and_then(|re| re.captures(&line)) {
            keep = all
                || ids
                    .iter()
                    .any(|id| Some(id.as_str()) == found.get(1).map(|m| m.as_str()));
        } else if !line.starts_with(' ') {
            keep = false;
        }
        if keep {
            out.push(line);
        }
    }
    out
}

/// Keine Session frei, je aktivem Projekt kurz der Grund.
fn none_text(plan: &[ProjectNext], candidates: &[Candidate], want: &str) -> String {
    let mut title = "Keine Session frei".to_owned();
    if !want.is_empty() {
        title = format!("{title} in Domäne {want}");
    }
    let mut out = vec![format!("{title}. Gründe je Projekt:")];
    if plan.is_empty() {
        out.push("- kein aktives Projekt".to_owned());
    }
    for p in plan {
        let own = candidates.get(p.candidates.clone()).unwrap_or_default();
        out.push(format!("- {}: {}", p.project.id, reason(&p.active, own)));
    }
    out.join("\n")
}

/// Warum im Projekt keine Session dran ist; eine freie Session fiel nur dem Domänen-Filter zum Opfer.
fn reason(active: &[&str], candidates: &[Candidate]) -> String {
    if active.is_empty() {
        return "kein aktiver Sprint".to_owned();
    }
    if candidates.is_empty() {
        return format!("keine offene Session ({})", active.join(", "));
    }
    let mut why: Vec<String> = candidates
        .iter()
        .take(MAX_REASONS)
        .map(|c| match &c.why {
            Some(why) => format!("{} {why}", c.session.nr),
            None => format!("{} andere Domäne {}", c.session.nr, c.session.domain),
        })
        .collect();
    if candidates.len() > MAX_REASONS {
        why.push(format!("… {} weitere", candidates.len() - MAX_REASONS));
    }
    why.join("; ")
}
