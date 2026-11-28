//! Go `wz55_test.go` (B-867): ein gescheitertes Entfernen nimmt alles byte-gleich zurück.

use super::fixtures::snapshot;
use super::roadmap_refresh_tests::refresh_repo;
use super::store::{ChangeSet, docs_path};
use super::testhooks::fail_remove;

/// Nach zwei Schreibschritten (eine Datei geändert, eine neu) scheitert das Entfernen von `target`.
fn failing_removal(target: &str) {
    let tmp = refresh_repo();
    let root = tmp.path();
    let before = snapshot(root);
    fail_remove();
    let mut c = ChangeSet::new(root);
    c.write("backlog/README.md", "# Backlog\n\ngeändert\n".to_owned());
    c.write("backlog/B-001-neu.md", "# B-001 · neu\n".to_owned());
    c.remove.push(target.to_owned());
    let error = c.apply().expect_err("Löschfehler");
    assert!(
        error.contains("erzwungener Löschfehler") && error.contains("zurückgenommen"),
        "{error}"
    );
    assert!(snapshot(root) == before, "{target}: Stand nicht byte-gleich");
    assert!(!docs_path(root, "backlog/B-001-neu.md").exists(), "{target}");
}

#[test]
fn entfernen_eines_ordners_scheitert() {
    failing_removal("sprints/erledigt/NG15-ton");
}

#[test]
fn entfernen_einer_datei_scheitert() {
    failing_removal("sprints/aktiv/A1-a/A1.3-x.md");
}

#[test]
fn entfernen_ohne_fehler() {
    let tmp = refresh_repo();
    let root = tmp.path();
    let mut c = ChangeSet::new(root);
    c.remove = vec![
        "sprints/erledigt/NG15-ton".to_owned(),
        "sprints/erledigt/gibt-es-nicht".to_owned(),
    ];
    c.apply().expect("apply");
    assert!(!docs_path(root, "sprints/erledigt/NG15-ton").exists());
}
