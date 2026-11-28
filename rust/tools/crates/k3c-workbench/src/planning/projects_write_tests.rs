//! Go `projects_test.go`, der Teil mit `create` und `delete` (B-357). Ränge über `set` prüft `projects_tests.rs`.

use std::path::Path;

use super::create::{create, delete, get};
use super::create_fixtures::{check_consistent, check_projects, make, new_doc, new_project, new_sprint, new_ticket};
use super::edit::{section, set};
use super::fixtures::{doc, repo, snapshot, values};
use super::list::{Filter, list};
use super::projects::{find_project, load_projects};

fn ranks(root: &Path) -> Vec<(String, String)> {
    load_projects(root).into_iter().map(|p| (p.id, p.rang)).collect()
}

fn pairs(list: &[(&str, &str)]) -> Vec<(String, String)> {
    list.iter().map(|(a, b)| ((*a).to_owned(), (*b).to_owned())).collect()
}

fn sprints_of(root: &Path, id: &str) -> Vec<String> {
    find_project(&load_projects(root), id).expect(id).sprints.clone()
}

fn projects_list(root: &Path, kind: &str, projekt: &str) -> String {
    let filter = Filter {
        kind: kind.to_owned(),
        projekt: projekt.to_owned(),
        ..Filter::default()
    };
    list(root, &filter).expect("list")
}

#[test]
fn projekt_anlegen_lesen_listen() {
    let tmp = repo();
    let root = tmp.path();
    new_project(root, "GRA", "grafik");
    new_project(root, "SND", "sound");
    assert_eq!(ranks(root), pairs(&[("GRA", "1"), ("SND", "2")]));
    let overview = doc(root, "projekte/README.md");
    let rows = "| 1 | GRA · Titel GRA | – | [GRA-grafik.md](GRA-grafik.md) |\n| 2 | SND · Titel SND |";
    assert!(overview.contains(rows), "{overview}");
    assert!(
        !overview.contains("Noch keine Projekte") && !overview.contains("\n\n\n"),
        "{overview}"
    );
    let text = doc(root, "projekte/GRA-grafik.md");
    assert!(!text.contains("SP00"), "{text}");
    assert!(
        text.contains("- **Status:** aktiv\n- **Rang:** 1\n- **Ziel-Tickets:** –\n"),
        "{text}"
    );
    assert!(
        get(root, "GRA")
            .expect("get")
            .starts_with("docs/projekte/GRA-grafik.md\n")
    );
    section(root, "GRA", "Ziel", "Alles hübsch.").expect("section");
    assert!(doc(root, "projekte/GRA-grafik.md").contains("## Ziel\n\nAlles hübsch.\n\n## Sprints"));
    new_sprint(root, "GR1", &[("Projekt", "GRA")]);
    let out = projects_list(root, "projekt", "");
    let head = "1 GRA aktiv Sprints 0/1 · Titel GRA\n  GR1 geplant\n2 SND aktiv Sprints 0/0";
    assert!(out.starts_with(head), "{out}");
    let only = projects_list(root, "", "GRA");
    assert!(only.starts_with("GR1 ") && !only.contains("B-001"), "{only}");
    check_projects(root);
}

#[test]
fn abn_bekommt_keinen_rang() {
    let tmp = repo();
    let root = tmp.path();
    for (id, slug) in [("GRA", "grafik"), ("ABN", "abnahmen"), ("SND", "sound")] {
        new_project(root, id, slug);
    }
    assert_eq!(ranks(root), pairs(&[("GRA", "1"), ("SND", "2"), ("ABN", "–")]));
    set(root, "ABN", &values(&[("Rang", "1")])).expect_err("ABN mit Rang");
    let out = projects_list(root, "projekt", "");
    assert!(
        out.contains("2 SND aktiv Sprints 0/0 · Titel SND\n– ABN aktiv"),
        "{out}"
    );
}

#[test]
fn projekt_zuordnung_und_loeschen() {
    let tmp = repo();
    let root = tmp.path();
    new_project(root, "GRA", "grafik");
    new_project(root, "SND", "sound");
    new_sprint(root, "X1", &[]);
    new_sprint(root, "X2", &[("Projekt", "GRA")]);
    set(root, "X1", &values(&[("Projekt", "GRA")])).expect("Projekt");
    assert_eq!(sprints_of(root, "GRA"), ["X2", "X1"]);
    set(root, "GRA", &values(&[("Sprints", "X1, X2")])).expect("Reihenfolge");
    assert_eq!(sprints_of(root, "GRA"), ["X1", "X2"]);
    let release = [
        ("Reife", "bereit"),
        ("Spec", "freigegeben"),
        ("Freigabe", "2026-10-08 Test"),
        ("Status", "aktiv"),
    ];
    set(root, "X1", &values(&release)).expect("aktivieren");
    let text = doc(root, "projekte/GRA-grafik.md");
    assert!(
        text.contains("| X1 | Sprint X1 | aktiv |\n| X2 | Sprint X2 | geplant |"),
        "{text}"
    );
    set(root, "X2", &values(&[("Projekt", "SND")])).expect("Projekt");
    assert_eq!(sprints_of(root, "GRA"), ["X1"]);
    let ticket = [("Domäne", "CLI"), ("Typ", "Idee"), ("Prio", "hoch"), ("Projekt", "SND")];
    make(root, "ticket", "", "ton", "Ton", &ticket);
    let error = delete(root, "SND").expect_err("Projekt mit Sprint");
    assert!(error.contains("SND hat Sprints (X2)"), "{error}");
    delete(root, "X2").expect("Sprint löschen");
    let error = delete(root, "SND").expect_err("Projekt mit Ticket");
    assert!(error.contains("Ticket B-002 nennt SND"), "{error}");
    set(root, "B-002", &values(&[("Projekt", "–")])).expect("lösen");
    delete(root, "SND").expect("löschen");
    assert!(!doc(root, "projekte/README.md").contains("SND"));
    check_projects(root);
    check_consistent(root);
}

/// Auch ein Ticket im Archiv und ein Sprint, den die Tabelle nicht führt, halten das Projekt.
#[test]
fn loeschen_sieht_archiv_und_sprint_ohne_zeile() {
    let tmp = repo();
    let root = tmp.path();
    new_project(root, "GRA", "grafik");
    new_ticket(root, "t", "T", "SRV", "hoch");
    set(root, "B-002", &values(&[("Projekt", "GRA")])).expect("Projekt");
    set(root, "B-002", &values(&[("Status", "erledigt")])).expect("erledigt");
    let error = delete(root, "GRA").expect_err("Ticket im Archiv");
    assert!(error.contains("Ticket B-002 nennt GRA"), "{error}");
    let readme = "# X9 · SRV · S\n\n- **Projekt:** GRA\n";
    super::testsupport::write(root, "docs/sprints/aktiv/X9-s/README.md", readme);
    let error = delete(root, "GRA").expect_err("Sprint");
    assert_eq!(error, "Sprint X9 nennt GRA");
}

/// Aufrufe, die an einem Projekt scheitern.
fn bad_calls(root: &Path) -> Vec<(&'static str, Result<String, String>)> {
    let change = |id: &str, name: &str, value: &str| set(root, id, &values(&[(name, value)]));
    let make = |kind: &str, id: &str, fields: &[(&str, &str)]| create(root, &new_doc(kind, id, "x", "x", fields));
    vec![
        ("unbekanntes Projekt am Sprint", change("X1", "Projekt", "XYZ")),
        ("unbekanntes Projekt am Ticket", change("B-002", "Projekt", "XYZ")),
        ("Rang 0", change("GRA", "Rang", "0")),
        ("Rang über n", change("GRA", "Rang", "3")),
        ("Rang keine Zahl", change("GRA", "Rang", "eins")),
        ("Rang ruhend", change("BAL", "Rang", "1")),
        ("Sprints fremd", change("GRA", "Sprints", "X1, X2")),
        ("Sprints unvollständig", change("GRA", "Sprints", "X2")),
        ("Status falsch", change("GRA", "Status", "fertig")),
        ("Löschen mit Sprint", delete(root, "GRA")),
        ("Kürzel falsch", make("projekt", "GR", &[])),
        ("Kürzel doppelt", make("projekt", "GRA", &[])),
        ("Rang beim Anlegen", make("projekt", "NEU", &[("Rang", "1")])),
        (
            "Sprint mit unbekanntem Projekt",
            make("sprint", "X9", &[("Domäne", "SRV"), ("Projekt", "XYZ")]),
        ),
    ]
}

#[test]
fn ablehnungen_aendern_nichts() {
    let tmp = repo();
    let root = tmp.path();
    for (id, slug) in [("GRA", "grafik"), ("SND", "sound"), ("BAL", "bal")] {
        new_project(root, id, slug);
    }
    set(root, "BAL", &values(&[("Status", "ruht")])).expect("ruht");
    new_sprint(root, "X1", &[("Projekt", "GRA")]);
    new_sprint(root, "X2", &[("Projekt", "SND")]);
    new_ticket(root, "t", "T", "SRV", "hoch");
    let before = snapshot(root);
    for (name, result) in bad_calls(root) {
        assert!(result.is_err(), "{name}: kein Fehler: {result:?}");
        assert!(snapshot(root) == before, "{name}: Dateien geändert");
    }
}
