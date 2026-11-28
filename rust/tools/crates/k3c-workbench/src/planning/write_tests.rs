//! Go `write_test.go`: Lebenslauf von Ticket, Sprint und Session, Ablehnungen, Domänen.

use std::collections::BTreeMap;

use super::create::{create, delete, get};
use super::create_fixtures::{check_consistent, make, new_doc, new_ticket};
use super::edit::{section, sections, set};
use super::fixtures::{doc, repo, snapshot, values};
use super::index::{ARCHIVE_INDEX, MAIN_INDEX};
use super::list::{Filter, list};
use super::store::{docs_path, exists};

#[test]
fn ticket_anlegen_aendern_archivieren() {
    let tmp = repo();
    let root = tmp.path();
    let out = new_ticket(root, "neu", "Neues Ticket", "SRV", "hoch");
    assert!(out.contains("B-002 angelegt: docs/backlog/B-002-neu.md"), "{out}");
    section(root, "B-002", "Ausgangslage", "Heute fehlt es.\n\n### Detail\n\nmehr").expect("section");
    let text = doc(root, "backlog/B-002-neu.md");
    assert!(
        text.contains("## Ausgangslage\n\nHeute fehlt es.\n\n### Detail\n\nmehr\n\n## Ziel"),
        "{text}"
    );
    let bodies = values(&[("Ziel", "Es geht."), ("Ausgangslage", "Neu.")]);
    let out = sections(root, "B-002", &bodies).expect("sections");
    assert!(out.starts_with("B-002 · Ausgangslage, Ziel ersetzt"), "{out}");
    let text = doc(root, "backlog/B-002-neu.md");
    assert!(
        text.contains("## Ausgangslage\n\nNeu.\n\n## Ziel\n\nEs geht.\n\n## "),
        "{text}"
    );
    set(root, "B-002", &values(&[("Status", "erledigt")])).expect("Status");
    assert!(
        docs_path(root, "backlog/archiv/B-002-neu.md").exists(),
        "nicht archiviert"
    );
    // Altes Format mit Archiv-Tabelle bleibt bis zum Neuaufbau (B-696).
    let index = doc(root, MAIN_INDEX);
    let row = "| [B-002](archiv/B-002-neu.md) | SRV | Idee | hoch | erledigt | – | Neues Ticket |";
    assert!(index.trim().ends_with(row), "{index}");
    assert!(!exists(root, ARCHIVE_INDEX));
    check_consistent(root);
}

#[test]
fn sprint_und_session_lebenslauf() {
    let tmp = repo();
    let root = tmp.path();
    make(root, "sprint", "X1", "test", "Test-Sprint", &[("Domäne", "SRV")]);
    make(root, "session", "X1.1", "eins", "Erste", &[("Domäne", "SRV")]);
    set(root, "X1.1", &values(&[("Status", "fertig")])).expect("fertig");
    let readme = doc(root, "sprints/geplant/X1-test/README.md");
    assert!(
        readme.contains("| X1.1 | `X1.1-eins.md` | Umsetzung | autonom | fertig |"),
        "{readme}"
    );
    set(root, "X1", &values(&[("Status", "aktiv")])).expect_err("aktiv ohne Reife bereit");
    let release = [
        ("Reife", "bereit"),
        ("Spec", "freigegeben"),
        ("Freigabe", "2026-10-04 Test"),
        ("Status", "aktiv"),
    ];
    set(root, "X1", &values(&release)).expect("aktivieren");
    let roadmap = doc(root, "sprints/README.md");
    assert!(
        roadmap.contains("| X1 | SRV | Test-Sprint | – | `aktiv/X1-test/` |\n\n## Offen am Gerät"),
        "{roadmap}"
    );
    delete(root, "X1.1").expect_err("Session eines aktiven Sprints gelöscht");
    set(root, "X1", &values(&[("Status", "erledigt")])).expect("erledigt");
    let roadmap = doc(root, "sprints/README.md");
    assert!(
        roadmap.contains("| X1 | Test-Sprint | `erledigt/X1-test/` |"),
        "{roadmap}"
    );
    assert_eq!(roadmap.matches("| X1 |").count(), 1, "{roadmap}");
    // „Offen am Gerät“ entsteht aus den Session-Dateien (B-842): die Freitext-Zeile des erledigten X1 ist weg.
    assert!(!roadmap.contains("| X1.3 | Xbox |"), "{roadmap}");
    assert!(
        roadmap.contains("mit Freigaben und Entscheidungen: `plan_queue`"),
        "{roadmap}"
    );
    check_consistent(root);
}

#[test]
fn entwurf_loeschen() {
    let tmp = repo();
    let root = tmp.path();
    make(root, "sprint", "X2", "weg", "Weg", &[("Domäne", "SIM")]);
    make(root, "session", "X2.1", "a", "A", &[("Domäne", "SRV")]);
    let row = "| X2 | SRV | Weg | – | Entwurf | `geplant/X2-weg/` |\n\n## Erledigt";
    assert!(
        doc(root, "sprints/README.md").contains(row),
        "{}",
        doc(root, "sprints/README.md")
    );
    delete(root, "X2.1").expect("Session löschen");
    assert!(!doc(root, "sprints/geplant/X2-weg/README.md").contains("X2.1"));
    assert!(!docs_path(root, "sprints/geplant/X2-weg/X2.1-a.md").exists());
    delete(root, "X2").expect("Sprint löschen");
    assert!(!doc(root, "sprints/README.md").contains("X2"));
    assert!(!docs_path(root, "sprints/geplant/X2-weg").exists());
    delete(root, "B-001").expect_err("Ticket gelöscht");
    check_consistent(root);
}

/// Aufrufe, die nichts ändern dürfen: Pfade, IDs und Werte von außen (AC-03).
fn bad_calls(root: &std::path::Path) -> Vec<(&'static str, Result<String, String>)> {
    let absolute = docs_path(root, "backlog/B-001-alt.md").display().to_string();
    let one = |name: &str, value: &str| values(&[(name, value)]);
    let ticket = [("Domäne", "XYZ"), ("Typ", "Idee"), ("Prio", "hoch")];
    vec![
        ("Set ../", set(root, "../B-001", &one("Status", "offen"))),
        ("Get absolut", get(root, &absolute)),
        ("Section ../", section(root, "../../x", "Ziel", "x")),
        ("Slug ../", create(root, &new_doc("ticket", "", "../böse", "x", &[]))),
        ("Art ../", create(root, &new_doc("../../ticket", "", "a", "x", &[]))),
        (
            "Sprint-ID ../",
            create(root, &new_doc("sprint", "../X9", "a", "x", &[])),
        ),
        ("unbekannte ID", set(root, "B-999", &one("Status", "offen"))),
        ("unbekanntes Feld", set(root, "X3", &one("Pfad", "x"))),
        ("falscher Wert", set(root, "X3", &one("Status", "fertig"))),
        ("Domäne XYZ", create(root, &new_doc("ticket", "", "a", "x", &ticket))),
        ("Wert mit |", set(root, "X3", &one("Tickets", "B-001 | x"))),
        ("Überschrift im Text", section(root, "X3", "Ziel", "## Neu")),
        ("unbekannter Abschnitt", section(root, "X3", "Pfad", "x")),
        (
            "Sections mit unbekanntem",
            sections(root, "X3", &values(&[("Ziel", "neu"), ("Pfad", "x")])),
        ),
        ("Sections leer", sections(root, "X3", &BTreeMap::new())),
        ("freigegeben ohne Text", set(root, "X3", &one("Spec", "freigegeben"))),
        (
            "Ticket ohne Pflicht",
            create(root, &new_doc("ticket", "", "a", "x", &[])),
        ),
        (
            "Sprint doppelt",
            create(root, &new_doc("sprint", "X3", "b", "x", &[("Domäne", "SRV")])),
        ),
    ]
}

#[test]
fn ungueltige_eingabe_aendert_nichts() {
    let tmp = repo();
    let root = tmp.path();
    make(root, "sprint", "X3", "s", "S", &[("Domäne", "SRV")]);
    let before = snapshot(root);
    for (name, result) in bad_calls(root) {
        assert!(result.is_err(), "{name}: kein Fehler: {result:?}");
        assert!(snapshot(root) == before, "{name}: Dateien geändert");
    }
}

/// DV1 AC-03: DEV anlegen und umstellen, eine unbekannte Domäne nennt die Liste.
#[test]
fn domaene_dev() {
    let tmp = repo();
    let root = tmp.path();
    new_ticket(root, "werkzeug", "Werkzeug", "DEV", "hoch");
    new_ticket(root, "server", "Server", "SRV", "hoch");
    set(root, "B-003", &values(&[("Domäne", "DEV")])).expect("Domäne");
    let index = doc(root, MAIN_INDEX);
    assert!(index.contains("| [B-003](B-003-server.md) | DEV |"), "{index}");
    let filter = Filter {
        kind: "ticket".to_owned(),
        domain: "DEV".to_owned(),
        ..Filter::default()
    };
    let out = list(root, &filter).expect("list");
    assert!(out.contains("B-002 DEV") && out.contains("B-003 DEV"), "{out}");
    let fields = [("Domäne", "TOOL"), ("Typ", "Idee"), ("Prio", "hoch")];
    let error = create(root, &new_doc("ticket", "", "tool", "x", &fields)).expect_err("TOOL");
    assert!(error.contains("REG, SIM, SRV, CLI, PLAT, INF, DEV"), "{error}");
    check_consistent(root);
}

/// DV1 AC-04: Sprints bekommen weder Prio noch Einschiebbar.
#[test]
fn sprint_ohne_prio_und_einschiebbar() {
    let tmp = repo();
    let root = tmp.path();
    new_ticket(root, "neu", "Neu", "SRV", "hoch");
    let fields = [("Domäne", "SRV"), ("Tickets", "B-001, B-002 teils")];
    make(root, "sprint", "X3", "p", "P", &fields);
    set(root, "X3", &values(&[("Tickets", "B-002")])).expect("Tickets");
    let readme = doc(root, "sprints/geplant/X3-p/README.md");
    assert!(!readme.contains("Prio") && !readme.contains("Einschiebbar"), "{readme}");
    let before = snapshot(root);
    for field in ["Prio", "Einschiebbar"] {
        set(root, "X3", &values(&[(field, "ja")])).expect_err(field);
        assert!(snapshot(root) == before, "{field}: Dateien geändert");
    }
    let filter = Filter {
        kind: "sprint".to_owned(),
        ..Filter::default()
    };
    let out = list(root, &filter).expect("list");
    assert!(!out.contains("Prio"), "{out}");
}
