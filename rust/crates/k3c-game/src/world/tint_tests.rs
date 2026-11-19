//! RU55.4 (AC-04): eine Basisfarbe je Phase und Event für Himmel, Ebenen und Figuren; Höhle ohne Himmels-Tönung.

use k3c_data::look::{current, with_profile};
use k3c_sim::state::{ActiveEvent, World as SimWorld};

use super::*;
use crate::world::lightdraw::{LIGHT_COLUMNS, LightMesh};
use crate::world::testutil::{cycle, layer_cell};

fn world(phase: &str, progress: f64, events: &[&str]) -> SimWorld {
    SimWorld {
        cycle: cycle(phase, progress),
        night_events: events
            .iter()
            .map(|id| ActiveEvent {
                id: (*id).to_string(),
                seconds_left: 10.0,
            })
            .collect(),
        ..SimWorld::default()
    }
}

fn close(a: [f64; 3], b: [f64; 3]) -> bool {
    a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-9)
}

#[test]
fn phasen_gehen_stetig_ineinander_ueber() {
    with_profile("k2c", || {
        let t = &current().tint;
        let at = |ph: &str, p: f64| phase_tint(t, &cycle(ph, p));
        assert!(is_neutral(at("day", 0.0)), "Tag bleibt neutral");
        assert!(close(at("dusk", 0.0), at("day", 0.5)), "Dämmerung beginnt am Tag");
        assert!(close(at("dusk", 1.0), at("night", 0.0)), "Dämmerung endet in der Nacht");
        assert!(
            close(at("dawn", 0.0), at("night", 0.0)),
            "Morgengrauen beginnt in der Nacht"
        );
        assert!(close(at("dawn", 1.0), at("day", 0.0)), "Morgengrauen endet am Tag");
        let n = at("night", 0.3);
        assert!(n[2] > n[0], "Nacht ist bläulich: {n:?}");
        let d = at("dusk", 0.5);
        assert!(d[0] > d[2], "Abend ist warm: {d:?}");
    });
}

#[test]
fn events_taenen_nur_nachts() {
    with_profile("k2c", || {
        let t = &current().tint;
        assert!(
            is_neutral(event_tint(t, &world("day", 0.0, &["bloodMoon"]))),
            "am Tag keine Wirkung"
        );
        let blood = event_tint(t, &world("night", 0.5, &["bloodMoon"]));
        assert!(blood[0] > blood[1] * 1.5, "Blutmond rötlich: {blood:?}");
        let full = event_tint(t, &world("night", 0.5, &["fullMoon"]));
        assert!(full[2] > full[0], "Vollmond bläulich: {full:?}");
        assert!(is_neutral(event_tint(t, &world("night", 0.5, &["unbekannt"]))));
    });
}

#[test]
fn hoehle_und_mine_ohne_himmels_toenung_und_dunst() {
    with_profile("k2c", || {
        let w = world("night", 0.5, &["bloodMoon"]);
        assert!(!is_neutral(tint_of("meadow", &w)));
        for b in ["cave", "mine"] {
            assert!(is_neutral(tint_of(b, &w)), "{b} bleibt neutral");
            assert_eq!(haze_of(b, 0), 0.0);
        }
        assert!(
            haze_of("meadow", 0) > haze_of("meadow", 1),
            "ferner Dunst dichter als mittlerer"
        );
        assert_eq!(haze_of("meadow", 2), 0.0, "nah ohne Dunst");
        assert_eq!(haze_of("meadow", 9), 0.0);
    });
    with_profile("go", || {
        let w = world("night", 0.5, &["bloodMoon"]);
        assert!(is_neutral(tint_of("meadow", &w)), "Profil go ohne Tönung");
        assert_eq!(haze_of("meadow", 0), 0.0, "Profil go ohne Dunst");
    });
}

#[test]
fn lichtkarte_traegt_die_toenung_fuer_alles_gleich() {
    with_profile("k2c", || {
        let light = crate::world::light::LightData::default(); // night 1: Grundhelligkeit 1
        let w = world("night", 0.5, &[]);
        let tint = tint_of("meadow", &w);
        let (c, mut m) = (layer_cell(0.0), LightMesh::default());
        m.fill_tinted(&c, &light, &w, tint);
        for i in 0..=LIGHT_COLUMNS {
            let v = m.verts[2 * i].color;
            for k in 0..3 {
                assert!(
                    (f64::from(v[k]) - tint[k]).abs() < 1e-6,
                    "Spalte {i} Kanal {k}: {v:?} gegen {tint:?}"
                );
            }
        }
        m.fill(&c, &light, &w);
        assert_eq!(m.verts[0].color[..3], [1.0, 1.0, 1.0], "fill bleibt neutral");
    });
}
