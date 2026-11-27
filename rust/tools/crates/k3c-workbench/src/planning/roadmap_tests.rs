//! Go `roadmap_test.go` (B-366, B-403, B-404), soweit über `Set`. `TestFahrplanZeileNachProjektRang` und die
//! Anlege-Schritte prüfen `Create` und folgen in RU69.10.

use std::path::Path;

use super::edit::set;
use super::fixtures::{ROADMAP_PROJEKT_MD, doc, project, repo, sprint, values};
use super::tables::{cells, find_table, heading, split_lines};
use super::testsupport::write;

/// Sprint-IDs in der Reihenfolge der Tabelle unter `title`.
fn order(root: &Path, title: &str) -> Vec<String> {
    let lines = split_lines(&doc(root, "sprints/README.md"));
    let table = find_table(&lines, heading(title)).expect("Tabelle");
    lines[table.first..table.end]
        .iter()
        .map(|l| cells(l).swap_remove(0))
        .collect()
}

/// Fahrplan mit Projekt-Spalte; GRA (Rang 1) und SND (Rang 2).
fn roadmap_repo(gra: &[&str], snd: &[&str]) -> tempfile::TempDir {
    let tmp = repo();
    let root = tmp.path();
    write(root, "docs/sprints/README.md", ROADMAP_PROJEKT_MD);
    project(root, "GRA", "1", gra);
    project(root, "SND", "2", snd);
    tmp
}

const FREIGABE: [(&str, &str); 3] = [
    ("Reife", "bereit"),
    ("Spec", "freigegeben"),
    ("Freigabe", "2026-10-11, Test"),
];

#[test]
fn aktivieren_setzt_die_zeile_nach_rang() {
    let tmp = roadmap_repo(&["GRA1"], &["SND1"]);
    let root = tmp.path();
    sprint(root, "SND1", "SND");
    sprint(root, "GRA1", "GRA");
    for id in ["SND1", "GRA1"] {
        set(root, id, &values(&FREIGABE)).expect("Freigabe");
        set(root, id, &values(&[("Status", "aktiv")])).expect("aktivieren");
    }
    assert_eq!(
        order(root, "## Aktiv"),
        ["GRA1", "SND1"],
        "{}",
        doc(root, "sprints/README.md")
    );
    assert!(order(root, "## Geplant").is_empty());
}

#[test]
fn neues_projekt_setzt_die_zeile_nach_rang_und_sprint_folge() {
    let tmp = roadmap_repo(&["GRA1"], &["SND1"]);
    let root = tmp.path();
    sprint(root, "GRA1", "GRA");
    sprint(root, "SND1", "SND");
    sprint(root, "X1", "–");
    set(root, "X1", &values(&[("Projekt", "GRA")])).expect("Projekt");
    assert_eq!(order(root, "## Geplant"), ["GRA1", "X1", "SND1"]);
}

#[test]
fn ohne_projekt_am_tabellenende_mit_hinweis() {
    let tmp = roadmap_repo(&["GRA1"], &[]);
    let root = tmp.path();
    sprint(root, "X1", "–");
    sprint(root, "GRA1", "GRA");
    set(root, "X1", &values(&FREIGABE)).expect("Freigabe");
    set(root, "GRA1", &values(&FREIGABE)).expect("Freigabe");
    set(root, "GRA1", &values(&[("Status", "aktiv")])).expect("aktivieren");
    let out = set(root, "X1", &values(&[("Status", "aktiv")])).expect("aktivieren");
    assert_eq!(order(root, "## Aktiv"), ["GRA1", "X1"]);
    assert!(out.contains("kein bekanntes Projekt"), "{out}");
}

/// B-403/N4: ein Sprint ohne Projekt behält seinen Platz, wenn sich nur die Reife ändert.
#[test]
fn ohne_projekt_bleibt_der_platz() {
    let tmp = roadmap_repo(&[], &[]);
    let root = tmp.path();
    sprint(root, "X1", "–");
    sprint(root, "X2", "–");
    let rel = "sprints/geplant/X1-s/README.md";
    let empty = doc(root, rel).replacen("- **Projekt:** –", "- **Projekt:**", 1);
    write(root, &format!("docs/{rel}"), &empty);
    set(root, "X1", &values(&[("Reife", "bereit")])).expect("Reife");
    assert_eq!(order(root, "## Geplant"), ["X1", "X2"]);
}

/// B-404: die Fahrplan-Zeile trägt das Projekt aus dem Kopffeld, auch nach Reife, Freigabe und Aktivieren.
#[test]
fn die_zeile_traegt_das_projekt() {
    let tmp = roadmap_repo(&["GRA1"], &[]);
    let root = tmp.path();
    sprint(root, "GRA1", "GRA");
    let steps: [&[(&str, &str)]; 4] = [
        &[("Reife", "Entwurf")],
        &[("Spec", "freigegeben"), ("Freigabe", "2026-10-11, Test")],
        &[("Reife", "bereit")],
        &[("Status", "aktiv"), ("Start-Commit", "abc1234")],
    ];
    for step in steps {
        set(root, "GRA1", &values(step)).expect("Set");
        let text = doc(root, "sprints/README.md");
        let row = text
            .lines()
            .find(|l| l.starts_with("| GRA1 |"))
            .map(cells)
            .expect("Zeile");
        assert_eq!(row[2], "GRA", "{step:?}\n{text}");
    }
}
