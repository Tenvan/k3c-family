//! Go `relations_test.go`, soweit ohne `Set` und `Create`: Prüfung, Status verworfen, Gegenrichtung.

use std::collections::BTreeMap;

use super::create::get;
use super::edit::head_fields;
use super::list::{Filter, list};
use super::relations::{FIELD_DUP, FIELD_REPLACE, check_relations, ticket_relations};
use super::store::{ChangeSet, resolve};
use super::testsupport::{read, temp_repo, ticket};

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
