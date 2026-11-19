//! RU55.6 (AC-06): Parallax-Satz einer Welt mit und ohne Eintrag in `data/look.json`.

use crate::look::{profile, with_profile};

#[test]
fn welt_mit_eintrag_hat_eigenen_satz() {
    let k2c = profile("k2c").expect("Profil k2c");
    let w = k2c.world("forest");
    assert!(w.forest, "Wald-Wirkung im Level forest");
    assert!(
        w.motif.size >= 3.0 && w.motif.size <= 5.0,
        "Leitmotiv 3 … 5 M: {}",
        w.motif.size
    );
    assert_eq!(k2c.haze_for("forest", 0), w.haze.expect("Dunst-Satz")[0]);
    assert!(
        k2c.haze_for("forest", 0) > k2c.haze[0],
        "Wald dunstiger als der Standard"
    );
    assert_eq!(
        k2c.scroll_for("forest", 1, 0.35),
        0.35,
        "ohne eigenes Tempo gilt das Biom"
    );
}

#[test]
fn welt_ohne_eintrag_nutzt_den_standard() {
    let k2c = profile("k2c").expect("Profil k2c");
    let w = k2c.world("gibt-es-nicht");
    assert!(!w.forest && w.scroll.is_none() && w.haze.is_none());
    assert_eq!(k2c.haze_for("gibt-es-nicht", 0), k2c.haze[0]);
    assert_eq!(k2c.haze_for("gibt-es-nicht", 7), 0.0, "Schicht außerhalb");
    assert_eq!(k2c.scroll_for("gibt-es-nicht", 2, 0.5), 0.5);
}

#[test]
fn eigenes_tempo_ersetzt_das_biom() {
    let mut l = profile("k2c").expect("Profil k2c").clone();
    l.worlds.insert(
        "x".into(),
        crate::look_world::WorldLook {
            scroll: Some([0.0, 0.1, 0.9]),
            ..Default::default()
        },
    );
    assert_eq!(l.scroll_for("x", 2, 0.5), 0.9);
    assert_eq!(l.scroll_for("x", 1, 0.5), 0.1);
}

#[test]
fn profil_go_ohne_wald() {
    with_profile("go", || {
        let go = crate::look::current();
        assert!(!go.forest.enabled && go.worlds.is_empty());
    });
}
