//! Port von native/k3c-rig/rig/band.go: Bänder (Bein, Umhang) entlang von Stützpunkten biegen.

use super::curve::hermite;
use super::load::{Band, Rig};
use super::matrix::Point;

/// Vertex ist eine Ecke eines Bands: Ort in Einheiten und Stelle im Bild in Pixeln.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub u: f64,
    pub v: f64,
}

/// Station ist ein Querschnitt des Bands: linke und rechte Kante (Ort) und ihre Stelle im Bild (Pixel).
#[derive(Debug, Clone, Copy, Default)]
struct Station {
    l: Point,
    r: Point,
    ul: f64,
    ur: f64,
    v: f64,
}

/// MAX_STATIONS begrenzt die Stützpunkte eines Bands (Umhang 8, Bein 7); mehr werden abgeschnitten.
const MAX_STATIONS: usize = 16;

/// BAND_SUB ist die Zahl der Zwischenschritte je Abschnitt zwischen zwei Stützpunkten (Catmull-Rom).
pub const BAND_SUB: i64 = 4;

/// BAND_INSET hält die Ecken einen Pixel vom Bildrand fern, damit der lineare Filter nichts von außen liest.
const BAND_INSET: f64 = 1.0;

impl Rig {
    /// `leg_points` liefert die Stützpunkte eines Beins im Rahmen des Rumpf-Teils (tools/paper/rig/beinpfad.ts ›
    /// beinPunkte): Ansatz 16 über der Hüfte, Hüfte, Oberbein-Mitte, Knie, Unterbein-Mitte, Fessel und die Spitze des Fußes.
    pub fn leg_points(&self, leg: usize, angles: &[f64], out: &mut Vec<Point>) {
        let x = &self.legs;
        let part = |i: i64| &self.parts[i as usize];
        let hip = part(x.upper[leg]).pivot;
        let (knee_y, fetlock_y) = (part(x.lower[leg]).pivot[1], part(x.hoof[leg]).pivot[1]);
        let a1 = angles[x.upper[leg] as usize];
        let (d1, d2) = (direction(a1), direction(a1 + angles[x.lower[leg] as usize]));
        let (upper, lower) = (knee_y - hip[1], fetlock_y - knee_y);
        let knee = along(hip, d1, upper);
        let fetlock = along(knee, d2, lower);
        let tip = self.leg.as_ref().map_or(0.0, |l| l.tip);
        out.clear();
        out.extend([
            [hip[0], hip[1] - 16.0],
            hip,
            along(hip, d1, upper * 0.48),
            knee,
            along(knee, d2, lower * 0.5),
            fetlock,
            along(fetlock, d2, tip),
        ]);
    }
}

/// `direction`: Richtung eines hängenden Knochens bei Winkel deg (0 = senkrecht nach unten).
pub fn direction(deg: f64) -> Point {
    let rad = deg * std::f64::consts::PI / 180.0;
    [-rad.sin(), rad.cos()]
}

pub fn along(p: Point, d: Point, l: f64) -> Point {
    [p[0] + d[0] * l, p[1] + d[1] * l]
}

impl Band {
    /// `strip` biegt das Bild des Bands entlang der Stützpunkte pts (einer je Rest-Wert) und liefert die Ecken paarweise
    /// (links, rechts) von oben nach unten. Je Stützpunkt liegen die Kanten Width neben der Mittellinie (die Papierkante
    /// samt Rand für den Schatten), nicht am Bildrand: So faltet sich das Band an einem stark gebeugten Knie nicht. Die
    /// Kanten laufen als Catmull-Rom-Kurve durch die Stützpunkte, wie die Umrisse in tools/paper; oben und unten
    /// verlängert der Rand des Bilds das Band gerade. density = Pixel je Einheit. In Ruhelage ergibt das genau das Bild.
    pub fn strip(&self, pts: &[Point], density: f64, out: &mut Vec<Vertex>) {
        out.clear();
        let n = pts.len().min(self.rest.len()).min(self.width.len()).min(MAX_STATIONS);
        if n < 2 {
            return;
        }
        let mut st = [Station::default(); MAX_STATIONS];
        let pts = &pts[..n];
        let (width, height) = (self.image.size[0] as f64, self.image.size[1] as f64);
        let offset = self.image.offset;
        // größter Abstand von der Mittellinie, der noch im Bild liegt
        let max_l = self.center - offset[0] - BAND_INSET / density;
        let max_r = offset[0] + width / density - self.center - BAND_INSET / density;
        for (i, p) in pts.iter().enumerate() {
            let (nx, ny) = normal(pts[i.saturating_sub(1)], pts[(i + 1).min(n - 1)]);
            let (l, r) = (self.width[i][0].min(max_l), self.width[i][1].min(max_r));
            st[i] = Station {
                l: [p[0] - nx * l, p[1] - ny * l],
                r: [p[0] + nx * r, p[1] + ny * r],
                ul: (self.center - l - offset[0]) * density,
                ur: (self.center + r - offset[0]) * density,
                v: (self.rest[i] - offset[1]) * density,
            };
        }
        let mut lim = [0.0; MAX_STATIONS];
        tangent_limits(pts, &mut lim[..n]);
        emit(
            out,
            &extend(&st[0], pts[0], pts[1], -(st[0].v - BAND_INSET) / density, BAND_INSET),
        );
        for i in 0..n - 1 {
            let (s0, s1, s2, s3) = (&st[i.saturating_sub(1)], &st[i], &st[i + 1], &st[(i + 2).min(n - 1)]);
            for k in 0..BAND_SUB {
                emit(
                    out,
                    &between(s0, s1, s2, s3, k as f64 / BAND_SUB as f64, lim[i], lim[i + 1]),
                );
            }
        }
        emit(out, &st[n - 1]);
        let to_v = (height - BAND_INSET - st[n - 1].v) / density;
        emit(
            out,
            &extend(&st[n - 1], pts[n - 2], pts[n - 1], to_v, height - BAND_INSET),
        );
        unfold(out);
    }
}

/// TANGENT_REACH: höchstens so viele Längen des kürzeren anliegenden Abschnitts darf die Tangente eines Stützpunkts
/// lang sein (Fritsch-Carlson: darüber schießt die Hermite-Kurve über das Abschnittsende hinaus).
const TANGENT_REACH: f64 = 3.0;

/// `tangent_limits` schreibt je Stützpunkt den Faktor (0..1) auf die Catmull-Rom-Tangente: Folgt auf einen langen
/// Abschnitt ein kurzer (Elefant: 11 Einheiten Unterbein, 1,2 Fußspitze), schießt die Kurve sonst über das Band-Ende
/// hinaus (B-791). Ein Faktor je Stützpunkt für alle Größen hält die Kurve glatt und die Ruhelage unverzerrt.
pub fn tangent_limits(pts: &[Point], out: &mut [f64]) {
    let n = out.len();
    for i in 0..n {
        let (p0, p2) = (pts[i.saturating_sub(1)], pts[(i + 1).min(n - 1)]);
        let m = (p2[0] - p0[0]).hypot(p2[1] - p0[1]) / 2.0;
        let mut chord = f64::INFINITY;
        if i > 0 {
            chord = dist(pts[i - 1], pts[i]);
        }
        if i < n - 1 {
            chord = chord.min(dist(pts[i], pts[i + 1]));
        }
        out[i] = 1.0;
        if m > TANGENT_REACH * chord {
            out[i] = TANGENT_REACH * chord / m;
        }
    }
}

fn dist(a: Point, b: Point) -> f64 {
    (b[0] - a[0]).hypot(b[1] - a[1])
}

/// `unfold` hält eine Kante an, wo sie gegen die Laufrichtung des Bands zurückliefe: An einer Beugung, deren Radius
/// kleiner ist als die Breite (Hüfte des Wolfs im Galopp), läge die innere Kante sonst rückwärts und das Dreieck
/// klappte um. Die Ecke kommt per Lot auf die Linie des vorigen Querschnitts bzw. der vorigen Diagonale: Das Dreieck
/// wird zur Linie (Fläche 0), das Bild staucht sich in der Ecke, und weil das Lot eines Punkts auf der Linie er selbst
/// ist, bewegt sich die Kante beim Vorzeichenwechsel stetig (kein Sprung zwischen zwei Bildern).
pub fn unfold(v: &mut [Vertex]) {
    let mut i = 0;
    while i + 3 < v.len() {
        if area(&v[i], &v[i + 1], &v[i + 2]) < 0.0 {
            (v[i + 2].x, v[i + 2].y) = foot(&v[i], &v[i + 1], &v[i + 2]);
        }
        if area(&v[i + 1], &v[i + 3], &v[i + 2]) < 0.0 {
            (v[i + 3].x, v[i + 3].y) = foot(&v[i + 1], &v[i + 2], &v[i + 3]);
        }
        i += 2;
    }
}

/// `foot` ist der Fußpunkt des Lots von p auf die Gerade durch a und b (a, wenn a und b zusammenfallen).
fn foot(a: &Vertex, b: &Vertex, p: &Vertex) -> (f64, f64) {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let d = dx * dx + dy * dy;
    if d == 0.0 {
        return (a.x, a.y);
    }
    let t = ((p.x - a.x) * dx + (p.y - a.y) * dy) / d;
    (a.x + t * dx, a.y + t * dy)
}

/// `area`: doppelte Fläche des Dreiecks mit Vorzeichen (Umlaufsinn; positiv = Band läuft ungefaltet).
pub fn area(a: &Vertex, b: &Vertex, c: &Vertex) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)
}

/// `normal`: Einheits-Normale zur Richtung a → b (zeigt nach rechts, wenn die Richtung nach unten läuft).
fn normal(a: Point, b: Point) -> (f64, f64) {
    let (tx, ty) = (b[0] - a[0], b[1] - a[1]);
    let mut d = tx.hypot(ty);
    if d == 0.0 {
        d = 1.0;
    }
    (ty / d, -tx / d)
}

/// `extend` verschiebt einen Querschnitt um dist entlang der Richtung a → b (negativ = zurück) auf die Bildhöhe v.
fn extend(s: &Station, a: Point, b: Point, dist: f64, v: f64) -> Station {
    let (tx, ty) = (b[0] - a[0], b[1] - a[1]);
    let mut d = tx.hypot(ty);
    if d == 0.0 {
        d = 1.0;
    }
    let (dx, dy) = (tx / d * dist, ty / d * dist);
    Station {
        l: [s.l[0] + dx, s.l[1] + dy],
        r: [s.r[0] + dx, s.r[1] + dy],
        v,
        ..*s
    }
}

/// `between`: Querschnitt zwischen s1 und s2 bei t = 0..1; Kanten und Stellen im Bild mit derselben Kurve, die
/// Tangenten in s1 und s2 mit den Faktoren f1 und f2 (tangent_limits; 1 = Catmull-Rom).
#[allow(clippy::many_single_char_names)]
fn between(s0: &Station, s1: &Station, s2: &Station, s3: &Station, t: f64, f1: f64, f2: f64) -> Station {
    if t == 0.0 {
        return *s1;
    }
    let c = |p0: f64, p1: f64, p2: f64, p3: f64| hermite(p0, p1, p2, p3, t, f1, f2);
    Station {
        l: [
            c(s0.l[0], s1.l[0], s2.l[0], s3.l[0]),
            c(s0.l[1], s1.l[1], s2.l[1], s3.l[1]),
        ],
        r: [
            c(s0.r[0], s1.r[0], s2.r[0], s3.r[0]),
            c(s0.r[1], s1.r[1], s2.r[1], s3.r[1]),
        ],
        ul: c(s0.ul, s1.ul, s2.ul, s3.ul),
        ur: c(s0.ur, s1.ur, s2.ur, s3.ur),
        v: c(s0.v, s1.v, s2.v, s3.v),
    }
}

fn emit(out: &mut Vec<Vertex>, s: &Station) {
    out.push(Vertex {
        x: s.l[0],
        y: s.l[1],
        u: s.ul,
        v: s.v,
    });
    out.push(Vertex {
        x: s.r[0],
        y: s.r[1],
        u: s.ur,
        v: s.v,
    });
}
