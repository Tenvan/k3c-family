//! Zustand der Demo je Takt (30 Hz): Kamera, Reiter und Figuren laufen hin und her; keine Sim.

use super::driver::Driver;

/// Ticks je Sekunde (Takt der Plattform).
pub const TICKS: f64 = 30.0;

/// Eine Figur: Lage vom Takt davor (`prev_x`) bleibt für die Mischung mit `alpha`.
#[derive(Debug, Clone, PartialEq)]
pub struct Actor {
    /// Index in `World::assets`.
    pub asset: usize,
    pub anim: String,
    pub x: f64,
    pub prev_x: f64,
    /// +1 nach rechts, -1 nach links (Spiegelung beim Wenden).
    pub dir: f64,
    /// Units je Takt.
    pub speed: f64,
    pub von: f64,
    pub bis: f64,
    /// Versatz der Animation in Takten.
    pub phase: f64,
    /// Tiefenversatz in Pixeln (Figuren laufen nicht alle auf einer Linie).
    pub row: f32,
}

impl Actor {
    fn step(&mut self) {
        self.prev_x = self.x;
        self.x += self.dir * self.speed;
        if self.x >= self.bis {
            self.x = self.bis;
            self.dir = -1.0;
        } else if self.x <= self.von {
            self.x = self.von;
            self.dir = 1.0;
        }
    }

    /// Lage zwischen Takt davor und jetzt.
    pub fn x_at(&self, alpha: f64) -> f64 {
        self.prev_x + (self.x - self.prev_x) * alpha
    }
}

/// Weltzustand; der Reiter (`actors[0]`) ist die Figur, der die Kamera folgt.
pub struct World {
    pub tick: u64,
    pub assets: Vec<String>,
    pub actors: Vec<Actor>,
}

/// Zahl in [0, 1) aus Index und Salz (splitmix64), damit jede Figur anders wirkt, ohne Zufallsquelle.
pub fn unit(i: u64, salt: u64) -> f64 {
    let mut z = i
        .wrapping_add(salt.wrapping_mul(0x9E37_79B9_7F4A_7C15))
        .wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    (z >> 11) as f64 / (1u64 << 53) as f64
}

impl World {
    /// Baut die Welt aus dem Skript; `rider_speed` in Units/s ist das Tempo des Reiters.
    pub fn new(d: &Driver, rider_speed: f64) -> Self {
        let assets = d.assets();
        let index = |a: &str| assets.iter().position(|x| x == a).unwrap_or(0);
        let mut actors = vec![Actor {
            asset: index(&d.kamera.figur),
            anim: d.kamera.anim.clone(),
            x: 0.0,
            prev_x: 0.0,
            dir: 1.0,
            speed: rider_speed / TICKS,
            von: 0.0,
            bis: d.laenge,
            phase: 0.0,
            row: 0.0,
        }];
        let mut n = 0u64;
        for f in &d.figuren {
            for k in 0..f.anzahl {
                n += 1;
                let frac = (f64::from(k) + unit(n, 1)) / f64::from(f.anzahl.max(1));
                let x = f.von + (f.bis - f.von) * frac;
                actors.push(Actor {
                    asset: index(&f.asset),
                    anim: f.anim.clone(),
                    x,
                    prev_x: x,
                    dir: if unit(n, 2) < 0.5 { -1.0 } else { 1.0 },
                    speed: f.tempo * (0.8 + 0.4 * unit(n, 3)) / TICKS,
                    von: f.von,
                    bis: f.bis,
                    phase: unit(n, 4) * 8.0,
                    row: (unit(n, 5) * 40.0) as f32,
                });
            }
        }
        Self {
            tick: 0,
            assets,
            actors,
        }
    }

    /// Ein Takt.
    pub fn step(&mut self) {
        self.tick += 1;
        for a in &mut self.actors {
            a.step();
        }
    }

    /// Kamera-x in Units, gemischt mit `alpha`.
    pub fn camera(&self, alpha: f64) -> f64 {
        self.actors[0].x_at(alpha)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn driver() -> Driver {
        serde_json::from_str(
            r##"{"laenge":10,"biom":"x","ebenen":[],"boden":"#000000","kamera":{"figur":"figuren/test"},
            "figuren":[{"asset":"figuren/test","anzahl":5,"tempo":3.0,"von":2,"bis":6}]}"##,
        )
        .unwrap()
    }

    #[test]
    fn figuren_wenden_an_den_enden() {
        let mut w = World::new(&driver(), 6.0);
        assert_eq!(w.actors.len(), 6);
        let mut seen_left = false;
        let mut seen_right = false;
        for _ in 0..2000 {
            w.step();
            for a in &w.actors[1..] {
                assert!((2.0..=6.0).contains(&a.x), "x = {}", a.x);
                seen_left |= a.dir < 0.0;
                seen_right |= a.dir > 0.0;
            }
        }
        assert!(seen_left && seen_right);
    }

    #[test]
    fn reiter_reitet_die_strecke_hin_und_zurueck() {
        let mut w = World::new(&driver(), 6.0);
        let mut max = 0.0f64;
        for _ in 0..200 {
            w.step();
            max = max.max(w.actors[0].x);
        }
        assert_eq!(max, 10.0, "erreicht das Ende");
        assert!(w.actors[0].x < 10.0, "ist wieder auf dem Rückweg");
    }

    #[test]
    fn alpha_mischt_zwischen_zwei_takten() {
        let mut w = World::new(&driver(), 6.0);
        w.step();
        let (a, b) = (w.actors[0].prev_x, w.actors[0].x);
        assert!(b > a);
        assert_eq!(w.camera(0.0), a);
        assert!((w.camera(0.5) - (a + b) / 2.0).abs() < 1e-12);
    }

    #[test]
    fn variation_ist_deterministisch_und_gestreut() {
        assert_eq!(unit(3, 1), unit(3, 1));
        assert_ne!(unit(3, 1), unit(4, 1));
        assert!((0..100).all(|i| (0.0..1.0).contains(&unit(i, 7))));
    }
}
