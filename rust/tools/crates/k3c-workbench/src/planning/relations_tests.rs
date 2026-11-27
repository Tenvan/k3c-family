//! Go `relations_test.go` ohne die Fälle mit `Create` (RU69.10): Prüfung, Status verworfen, Gegenrichtung und die
//! Wege über `set`.

use std::collections::BTreeMap;

use super::create::get;
use super::edit::{head_fields, set};
use super::fixtures::{self, doc, snapshot};
use super::list::{Filter, list};
use super::relations::{FIELD_DUP, FIELD_REPLACE, check_relations, ticket_relations};
use super::store::{ChangeSet, docs_path, resolve};
use super::testsupport::{read, temp_repo, ticket};

/// B-696/AC-02 über `set`: Ablehnungen ändern keine Datei.
#[test]
fn ungueltige_relationen_aendern_nichts() {
    let tmp = fixtures::rel_repo();
    let root = tmp.path();
    for (id, name, value) in [
        ("B-003", FIELD_REPLACE, "B-002"),
        ("B-004", FIELD_REPLACE, "B-003"),
        ("B-001", "Status", "erledigt"),
    ] {
        set(root, id, &fixtures::values(&[(name, value)])).expect("Aufbau");
    }
    let before = snapshot(root);
    for (id, name, value, want) in [
        ("B-002", FIELD_REPLACE, "B-099", "B-099 gibt es nicht"),
        ("B-002", FIELD_REPLACE, "B-002", "auf sich selbst"),
        ("B-002", FIELD_REPLACE, "B-003", "Zyklus B-002 → B-003 → B-002"),
        ("B-002", FIELD_REPLACE, "B-004", "Zyklus B-002 → B-004 → B-003 → B-002"),
        ("B-002", FIELD_DUP, "B-004", "Zyklus"),
        ("B-002", FIELD_DUP, "B-003, B-004", "genau eine ID"),
        ("B-001", FIELD_DUP, "B-002", "erledigt"),
        ("B-002", FIELD_REPLACE, "B-3", "keine Ticket-ID"),
        ("B-002", FIELD_REPLACE, "B-001, B-001", "doppelt"),
        ("B-002", FIELD_REPLACE, "B-001,B-003", "trennen"),
    ] {
        let error = set(root, id, &fixtures::values(&[(name, value)])).expect_err(value);
        assert!(error.contains(want), "{value}: {error:?}, erwartet {want:?}");
        assert!(snapshot(root) == before, "{value}: Dateien geändert");
    }
}

/// B-696/AC-03 über `set`: archiviert samt Index.
#[test]
fn duplikat_wird_archiviert() {
    let tmp = fixtures::rel_repo();
    let root = tmp.path();
    set(root, "B-003", &fixtures::values(&[(FIELD_REPLACE, "B-002")])).expect("Ersetzt");
    let out = set(root, "B-004", &fixtures::values(&[(FIELD_DUP, "B-002")])).expect("Duplikat");
    assert!(
        out.contains("Status verworfen") && out.contains("Wurzel der Kette ist B-003"),
        "{out}"
    );
    let found = resolve(root, "B-004").expect("B-004");
    assert_eq!(found.state, "archiv");
    let head = head_fields(&doc(root, &found.rel));
    assert_eq!(
        (head["Status"].as_str(), head[FIELD_DUP].as_str()),
        ("verworfen", "B-002")
    );
    let archive = doc(root, "backlog/archiv/README.md");
    assert!(
        archive.contains("| [B-004](B-004-t.md) | DEV | Idee | mittel | verworfen |"),
        "{archive}"
    );
    assert!(!doc(root, "backlog/README.md").contains("[B-004]"));
}

/// B-696/AC-05: die Felder kommen hinter Freigabe, CRLF bleibt.
#[test]
fn altticket_bekommt_die_felder() {
    let tmp = fixtures::rel_repo();
    let root = tmp.path();
    let old = "# B-010 · Altes Ticket mit CRLF\r\n\r\n- **Domäne:** DEV\r\n- **Typ:** Idee\r\n- **Prio:** mittel\r\n\
               - **Umgebung:** offline\r\n- **Status:** offen\r\n- **Sprint:** –\r\n- **Projekt:** –\r\n\
               - **Erstellt:** 2026-01-01\r\n- **Spec:** Entwurf\r\n- **Revision:** 1\r\n- **Freigabe:** –\r\n\r\n\
               ## Ausgangslage\r\n\r\nText\r\n";
    super::testsupport::write(root, "docs/backlog/B-010-alt.md", old);
    set(root, "B-010", &fixtures::values(&[(FIELD_REPLACE, "B-002, B-003")])).expect("Ersetzt");
    let got = std::fs::read_to_string(docs_path(root, "backlog/B-010-alt.md")).expect("lesen");
    let want = "- **Freigabe:** –\r\n- **Ersetzt:** B-002, B-003\r\n\r\n## Ausgangslage";
    assert!(
        got.contains(want) && got.matches('\n').count() == got.matches("\r\n").count(),
        "{got:?}"
    );
    set(root, "B-010", &fixtures::values(&[(FIELD_DUP, "B-004")])).expect("Duplikat");
    let got = std::fs::read_to_string(docs_path(root, "backlog/archiv/B-010-alt.md")).expect("lesen");
    let want = "- **Freigabe:** –\r\n- **Duplikat von:** B-004\r\n- **Ersetzt:** B-002, B-003\r\n";
    assert!(got.contains(want), "{got:?}");
}

/// B-002 bis B-004 mit den Relationen `extra` je Ticket; B-001 ist erledigt.
fn rel_repo(extra: [&str; 3]) -> tempfile::TempDir {
    let tmp = temp_repo();
    let root = tmp.path();
    let old = "# B-001 · Altes Ticket\n\n- **Domäne:** SRV\n- **Typ:** Idee\n- **Status:** erledigt\n- **Sprint:** –\n";
    super::testsupport::write(root, "docs/backlog/B-001-alt.md", old);
    let titles = ["Zweites Ticket", "Drittes Ticket", "Viertes Ticket"];
    for (i, (title, lines)) in titles.iter().zip(extra).enumerate() {
        let nr = format!("B-00{}", i + 2);
        ticket(root, &format!("backlog/{nr}-t.md"), &nr, title, lines);
    }
    tmp
}

fn values(name: &str, value: &str) -> BTreeMap<String, String> {
    BTreeMap::from([(name.to_owned(), value.to_owned())])
}

/// B-696/AC-02 (ohne den Fall „Duplikat beim Anlegen“, der gehört zu `Create`).
#[test]
fn ungueltige_relationen_werden_abgelehnt() {
    let tmp = rel_repo(["", "- **Ersetzt:** B-002\n", "- **Ersetzt:** B-003\n"]);
    let root = tmp.path();
    let cases = [
        ("B-002", FIELD_REPLACE, "B-099", "B-099 gibt es nicht"),
        ("B-002", FIELD_REPLACE, "B-002", "auf sich selbst"),
        ("B-002", FIELD_REPLACE, "B-003", "Zyklus B-002 → B-003 → B-002"),
        ("B-002", FIELD_REPLACE, "B-004", "Zyklus B-002 → B-004 → B-003 → B-002"),
        ("B-002", FIELD_DUP, "B-004", "Zyklus"),
        ("B-002", FIELD_DUP, "B-003, B-004", "genau eine ID"),
        ("B-001", FIELD_DUP, "B-002", "erledigt"),
        ("B-002", FIELD_REPLACE, "B-3", "keine Ticket-ID"),
        ("B-002", FIELD_REPLACE, "B-001, B-001", "doppelt"),
        ("B-002", FIELD_REPLACE, "B-001,B-003", "trennen"),
        ("B-002", FIELD_REPLACE, "", "`–` oder Ticket-IDs"),
        ("B-002", FIELD_REPLACE, "B-009", "ähnlich: B-001, B-002, B-003, B-004"),
    ];
    for (id, name, value, want) in cases {
        let found = resolve(root, id).expect("Ticket");
        let head = head_fields(&read(root, &format!("docs/{}", found.rel)));
        let result = check_relations(root, id, &head, &values(name, value));
        let error = result.expect_err(value);
        assert!(error.contains(want), "{value}: {error:?}, erwartet {want:?}");
    }
    assert_eq!(
        check_relations(root, "B-002", &Default::default(), &values("Prio", "x")),
        Ok(Vec::new())
    );
}

/// B-696/AC-03: Duplikat setzt den Status verworfen und nennt die Wurzel der Kette.
#[test]
fn duplikat_verwirft_und_nennt_die_wurzel() {
    let tmp = rel_repo(["", "- **Ersetzt:** B-002\n", ""]);
    let root = tmp.path();
    let found = resolve(root, "B-004").expect("B-004");
    let mut changes = ChangeSet::new(root);
    let text = changes.read(&found.rel).expect("lesen");
    let out = ticket_relations(&mut changes, &found, &text, &values(FIELD_DUP, "B-002")).expect("Relationen");
    assert_eq!(out.get("Status").map(String::as_str), Some("verworfen"));
    assert_eq!(
        changes.notes,
        [
            "Hinweis: B-002 ist selbst verworfen oder ersetzt; Wurzel der Kette ist B-003",
            "B-004 ist Duplikat von B-002: Status verworfen",
        ]
    );
}

/// B-696/AC-04: die Datei des Gegenstücks bleibt byte-gleich.
#[test]
fn gegenrichtung_wird_berechnet() {
    let tmp = rel_repo(["", "- **Ersetzt:** B-002\n", "- **Duplikat von:** B-002\n"]);
    let root = tmp.path();
    let before = read(root, "docs/backlog/B-002-t.md");
    let out = get(root, "B-002").expect("get");
    assert_eq!(
        read(root, "docs/backlog/B-002-t.md"),
        before,
        "get hat die Datei geändert"
    );
    let wanted = "> **Ersetzt durch:** B-003 (berechnet)\n> **Duplikate:** B-004 (berechnet)\n\n## Ausgangslage";
    assert!(out.contains(wanted), "{out}");
    assert!(!get(root, "B-003").expect("get").contains("(berechnet)"));
    let filter = Filter {
        kind: "ticket".to_owned(),
        archive: true,
        ..Filter::default()
    };
    let lines = list(root, &filter).expect("list");
    assert!(lines.contains("Drittes Ticket · ersetzt B-002"), "{lines}");
    assert!(lines.contains("Viertes Ticket → B-002"), "{lines}");
}
