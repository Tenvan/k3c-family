//! Port von `frameohne_test.go`.

use crate::assetsui::test_helpers::{dienst, test_repo};

/// Das ausgeblendete Teil fehlt in den Boxen, das Bild weicht ab; ein ungültiger Name wird abgelehnt (B-734).
#[test]
fn assets_frame_ohne() {
    let Some(repo) = test_repo() else { return };
    let d = dienst(&repo.pfad);
    let mit = d.assets_frame("monarch", "idle", 0, "", "").unwrap();
    let ohne = d
        .assets_frame_ohne("monarch", "idle", 0, &["krone".to_owned()])
        .unwrap();
    assert!(mit.teile.contains_key("krone"), "krone fehlt schon mit allen Teilen");
    assert!(!ohne.teile.contains_key("krone"), "{:?}", ohne.teile);
    assert_ne!(ohne.bild, mit.bild);
    let e = d
        .assets_frame_ohne("monarch", "idle", 0, &["--out".to_owned()])
        .unwrap_err();
    assert_eq!(e, "Teil „--out“ ungültig");
}
