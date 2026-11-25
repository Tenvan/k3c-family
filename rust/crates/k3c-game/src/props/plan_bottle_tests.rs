//! Tests zu `plan_bottle.rs` (GS4.4): gezeichnet nur, wenn die Sim die Flasche liegen hat; Glitzer pulsiert mit der Uhr.

use k3c_sim::state::World;

use crate::props::plan::{Item, Planner, Shape};
use crate::view::cell::Camera;

/// Stufe der Tiefe 0 mit einer liegenden Flasche bei `x` (oder ohne Flasche bei `None`).
fn stufe(ashore: Option<f64>) -> World {
    let mut w = World::default();
    w.biome.depth = 0;
    w.bottle.ashore = ashore;
    w
}

/// `plan_bottle` zeichnet die Flasche der Welt `w` bei Uhr `clock` ohne Grafik und gibt die Zeichenliste zurück.
fn plan_bottle(w: &World, k: &Camera, clock: f64) -> Vec<Item> {
    let mut p = Planner::new(None);
    p.bottle(w, k, clock);
    p.body
}

#[test]
fn ohne_flasche_nichts_zeichnen() {
    assert!(plan_bottle(&stufe(None), &Camera::default(), 0.0).is_empty());
}

#[test]
fn liegende_flasche_mit_glitzer() {
    let items = plan_bottle(&stufe(Some(0.0)), &Camera::default(), 0.0);
    // ohne Grafik: eine Glasfläche und drei Funken
    assert_eq!(
        items.iter().filter(|it| it.shape == Shape::Rect).count(),
        1,
        "Glasfläche"
    );
    assert_eq!(
        items.iter().filter(|it| it.shape == Shape::Circle).count(),
        3,
        "drei Funken"
    );
}

#[test]
fn glitzer_pulsiert_mit_der_uhr() {
    let w = stufe(Some(0.0));
    let frueh: Vec<f32> = plan_bottle(&w, &Camera::default(), 0.0)
        .iter()
        .filter(|it| it.shape == Shape::Circle)
        .map(|it| it.alpha)
        .collect();
    let spaet: Vec<f32> = plan_bottle(&w, &Camera::default(), 1.3)
        .iter()
        .filter(|it| it.shape == Shape::Circle)
        .map(|it| it.alpha)
        .collect();
    assert_ne!(frueh, spaet, "Deckkraft der Funken ändert sich mit der Zeit");
}

#[test]
fn unter_tage_nichts_zeichnen() {
    let mut w = stufe(Some(0.0));
    w.biome.depth = 1;
    assert!(
        plan_bottle(&w, &Camera::default(), 0.0).is_empty(),
        "nur Stufe der Tiefe 0"
    );
}

#[test]
fn ausserhalb_des_bildes_nichts_zeichnen() {
    // Kamera weit links, Flasche bei x = 0: nicht im Bild
    assert!(plan_bottle(&stufe(Some(0.0)), &crate::props::testutil::camera(-10000.0), 0.0).is_empty());
    // gleiche Flasche, Kamera davor: sichtbar
    assert!(!plan_bottle(&stufe(Some(0.0)), &crate::props::testutil::camera(0.0), 0.0).is_empty());
}

/// GS4.7: Das Leuchten am Ziel besteht aus drei Scheiben im Bild, pulsiert mit der Uhr und fehlt außerhalb des Bildes.
#[test]
fn leuchten_am_ziel_pulsiert_und_fehlt_ausserhalb() {
    let glow = |x: f64, clock: f64| {
        let mut p = Planner::new(None);
        p.bottle_glow(x, &crate::props::testutil::camera(0.0), clock);
        p.body
    };
    let a = glow(0.0, 0.0);
    let b = glow(0.0, 0.8);
    assert_eq!(a.len(), 3, "drei Scheiben");
    assert!(
        a.iter().zip(&b).any(|(p, q)| p.alpha != q.alpha),
        "pulsiert mit der Uhr"
    );
    assert!(glow(5000.0, 0.0).is_empty(), "außerhalb des Bildes nichts");
}
