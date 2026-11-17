//! Port von native/k3c-rig/rig/curve.go: Catmull-Rom, Hermite, Spline und Zwei-Knochen-IK.

use super::matrix::Point;

/// `catmull`: Catmull-Rom-Wert zwischen p1 und p2 bei t = 0..1 (Nachbarn p0, p3), wie tools/paper/rig/kurve.ts.
pub fn catmull(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
    0.5 * (2.0 * p1
        + (p2 - p0) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t * t
        + (3.0 * p1 - p0 - 3.0 * p2 + p3) * t * t * t)
}

/// `hermite`: Catmull-Rom zwischen p1 und p2 bei t = 0..1 mit den Tangenten (p2 − p0)/2 und (p3 − p1)/2, gekürzt um die
/// Faktoren f1 und f2; f1 = f2 = 1 ergibt genau catmull.
pub fn hermite(p0: f64, p1: f64, p2: f64, p3: f64, t: f64, f1: f64, f2: f64) -> f64 {
    let (t2, t3) = (t * t, t * t * t);
    let (h00, h01) = (2.0 * t3 - 3.0 * t2 + 1.0, -2.0 * t3 + 3.0 * t2);
    let (h10, h11) = (t3 - 2.0 * t2 + t, t3 - t2);
    h00 * p1 + h01 * p2 + h10 * f1 * (p2 - p0) / 2.0 + h11 * f2 * (p3 - p1) / 2.0
}

/// `spline`: Punkt auf der Catmull-Rom-Kurve durch alle Stützpunkte, k = 0..1.
pub fn spline(pts: &[Point], k: f64) -> Point {
    let n = pts.len() as i64 - 1;
    let s = k.min(0.9999) * n as f64;
    let i = s.floor() as i64;
    let t = s - i as f64;
    let at = |j: i64| pts[j as usize];
    let (p0, p1, p2, p3) = (at((i - 1).max(0)), at(i), at(i + 1), at((i + 2).min(n)));
    [
        catmull(p0[0], p1[0], p2[0], p3[0], t),
        catmull(p0[1], p1[1], p2[1], p3[1], t),
    ]
}

/// `ik`: Zwei-Knochen-IK des Vierbeiners (tools/paper/rig/ik.ts): Winkel [Oberbein, Unterbein relativ] in Grad für ein
/// Ziel (dx, dy) ab Hüfte; 0 = Bein hängt senkrecht, knee_front = Vorderbein (Knie zeigt nach vorn).
pub fn ik(dx: f64, dy: f64, knee_front: bool, l1: f64, l2: f64) -> (f64, f64) {
    const DEG: f64 = 180.0 / std::f64::consts::PI;
    let d = (l1 + l2 - 0.01).min(((l1 - l2).abs() + 0.01).max(dx.hypot(dy)));
    let at = (-dx).atan2(dy);
    let al = ((l1 * l1 + d * d - l2 * l2) / (2.0 * l1 * d)).acos();
    let bend = std::f64::consts::PI - ((l1 * l1 + l2 * l2 - d * d) / (2.0 * l1 * l2)).acos();
    if knee_front {
        return ((at - al) * DEG, bend * DEG);
    }
    ((at + al) * DEG, -bend * DEG)
}
