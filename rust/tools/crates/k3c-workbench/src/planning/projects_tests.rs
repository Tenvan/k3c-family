//! Go `projects_test.go`, Lese-Teil von `TestProjektAnlegenLesenListen`, und der Lese-Teil von
//! `domains_test.go › TestProjektSprintVorSprintOhneProjekt`. Anlegen, Rang und Ablehnungen: RU69.9 und RU69.10.

use std::path::Path;

use super::create::get;
use super::list::{Filter, list};
use super::planning::load;
use super::projects::{load_projects, projects_in};
use super::store::ChangeSet;
use super::testsupport::{sprint, temp_repo, write};

fn project(root: &Path, id: &str, status: &str, rank: &str, sprints: &[&str]) {
    let rows: String = sprints.iter().map(|s| format!("| {s} | x |\n")).collect();
    let text = format!(
        "# {id} · Titel {id}\n\n- **Status:** {status}\n- **Rang:** {rank}\n\n## Sprints\n\n\
         | Sprint | Inhalt |\n|---|---|\n{rows}\n## Ziel\n"
    );
    write(root, &format!("docs/projekte/{id}-p.md"), &text);
}

fn run(root: &Path, kind: &str, projekt: &str) -> String {
    let filter = Filter {
        kind: kind.to_owned(),
        projekt: projekt.to_owned(),
        ..Filter::default()
    };
    list(root, &filter).expect("list")
}

#[test]
fn projekte_lesen_und_listen() {
    let tmp = temp_repo();
    let root = tmp.path();
    write(root, "docs/projekte/README.md", "# Projekte\n");
    project(root, "SND", "aktiv", "2", &[]);
    project(root, "GRA", "aktiv", "1", &["GR1"]);
    project(root, "ABN", "aktiv", "–", &[]);
    project(root, "ALT", "erledigt", "–", &["XX9"]);
    project(root, "RUH", "ruht", "–", &[]);
    let ids: Vec<String> = load_projects(root).into_iter().map(|p| p.id).collect();
    assert_eq!(ids, ["GRA", "SND", "ABN", "RUH", "ALT"]);
    sprint(root, "GR1", "GRA", "–");
    assert!(get(root, "GRA").expect("get").starts_with("docs/projekte/GRA-p.md\n"));
    let out = run(root, "projekt", "");
    let head = "1 GRA aktiv Sprints 0/1 · Titel GRA\n  GR1 geplant\n2 SND aktiv Sprints 0/0 · Titel SND\n";
    assert!(out.starts_with(head), "{out}");
    assert!(
        out.ends_with("– ALT erledigt Sprints 0/1 · Titel ALT\n  XX9 fehlt"),
        "{out}"
    );
    let only = run(root, "", "GRA");
    assert!(only.starts_with("GR1 ") && !only.contains("B-001"), "{only}");
}

/// Die Projekte sehen geplante Änderungen des `ChangeSet`.
#[test]
fn projekte_mit_geplanten_aenderungen() {
    let tmp = temp_repo();
    let root = tmp.path();
    project(root, "GRA", "aktiv", "1", &[]);
    project(root, "SND", "aktiv", "2", &[]);
    let mut changes = ChangeSet::new(root);
    changes.write("projekte/NEU-n.md", "# NEU · Neu\n\n- **Status:** ruht\n".to_owned());
    changes.remove.push("projekte/SND-p.md".to_owned());
    let ids: Vec<String> = projects_in(&mut changes)
        .expect("lesen")
        .into_iter()
        .map(|p| p.id)
        .collect();
    assert_eq!(ids, ["GRA", "NEU"]);
}

/// Sprints mit Projekt stehen vor einem ohne, innerhalb des Projekts zählt der Platz in der Sprint-Tabelle (AC-04).
#[test]
fn projekt_sprint_vor_sprint_ohne_projekt() {
    let tmp = temp_repo();
    let root = tmp.path();
    project(root, "GRA", "aktiv", "1", &["X9", "X8"]);
    sprint(root, "X7", "–", "–");
    sprint(root, "X8", "GRA", "–");
    sprint(root, "X9", "GRA", "–");
    let out = run(root, "sprint", "");
    let at = |id: &str| out.find(id).expect(id);
    assert!(at("X9 ") < at("X8 ") && at("X8 ") < at("X7 "), "{out}");
    let ids: Vec<String> = load(root).expect("load").sprints.into_iter().map(|s| s.id).collect();
    assert_eq!(ids, ["X9", "X8", "X7"]);
}
