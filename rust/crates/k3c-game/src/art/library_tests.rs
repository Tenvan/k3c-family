//! Tests zu library.rs und scales.rs; Vorlage: `TestSetLevelsMissingKeepsState`, `TestDiffSheetsSeveralLevels`,
//! `TestApplyScales` (ohne `DataScales`, das kommt mit RU7.3), `TestBeginLevelsSteps`, `TestAbortLevelLoad` aus
//! `native/k3c-game/art/art_test.go`. Die nil-Library-Fälle entfallen (Rust: `Option<Library>`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use k3c_platform::NullCanvas;

use super::scales::apply_scales;
use super::testutil::{temp_dir, test_grafik};
use super::*;

fn lib() -> Library {
    lib_in(&test_grafik())
}

fn lib_in(grafik: &Path) -> Library {
    Library::new(Config {
        grafik_dir: grafik.to_path_buf(),
        font_path: temp_dir("lib-font").join("fehlt.ttf"),
        ..Config::default()
    })
    .unwrap()
}

fn levels(pairs: &[(&str, &[&str])]) -> Levels {
    pairs
        .iter()
        .map(|(lv, a)| (lv.to_string(), a.iter().map(|s| s.to_string()).collect()))
        .collect()
}

#[test]
fn set_levels_missing_keeps_state() {
    let mut l = lib();
    let mut c = NullCanvas::default();
    let err = l
        .set_levels(
            &levels(&[("wald", &["figuren/zwerg"]), ("kaputt", &["figuren/gibtsnicht"])]),
            &mut c,
        )
        .unwrap_err();
    assert!(
        err.is_missing() && l.levels().is_empty() && l.sheet_count() == 0,
        "{err}, Level {:?}",
        l.levels()
    );
    assert_eq!(c.textures, 0, "nichts hochgeladen");
    l.set_levels(&levels(&[("leer", &[])]), &mut c).unwrap();
    assert_eq!(l.levels(), vec!["leer".to_string()], "Level ohne Assets");
}

fn spec(asset: &str, anim: &str, w: i64, h: i64) -> SheetSpec {
    SheetSpec {
        asset: asset.into(),
        anim: anim.into(),
        w,
        h,
        ..SheetSpec::default()
    }
}

fn key(asset: &str, anim: &str) -> SheetKey {
    (asset.to_string(), anim.to_string())
}

/// Zwei Level zugleich: gemeinsame Sheets einmal laden und zählen; frei wird nur, was kein Level mehr nennt.
#[test]
fn diff_sheets_several_levels() {
    let plan = |level: &str, sheets: Vec<SheetSpec>| Plan {
        level: level.into(),
        sheets,
        budget: Budget::default(),
    };
    let wald = plan(
        "wald",
        vec![spec("figuren/zwerg", "idle", 2, 2), spec("natur/baum", "idle", 4, 4)],
    );
    let mine = plan(
        "mine",
        vec![spec("figuren/zwerg", "idle", 2, 2), spec("natur/stein", "idle", 1, 1)],
    );
    let both = [mine, wald.clone()];
    let (load, free) = diff_sheets(&BTreeSet::new(), &both);
    assert!(
        load.len() == 3 && free.is_empty(),
        "leer → beide: laden {load:?}, frei {free:?}"
    );
    let b = budget_of(&both);
    assert!(b.images == 3 && b.bytes == (4 + 16 + 1) * 4, "Budget {b:?}");
    let have: BTreeSet<SheetKey> = [
        key("figuren/zwerg", "idle"),
        key("natur/baum", "idle"),
        key("natur/stein", "idle"),
    ]
    .into();
    let (load, free) = diff_sheets(&have, std::slice::from_ref(&wald));
    assert!(
        load.is_empty() && free == vec![key("natur/stein", "idle")],
        "nur noch wald: {load:?} {free:?}"
    );
    let (_, free) = diff_sheets(&have, &[]);
    assert!(
        free.len() == 3 && free[0].0 == "figuren/zwerg",
        "kein Level: frei {free:?}"
    );
}

/// B-948: Der Faktor vergrößert Bäume; ohne Eintrag bleibt der Maßstab.
#[test]
fn apply_scales_faktor() {
    let sc: BTreeMap<String, f64> = [("objekte/tanne".to_string(), 1.5)].into();
    let sheet = |upp| Sheet {
        units_per_px: upp,
        ..Sheet::default()
    };
    let mut sheets: BTreeMap<SheetKey, Sheet> = [
        (key("objekte/tanne", "idle"), sheet(2.0)),
        (key("objekte/stein", "idle"), sheet(2.0)),
    ]
    .into();
    apply_scales(&mut sheets, &sc);
    assert_eq!(sheets[&key("objekte/tanne", "idle")].units_per_px, 3.0, "Tanne");
    assert_eq!(sheets[&key("objekte/stein", "idle")].units_per_px, 2.0, "Stein");
}

/// B-949: `begin_levels` meldet fehlende Grafik sofort, ein Level ohne Assets ist mit dem ersten Schritt fertig.
#[test]
fn begin_levels_steps() {
    let mut l = lib();
    let mut c = NullCanvas::default();
    let err = l
        .begin_levels(&levels(&[("kaputt", &["figuren/gibtsnicht"])]))
        .unwrap_err();
    assert!(err.is_missing(), "fehlende Grafik: {err}");
    let mut ld = l.begin_levels(&levels(&[("leer", &[])])).unwrap();
    assert_eq!(ld.progress(), 1.0, "Level ohne Assets");
    assert!(ld.step(&mut l, 4, &mut c).unwrap());
    assert_eq!(l.levels(), vec!["leer".to_string()]);
}

/// Abort beendet ein Laden ohne Wirkung auf den alten Stand; ein zweiter Abort und Step danach sind harmlos.
#[test]
fn abort_level_load() {
    let mut l = lib();
    let mut c = NullCanvas::default();
    l.set_levels(&levels(&[("alt", &[])]), &mut c).unwrap();
    let mut ld = l.begin_levels(&levels(&[("neu", &["figuren/zwerg"])])).unwrap();
    assert!(!ld.step(&mut l, 1, &mut c).unwrap());
    ld.abort(&mut c);
    ld.abort(&mut c);
    assert!(ld.step(&mut l, 1, &mut c).unwrap());
    assert_eq!(l.levels(), vec!["alt".to_string()], "nach Abort");
    assert_eq!((c.textures, c.freed), (1, 1), "das geladene Sheet ist wieder frei");
}

/// Fehler im zweiten Sheet: das erste wird freigegeben, ein folgendes Abort gibt es nicht noch einmal frei.
#[test]
fn step_error_then_abort_frees_once() {
    let dir = temp_dir("doppel-free");
    super::testutil::copy_dir(&test_grafik(), &dir);
    let mut l = lib_in(&dir);
    let mut c = NullCanvas::default();
    let mut ld = l.begin_levels(&levels(&[("wald", &["figuren/zwerg"])])).unwrap();
    std::fs::remove_file(dir.join("figuren/zwerg/lauf.png")).unwrap(); // nach der Planung, vor dem Hochladen
    assert!(ld.step(&mut l, 2, &mut c).is_err(), "kaputtes zweites Sheet");
    assert_eq!((c.textures, c.freed), (1, 1), "das erste Sheet ist frei");
    ld.abort(&mut c);
    assert_eq!(c.freed, 1, "Abort gibt nicht ein zweites Mal frei");
}

/// AC-02: schrittweise laden wie Go, Budget wie Go, Levelwechsel gibt nicht mehr gebrauchte Sheets frei.
#[test]
fn level_wechsel_gibt_frei() {
    let mut l = lib();
    let mut c = NullCanvas::default();
    let mut ld = l.begin_levels(&levels(&[("wald", &["figuren/zwerg"])])).unwrap();
    assert!(!ld.step(&mut l, 1, &mut c).unwrap());
    assert_eq!(ld.progress(), 0.5);
    assert!(
        l.levels().is_empty() && l.sheet("figuren/zwerg", "idle").is_none(),
        "alter Stand bis zum Ende"
    );
    assert!(ld.step(&mut l, 1, &mut c).unwrap());
    assert_eq!(l.budget().bytes, (12 * 6 + 16 * 6) * 4);
    let s = l.sheet("figuren/zwerg", "lauf").unwrap();
    assert_eq!((s.frames.len(), s.fw, s.fh), (4, 4, 6));
    // gleiche Sheets in zwei Leveln: nichts neu, nichts frei
    l.set_levels(
        &levels(&[("wald", &["figuren/zwerg"]), ("mine", &["figuren/zwerg"])]),
        &mut c,
    )
    .unwrap();
    assert_eq!((c.textures, c.freed, l.budget().images), (2, 0, 2));
    l.set_levels(&levels(&[("leer", &[])]), &mut c).unwrap();
    assert_eq!((c.freed, l.sheet_count(), l.budget()), (2, 0, Budget::default()));
}

/// Die Fixture der Demo lädt über die Library (RU2-Lader ersetzt).
#[test]
fn fixture_laedt_ueber_library() {
    let g = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/art/grafik");
    let mut l = lib_in(&g);
    let mut c = NullCanvas::default();
    let assets = group_assets(&g, "figuren").unwrap();
    assert_eq!(assets, vec!["figuren/test".to_string()]);
    l.set_levels(&BTreeMap::from([("demo".to_string(), assets)]), &mut c)
        .unwrap();
    let s = l.sheet("figuren/test", "idle").unwrap();
    assert_eq!(s.frames.len(), 2);
    assert_eq!((s.origin_x, s.origin_y), (0.5, 1.0));
    assert_eq!(s.frame(1).unwrap().src, k3c_platform::Rect::new(4.0, 0.0, 4.0, 4.0));
}
