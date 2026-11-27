//! Go `projects_test.go`: Lesen, Rang, Zuordnung und Ablehnungen über `set`, dazu der Lese-Teil von
//! `domains_test.go › TestProjektSprintVorSprintOhneProjekt`. Anlegen und Löschen: `projects_write_tests.rs`.

use std::path::Path;

use super::create::get;
use super::edit::set;
use super::fixtures::{self, doc, full_ticket, repo, snapshot, values};
use super::list::{Filter, list};
use super::planning::load;
use super::projects::{load_projects, projects_in};
use super::store::ChangeSet;
use super::testsupport::{sprint, temp_repo, write};

/// GRA (Rang 1) und SND (Rang 2); die übrigen mit dem genannten Rang.
fn ranked_repo(ids: &[&str]) -> tempfile::TempDir {
    let tmp = repo();
    for (i, id) in ids.iter().enumerate() {
        fixtures::project(tmp.path(), id, &(i + 1).to_string(), &[]);
    }
    tmp
}

fn ranks(root: &Path) -> String {
    let projects = load_projects(root);
    let rank = |id: &str| {
        projects
            .iter()
            .find(|p| p.id == id)
            .map_or("fehlt", |p| p.rang.as_str())
    };
    format!("{} {} {}", rank("GRA"), rank("SND"), rank("BAL"))
}

#[test]
fn rang_bleibt_lueckenlos() {
    let tmp = ranked_repo(&["GRA", "SND", "BAL"]);
    let root = tmp.path();
    for (id, name, value, want) in [
        ("BAL", "Rang", "1", "2 3 1"),
        ("GRA", "Rang", "3", "3 2 1"),
        ("SND", "Status", "ruht", "2 – 1"),
        ("BAL", "Status", "erledigt", "1 – –"),
        ("SND", "Status", "aktiv", "1 2 –"),
        ("SND", "Rang", "1", "2 1 –"),
    ] {
        set(root, id, &values(&[(name, value)])).expect("Set");
        assert_eq!(ranks(root), want, "{id} {name}={value}");
    }
    let overview = doc(root, "projekte/README.md");
    let done = "## Erledigt\n\n| Projekt | Datei |\n|---|---|\n| BAL · Titel BAL | [BAL-bal.md](BAL-bal.md) |";
    assert!(overview.contains(done), "{overview}");
    assert!(
        !overview.contains("Noch keine Projekte") && !overview.contains("\n\n\n"),
        "{overview}"
    );
}

#[test]
fn abn_laeuft_ohne_rang() {
    let tmp = ranked_repo(&["GRA"]);
    let root = tmp.path();
    fixtures::project(root, "ABN", "–", &[]);
    set(root, "ABN", &values(&[("Rang", "1")])).expect_err("ABN mit Rang");
}

#[test]
fn sprint_zuordnung_zieht_die_tabellen_nach() {
    let tmp = repo();
    let root = tmp.path();
    fixtures::project(root, "GRA", "1", &["X2"]);
    fixtures::project(root, "SND", "2", &[]);
    fixtures::sprint(root, "X1", "–");
    fixtures::sprint(root, "X2", "GRA");
    let gra = || load_projects(root).swap_remove(0).sprints;
    set(root, "X1", &values(&[("Projekt", "GRA")])).expect("Projekt");
    assert_eq!(gra(), ["X2", "X1"]);
    set(root, "GRA", &values(&[("Sprints", "X1, X2")])).expect("Reihenfolge");
    assert_eq!(gra(), ["X1", "X2"]);
    let activate = [
        ("Reife", "bereit"),
        ("Spec", "freigegeben"),
        ("Freigabe", "2026-10-08 Test"),
        ("Status", "aktiv"),
    ];
    set(root, "X1", &values(&activate)).expect("aktivieren");
    let text = doc(root, "projekte/GRA-gra.md");
    assert!(
        text.contains("| X1 | Sprint X1 | aktiv |\n| X2 | Sprint X2 | geplant |"),
        "{text}"
    );
    set(root, "X2", &values(&[("Projekt", "SND")])).expect("umhängen");
    assert_eq!(gra(), ["X1"]);
}

/// B-357 › Ausnahme- und Fehlerfälle, soweit über `set`.
#[test]
fn ablehnungen_aendern_nichts() {
    let tmp = repo();
    let root = tmp.path();
    fixtures::project(root, "GRA", "1", &["X1"]);
    fixtures::project(root, "SND", "2", &["X2"]);
    fixtures::project(root, "BAL", "3", &[]);
    set(root, "BAL", &values(&[("Status", "ruht")])).expect("ruht");
    fixtures::sprint(root, "X1", "GRA");
    fixtures::sprint(root, "X2", "SND");
    full_ticket(root, "B-002", "t", "T");
    let before = snapshot(root);
    for (id, name, value, want) in [
        ("X1", "Projekt", "XYZ", "unbekanntes Projekt \"XYZ\""),
        ("B-002", "Projekt", "XYZ", "unbekanntes Projekt \"XYZ\""),
        ("GRA", "Rang", "0", "keine Zahl ab 1"),
        ("GRA", "Rang", "3", "außerhalb 1 … 2"),
        ("GRA", "Rang", "eins", "keine Zahl ab 1"),
        ("BAL", "Rang", "1", "nur aktive Projekte"),
        ("GRA", "Sprints", "X1, X2", "steht nicht in der Tabelle"),
        ("GRA", "Sprints", "X2", "steht nicht in der Tabelle"),
        ("GRA", "Status", "fertig", "ungültig"),
    ] {
        let error = set(root, id, &values(&[(name, value)])).expect_err(value);
        assert!(error.contains(want), "{id} {name}={value}: {error:?}");
        assert!(snapshot(root) == before, "{id} {name}={value}: Dateien geändert");
    }
}

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
