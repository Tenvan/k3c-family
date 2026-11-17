//! Tests zu manifest.rs; Vorlage: `TestGroupAssets` aus `native/k3c-game/art/art_test.go`, dazu Manifest-Lesen gegen
//! dieselbe Testgrafik.

use super::testutil::test_grafik;
use super::*;

#[test]
fn group_assets_liest_index() {
    let got = group_assets(&test_grafik(), "figuren").unwrap();
    assert_eq!(got, vec!["figuren/zwerg".to_string()]);
    let err = group_assets(&test_grafik(), "nix").unwrap_err();
    assert!(
        err.is_missing() && err.to_string().contains("index.paper.json"),
        "fehlender Index: {err}"
    );
}

#[test]
fn read_manifest_testgrafik() {
    let m = read_manifest(&test_grafik(), "figuren/zwerg").unwrap();
    assert_eq!(m.frame, [-2.0, -5.0, 4.0, 6.0]);
    assert_eq!((m.frame_w, m.frame_h), (4, 6));
    assert_eq!(m.anim_names(), vec!["idle".to_string(), "lauf".to_string()]);
    let lauf = &m.anims["lauf"];
    assert_eq!((lauf.frames, lauf.fps, lauf.file.as_str()), (4, 12.0, "lauf.png"));
    assert_eq!(m.resolve(lauf), (4, 6, [-2.0, -5.0, 4.0, 6.0]));
}

#[test]
fn resolve_animation_vor_asset() {
    let m: Manifest = serde_json::from_str(
        r#"{"rahmen":[-1,-1,2,2],"frameW":2,"frameH":2,
            "animationen":{"a":{"frames":1,"datei":"a.png","rahmen":[0,0,4,4],"frameW":8,"frameH":9}}}"#,
    )
    .unwrap();
    assert_eq!(m.resolve(&m.anims["a"]), (8, 9, [0.0, 0.0, 4.0, 4.0]));
}

#[test]
fn read_manifest_fehlt_nennt_pfad() {
    let err = read_manifest(&test_grafik(), "figuren/gibtsnicht").unwrap_err();
    let text = err.to_string();
    assert!(
        err.is_missing() && text.contains("gibtsnicht") && text.contains("task gen:paper"),
        "{text}"
    );
}
