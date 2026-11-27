//! Go `wz54_test.go` (B-862, B-863, B-865). Die Schritte mit `create` und `rebuild_index` stehen in `create_cases_tests.rs`.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;

use super::edit::set;
use super::fixtures::{doc, full_ticket, rel_repo, snapshot, values};
use super::index::{ARCHIVE_INDEX, INDEX_FILES, MAIN_INDEX, less};
use super::relations::{FIELD_REPLACE, reverse_lines};
use super::roadmap_refresh::ROADMAP_REL;
use super::roadmap_refresh_tests::refresh_repo;
use super::store::{ChangeSet, docs_path, resolve};
use super::testhooks::{fail_write, written};
use super::testsupport::write;
use super::ticketid::ticket_cmp;

/// In wie vielen Index-Dateien das Ticket eine Zeile hat.
fn index_count(root: &Path, nr: &str) -> usize {
    let start = format!("| [{nr}](");
    INDEX_FILES.iter().filter(|f| doc(root, f).contains(&start)).count()
}

#[test]
fn schreibfehler_beim_statuswechsel_verliert_keine_zeile() {
    for broken in [MAIN_INDEX, ARCHIVE_INDEX] {
        let tmp = rel_repo();
        let root = tmp.path();
        let before = snapshot(root);
        fail_write(docs_path(root, broken));
        let error = set(root, "B-002", &values(&[("Status", "erledigt")])).expect_err("Schreibfehler");
        assert!(error.contains("zurückgenommen"), "{broken}: {error}");
        assert_eq!(index_count(root, "B-002"), 1, "{broken}");
        assert!(snapshot(root) == before, "{broken}: Zustand geändert");
        assert!(docs_path(root, "backlog/B-002-t.md").exists(), "{broken}");
    }
}

#[test]
fn statuswechsel_schreibt_den_ziel_index_zuerst() {
    let tmp = rel_repo();
    let root = tmp.path();
    fail_write(docs_path(root, "gibt-es-nicht"));
    set(root, "B-002", &values(&[("Status", "erledigt")])).expect("Set");
    let order = written();
    let position = |rel: &str| order.iter().position(|p| *p == docs_path(root, rel));
    let (archive, main) = (position(ARCHIVE_INDEX), position(MAIN_INDEX));
    assert!(archive.is_some() && main.is_some() && archive < main, "{order:?}");
    assert_eq!(index_count(root, "B-002"), 1);
}

#[test]
fn vierstellige_nummern_gehen_durch_set() {
    let tmp = rel_repo();
    let root = tmp.path();
    for nr in ["B-999", "B-1000", "B-1001"] {
        full_ticket(root, nr, "t", nr);
    }
    let changes = values(&[(FIELD_REPLACE, "B-999, B-1000"), ("Prio", "hoch")]);
    set(root, "B-1001", &changes).expect("Set B-1001");
    let found = resolve(root, "B-1000").expect("resolve");
    assert!(found.rel.ends_with("B-1000-t.md"), "{found:?}");
    set(root, "B-1000", &values(&[("Status", "erledigt")])).expect("Set B-1000");
    assert_eq!(index_count(root, "B-1000"), 1);
    assert!(doc(root, ARCHIVE_INDEX).contains("| [B-1000](B-1000-"));
    let reverse = reverse_lines(root, "B-1000").join("\n");
    assert!(reverse.contains("Ersetzt durch:** B-1001"), "{reverse}");
}

#[test]
fn nummern_ordnen_nach_zahl() {
    for (a, b) in [("B-999", "B-1000"), ("B-1000", "B-1001"), ("B-002", "B-010")] {
        assert_eq!(ticket_cmp(a, b), Ordering::Less, "{a} {b}");
        assert!(less(&HashMap::new(), a, b) && !less(&HashMap::new(), b, a), "{a} {b}");
    }
}

#[test]
fn der_fahrplan_heilt_nur_bei_sprint_aenderungen() {
    let tmp = refresh_repo();
    let root = tmp.path();
    let before = doc(root, ROADMAP_REL);
    let mut c = ChangeSet::new(root);
    c.write("backlog/README.md", "# Backlog\n\nneu\n".to_owned());
    c.apply().expect("apply");
    assert_eq!(
        doc(root, ROADMAP_REL),
        before,
        "Ticket-Änderung hat den Fahrplan geschrieben"
    );

    let session = "sprints/aktiv/A1-a/A1.3-x.md";
    let mut c = ChangeSet::new(root);
    c.write(session, doc(root, session));
    c.apply().expect("apply");
    let after = doc(root, ROADMAP_REL);
    assert!(after != before && !after.contains("`aktiv/NG15-ton/`"), "{after}");

    // Über Set: eine Ticket-Änderung lässt einen veralteten Fahrplan stehen.
    let tmp = rel_repo();
    let repo = tmp.path();
    let stale = doc(repo, ROADMAP_REL).replacen(
        "|---|---|---|---|---|\n\n## Offen am Gerät",
        "|---|---|---|---|---|\n| ZZ9 | DEV | Weg | – | `aktiv/ZZ9-weg/` |\n\n## Offen am Gerät",
        1,
    );
    assert!(stale.contains("ZZ9"));
    write(repo, "docs/sprints/README.md", &stale);
    set(repo, "B-002", &values(&[("Prio", "hoch")])).expect("Set");
    assert_eq!(doc(repo, ROADMAP_REL), stale);
}
