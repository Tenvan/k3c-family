//! Go `list_unplanned_test.go`.

use std::path::Path;

use super::list::{Filter, list};
use super::testsupport::{sprint, temp_repo, ticket};

/// B-001 (vom Grundgerüst, ohne Sprint-Feld), B-002 in X1, B-003 mit leerem Feld, B-004 mit `–`.
fn repo() -> tempfile::TempDir {
    let tmp = temp_repo();
    let root = tmp.path();
    for (nr, line) in [
        ("B-002", "- **Sprint:** X1\n"),
        ("B-003", "- **Sprint:**\n"),
        ("B-004", "- **Sprint:** –\n"),
    ] {
        ticket(root, &format!("backlog/{nr}-t.md"), nr, "Titel", line);
    }
    sprint(root, "X1", "–", "B-002");
    tmp
}

fn run(root: &Path, kind: &str, wanted: &str) -> String {
    let filter = Filter {
        kind: kind.to_owned(),
        sprint: wanted.to_owned(),
        ..Filter::default()
    };
    list(root, &filter).expect("list")
}

/// „–“ und „ungeplant“ liefern wie der Schnellfilter Ungeplant alle Tickets ohne Sprint und keine Sprints.
#[test]
fn ungeplant() {
    let tmp = repo();
    for wanted in ["–", "-", "ungeplant", "Ungeplant"] {
        let out = run(tmp.path(), "", wanted);
        assert!(out.contains("B-001 ") && out.contains("B-003 "), "{wanted}:\n{out}");
        assert!(!out.contains("B-002 ") && !out.contains("X1 "), "{wanted}:\n{out}");
    }
    let out = run(tmp.path(), "ticket", "x1");
    assert!(out.contains("B-002 ") && !out.contains("B-001 "), "{out}");
}

/// `kind` ticket beginnt mit der Kennzahl-Zeile; „ungeplant“ zählt unabhängig von Status und Filter.
#[test]
fn kennzahlen() {
    let tmp = repo();
    let out = run(tmp.path(), "ticket", "X1");
    assert!(
        out.starts_with("Tickets: 1 von 4 · 4 offen · 0 eingeplant · 3 ungeplant\n"),
        "{out}"
    );
    let first = run(tmp.path(), "", "");
    assert!(first.starts_with("X1 "), "Sprint-Zeile zuerst erwartet:\n{first}");
    assert_eq!(run(tmp.path(), "sprint", "Z9"), "keine Treffer");
    assert_eq!(run(tmp.path(), "", "Z9"), "keine Treffer");
    assert!(run(tmp.path(), "ticket", "Z9").starts_with("Tickets: 0 von 4"));
}
