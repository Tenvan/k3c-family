//! Go `titel_test.go`: das Pseudo-Feld `Titel` bei Tickets und Sprints.

use super::edit::set;
use super::fixtures::{doc, repo, session, sprint, values};

#[test]
fn titel_setzen() {
    let tmp = repo();
    let root = tmp.path();
    set(root, "B-001", &values(&[("Titel", "Neuer Ticket-Titel")])).expect("Ticket");
    let index = doc(root, "backlog/README.md");
    assert!(
        index.contains("Neuer Ticket-Titel") && !index.contains("Altes Ticket"),
        "{index}"
    );

    sprint(root, "X1", "–");
    session(root, "X1.1", "SRV", true);
    let error = set(root, "X1.1", &values(&[("Titel", "x")])).expect_err("Titel bei Session");
    assert!(error.contains("Sprints und Tickets"), "{error}");

    set(root, "X1", &values(&[("Titel", "Neuer Sprint-Titel")])).expect("Sprint");
    let text = doc(root, "sprints/geplant/X1-s/README.md");
    assert!(
        text.starts_with("# X1 · SRV · Neuer Sprint-Titel\n"),
        "Sprint-Überschrift: {text}"
    );
    let roadmap = doc(root, "sprints/README.md");
    assert!(roadmap.contains("Neuer Sprint-Titel"), "{roadmap}");
}
