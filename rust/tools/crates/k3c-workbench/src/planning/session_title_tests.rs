//! Go `session_title_test.go` (B-402): Sessions mit Datei tragen den Titel aus der Überschrift der Datei.

use super::fixtures::{repo, session, sprint};
use super::planning::load_sprint;
use super::store::docs_path;
use super::testsupport::{read, write};

#[test]
fn session_titel_aus_der_datei() {
    let tmp = repo();
    let root = tmp.path();
    sprint(root, "K4", "–");
    session(root, "K4.1", "SRV", true);
    let rel = "docs/sprints/geplant/K4-s/K4.1-s.md";
    let text = read(root, rel).replacen("Session K4.1", "Erste Session · mit Mittelpunkt", 1);
    write(root, rel, &text);
    let loaded = load_sprint(&docs_path(root, "sprints/geplant/K4-s"), "K4-s", "geplant").expect("Sprint");
    assert_eq!(loaded.sessions.len(), 1);
    assert_eq!(loaded.sessions[0].titel, "Erste Session · mit Mittelpunkt");
}
