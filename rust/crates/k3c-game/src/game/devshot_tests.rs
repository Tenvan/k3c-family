//! Tests zu `devshot.rs` (Go hat keinen Test für `saveShot`); die Dateien entstehen im Ordner `.work/devshot-test/` des
//! Repos (ignoriert) und werden danach entfernt.

use std::path::Path;

use super::*;

#[test]
fn save_shot_names_the_file_by_time() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.work/devshot-test");
    let _ = std::fs::remove_dir_all(&dir);
    let img = Image {
        width: 2,
        height: 2,
        pix: vec![255; 16],
    };
    let now = Zeit::from_unix(1_700_000_000, 123_456_789);
    let path = save_shot(&dir, &now, &img);
    let name = path
        .as_ref()
        .ok()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().into_owned());
    let exists = path.as_ref().is_ok_and(|p| p.is_file());
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(name.as_deref(), Some("shot-20231114-221320.123.png"), "{path:?}");
    assert!(exists, "Datei fehlt");
}

#[test]
fn dev_shot_without_request_does_nothing() {
    let (mut g, _) = crate::game::play_testutil::play_game(Vec::new());
    g.dev_shot(0.0);
    assert!(!g.shot_next);
}
