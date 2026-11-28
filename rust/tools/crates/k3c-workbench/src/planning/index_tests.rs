//! Go `index_test.go` (B-696/AC-07, AC-09), dazu `index_tickets`, `row_order` und `join_rows`. Der Go-Test auf einem
//! echten Checkout (`K3C_REBUILD_INDEX`) ist nicht portiert: kein Test schreibt in die echte Planung.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use super::create_fixtures::{index_links, rel_repo};
use super::edit::set;
use super::fixtures::{doc, snapshot, values};
use super::index::{
    ARCHIVE_INDEX, INDEX_FILES, MAIN_ARCHIVE, MAIN_INDEX, index_tickets, join_rows, rebuild_index, row_order,
};
use super::planning::parse_ticket;
use super::relations::relations;
use super::store::{docs_path, exists};
use super::testsupport::write;

fn status(root: &Path, id: &str, value: &str) {
    set(root, id, &values(&[("Status", value)])).expect("Status");
}

fn bytes(root: &Path, rel: &str) -> String {
    fs::read_to_string(docs_path(root, rel)).expect(rel)
}

/// Jedes Ticket steht genau einmal in einem der beiden Index-Dateien, im richtigen.
fn check_indexes(root: &Path) {
    let mut seen = BTreeSet::new();
    for rel in INDEX_FILES.into_iter().filter(|rel| exists(root, rel)) {
        for (nr, link, state) in index_links(&doc(root, rel)) {
            let folder = if rel == ARCHIVE_INDEX {
                "backlog/archiv"
            } else {
                "backlog"
            };
            let file = format!("{folder}/{link}");
            assert!(seen.insert(format!("{nr} {file}")), "{nr} {file} doppelt");
            assert_eq!(parse_ticket(&doc(root, &file)).status, state, "{nr}");
        }
    }
    for x in index_tickets(root).expect("Tickets") {
        let folder = if x.archived { "backlog/archiv" } else { "backlog" };
        let key = format!("{} {folder}/{}", x.ticket.nr, x.file);
        assert!(seen.contains(&key), "{key} fehlt im Index");
    }
}

/// Datei und Zeile wandern zwischen beiden Index-Dateien, der Ticket-Text bleibt bis auf den Status byte-gleich.
#[test]
fn statuswechsel_zwischen_den_index_dateien() {
    let tmp = rel_repo();
    let root = tmp.path();
    rebuild_index(root, false).expect("Neuaufbau");
    let before = bytes(root, "backlog/B-002-t.md");
    status(root, "B-002", "erledigt");
    let moved = bytes(root, "backlog/archiv/B-002-t.md");
    assert_eq!(
        before.replacen("- **Status:** offen", "- **Status:** erledigt", 1),
        moved
    );
    let (main, archive) = (doc(root, MAIN_INDEX), doc(root, ARCHIVE_INDEX));
    assert!(!main.contains("[B-002]"), "{main}");
    assert!(
        archive.contains("| [B-002](B-002-t.md) | DEV | Idee | mittel | erledigt |"),
        "{archive}"
    );
    check_indexes(root);
    status(root, "B-002", "offen");
    assert_eq!(bytes(root, "backlog/B-002-t.md"), before);
    let (main, archive) = (doc(root, MAIN_INDEX), doc(root, ARCHIVE_INDEX));
    assert!(!archive.contains("[B-002]"), "{archive}");
    assert!(
        main.contains("| [B-002](B-002-t.md) | DEV | Idee | mittel | offen |"),
        "{main}"
    );
    check_indexes(root);
}

/// Mit Archiv-Tabelle im Haupt-Index bleibt das alte Format; ohne sie legt der erste Wechsel die Archiv-Datei an.
#[test]
fn archiv_index_altes_format_und_neu_anlegen() {
    let tmp = rel_repo();
    let root = tmp.path();
    status(root, "B-003", "verworfen");
    assert!(!exists(root, ARCHIVE_INDEX));
    let main = doc(root, MAIN_INDEX);
    assert!(main.contains("| [B-003](archiv/B-003-t.md) |"), "{main}");
    let head = &main[..main.find("## Archiv").expect("Archiv-Tabelle")];
    write(root, "docs/backlog/README.md", &format!("{head}{MAIN_ARCHIVE}"));
    status(root, "B-004", "verworfen");
    let archive = doc(root, ARCHIVE_INDEX);
    assert!(
        archive.starts_with("# Backlog · Archiv") && archive.contains("| [B-004](B-004-t.md) |"),
        "{archive}"
    );
    assert!(!doc(root, MAIN_INDEX).contains("[B-004]"));
}

/// Zwei Tickets mit derselben Nummer behalten beide ihre Zeile; Relationen sehen das offene.
#[test]
fn doppelte_nummer_bleibt_getrennt() {
    let tmp = rel_repo();
    let root = tmp.path();
    let twin = "# B-002 · Zwilling im Archiv\n\n- **Domäne:** DEV\n- **Typ:** Idee\n- **Prio:** mittel\n\
                - **Status:** erledigt\n- **Sprint:** –\n";
    write(root, "docs/backlog/archiv/B-002-zwilling.md", twin);
    rebuild_index(root, false).expect("Neuaufbau");
    assert_eq!(relations(root)["B-002"].status, "offen");
    status(root, "B-002", "erledigt");
    let archive = doc(root, ARCHIVE_INDEX);
    assert!(
        archive.contains("| [B-002](B-002-zwilling.md) |") && archive.contains("| [B-002](B-002-t.md) |"),
        "{archive}"
    );
    status(root, "B-002", "offen");
    let archive = doc(root, ARCHIVE_INDEX);
    assert!(
        archive.contains("| [B-002](B-002-zwilling.md) |") && !archive.contains("B-002-t.md"),
        "{archive}"
    );
}

/// B-696/AC-07: `dry` schreibt nichts, ein zweiter Lauf ändert nichts, ein Ticket im falschen Ordner bricht ab.
#[test]
fn neuaufbau() {
    let tmp = rel_repo();
    let root = tmp.path();
    status(root, "B-003", "erledigt");
    let before = snapshot(root);
    let out = rebuild_index(root, true).expect("dry");
    assert!(out.contains("3 offen/eingeplant") && snapshot(root) == before, "{out}");
    rebuild_index(root, false).expect("Neuaufbau");
    let (main, archive) = (doc(root, MAIN_INDEX), doc(root, ARCHIVE_INDEX));
    assert!(!main.contains("erledigt |"), "{main}");
    assert!(main.contains("[`archiv/README.md`](archiv/README.md)"), "{main}");
    assert!(archive.contains("| [B-003](B-003-t.md) |"), "{archive}");
    check_indexes(root);
    let again = snapshot(root);
    rebuild_index(root, false).expect("zweiter Lauf");
    assert!(snapshot(root) == again, "zweiter Lauf ändert etwas");
    write(
        root,
        "docs/backlog/archiv/B-030-x.md",
        "# B-030 · X\n\n- **Status:** offen\n",
    );
    let before = snapshot(root);
    let error = rebuild_index(root, false).expect_err("falscher Ordner");
    assert!(error.contains("B-030-x.md: Status offen passt nicht"), "{error}");
    assert!(snapshot(root) == before);
}

/// CRLF des Haupt-Index gilt auch für den neu angelegten Archiv-Index.
#[test]
fn neuaufbau_behaelt_crlf() {
    let tmp = rel_repo();
    let root = tmp.path();
    let main = bytes(root, MAIN_INDEX).replace('\n', "\r\n");
    write(root, "docs/backlog/README.md", &main);
    rebuild_index(root, false).expect("Neuaufbau");
    for rel in INDEX_FILES {
        let text = bytes(root, rel);
        assert_eq!(text.matches('\n').count(), text.matches("\r\n").count(), "{rel}");
    }
}

#[test]
fn tickets_fuer_den_neuaufbau() {
    let tmp = rel_repo();
    let root = tmp.path();
    write(
        root,
        "docs/backlog/archiv/B-009-weg.md",
        "# B-009 · Weg\n\n- **Status:** verworfen\n",
    );
    write(root, "docs/backlog/notiz.md", "kein Ticket\n");
    let got: Vec<(String, bool)> = index_tickets(root)
        .expect("Tickets")
        .into_iter()
        .map(|x| (x.file, x.archived))
        .collect();
    let open = ["B-001-alt.md", "B-002-t.md", "B-003-t.md", "B-004-t.md"];
    let mut want: Vec<(String, bool)> = open.map(|f| (f.to_owned(), false)).to_vec();
    want.push(("B-009-weg.md".to_owned(), true));
    assert_eq!(got, want);
    fs::remove_dir_all(docs_path(root, "backlog")).expect("entfernen");
    index_tickets(root).expect_err("ohne Backlog");
}

#[test]
fn reihenfolge_der_bisherigen_zeilen() {
    let text = "| Nr. | x |\n\
                | [B-020](a.md) | DEV | Idee | mittel | offen | – | Zwei |\n\
                | [B-003](b.md) | DEV | Idee | mittel | offen | – | Drei |\n\
                | [B-020](c.md) | DEV | Idee | mittel | offen | – | Nochmal |\n";
    let order = row_order(text);
    assert_eq!((order.len(), order["B-020"], order["B-003"]), (2, 0, 1));
}

#[test]
fn zeilen_mit_zeilenende() {
    assert_eq!(join_rows(&[]), "");
    assert_eq!(join_rows(&["a".to_owned(), "b".to_owned()]), "a\nb\n");
}

/// Bekannte Zeilen behalten ihre Reihenfolge, neue kommen nach Nummer ans Ende.
#[test]
fn neuaufbau_behaelt_die_reihenfolge() {
    let tmp = rel_repo();
    let root = tmp.path();
    let index = doc(root, MAIN_INDEX);
    let row = |nr: &str| {
        index
            .lines()
            .find(|l| l.starts_with(&format!("| [{nr}]")))
            .expect("Zeile")
    };
    let swapped = format!("# Backlog\n\n## Offen\n\n{}\n{}\n", row("B-004"), row("B-002"));
    write(root, "docs/backlog/README.md", &swapped);
    rebuild_index(root, false).expect("Neuaufbau");
    let order: Vec<String> = index_links(&doc(root, MAIN_INDEX)).into_iter().map(|r| r.0).collect();
    assert_eq!(order, ["B-004", "B-002", "B-001", "B-003"]);
}
