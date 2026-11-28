//! Go `watch_test.go`. `Watch` ohne IDs gibt es hier nicht (nur der Go-Test nutzt es); `TestWatch` prüft
//! [`watch_ids`].

use std::fs;
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

use super::testsupport::write;
use super::watch::{MAX_CHANGED_IDS, Snapshot, Watcher, changed_ids, snapshot, stamp, watch_ids};
use crate::testutil::tmp_dir;

fn ids(list: &[&str]) -> Option<Vec<String>> {
    Some(list.iter().map(|id| (*id).to_owned()).collect())
}

fn watched(root: &std::path::Path) -> (Watcher, Receiver<Option<Vec<String>>>) {
    let (hit, seen) = mpsc::channel();
    let watcher = watch_ids(root, Duration::from_millis(10), move |found| {
        // Nach dem Ende des Tests hört niemand mehr zu.
        let _ = hit.send(found);
    });
    (watcher, seen)
}

#[test]
fn stamp_aendert_sich_nur_mit_der_planung() {
    let tmp = tmp_dir();
    let root = tmp.path();
    write(root, "docs/sprints/aktiv/X1-a/README.md", "# X1");
    write(root, "docs/backlog/B-001-a.md", "# B-001");
    let base = stamp(root);
    assert_eq!(stamp(root), base, "unverändert muss gleich bleiben");
    write(root, "docs/sprints/aktiv/X1-a/X1.1-s.md", "neu");
    let changed = stamp(root);
    assert_ne!(changed, base, "neue Datei im aktiven Sprint");
    write(root, "docs/backlog/archiv/B-000-x.md", "archiviert");
    write(root, "src/main.ts", "egal");
    assert_eq!(stamp(root), changed, "Archiv und fremde Dateien zählen nicht");
    write(root, "docs/plan-weiterentwicklung.md", "Plan");
    assert_ne!(stamp(root), changed, "Plan-Dokument zählt");
}

#[test]
fn stamp_ist_fnv_1a_wie_go() {
    // Ohne Planungsdateien bleibt der Startwert von FNV-1a (Go `fnv.New64a`).
    let tmp = tmp_dir();
    assert_eq!(stamp(tmp.path()), "cbf29ce484222325");
}

#[test]
fn watch_meldet_nur_aenderungen() {
    let tmp = tmp_dir();
    let root = tmp.path();
    write(root, "docs/backlog/B-001-a.md", "# B-001");
    let (_watcher, seen) = watched(root);
    let early = seen.recv_timeout(Duration::from_millis(60));
    assert!(early.is_err(), "der Ausgangsstand löst nichts aus: {early:?}");
    write(root, "docs/backlog/B-002-b.md", "# B-002");
    let found = seen
        .recv_timeout(Duration::from_secs(2))
        .expect("Änderung nicht erkannt");
    assert_eq!(found, ids(&["B-002"]));
}

#[test]
fn fallengelassener_waechter_endet() {
    let tmp = tmp_dir();
    let (watcher, seen) = watched(tmp.path());
    drop(watcher);
    write(tmp.path(), "docs/backlog/B-002-b.md", "# B-002");
    // Der Thread ist beendet, also ist auch der Sender weg.
    assert_eq!(
        seen.recv_timeout(Duration::from_secs(1)),
        Err(mpsc::RecvTimeoutError::Disconnected)
    );
}

#[test]
fn changed_ids_nennt_neue_geaenderte_und_geloeschte() {
    let tmp = tmp_dir();
    let root = tmp.path();
    write(root, "docs/sprints/aktiv/X1-a/README.md", "# X1");
    write(root, "docs/sprints/aktiv/X1-a/X1.1-s.md", "a");
    write(root, "docs/sprints/geplant/Y1-b/README.md", "# Y1");
    write(root, "docs/backlog/B-001-a.md", "# B-001");
    let before = snapshot(root);
    assert_eq!(changed_ids(&before, &snapshot(root)), None, "keine Änderung");
    write(root, "docs/sprints/aktiv/X1-a/X1.1-s.md", "ab"); // Session, Größe ändert sich
    write(root, "docs/backlog/B-002-b.md", "# B-002");
    write(root, "docs/plan-weiterentwicklung.md", "Plan");
    fs::remove_file(root.join("docs/backlog/B-001-a.md")).expect("löschen");
    // Sprint geplant nach aktiv: beide Pfade nennen dieselbe ID.
    fs::rename(
        root.join("docs/sprints/geplant/Y1-b"),
        root.join("docs/sprints/aktiv/Y1-b"),
    )
    .expect("verschieben");
    assert_eq!(
        changed_ids(&before, &snapshot(root)),
        ids(&["B-001", "B-002", "X1.1", "Y1", "plan"])
    );
}

fn snap(paths: &[String]) -> Snapshot {
    paths.iter().map(|p| (p.clone(), "1|1".to_owned())).collect()
}

fn from_empty(paths: &[&str]) -> Option<Vec<String>> {
    let paths: Vec<String> = paths.iter().map(|p| (*p).to_owned()).collect();
    changed_ids(&Snapshot::new(), &snap(&paths))
}

#[test]
fn changed_ids_grenzen() {
    let many: Vec<String> = (0..=MAX_CHANGED_IDS)
        .map(|i| format!("docs/backlog/B-{}-x.md", 100 + i))
        .collect();
    assert_eq!(changed_ids(&Snapshot::new(), &snap(&many)), None, "zu viele Dateien");
    assert_eq!(
        from_empty(&["docs/sprints/aktiv/x/y/z.md"]),
        None,
        "keine erkennbare ID"
    );
    assert_eq!(
        from_empty(&[
            "docs/backlog/README.md",
            "docs/sprints/aktiv/README.md",
            "docs/backlog/B-005-x.md"
        ]),
        ids(&["B-005"]),
        "Index-READMEs zählen nicht"
    );
    assert_eq!(
        from_empty(&["docs/sprints/erledigt/Z9-x/"]),
        ids(&["Z9"]),
        "erledigter Sprint"
    );
}

/// Projekte werden beobachtet: die Projekt-ID kommt, die Übersicht gilt als Index (B-909).
#[test]
fn changed_ids_projekte() {
    let tmp = tmp_dir();
    let root = tmp.path();
    write(root, "docs/projekte/WZG-werkzeuge.md", "# WZG");
    write(root, "docs/projekte/README.md", "# Projekte");
    let before = snapshot(root);
    write(root, "docs/projekte/WZG-werkzeuge.md", "# WZG neu");
    write(root, "docs/projekte/README.md", "# Projekte neu");
    assert_eq!(changed_ids(&before, &snapshot(root)), ids(&["WZG"]));
}

/// Ein Sprintwechsel ändert Ticket und beide Sprint-READMEs: alle IDs kommen im selben Ereignis.
#[test]
fn changed_ids_sprintwechsel_eines_tickets() {
    let tmp = tmp_dir();
    let root = tmp.path();
    write(root, "docs/backlog/B-001-a.md", "- **Sprint:** X1");
    write(root, "docs/sprints/aktiv/X1-a/README.md", "Tickets: B-001");
    write(root, "docs/sprints/aktiv/Y1-b/README.md", "Tickets: –");
    let before = snapshot(root);
    write(root, "docs/backlog/B-001-a.md", "- **Sprint:** Y1 xx");
    write(root, "docs/sprints/aktiv/X1-a/README.md", "Tickets: –");
    write(root, "docs/sprints/aktiv/Y1-b/README.md", "Tickets: B-001");
    assert_eq!(changed_ids(&before, &snapshot(root)), ids(&["B-001", "X1", "Y1"]));
}
