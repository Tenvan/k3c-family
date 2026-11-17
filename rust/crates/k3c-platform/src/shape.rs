//! Formen → Dreiecke (wie `vector.*` bei Ebitengine): Kreis und Ring mit fester Segmentzahl, Polygon per
//! Ohren-Schnitt. Reiner Code ohne GPU; die Puffer werden wiederverwendet (keine Allokation im Leerlauf).

/// Segmente je Kreis.
pub const SEGMENTS: u32 = 48;

/// Wiederverwendbarer Dreiecks-Puffer: Punkte und Indizes (je drei ein Dreieck).
#[derive(Default, Debug, Clone)]
pub struct Tris {
    pub pos: Vec<[f32; 2]>,
    pub idx: Vec<u32>,
    ring: Vec<u32>,
}

impl Tris {
    pub fn clear(&mut self) {
        self.pos.clear();
        self.idx.clear();
    }

    /// Gefüllter Kreis als Fächer um den Mittelpunkt.
    pub fn circle(&mut self, cx: f32, cy: f32, r: f32) {
        let base = self.pos.len() as u32;
        self.pos.push([cx, cy]);
        for i in 0..SEGMENTS {
            self.pos.push(on_circle(cx, cy, r, i));
        }
        for i in 0..SEGMENTS {
            let (p, q) = (base + 1 + i, base + 1 + (i + 1) % SEGMENTS);
            self.idx.extend_from_slice(&[base, p, q]);
        }
    }

    /// Kreislinie der Breite `w`, mittig auf dem Radius (wie `vector.StrokeCircle`).
    pub fn ring(&mut self, cx: f32, cy: f32, r: f32, w: f32) {
        let base = self.pos.len() as u32;
        let (inner, outer) = ((r - w / 2.0).max(0.0), r + w / 2.0);
        for i in 0..SEGMENTS {
            self.pos.push(on_circle(cx, cy, outer, i));
            self.pos.push(on_circle(cx, cy, inner, i));
        }
        for i in 0..SEGMENTS {
            let j = (i + 1) % SEGMENTS;
            let (o0, i0, o1, i1) = (base + 2 * i, base + 2 * i + 1, base + 2 * j, base + 2 * j + 1);
            self.idx.extend_from_slice(&[o0, o1, i0, i0, o1, i1]);
        }
    }

    /// Gefülltes einfaches Polygon (konvex oder konkav, ohne Selbstschnitt) per Ohren-Schnitt.
    /// Liefert `false`, wenn kein Ohr mehr gefunden wird (entartetes Polygon); Bisheriges bleibt.
    pub fn polygon(&mut self, pts: &[[f32; 2]]) -> bool {
        // Geschlossener Pfad (letzter Punkt = erster) wie ein offener.
        let pts = match pts {
            [first, .., last] if pts.len() > 3 && first == last => &pts[..pts.len() - 1],
            _ => pts,
        };
        if pts.len() < 3 {
            return pts.is_empty();
        }
        let base = self.pos.len() as u32;
        self.pos.extend_from_slice(pts);
        let ccw = signed_area(pts) > 0.0;
        self.ring.clear();
        self.ring.extend(0..pts.len() as u32);
        let mut misses = 0;
        let mut i = 0;
        while self.ring.len() > 3 {
            let n = self.ring.len();
            let (a, b, c) = (self.ring[(i + n - 1) % n], self.ring[i % n], self.ring[(i + 1) % n]);
            if is_ear(pts, &self.ring, a, b, c, ccw) {
                self.idx.extend_from_slice(&[base + a, base + b, base + c]);
                self.ring.remove(i % n);
                misses = 0;
            } else {
                i += 1;
                misses += 1;
                if misses > n {
                    return false;
                }
            }
        }
        let r = &self.ring;
        self.idx.extend_from_slice(&[base + r[0], base + r[1], base + r[2]]);
        true
    }

    /// Summe der Dreiecksflächen (für Tests).
    #[cfg(test)]
    pub fn area(&self) -> f32 {
        self.idx
            .as_chunks::<3>()
            .0
            .iter()
            .map(|t| {
                let p = |k: usize| self.pos[t[k] as usize];
                signed_area(&[p(0), p(1), p(2)]).abs()
            })
            .sum()
    }
}

fn on_circle(cx: f32, cy: f32, r: f32, i: u32) -> [f32; 2] {
    let t = i as f32 / SEGMENTS as f32 * std::f32::consts::TAU;
    [cx + r * t.cos(), cy + r * t.sin()]
}

/// Vorzeichenbehaftete Fläche (Schnürsenkel-Formel); positiv bei mathematisch positivem Umlauf.
pub fn signed_area(pts: &[[f32; 2]]) -> f32 {
    let n = pts.len();
    (0..n)
        .map(|i| {
            let (p, q) = (pts[i], pts[(i + 1) % n]);
            p[0] * q[1] - q[0] * p[1]
        })
        .sum::<f32>()
        / 2.0
}

fn cross(o: [f32; 2], a: [f32; 2], b: [f32; 2]) -> f32 {
    (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
}

/// `b` ist ein Ohr: Ecke konvex im Umlaufsinn und kein anderer Restpunkt im Dreieck.
fn is_ear(pts: &[[f32; 2]], ring: &[u32], a: u32, b: u32, c: u32, ccw: bool) -> bool {
    let (pa, pb, pc) = (pts[a as usize], pts[b as usize], pts[c as usize]);
    let turn = cross(pa, pb, pc);
    if (ccw && turn <= 0.0) || (!ccw && turn >= 0.0) {
        return false;
    }
    ring.iter().filter(|&&k| k != a && k != b && k != c).all(|&k| {
        let p = pts[k as usize];
        let (d1, d2, d3) = (cross(pa, pb, p), cross(pb, pc, p), cross(pc, pa, p));
        let inside = if ccw {
            d1 >= 0.0 && d2 >= 0.0 && d3 >= 0.0
        } else {
            d1 <= 0.0 && d2 <= 0.0 && d3 <= 0.0
        };
        !inside
    })
}

#[cfg(test)]
#[path = "shape_tests.rs"]
mod tests;
