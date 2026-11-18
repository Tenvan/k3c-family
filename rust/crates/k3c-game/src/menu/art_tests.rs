//! Tests aus `native/k3c-game/menu/art_test.go`.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use k3c_platform::NullCanvas;

use super::art::{ART_ANIM, MAIN_ART, PAUSE_ART, art_list};
use super::draw::Painter;
use super::testutil::new_menus;
use crate::art::{Config, Library, WorldImages, read_manifest};

fn grafik() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/grafik")
}

/// B-818 AC-01/AC-02: Jeder Eintrag von Haupt- und Pause-Menü hat eine Illustration aus vorhandener Spielgrafik (Spec
/// unter public/grafik, gerendert mit der Animation `ART_ANIM`); `art_list` nennt jede genau einmal und sortiert.
#[test]
fn menu_art_for_every_entry() {
    for (i, a) in MAIN_ART.iter().chain(PAUSE_ART.iter()).enumerate() {
        assert!(!a.is_empty(), "Eintrag {i} ohne Illustration");
        // Eingecheckt ist nur die Spec; manifest.json entsteht erst mit gen:paper (B-737) und fehlt z. B. in der CI.
        let spec = grafik().join(format!("{a}.paper.json"));
        assert!(spec.exists(), "{a}: Spec fehlt: {}", spec.display());
        let Ok(m) = read_manifest(&grafik(), a) else {
            continue; // nicht gerendert: Animation prüft der Lauf mit gen:paper
        };
        assert!(
            m.anim_names().iter().any(|n| n == ART_ANIM),
            "{a} ohne Animation {ART_ANIM:?}: {:?}",
            m.anim_names()
        );
    }
    let worlds = WorldImages::from_data();
    let all = |_: &str| true;
    let got = art_list(&worlds, true, Some(&all));
    let mut sorted = got.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(got, sorted, "Art nicht sortiert oder doppelt");
    // Im Level nur die kleinen Weltbilder, die großen nur im Hauptmenü (B-937).
    let shared = new_menus();
    let mut m = shared.borrow_mut();
    m.usable = Some(Box::new(|_| true));
    let lvl = m.assets();
    for (w, img) in &m.worlds.0 {
        assert!(
            got.contains(&img.big) && got.contains(&img.small),
            "Art ohne Weltbilder von {w}: {got:?}"
        );
        assert!(
            !lvl.contains(&img.big) && lvl.contains(&img.small),
            "Assets je Level für {w}: {lvl:?}"
        );
    }
}

/// Ohne Grafik-Bibliothek oder ohne Sheet zeichnet die Karte nur Text (`art_box` meldet false, nichts wird gezeichnet).
#[test]
fn menu_art_missing_is_text_card() {
    let mut dst = NullCanvas::default();
    let mut p = Painter::new(None, None);
    assert!(
        !p.art_box(&mut dst, "objekte/portal", 0.0, 0.0, 10.0, 10.0),
        "ohne Bibliothek gezeichnet"
    );
    let cfg = Config {
        grafik_dir: grafik(),
        font_path: "fehlt.ttf".into(),
        ..Config::default()
    };
    let lib = Library::new(cfg).expect("Bibliothek");
    p.lib = Some(Rc::new(RefCell::new(lib)));
    assert!(
        !p.art_box(&mut dst, "objekte/portal", 0.0, 0.0, 10.0, 10.0),
        "ohne geladenes Sheet gezeichnet"
    );
}
