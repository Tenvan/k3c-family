//! Die Fälle mit `create` und `delete` aus den Go-Tests, die RU69.9 hierher verschoben hat: `placeholder_rows`,
//! `roadmap`, `review_fixes`, `session_title`, `session_ids`, `titel`, `domains`, `relations` und `wz54`.

use std::path::Path;
use std::thread;

use super::create::{create, delete, get};
use super::create_fixtures::{
    check_consistent, make, new_doc, new_project, new_session, new_sprint, new_ticket, rel_repo,
};
use super::edit::set;
use super::fixtures::{ROADMAP_PROJEKT_MD, doc, repo, snapshot, values};
use super::index::{ARCHIVE_INDEX, INDEX_FILES, MAIN_INDEX, rebuild_index};
use super::list::{Filter, list};
use super::planning::{load, load_sprint, parse_sprint};
use super::relations::{FIELD_DUP, FIELD_REPLACE, reverse_lines};
use super::store::{docs_path, exists, resolve};
use super::tables::{cells, find_table, heading, split_lines};
use super::testsupport::write;

const SPRINT_README: &str = "sprints/geplant/GEB1-s/README.md";

/// Ersetzt die Session-Tabelle von GEB1 durch `rows`.
fn session_rows(root: &Path, rows: &[&str]) {
    let readme = doc(root, SPRINT_README);
    let (head, rest) = readme.split_once("## Sessions\n").expect("Abschnitt Sessions");
    let tail = rest.find("\n## ").map_or("", |at| &rest[at..]);
    let table = "## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n";
    let text = format!("{head}{table}{}\n{tail}", rows.join("\n"));
    write(root, &format!("docs/{SPRINT_README}"), &text);
}

fn first_session() -> super::create::NewDoc {
    new_doc("session", "GEB1.1", "erste", "Erste", &[("Domäne", "SRV")])
}

#[test]
fn platzhalter_zeile_wird_beim_anlegen_ersetzt() {
    let tmp = repo();
    let root = tmp.path();
    new_sprint(root, "GEB1", &[]);
    session_rows(
        root,
        &[
            "| GEB1.1 | – (Entwurf) | Umsetzung | autonom | offen |",
            "| GEB1.2 | – | Umsetzung | autonom | offen |",
        ],
    );
    create(root, &first_session()).expect("anlegen");
    let readme = doc(root, SPRINT_README);
    assert!(readme.contains("| GEB1.1 | `GEB1.1-erste.md` |"), "{readme}");
    assert!(!readme.contains("– (Entwurf)"), "{readme}");
    assert_eq!(readme.matches("| GEB1.1 |").count(), 1, "{readme}");
    assert!(readme.contains("| GEB1.2 | – |"), "{readme}");
    set(root, "GEB1.1", &values(&[("Agent", "Mensch")])).expect("Set trotz Platzhalter");
}

#[test]
fn platzhalter_anderer_nummer_bleibt() {
    let tmp = repo();
    let root = tmp.path();
    new_sprint(root, "GEB1", &[]);
    session_rows(root, &["| GEB1.2 | – (Entwurf) | Umsetzung | autonom | offen |"]);
    create(root, &first_session()).expect("anlegen");
    let readme = doc(root, SPRINT_README);
    assert!(readme.contains("| GEB1.2 | – (Entwurf) |"), "{readme}");
    assert!(readme.contains("| GEB1.1 | `GEB1.1-erste.md` |"), "{readme}");
}

#[test]
fn zeile_mit_fehlender_datei_bleibt_ein_fehler() {
    let tmp = repo();
    let root = tmp.path();
    new_sprint(root, "GEB1", &[]);
    session_rows(root, &["| GEB1.2 | `GEB1.2-weg.md` | Umsetzung | autonom | offen |"]);
    let before = snapshot(root);
    create(root, &first_session()).expect_err("fehlende Datei");
    assert!(snapshot(root) == before);
}

/// Sprint-IDs in der Reihenfolge der Tabelle unter `title`.
fn order(root: &Path, title: &str) -> Vec<String> {
    let lines = split_lines(&doc(root, "sprints/README.md"));
    let table = find_table(&lines, heading(title)).expect("Tabelle");
    lines[table.first..table.end]
        .iter()
        .map(|l| cells(l).swap_remove(0))
        .collect()
}

fn roadmap_repo() -> tempfile::TempDir {
    let tmp = repo();
    let root = tmp.path();
    write(root, "docs/sprints/README.md", ROADMAP_PROJEKT_MD);
    new_project(root, "GRA", "grafik");
    new_project(root, "SND", "sound");
    tmp
}

/// B-366/AC-01: neue Zeilen tragen das Projekt und stehen nach Rang und Sprint-Folge.
#[test]
fn neue_fahrplan_zeile_nach_projekt_rang() {
    let tmp = roadmap_repo();
    let root = tmp.path();
    new_sprint(root, "SND1", &[("Projekt", "SND")]);
    new_sprint(root, "GRA1", &[("Projekt", "GRA")]);
    new_sprint(root, "GRA2", &[("Projekt", "GRA")]);
    assert_eq!(order(root, "## Geplant"), ["GRA1", "GRA2", "SND1"]);
    let text = doc(root, "sprints/README.md");
    assert!(text.contains("| GRA2 | SRV | GRA | Sprint GRA2 |"), "{text}");
}

#[test]
fn neuer_sprint_ohne_projekt_am_ende() {
    let tmp = roadmap_repo();
    let root = tmp.path();
    new_sprint(root, "GRA1", &[("Projekt", "GRA")]);
    let out = make(root, "sprint", "X1", "x", "X", &[("Domäne", "SRV")]);
    assert_eq!(order(root, "## Geplant"), ["GRA1", "X1"]);
    assert!(out.contains("kein bekanntes Projekt"), "{out}");
}

/// B-404: die Fahrplan-Zeile trägt das Projekt, auch nach Anlegen von Sprint und Session.
#[test]
fn angelegte_zeile_traegt_das_projekt() {
    let tmp = roadmap_repo();
    let root = tmp.path();
    let project = |step: &str| {
        let text = doc(root, "sprints/README.md");
        let row = text
            .lines()
            .find(|l| l.starts_with("| GRA1 |"))
            .map(cells)
            .expect("Zeile");
        assert_eq!(row[2], "GRA", "{step}\n{text}");
    };
    let fields = [
        ("Domäne", "DEV"),
        ("Projekt", "GRA"),
        ("Reife", "bereit"),
        ("Tickets", "B-001"),
    ];
    make(root, "sprint", "GRA1", "s", "Sprint GRA1", &fields);
    project("Anlegen");
    let fields = [
        ("Domäne", "DEV"),
        ("Typ", "Umsetzung"),
        ("Agent", "autonom"),
        ("Umgebung", "offline"),
    ];
    make(root, "session", "GRA1.1", "u", "Umsetzung", &fields);
    project("Session anlegen");
}

/// Review WZ2.8 (B-363/AC-01) mit `create`.
#[test]
fn ticket_felder_bleiben_regelgerecht() {
    let tmp = repo();
    let root = tmp.path();
    new_project(root, "GRA", "grafik");
    new_ticket(root, "neu", "Neues Ticket", "SRV", "niedrig");
    let changes = values(&[("Prio", "hoch"), ("Umgebung", "live"), ("Projekt", "GRA")]);
    set(root, "B-002", &changes).expect("Set");
    assert!(doc(root, MAIN_INDEX).contains("| SRV | Idee | hoch | offen |"));
    check_consistent(root);
}

/// B-402 und B-372 über `create`: Titel aus der Datei, Session-IDs mit Buchstaben.
#[test]
fn angelegte_sessions_titel_und_buchstabe() {
    let tmp = repo();
    let root = tmp.path();
    new_sprint(root, "K3", &[]);
    let title = "Erste Session · mit Mittelpunkt";
    make(root, "session", "K3.1", "eins", title, &[("Domäne", "SRV")]);
    let out = make(root, "session", "K3.2a", "teil", "Teil a", &[("Domäne", "SRV")]);
    assert!(out.contains("K3.2a angelegt"), "{out}");
    let loaded = load_sprint(&docs_path(root, "sprints/geplant/K3-s"), "K3-s", "geplant").expect("Sprint");
    assert_eq!(loaded.sessions[0].titel, title);
    assert!(doc(root, "sprints/geplant/K3-s/README.md").contains("| K3.2a |"));
    assert!(get(root, "K3.2a").expect("get").contains("K3.2a · Teil a"));
    set(root, "K3.2a", &values(&[("Agent", "Mensch")])).expect("Set");
    let sprint = parse_sprint(&doc(root, "sprints/geplant/K3-s/README.md"), "K3-s", "geplant");
    assert_eq!(sprint.sessions.len(), 2, "{sprint:?}");
}

/// AC-04: Domänen in Reihenfolge des ersten Auftretens, auch nach dem Löschen.
#[test]
fn sprint_domaene_folgt_anlegen_und_loeschen() {
    let tmp = repo();
    let root = tmp.path();
    new_sprint(root, "X5", &[]);
    new_session(root, "X5.1", "SIM");
    new_session(root, "X5.2", "SRV");
    new_session(root, "X5.3", "SIM");
    let rel = "sprints/geplant/X5-s/README.md";
    let readme = doc(root, rel);
    assert!(readme.starts_with("# X5 · SIM, SRV · Sprint X5\n"), "{readme}");
    assert!(readme.contains("- **Domäne:** SIM, SRV\n"), "{readme}");
    assert!(doc(root, "sprints/README.md").contains("| X5 | SIM, SRV | Sprint X5 |"));
    set(root, "X5.1", &values(&[("Domäne", "CLI")])).expect("Set");
    delete(root, "X5.1").expect("löschen");
    let readme = doc(root, rel);
    assert!(readme.starts_with("# X5 · SRV, SIM · "), "{readme}");
    let data = load(root).expect("load");
    let sprint = &data.sprints[0];
    let domains: Vec<&str> = sprint.sessions.iter().map(|s| s.domain.as_str()).collect();
    assert_eq!((sprint.domain.as_str(), domains), ("SRV, SIM", vec!["SRV", "SIM"]));
    check_consistent(root);
    let before = snapshot(root);
    create(root, &new_doc("session", "X5.4", "a", "x", &[])).expect_err("ohne Domäne");
    assert!(snapshot(root) == before);
}

#[test]
fn duplikat_beim_anlegen_wird_abgelehnt() {
    let tmp = rel_repo();
    let root = tmp.path();
    let before = snapshot(root);
    let fields = [
        ("Domäne", "DEV"),
        ("Typ", "Idee"),
        ("Prio", "mittel"),
        (FIELD_DUP, "B-002"),
    ];
    let error = create(root, &new_doc("ticket", "", "d", "Neu", &fields)).expect_err("Duplikat");
    assert!(error.contains("kein Duplikat"), "{error}");
    assert!(snapshot(root) == before);
}

fn similar_ticket(root: &Path, slug: &str, title: &str) -> String {
    new_ticket(root, slug, title, "DEV", "mittel")
}

/// B-696/AC-06.
#[test]
fn aehnliche_titel() {
    let tmp = repo();
    let root = tmp.path();
    let rows = "| Altes Ticket |\n\
        | [B-020](B-020-a.md) | DEV | Idee | mittel | offen | – | Balancing-Seite zeigt eine Liste mit Varianten |\n";
    let archived = "| [B-021](archiv/B-021-b.md) | DEV | Idee | mittel | erledigt | – | \
        Die Varianten stehen auf der Balancing-Seite als Liste |\n\
        | [B-022](archiv/B-022-c.md) | SRV | Idee | mittel | erledigt | – | Server speichert Spielstände schneller |\n";
    let index = doc(root, MAIN_INDEX).replacen("| Altes Ticket |\n", rows, 1) + archived;
    write(root, "docs/backlog/README.md", &index);
    let out = similar_ticket(root, "a", "Die Balancing-Seite listet Varianten in einer Liste");
    for want in ["B-002 angelegt", "B-020 (offen)", "B-021 (archiv, erledigt)"] {
        assert!(out.contains(want), "ohne {want:?}:\n{out}");
    }
    assert!(!out.contains("B-022"), "{out}");
    let out = similar_ticket(root, "b", "Monarch reitet über die Brücke");
    assert!(!out.contains("ähnlich"), "{out}");
    std::fs::create_dir_all(docs_path(root, ARCHIVE_INDEX)).expect("Ordner statt Datei");
    let out = similar_ticket(root, "c", "Balancing-Seite zeigt Varianten");
    assert!(out.contains("B-004 angelegt"), "{out}");
    assert!(out.contains("Ähnlichkeitsprüfung übersprungen"), "{out}");
}

/// B-863/AC-01: mit B-999 im Backlog vergibt `create` B-1000 und B-1001; Index numerisch sortiert.
#[test]
fn nummern_ab_b_1000() {
    let tmp = rel_repo();
    let root = tmp.path();
    let head = "# B-999 · Neunhundert\n\n- **Domäne:** DEV\n- **Typ:** Idee\n- **Prio:** mittel\n\
                - **Status:** offen\n- **Sprint:** –\n";
    write(root, "docs/backlog/B-999-neun.md", head);
    rebuild_index(root, false).expect("Neuaufbau");
    for want in ["B-1000 angelegt", "B-1001 angelegt"] {
        let out = new_ticket(root, "neu", "Neu", "DEV", "hoch");
        assert!(out.contains(want), "{out}");
    }
    let changes = values(&[(FIELD_REPLACE, "B-999, B-1000"), ("Prio", "hoch")]);
    set(root, "B-1001", &changes).expect("Set");
    assert!(resolve(root, "B-1000").expect("B-1000").rel.contains("/B-1000-"));
    rebuild_index(root, false).expect("Neuaufbau");
    let main = doc(root, MAIN_INDEX);
    let at = |nr: &str| main.find(&format!("[{nr}]")).expect("Zeile");
    assert!(at("B-999") < at("B-1000") && at("B-1000") < at("B-1001"), "{main}");
    set(root, "B-1000", &values(&[("Status", "erledigt")])).expect("erledigt");
    let rows = INDEX_FILES
        .iter()
        .filter(|f| exists(root, f) && doc(root, f).contains("| [B-1000]("));
    assert_eq!(rows.count(), 1);
    assert!(doc(root, ARCHIVE_INDEX).contains("| [B-1000](B-1000-"));
    assert!(
        reverse_lines(root, "B-1000")
            .join("\n")
            .contains("Ersetzt durch:** B-1001")
    );
}

/// Sprints mit Projekt stehen vor einem ohne, egal welche Prio seine Tickets haben (AC-04, DV1 B-361).
#[test]
fn projekt_sprint_vor_sprint_ohne_projekt() {
    let tmp = repo();
    let root = tmp.path();
    new_ticket(root, "h", "Hoch", "SRV", "hoch");
    new_project(root, "GRA", "grafik");
    new_sprint(root, "X7", &[("Tickets", "B-002")]);
    new_sprint(root, "X8", &[("Projekt", "GRA"), ("Tickets", "B-001")]);
    new_sprint(root, "X9", &[("Projekt", "GRA")]);
    set(root, "GRA", &values(&[("Sprints", "X9, X8")])).expect("Reihenfolge");
    let filter = Filter {
        kind: "sprint".to_owned(),
        ..Filter::default()
    };
    let out = list(root, &filter).expect("list");
    let at = |id: &str| out.find(id).expect(id);
    assert!(at("X9 ") < at("X8 ") && at("X8 ") < at("X7 "), "{out}");
}

/// `create` hält die Schreibsperre: gleichzeitige Anlagen bekommen verschiedene Nummern und stehen alle im Index.
#[test]
fn gleichzeitiges_anlegen_vergibt_verschiedene_nummern() {
    let tmp = repo();
    let root = tmp.path();
    thread::scope(|scope| {
        for i in 0..6 {
            scope.spawn(move || new_ticket(root, &format!("p{i}"), &format!("Parallel {i}"), "DEV", "hoch"));
        }
    });
    let index = doc(root, MAIN_INDEX);
    for n in 2..=7 {
        assert_eq!(index.matches(&format!("| [B-{n:03}](")).count(), 1, "{index}");
    }
}

/// Eine neue Datei bekommt die Zeilenenden ihrer Vorlage.
#[test]
fn zeilenenden_wie_die_vorlage() {
    let tmp = repo();
    let root = tmp.path();
    let template = doc(root, "vorlagen/session.md").replace('\n', "\r\n");
    write(root, "docs/vorlagen/session.md", &template);
    new_sprint(root, "X1", &[]);
    new_session(root, "X1.1", "SRV");
    let text = std::fs::read_to_string(docs_path(root, "sprints/geplant/X1-s/X1.1-s.md")).expect("lesen");
    assert!(text.contains("\r\n"), "{text:?}");
    assert_eq!(text.matches('\n').count(), text.matches("\r\n").count());
}
