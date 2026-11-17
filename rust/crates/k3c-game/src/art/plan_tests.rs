//! Tests zu plan.rs, budget.rs und sheet.rs; Vorlage: `TestPlanLevel*`, `TestSheetFrameIndex` aus
//! `native/k3c-game/art/art_test.go`.

use k3c_platform::{NullCanvas, Rect, TexId};

use super::testutil::{copy_dir, temp_dir, test_grafik};
use super::*;

fn zwerg() -> Vec<String> {
    vec!["figuren/zwerg".to_string()]
}

#[test]
fn plan_level() {
    let p = super::plan_level(&test_grafik(), "test", &zwerg()).unwrap();
    assert_eq!(p.sheets.len(), 2, "Sheets {:?}", p.sheets);
    assert_eq!((p.sheets[0].anim.as_str(), p.sheets[1].anim.as_str()), ("idle", "lauf"));
    let s = &p.sheets[0];
    assert_eq!((s.frames, s.fw, s.fh, s.w, s.h), (3, 4, 6, 12, 6), "Maße {s:?}");
    assert!(
        s.origin_x == 0.5 && s.origin_y == 5.0 / 6.0 && s.units_per_px == 1.0,
        "Maße {s:?}"
    );
}

#[test]
fn plan_level_budget() {
    let p = super::plan_level(&test_grafik(), "test", &zwerg()).unwrap();
    // 12×6 + 16×6 Pixel, 4 Byte je Pixel
    assert_eq!(
        (p.budget.images, p.budget.bytes),
        (2, (12 * 6 + 16 * 6) * 4),
        "Budget {:?}",
        p.budget
    );
    assert_eq!(p.budget.to_string(), "2 Bilder, 0.0 MiB RGBA");
    let mut b = Budget::default();
    b.add(1024, 1024);
    b.add_bytes(1 << 20);
    assert!(b.mib() == 5.0 && b.images == 1, "MiB {} {b:?}", b.mib());
}

#[test]
fn plan_level_missing() {
    let assets = ["figuren/zwerg", "figuren/gibtsnicht", "gegner/auchnicht"].map(String::from);
    let err = super::plan_level(&test_grafik(), "test", &assets).unwrap_err();
    assert!(err.is_missing(), "erwartet Missing, bekam {err}");
    for want in ["gibtsnicht", "auchnicht", "manifest.json", "task gen:paper"] {
        assert!(err.to_string().contains(want), "Meldung ohne {want:?}: {err}");
    }
}

#[test]
fn plan_level_missing_sheet() {
    let dir = temp_dir("missing-sheet");
    let asset = dir.join("figuren").join("x");
    std::fs::create_dir_all(&asset).unwrap();
    let m = r#"{"rahmen":[-1,-1,2,2],"frameW":2,"frameH":2,"animationen":{"idle":{"frames":1,"fps":1,"datei":"idle.png"}}}"#;
    std::fs::write(asset.join("manifest.json"), m).unwrap();
    let err = super::plan_level(&dir, "t", &["figuren/x".to_string()]).unwrap_err();
    assert!(
        err.is_missing() && err.to_string().contains("idle.png"),
        "Meldung ohne Pfad des Sheets: {err}"
    );
}

#[test]
fn plan_level_bad_size() {
    let dir = temp_dir("bad-size");
    copy_dir(&test_grafik(), &dir);
    let m = r#"{"rahmen":[-2,-5,4,6],"frameW":4,"frameH":6,"animationen":{"idle":{"frames":9,"fps":6,"datei":"idle.png"}}}"#;
    std::fs::write(dir.join("figuren").join("zwerg").join("manifest.json"), m).unwrap();
    let err = super::plan_level(&dir, "t", &zwerg()).unwrap_err();
    assert!(
        err.to_string().contains("zu klein"),
        "zu kleines Bild nicht erkannt: {err}"
    );
}

#[test]
fn sheet_frame_index() {
    let s = Sheet::default();
    assert!(
        s.frame(3).is_none() && s.frame_at(1.0).is_none(),
        "leeres Sheet liefert Bild"
    );
}

#[test]
fn upload_schneidet_frames() {
    let p = super::plan_level(&test_grafik(), "test", &zwerg()).unwrap();
    let spec = &p.sheets[1];
    let mut c = NullCanvas::default();
    let pix = vec![0u8; (spec.w * spec.h * 4) as usize];
    let s = upload(spec, &pix, &mut c);
    assert_eq!(c.textures, 1);
    assert_eq!(
        (s.asset.as_str(), s.anim.as_str(), s.frames.len(), s.fps),
        ("figuren/zwerg", "lauf", 4, 12.0)
    );
    assert_eq!(s.texture(), Some(TexId(0)));
    assert_eq!(s.frame(-1).unwrap().src, Rect::new(12.0, 0.0, 4.0, 6.0));
    assert_eq!(s.frame_at(0.25).unwrap().src.x, 12.0); // 0,25 s × 12 fps = Bild 3
}
