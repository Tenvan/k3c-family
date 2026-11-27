//! Go `placeholder_rows_test.go` (B-436), Teil mit `Set`. Die Fälle, die `Create` prüfen, folgen in RU69.10.

use super::edit::set;
use super::fixtures::{doc, repo, session, sprint, values};
use super::testsupport::write;

const README: &str = "sprints/geplant/GEB1-s/README.md";

#[test]
fn platzhalter_zeile_wird_ersetzt_andere_bleiben() {
    let tmp = repo();
    let root = tmp.path();
    sprint(root, "GEB1", "–");
    let table = doc(root, README)
        + "| GEB1.1 | – (Entwurf) | Umsetzung | autonom | offen |\n| GEB1.2 | – | Umsetzung | autonom | offen |\n";
    write(root, &format!("docs/{README}"), &table);
    session(root, "GEB1.1", "SRV", false);

    set(root, "GEB1.1", &values(&[("Agent", "Mensch")])).expect("Set trotz Platzhalter-Zeilen");

    let readme = doc(root, README);
    assert!(
        readme.contains("| GEB1.1 | `GEB1.1-s.md` | Umsetzung | Mensch | offen |"),
        "{readme}"
    );
    assert!(!readme.contains("– (Entwurf)"), "{readme}");
    assert_eq!(readme.matches("| GEB1.1 |").count(), 1, "{readme}");
    assert!(readme.contains("| GEB1.2 | – |"), "{readme}");
}
