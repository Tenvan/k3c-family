//! RU55.6 (AC-06): Dichte steigt mit den Bäumen im Umkreis und fällt weich beim Roden; Leitmotiv und Waldwand.

use k3c_data::look::{current, with_profile};
use k3c_sim::state::{ResourceNode, World as SimWorld};

use super::*;
use crate::world::testutil::layer_cell;

fn trees(n: usize, x: f64) -> SimWorld {
    SimWorld {
        nodes: (0..n)
            .map(|i| ResourceNode {
                id: i as i64,
                kind: "tree".into(),
                x: x + i as f64 * 0.5,
                ..ResourceNode::default()
            })
            .collect(),
        ..SimWorld::default()
    }
}

#[test]
fn dichte_ist_reine_funktion_mit_grenzfaellen() {
    assert_eq!(forest_density(0.0, 10.0), 0.0);
    assert_eq!(forest_density(5.0, 10.0), 0.5);
    assert_eq!(forest_density(10.0, 10.0), 1.0);
    assert_eq!(forest_density(99.0, 10.0), 1.0, "begrenzt");
    assert_eq!(forest_density(5.0, 0.0), 0.0, "ohne full");
    assert!(
        forest_density(4.0, 10.0) < forest_density(5.0, 10.0),
        "steigt mit den Bäumen"
    );
}

#[test]
fn zaehlt_nur_stehende_baeume_im_umkreis() {
    let mut w = trees(6, 10.0); // x 10 … 12,5
    w.nodes[0].gone = true;
    w.nodes[1].kind = "rock".into();
    assert_eq!(count_trees(&w, 11.0, 14.0), 4);
    assert_eq!(count_trees(&w, 100.0, 14.0), 0, "weit weg");
    assert_eq!(count_trees(&w, 11.0, 0.6), 2, "kleiner Umkreis");
}

#[test]
fn dichte_faellt_weich_beim_roden() {
    with_profile("k2c", || {
        crate::world::register_look();
        let (c, w) = (layer_cell(11.0), World::new(None));
        let f = &current().forest;
        let dense = trees(f.full as usize, 10.0);
        // dichter Wald, lange stabil
        let mut d = 0.0;
        for k in 0..600 {
            d = w.density(&c, &dense, f64::from(k) / 60.0, "forest");
        }
        assert!((d - 1.0).abs() < 1e-6, "voll dicht: {d}");
        // ein Baum fällt: Ziel 0,9 — nie ein Sprung über 0,01 je Bild (60 Hz), am Ende 0,9
        let felled = trees(f.full as usize - 1, 10.0);
        let mut prev = d;
        for k in 600..1500 {
            let now = w.density(&c, &felled, f64::from(k) / 60.0, "forest");
            assert!((now - prev).abs() < 0.01, "Sprung {prev} → {now} im Bild {k}");
            assert!(now <= prev + 1e-12, "fällt nur");
            prev = now;
        }
        assert!((prev - 0.9).abs() < 0.01, "Ziel erreicht: {prev}");
        // alles gerodet: Dichte fällt weiter, ohne Sprung
        let clear = trees(0, 0.0);
        let first = w.density(&c, &clear, 25.0, "forest");
        assert!(first > 0.8, "ein Bild nach dem Roden fast unverändert: {first}");
    });
}

#[test]
fn dichte_im_selben_bild_aendert_sich_nicht_und_der_erste_aufruf_setzt_sofort() {
    with_profile("k2c", || {
        crate::world::register_look();
        let (c, w) = (layer_cell(11.0), World::new(None));
        let full = trees(current().forest.full as usize, 10.0);
        assert_eq!(w.density(&c, &full, 5.0, "forest"), 1.0, "Level-Start ohne Einblenden");
        assert_eq!(
            w.density(&c, &trees(0, 0.0), 5.0, "forest"),
            1.0,
            "selbes Bild: unverändert"
        );
    });
}

#[test]
fn ohne_wald_wirkung_dichte_null() {
    let w = World::new(None);
    let (c, full) = (layer_cell(11.0), trees(30, 10.0));
    with_profile("k2c", || {
        assert_eq!(w.density(&c, &full, 1.0, "cave"), 0.0, "Höhle ohne forest")
    });
    with_profile("go", || {
        assert_eq!(w.density(&c, &full, 2.0, "forest"), 0.0, "Profil go")
    });
}

#[test]
fn waldwand_und_toenung_folgen_der_dichte() {
    with_profile("k2c", || {
        let open = current().forest.open_alpha;
        assert_eq!(wall_alpha(0, 0.0, open), 1.0, "fern bleibt voll");
        assert_eq!(wall_alpha(1, 0.0, open), open, "gerodet: durchsichtig");
        assert_eq!(wall_alpha(2, 1.0, open), 1.0, "dicht: voll");
        assert!(wall_alpha(1, 0.5, open) > open && wall_alpha(1, 0.5, open) < 1.0);
        assert_eq!(forest_tint(0.0), [1.0; 3]);
        let t = forest_tint(1.0);
        assert!(t[0] < t[1] && t[2] < t[1], "dunkles Oliv: {t:?}");
    });
}

#[test]
fn leitmotiv_steht_in_der_ferne_drei_bis_fuenf_m_gross() {
    with_profile("k2c", || {
        crate::world::register_look();
        let c = layer_cell(0.0);
        for level in ["forest", "crystal", "unbekannt"] {
            let (x, foot, h) = motif_place(&c, level);
            let m = h / (MONARCH_UNITS * c.camera.scale);
            assert!((3.0..=5.0).contains(&m), "{level}: {m} M");
            assert!(x > 0.0 && x < 1920.0);
            assert_eq!(foot, c.ground_y() as f64);
        }
    });
}
