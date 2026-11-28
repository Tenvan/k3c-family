//! Go `review_fixes_test.go` (Review WZ2.8).

use super::edit::set;
use super::fixtures::{doc, full_ticket, project, repo, values};
use super::testsupport::write;

#[test]
fn fehlendes_feld_nennt_feld_und_datei() {
    let tmp = repo();
    let error = set(tmp.path(), "B-001", &values(&[("Prio", "hoch")])).expect_err("Feld fehlt");
    assert!(error.contains("Prio") && error.contains("B-001-alt.md"), "{error}");
}

#[test]
fn ticket_felder_ziehen_den_index_nach() {
    let tmp = repo();
    let root = tmp.path();
    project(root, "GRA", "1", &[]);
    full_ticket(root, "B-002", "neu", "Neues Ticket");
    let index = doc(root, "backlog/README.md").replace(
        "| Altes Ticket |\n",
        "| Altes Ticket |\n| [B-002](B-002-neu.md) | DEV | Idee | mittel | offen | – | Neues Ticket |\n",
    );
    write(root, "docs/backlog/README.md", &index);
    let changes = values(&[("Prio", "hoch"), ("Umgebung", "live"), ("Projekt", "GRA")]);
    set(root, "B-002", &changes).expect("Set");
    let text = doc(root, "backlog/B-002-neu.md");
    for wanted in ["- **Prio:** hoch", "- **Umgebung:** live", "- **Projekt:** GRA"] {
        assert!(text.contains(wanted), "{text}");
    }
    let index = doc(root, "backlog/README.md");
    assert!(index.contains("| DEV | Idee | hoch | offen |"), "{index}");
    assert_eq!(index.matches("| [B-002](").count(), 1, "{index}");
}
