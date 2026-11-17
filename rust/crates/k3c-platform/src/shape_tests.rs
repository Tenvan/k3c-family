use std::f32::consts::PI;

use super::*;

fn near(a: f32, b: f32, tol: f32) -> bool {
    (a - b).abs() <= tol * b.abs().max(1.0)
}

#[test]
fn kreis_ist_ein_faecher() {
    let mut t = Tris::default();
    t.circle(100.0, 50.0, 10.0);
    assert_eq!(t.idx.len() as u32, 3 * SEGMENTS);
    assert_eq!(t.pos.len() as u32, SEGMENTS + 1);
    assert!(near(t.area(), PI * 100.0, 0.01), "Fläche {}", t.area());
}

#[test]
fn ring_hat_zwei_dreiecke_je_segment() {
    let mut t = Tris::default();
    t.ring(0.0, 0.0, 10.0, 2.0);
    assert_eq!(t.idx.len() as u32, 6 * SEGMENTS);
    let want = PI * (11.0f32.powi(2) - 9.0f32.powi(2));
    assert!(near(t.area(), want, 0.01), "Fläche {} statt {want}", t.area());
}

#[test]
fn konvexes_polygon_beide_umlaufrichtungen() {
    let sq = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    for pts in [sq, [sq[3], sq[2], sq[1], sq[0]]] {
        let mut t = Tris::default();
        assert!(t.polygon(&pts));
        assert_eq!(t.idx.len(), 6, "zwei Dreiecke");
        assert!(near(t.area(), 100.0, 1e-4));
    }
}

#[test]
fn konkaves_polygon_bleibt_in_der_flaeche() {
    // L-Form: 3×3 minus Ecke 2×2 oben rechts → Fläche 5.
    let l = [[0.0, 0.0], [1.0, 0.0], [1.0, 2.0], [3.0, 2.0], [3.0, 3.0], [0.0, 3.0]];
    let mut t = Tris::default();
    assert!(t.polygon(&l));
    assert_eq!(t.idx.len(), 3 * (l.len() - 2));
    assert!(near(t.area(), 5.0, 1e-4), "Fläche {}", t.area());
    assert!(near(signed_area(&l).abs(), 5.0, 1e-4));
}

#[test]
fn stern_mit_spitzen() {
    let star: Vec<[f32; 2]> = (0..10)
        .map(|i| {
            let r = if i % 2 == 0 { 10.0 } else { 4.0 };
            let a = i as f32 * PI / 5.0;
            [r * a.cos(), r * a.sin()]
        })
        .collect();
    let mut t = Tris::default();
    assert!(t.polygon(&star));
    assert_eq!(t.idx.len(), 3 * 8);
    assert!(near(t.area(), signed_area(&star).abs(), 1e-3));
}

#[test]
fn zu_wenige_punkte_und_puffer_leeren() {
    let mut t = Tris::default();
    assert!(!t.polygon(&[[0.0, 0.0], [1.0, 1.0]]));
    assert!(t.polygon(&[]));
    t.circle(0.0, 0.0, 1.0);
    t.clear();
    assert!(t.pos.is_empty() && t.idx.is_empty());
}

#[test]
fn geschlossener_pfad_wie_offener() {
    let mut t = Tris::default();
    assert!(t.polygon(&[[0.0, 0.0], [4.0, 0.0], [4.0, 4.0], [0.0, 4.0], [0.0, 0.0]]));
    assert_eq!(t.idx.len(), 6);
    assert!(near(t.area(), 16.0, 1e-4));
}
