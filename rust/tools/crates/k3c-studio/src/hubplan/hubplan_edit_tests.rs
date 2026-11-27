//! Port von `hubplan_edit_test.go` (WZ9.3): Prüfung eines geänderten Stands, Arten zum Hinzufügen, Index der Zeilen und
//! Stand beim Laden.

use super::super::editor_tests::{ARMORY3, editor, planer_armory_from3, planer_copy, read_in, repo_root, row_of};
use super::super::hubplan_rules::RULES_FILE;
use super::super::{HUB_FILE, HubStand};
use crate::kitx::sha256_hex;

#[test]
fn hub_check_prueft_den_kandidaten() {
    let Some(planer) = editor(&repo_root()) else { return };
    let view = planer.check(&planer_armory_from3());
    assert!(
        view.error.is_empty() && view.findings.is_empty(),
        "{} {:?}",
        view.error,
        view.findings
    );
    assert_eq!(view.stand, HubStand::default());
    let row = row_of(&view.matrix, "armory@40");
    assert_eq!(
        (row.hub_level, row.source.as_str(), row.index),
        (3, "sites", 8),
        "{row:?}"
    );

    let level9 = r#"{ "kind": "armory", "offsetUnits": 40, "hubLevel": 9 }"#;
    let bad = planer.check(&planer_armory_from3().replacen(ARMORY3, level9, 1));
    assert!(
        bad.findings.first().is_some_and(|f| f.rule == "stufe"),
        "{:?}",
        bad.findings
    );
    assert!(!planer.check("{").error.is_empty(), "kaputter Kandidat ohne Fehler");
}

#[test]
fn hub_plan_index_arten_und_stand() {
    let dir = planer_copy(HUB_FILE);
    let root = dir.path();
    let Some(planer) = editor(root) else { return };
    let view = planer.plan_from(HUB_FILE);
    assert_eq!(view.stand.hub, sha256_hex(read_in(root, HUB_FILE).as_bytes()));
    assert_eq!(view.stand.rules, sha256_hex(read_in(root, RULES_FILE).as_bytes()));
    let storage = row_of(&view.matrix, "storage@-24");
    assert_eq!(
        (storage.source.as_str(), storage.index),
        ("islandSites", 0),
        "{storage:?}"
    );
    let zone = row_of(&view.matrix, "zone1");
    assert_eq!(
        (zone.index, zone.line, zone.source.as_str()),
        (0, 1, "sitePlacement"),
        "{zone:?}"
    );
}

/// Ein nicht lesbarer Kandidat (Kommazahl in einem ganzzahligen Feld) nennt den Fehler und trägt den Kandidaten als
/// `raw`, damit das Frontend die Antwort zuordnen kann.
#[test]
fn hub_check_nicht_lesbarer_kandidat_behaelt_raw() {
    let half = r#"{ "kind": "armory", "offsetUnits": 40, "hubLevel": 1.5 }"#;
    let candidate = planer_armory_from3().replacen(ARMORY3, half, 1);
    let Some(planer) = editor(&repo_root()) else { return };
    let view = planer.check(&candidate);
    assert!(!view.error.is_empty() && view.matrix.rows.is_empty(), "{view:?}");
    assert_eq!(view.raw, candidate);
}
