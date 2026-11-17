//! Port von native/k3c-rig/rig/band_test.go.
//! Die Tests mit Gangart-Posen (`leg_strip_never_folds`, `leg_strip_continuous`) stehen in `band_leg_tests.rs`.

use super::band::{BAND_SUB, Vertex, area, tangent_limits, unfold};
use super::curve::{catmull, hermite};
use super::load::Band;
use super::matrix::Point;
use super::testutil::{load_rig, rig_dirs};

/// restPoints: Stützpunkte eines Bands in Ruhelage (senkrecht nach unten auf der Mittellinie).
fn rest_points(b: &Band) -> Vec<Point> {
    b.rest.iter().map(|&y| [b.center, y]).collect()
}

/// foldEps: numerische Null der doppelten Dreiecksfläche (Rig-Einheiten²); unfold legt Ecken per Lot genau auf die
/// Linie, gerundet bleibt ein Rest um 1e-15.
const FOLD_EPS: f64 = 1e-9;

/// `fold_at` liefert den ersten Querschnitt, an dem das Band umklappt, oder -1.
pub(super) fn fold_at(verts: &[Vertex]) -> i64 {
    let mut i = 0;
    while i + 3 < verts.len() {
        // Viereck aus zwei Querschnitten: (links, rechts) oben und unten
        if area(&verts[i], &verts[i + 1], &verts[i + 2]) < -FOLD_EPS
            || area(&verts[i + 1], &verts[i + 3], &verts[i + 2]) < -FOLD_EPS
        {
            return (i / 2) as i64;
        }
        i += 2;
    }
    -1
}

fn vx(x: f64, y: f64) -> Vertex {
    Vertex { x, y, u: 0.0, v: 0.0 }
}

/// In Ruhelage bildet das Band sein Bild unverzerrt ab: Ort in Einheiten · Dichte = Stelle im Bild.
#[test]
fn strip_rest_is_identity() {
    for dir in rig_dirs() {
        let r = load_rig(&dir);
        for b in &r.bands {
            let mut verts = Vec::new();
            b.strip(&rest_points(b), r.density, &mut verts);
            assert_eq!(
                verts.len() as i64,
                2 * ((b.rest.len() as i64 - 1) * BAND_SUB + 3),
                "{}: {} Ecken",
                b.id,
                verts.len()
            );
            for v in &verts {
                let (u, w) = (
                    (v.x - b.image.offset[0]) * r.density,
                    (v.y - b.image.offset[1]) * r.density,
                );
                assert!(
                    (u - v.u).abs() <= 0.01 && (w - v.v).abs() <= 0.01,
                    "{}: Ecke ({:.2}, {:.2}) zeigt auf ({:.2}, {:.2}) statt ({u:.2}, {w:.2})",
                    b.id,
                    v.x,
                    v.y,
                    v.u,
                    v.v
                );
                let size = b.image.size;
                assert!(
                    v.u >= 0.0 && v.u <= size[0] as f64 && v.v >= 0.0 && v.v <= size[1] as f64,
                    "{}: Stelle ({:.2}, {:.2}) liegt außerhalb des Bilds {size:?}",
                    b.id,
                    v.u,
                    v.v
                );
            }
        }
    }
}

/// Mit Faktor 1 ist hermite genau Catmull-Rom; ein kleiner Faktor hält die Kurve im Abschnitt.
#[test]
fn hermite_limits_overshoot() {
    for tt in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let (a, b) = (
            hermite(1.0, 2.0, 5.0, 9.0, tt, 1.0, 1.0),
            catmull(1.0, 2.0, 5.0, 9.0, tt),
        );
        assert!((a - b).abs() <= 1e-12, "t={tt:.2}: hermite {a:.4}, catmull {b:.4}");
    }
    // langer Abschnitt davor (−11), kurzer Abschnitt 0 … 1,2: ungebremst schießt die Kurve über 1,2 hinaus
    let v = catmull(-11.0, 0.0, 1.2, 1.2, 0.75);
    assert!(v > 1.2, "Testfall schießt nicht über: {v:.3}");
    let mut lim = [0.0; 4];
    tangent_limits(&[[0.0, -11.0], [0.0, 0.0], [0.0, 1.2], [0.0, 1.2]], &mut lim);
    for k in 0..9 {
        let v = hermite(-11.0, 0.0, 1.2, 1.2, f64::from(k) / 8.0, lim[1], lim[2]);
        assert!(
            (0.0..=1.2 + 1e-9).contains(&v),
            "t={k}/8: {v:.3} liegt außerhalb 0 … 1,2"
        );
    }
}

/// unfold hält eine rückwärts laufende Kante an: Danach hat kein Dreieck eine negative Fläche.
#[test]
fn unfold_stops_backward_edge() {
    let mut v = [
        vx(0.0, 0.0),
        vx(4.0, 0.0),
        vx(0.0, 2.0),
        vx(4.0, -1.0),
        vx(0.0, 4.0),
        vx(4.0, 4.0),
    ];
    assert!(fold_at(&v) >= 0, "Testfall faltet nicht");
    unfold(&mut v);
    let i = fold_at(&v);
    assert!(i < 0, "faltet weiter bei {i}: {v:?}");
    let a = area(&v[1], &v[3], &v[2]); // per Lot auf die Diagonale: Dreieck wird zur Linie
    assert!(
        a.abs() <= FOLD_EPS,
        "rechte Kante nicht auf der Diagonale angehalten: {:?} (Fläche {a})",
        v[3]
    );
}

/// unfold bewegt eine Ecke stetig: Wandert die Ecke eines Querschnitts in kleinen Schritten über die Linie des vorigen
/// Querschnitts, springt sie nicht (vorher: Sprung auf die Ecke davor, B-791-Review).
#[test]
fn unfold_is_continuous() {
    let mut last = Vertex::default();
    for k in 0..=40 {
        let y = 0.01 - f64::from(k) * 0.0005; // von +0,01 (ungefaltet) bis -0,01 (gefaltet)
        let mut v = [vx(0.0, 0.0), vx(1.0, 0.0), vx(0.5, y), vx(1.0, 1.0)];
        unfold(&mut v);
        let a = area(&v[0], &v[1], &v[2]);
        assert!(a >= -FOLD_EPS, "y={y:.4}: Dreieck bleibt gefaltet ({a})");
        if k > 0 {
            assert!(
                (v[2].x - last.x).hypot(v[2].y - last.y) <= 0.001,
                "y={y:.4}: Ecke springt von ({:.4}, {:.4}) nach ({:.4}, {:.4})",
                last.x,
                last.y,
                v[2].x,
                v[2].y
            );
        }
        last = v[2];
    }
}
