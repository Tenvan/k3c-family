//! Hilfen der Tests, die über `create` bauen (Go `newTicket`, `newProject`, `newSprint`, `newSession`, `relRepo`) und
//! die Regelprüfungen `checkConsistent` und `checkProjects`. Wurzeln liegen unter `.work/` und tragen kein `.git`.

use std::path::Path;

use regex::Regex;

use super::create::{NewDoc, create};
use super::fixtures::{doc, repo, values};
use super::planning::{load, parse_ticket};
use super::projects::UNRANKED;
use super::store_resolve::find_sprint;
use super::testsupport::write;

pub(super) fn new_doc(kind: &str, id: &str, slug: &str, title: &str, fields: &[(&str, &str)]) -> NewDoc {
    NewDoc {
        kind: kind.to_owned(),
        id: id.to_owned(),
        slug: slug.to_owned(),
        title: title.to_owned(),
        fields: values(fields),
    }
}

/// Legt das Dokument an; ein Fehler bricht den Test ab.
pub(super) fn make(root: &Path, kind: &str, id: &str, slug: &str, title: &str, fields: &[(&str, &str)]) -> String {
    create(root, &new_doc(kind, id, slug, title, fields)).expect(title)
}

pub(super) fn new_ticket(root: &Path, slug: &str, title: &str, domain: &str, prio: &str) -> String {
    let fields = [("Domäne", domain), ("Typ", "Idee"), ("Prio", prio)];
    make(root, "ticket", "", slug, title, &fields)
}

pub(super) fn new_project(root: &Path, id: &str, slug: &str) {
    make(root, "projekt", id, slug, &format!("Titel {id}"), &[]);
}

/// Sprint `<id>-s` in der Domäne SRV; `fields` ergänzen oder ersetzen.
pub(super) fn new_sprint(root: &Path, id: &str, fields: &[(&str, &str)]) -> String {
    let mut all = values(&[("Domäne", "SRV")]);
    all.extend(values(fields));
    let mut input = new_doc("sprint", id, "s", &format!("Sprint {id}"), &[]);
    input.fields = all;
    create(root, &input).expect(id)
}

pub(super) fn new_session(root: &Path, id: &str, domain: &str) -> String {
    make(
        root,
        "session",
        id,
        "s",
        &format!("Session {id}"),
        &[("Domäne", domain)],
    )
}

/// B-002 bis B-004 über `create` (volle Vorlage mit Relationsfeldern), B-001 mit altem Kopf (Go `relRepo`).
pub(super) fn rel_repo() -> tempfile::TempDir {
    let tmp = repo();
    let root = tmp.path();
    let old = "# B-001 · Altes Ticket\n\n- **Domäne:** SRV\n- **Typ:** Idee\n- **Prio:** mittel\n- **Status:** offen\n\
               - **Sprint:** –\n";
    write(root, "docs/backlog/B-001-alt.md", old);
    for title in ["Zweites Ticket", "Drittes Ticket", "Viertes Ticket"] {
        new_ticket(root, "t", title, "DEV", "mittel");
    }
    tmp
}

/// Zeilen eines Index als (Nummer, Link, Status).
pub(super) fn index_links(text: &str) -> Vec<(String, String, String)> {
    let row = Regex::new(r"(?m)^\| \[(B-[0-9]{3,})\]\((.+?)\) \|.*\| (\S+) \| \S+ \| .* \|$").expect("Muster");
    row.captures_iter(text)
        .map(|m| (m[1].to_owned(), m[2].to_owned(), m[3].to_owned()))
        .collect()
}

/// Regeln aus tests/planning.test.ts, die die Schreibwege berühren: Index ↔ Tickets mit Status und Ordner,
/// Sprint-Status ↔ Ordner, Fahrplan nennt jeden Ordner, Session-Tabelle ↔ Dateien.
pub(super) fn check_consistent(root: &Path) {
    for (nr, link, status) in index_links(&doc(root, "backlog/README.md")) {
        let ticket = parse_ticket(&doc(root, &format!("backlog/{link}")));
        assert_eq!(ticket.status, status, "{nr}: Index und Datei");
        let archived = status == "erledigt" || status == "verworfen";
        assert_eq!(archived, link.starts_with("archiv/"), "{nr}: Ordner passt nicht");
    }
    let roadmap = doc(root, "sprints/README.md");
    for sprint in load(root).expect("load").sprints {
        let found = find_sprint(root, &sprint.id).expect("Sprint-Ordner");
        let head = super::edit::head_fields(&doc(root, &found.rel));
        assert_eq!(head.get("Status"), Some(&found.state), "{}", sprint.id);
        let folder = format!("`{}/`", found.dir.trim_start_matches("sprints/"));
        assert!(roadmap.contains(&folder), "{} fehlt im Fahrplan:\n{roadmap}", sprint.id);
        for session in sprint.sessions {
            assert!(!session.text.is_empty(), "{}: Datei {} fehlt", session.nr, session.file);
        }
    }
}

/// Regeln aus tests/planningProjects.test.ts: Ränge der aktiven Projekte lückenlos ab 1 (ABN ohne Rang), Sprint ↔
/// Sprint-Tabelle in beide Richtungen, Übersicht nennt jede Datei, Tickets nennen nur bestehende Projekte.
pub(super) fn check_projects(root: &Path) {
    let data = load(root).expect("load");
    let mut ranks = Vec::new();
    for p in &data.projects {
        if p.status == "aktiv" && p.id != UNRANKED {
            ranks.push(p.rang.parse::<usize>().unwrap_or(0));
        } else {
            assert_eq!(p.rang, "–", "{}: {} mit Rang", p.id, p.status);
        }
    }
    assert_eq!(
        ranks,
        (1..=ranks.len()).collect::<Vec<_>>(),
        "Ränge nicht lückenlos ab 1"
    );
    let overview = doc(root, "projekte/README.md");
    for p in &data.projects {
        assert!(overview.contains(&p.file), "Übersicht ohne {}", p.file);
        for id in &p.sprints {
            let named = data.sprints.iter().find(|s| &s.id == id).map(|s| s.project.as_str());
            assert_eq!(named, Some(p.id.as_str()), "{}: {id}", p.id);
        }
    }
    for s in data.sprints.iter().filter(|s| s.project != "–") {
        let listed = data
            .projects
            .iter()
            .any(|p| p.id == s.project && p.sprints.contains(&s.id));
        assert!(listed, "{}: fehlt in der Tabelle von {}", s.id, s.project);
    }
    for t in &data.tickets {
        let known = t.project == "–" || t.project.is_empty() || data.projects.iter().any(|p| p.id == t.project);
        assert!(known, "{}: Projekt {} fehlt", t.nr, t.project);
    }
}
