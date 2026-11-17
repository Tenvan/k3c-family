//! Port von native/k3c-rig/rig/cloak.go: Umhang als Verlet-Kette.

use super::load::CloakSpec;
use super::matrix::Point;

/// CLOAK_DT ist der feste Zeitschritt der Umhang-Physik (tools/paper/rig/umhang.ts).
pub const CLOAK_DT: f64 = 1.0 / 120.0;

// Einschwingen vor Frame 0 und Schritte je Sekunde (tools/paper/rig/animation.ts).
const CLOAK_SETTLE: i64 = 240;
const STEPS_PER_SEC: f64 = 120.0;
const CLOAK_BINDING: usize = 8;

#[derive(Debug, Clone, Copy, Default)]
struct CloakPoint {
    x: f64,
    y: f64,
    px: f64,
    py: f64,
}

/// Cloak ist der Umhang als Verlet-Kette: oben am Anker, Schwerkraft und Fahrtwind, fester Zeitschritt 1/120 s.
#[derive(Debug, Clone)]
pub struct Cloak {
    pts: Vec<CloakPoint>,
    seg: f64,
    time: f64,
}

impl Cloak {
    /// `new` hängt die Kette am Anker auf, leicht nach hinten geneigt.
    pub fn new(u: &CloakSpec, anchor: Point) -> Cloak {
        let n = if u.segments == 0 { 8 } else { u.segments };
        let seg = u.length / (n - 1) as f64;
        let pts = (0..n)
            .map(|i| {
                let (x, y) = (anchor[0] - i as f64 * 1.5, anchor[1] + i as f64 * seg);
                CloakPoint { x, y, px: x, py: y }
            })
            .collect();
        Cloak { pts, seg, time: 0.0 }
    }

    /// `step` rechnet einen Schritt: wind = Fahrtwind (Einheiten/s, 0 = Stand mit Böe), floor = tiefster erlaubter y-Wert.
    pub fn step(&mut self, anchor: Point, wind: f64, floor: f64) {
        self.time += CLOAK_DT;
        let (t, dt) = (self.time, CLOAK_DT);
        let mut gust = 0.0;
        if wind == 0.0 {
            gust = 22.0 + 18.0 * (0.5 + 0.5 * (t * 1.1).sin()) + 6.0 * (t * 2.9).sin();
        }
        let air = -wind * 0.8 - 8.0 - 6.0 * (t * 1.7).sin() - gust;
        for p in self.pts.iter_mut().skip(1) {
            let (vx, vy) = ((p.x - p.px) / dt, (p.y - p.py) / dt);
            let (ax, ay) = (2.4 * (air - vx), 260.0 - 1.2 * vy);
            let (nx, ny) = (
                p.x + (p.x - p.px) * 0.99 + ax * dt * dt,
                p.y + (p.y - p.py) * 0.99 + ay * dt * dt,
            );
            (p.px, p.py, p.x, p.y) = (p.x, p.y, nx, ny);
        }
        for _ in 0..CLOAK_BINDING {
            self.bind(anchor, floor);
        }
    }

    /// `bind` hält die Abstände der Kette, den Anker, die Grenze nach vorn und den Boden ein.
    fn bind(&mut self, anchor: Point, floor: f64) {
        (self.pts[0].x, self.pts[0].y) = (anchor[0], anchor[1]);
        for i in 1..self.pts.len() {
            let (head, tail) = self.pts.split_at_mut(i);
            let (a, b) = (&mut head[i - 1], &mut tail[0]);
            let (dx, dy) = (b.x - a.x, b.y - a.y);
            let mut d = dx.hypot(dy);
            if d == 0.0 {
                d = 1.0;
            }
            let k = (d - self.seg) / d;
            if i > 1 {
                a.x += dx * k * 0.5;
                a.y += dy * k * 0.5;
                b.x -= dx * k * 0.5;
                b.y -= dy * k * 0.5;
            } else {
                b.x -= dx * k;
                b.y -= dy * k;
            }
            b.x = b.x.min(anchor[0] + 2.0);
            b.y = b.y.min(floor);
        }
    }

    /// `points` schreibt die Kette nach out (wiederverwendet).
    pub fn points(&self, out: &mut Vec<Point>) {
        out.clear();
        out.extend(self.pts.iter().map(|p| [p.x, p.y]));
    }
}

/// `frame_steps`: Physik-Schritt je Frame eines Sheets: Frame n bei Schritt round(n · 120 / (freq · frames)).
pub fn frame_steps(freq: f64, frames: i64) -> Vec<i64> {
    (0..frames)
        .map(|n| (n as f64 * STEPS_PER_SEC / (freq * frames as f64) + 0.5).floor() as i64)
        .collect()
}

/// `cloak_run` liefert die Umhang-Kette je Frame wie im Sheet: Physik ab Schritt −240 bis zum letzten Frame; state(k)
/// liefert Anker und Wind bei Schritt k (Zeit k/120 s).
pub fn cloak_run(
    u: &CloakSpec,
    steps: &[i64],
    state: &mut dyn FnMut(i64) -> (Point, f64),
    floor: f64,
) -> Vec<Vec<Point>> {
    let (anchor, _) = state(-CLOAK_SETTLE);
    let mut c = Cloak::new(u, anchor);
    let mut out = Vec::with_capacity(steps.len());
    let (mut k, mut n) = (-CLOAK_SETTLE + 1, 0);
    while n < steps.len() {
        let (a, wind) = state(k);
        c.step(a, wind, floor);
        while n < steps.len() && steps[n] == k {
            let mut pts = Vec::new();
            c.points(&mut pts);
            out.push(pts);
            n += 1;
        }
        k += 1;
    }
    out
}

/// `step_time`: Zeit eines Physik-Schritts in Sekunden.
pub fn step_time(k: i64) -> f64 {
    k as f64 / STEPS_PER_SEC
}
