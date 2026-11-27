//! Schreib-Unterbau (`ChangeSet`, `write_atomic`): Zeilenenden, Reihenfolge, Zurücknehmen. Go prüft ihn über die
//! schreibenden Werkzeuge (RU69.9, RU69.10).

use super::store::{ChangeSet, write_atomic};
use super::testsupport::{read, temp_repo, write};

#[test]
fn zeilenenden_bleiben() {
    let tmp = temp_repo();
    let root = tmp.path();
    write(root, "docs/backlog/B-002-a.md", "# B-002 · A\r\n\r\nalt\r\n");
    let mut changes = ChangeSet::new(root);
    let text = changes.read("backlog/B-002-a.md").expect("lesen");
    assert_eq!(text, "# B-002 · A\n\nalt\n");
    changes.write("backlog/B-002-a.md", text.replace("alt", "neu"));
    assert_eq!(
        changes.read("backlog/B-002-a.md").expect("geplant"),
        "# B-002 · A\n\nneu\n"
    );
    changes.write("backlog/B-003-b.md", "a\nb\n".to_owned());
    changes.apply().expect("apply");
    assert_eq!(read(root, "docs/backlog/B-002-a.md"), "# B-002 · A\r\n\r\nneu\r\n");
    assert_eq!(read(root, "docs/backlog/B-003-b.md"), "a\nb\n");
    let leftovers = std::fs::read_dir(root.join("docs/backlog")).expect("Ordner");
    assert!(
        leftovers
            .flatten()
            .all(|e| !e.file_name().to_string_lossy().starts_with(".plan-"))
    );
}

#[test]
fn reihenfolge_und_sprint_bezug() {
    let tmp = temp_repo();
    let mut changes = ChangeSet::new(tmp.path());
    for rel in ["backlog/b", "backlog/a", "backlog/README.md"] {
        changes.write(rel, String::new());
    }
    changes.first = vec![
        "backlog/README.md".to_owned(),
        "fehlt".to_owned(),
        "backlog/README.md".to_owned(),
    ];
    assert_eq!(changes.write_order(), ["backlog/README.md", "backlog/a", "backlog/b"]);
    assert!(!changes.touches_sprints());
    changes
        .moves
        .push(("sprints/geplant/X1-a".to_owned(), "sprints/aktiv/X1-a".to_owned()));
    assert!(changes.touches_sprints());
}

#[test]
fn fehler_nimmt_alles_zurueck() {
    let tmp = temp_repo();
    let root = tmp.path();
    write(root, "docs/backlog/B-002-a.md", "alt\r\n");
    write(root, "docs/backlog/B-004-weg.md", "weg\n");
    // Ein Ordner mit dem Namen einer Zieldatei lässt das Schreiben scheitern.
    std::fs::create_dir_all(root.join("docs/backlog/zz.md")).expect("Ordner");
    let mut changes = ChangeSet::new(root);
    changes.moves.push((
        "backlog/B-004-weg.md".to_owned(),
        "backlog/archiv/B-004-weg.md".to_owned(),
    ));
    changes.write("backlog/B-002-a.md", "neu\n".to_owned());
    changes.write("backlog/B-003-neu.md", "neu\n".to_owned());
    changes.write("backlog/zz.md", "x".to_owned());
    let error = changes.apply().expect_err("Schreibfehler");
    assert!(error.ends_with("(Änderung zurückgenommen)"), "{error}");
    assert_eq!(read(root, "docs/backlog/B-002-a.md"), "alt\r\n");
    assert_eq!(read(root, "docs/backlog/B-004-weg.md"), "weg\n");
    assert!(!root.join("docs/backlog/B-003-neu.md").exists());
}

#[test]
fn entfernen_und_atomar_schreiben() {
    let tmp = temp_repo();
    let root = tmp.path();
    write(root, "docs/sprints/geplant/X1-a/README.md", "x\n");
    let mut changes = ChangeSet::new(root);
    changes.remove = vec!["sprints/geplant/X1-a".to_owned(), "fehlt".to_owned()];
    changes.apply().expect("apply");
    assert!(!root.join("docs/sprints/geplant/X1-a").exists());
    let file = root.join("neu/ordner/datei.md");
    write_atomic(&file, "a\nb\n", true).expect("schreiben");
    write_atomic(&file, "c\n", false).expect("ersetzen");
    assert_eq!(std::fs::read(&file).expect("lesen"), b"c\n");
}
