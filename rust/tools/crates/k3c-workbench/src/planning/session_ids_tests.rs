//! Go `session_ids_test.go` (B-372/AC-04): Session-IDs mit Buchstaben wie K3.2a lesen und ändern. Anlegen: RU69.10.

use super::create::get;
use super::edit::set;
use super::fixtures::{doc, repo, session, sprint, values};
use super::planning::parse_sprint;
use super::testsupport::{temp_repo, write};

#[test]
fn session_id_mit_buchstabe_aendern() {
    let tmp = repo();
    let root = tmp.path();
    sprint(root, "K3", "–");
    session(root, "K3.2a", "SRV", true);
    set(root, "K3.2a", &values(&[("Agent", "Mensch")])).expect("Set");
    let sprint = parse_sprint(&doc(root, "sprints/geplant/K3-s/README.md"), "K3-s", "geplant");
    assert_eq!(sprint.sessions.len(), 1, "{sprint:?}");
    assert_eq!(
        (sprint.sessions[0].nr.as_str(), sprint.sessions[0].agent.as_str()),
        ("K3.2a", "Mensch")
    );
}

#[test]
fn session_id_mit_buchstabe() {
    let tmp = temp_repo();
    let root = tmp.path();
    let readme = "# K3 · SRV · Sprint K3\n\n## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n\
        | K3.2a | `K3.2a-teil.md` | Umsetzung | autonom | offen |\n";
    write(root, "docs/sprints/geplant/K3-s/README.md", readme);
    write(root, "docs/sprints/geplant/K3-s/K3.2a-teil.md", "# K3.2a · Teil a\n");
    let out = get(root, "K3.2a").expect("get");
    assert_eq!(out, "docs/sprints/geplant/K3-s/K3.2a-teil.md\n\n# K3.2a · Teil a\n");
    let sprint = parse_sprint(readme, "K3-s", "geplant");
    assert_eq!(sprint.sessions.len(), 1, "{sprint:?}");
    assert_eq!(sprint.sessions[0].nr, "K3.2a");
}
