//! Port von native/k3c-rig/rig/matrix.go: Punkt und 2D-Affin-Matrix.
//! Papierschnitt-Figuren rechnen zur Laufzeit aus Teilen: Teile-Baum, Posen, Gangarten, Umhang, Bänder.
//! Formeln und Konstanten wie tools/paper (render/, rig/); ohne Zeichen-Stack.

/// Point ist ein Punkt in Einheiten der Figur (y wächst nach unten, Füße bei y = 0).
pub type Point = [f64; 2];

/// M ist eine 2D-Affin-Matrix im SVG-Format matrix(a b c d e f): x' = a·x + c·y + e, y' = b·x + d·y + f.
pub type M = [f64; 6];

/// IDENTITY ändert nichts.
pub const IDENTITY: M = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

/// `mul` liefert p · q: erst q, dann p anwenden.
pub fn mul(p: M, q: M) -> M {
    [
        p[0] * q[0] + p[2] * q[1],
        p[1] * q[0] + p[3] * q[1],
        p[0] * q[2] + p[2] * q[3],
        p[1] * q[2] + p[3] * q[3],
        p[0] * q[4] + p[2] * q[5] + p[4],
        p[1] * q[4] + p[3] * q[5] + p[5],
    ]
}

pub fn translate(x: f64, y: f64) -> M {
    [1.0, 0.0, 0.0, 1.0, x, y]
}

pub fn scale(s: f64) -> M {
    [s, 0.0, 0.0, s, 0.0, 0.0]
}

/// `rotation` dreht um (px, py); Winkel in Grad, positiv im Uhrzeigersinn (wie SVG rotate). 0° ist genau die Einheit.
pub fn rotation(deg: f64, px: f64, py: f64) -> M {
    if deg == 0.0 {
        return IDENTITY;
    }
    let r = deg * std::f64::consts::PI / 180.0;
    let (c, s) = (r.cos(), r.sin());
    [c, s, -s, c, px - c * px + s * py, py - s * px - c * py]
}

/// `apply` bildet einen Punkt ab.
pub fn apply(m: &M, p: Point) -> Point {
    [m[0] * p[0] + m[2] * p[1] + m[4], m[1] * p[0] + m[3] * p[1] + m[5]]
}

/// `invert` liefert die Inverse einer invertierbaren Matrix.
pub fn invert(m: &M) -> M {
    let det = m[0] * m[3] - m[1] * m[2];
    let (a, b, c, d) = (m[3] / det, -m[1] / det, -m[2] / det, m[0] / det);
    [a, b, c, d, -(a * m[4] + c * m[5]), -(b * m[4] + d * m[5])]
}
