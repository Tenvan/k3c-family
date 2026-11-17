//! Zwischenbilder: Port von `native/k3c-game/game/blend.go` (B-943).
//!
//! Die Sim taktet mit 30 Hz, gezeichnet wird schneller. `capture` merkt die Lage vom Takt davor, `apply` schiebt
//! Einheiten, Kameras und Uhr in `draw` um alpha auf die aktuelle Lage, `restore` stellt sie danach wieder her.

use std::collections::BTreeMap;

use k3c_sim::state::World;

/// Größter Weg in Units je Takt, der noch gleitet; weiter weg (Stufenwechsel, Zeitraffer) springt das Bild.
pub const JUMP_UNITS: f64 = 6.0;

/// Obergrenze der Kameras (Spieler-Zellen).
pub const MAX_CAMS: usize = 4;

/// Arten der Einheiten im Schlüssel (wie `kind*` in Go).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Player = 1,
    Troop,
    Enemy,
    Projectile,
    Coin,
}

fn key(stage: i64, kind: Kind, id: i64) -> u64 {
    ((stage as u64) << 48) | ((kind as u64) << 40) | u64::from(id as u32)
}

/// Wert zwischen `prev` und `cur` beim Anteil `a`; springt `cur` weit weg oder ändert es sich nicht, bleibt es `cur`.
pub fn glide(prev: f64, cur: f64, a: f64) -> f64 {
    let d = cur - prev;
    if d == 0.0 || d.abs() > JUMP_UNITS {
        cur
    } else {
        prev + d * a
    }
}

/// Zustand der Zwischenbilder.
#[derive(Default)]
pub struct Blend {
    prev: BTreeMap<u64, f64>,
    cam: Vec<f64>,
    clock: f64,
    stage: i64,
    saved: Vec<(Kind, usize, f64)>,
    saved_cam: Vec<f64>,
    saved_clock: f64,
    active: bool,
}

impl Blend {
    pub fn new(stage: i64) -> Self {
        Self {
            stage,
            ..Self::default()
        }
    }

    /// Merkt vor dem nächsten Takt die Lage von `w` (Stand vor dem Takt) und der Kameras `cams`.
    pub fn capture(&mut self, w: &World, cams: &[f64]) {
        self.prev.clear();
        let s = self.stage;
        let ps = &mut self.prev;
        w.players
            .iter()
            .for_each(|p| _ = ps.insert(key(s, Kind::Player, p.id), p.x));
        w.troops
            .iter()
            .for_each(|p| _ = ps.insert(key(s, Kind::Troop, p.id), p.x));
        w.enemies
            .iter()
            .for_each(|p| _ = ps.insert(key(s, Kind::Enemy, p.id), p.x));
        w.projectiles
            .iter()
            .for_each(|p| _ = ps.insert(key(s, Kind::Projectile, p.id), p.x));
        w.coins
            .iter()
            .for_each(|p| _ = ps.insert(key(s, Kind::Coin, p.id), p.x));
        self.cam = cams.iter().copied().take(MAX_CAMS).collect();
        self.clock = w.time;
    }

    /// Schiebt Einheiten, Kameras und Uhr auf den Anteil `a`; `restore` macht es rückgängig.
    pub fn apply(&mut self, w: &mut World, cams: &mut [f64], a: f64) {
        if a >= 1.0 {
            return;
        }
        let (s, prev) = (self.stage, &self.prev);
        let mut saved = std::mem::take(&mut self.saved);
        saved.clear();
        let mut shift = |kind: Kind, i: usize, id: i64, x: &mut f64| {
            if let Some(&p) = prev.get(&key(s, kind, id)) {
                let g = glide(p, *x, a);
                if g != *x {
                    saved.push((kind, i, *x));
                    *x = g;
                }
            }
        };
        w.players
            .iter_mut()
            .enumerate()
            .for_each(|(i, p)| shift(Kind::Player, i, p.id, &mut p.x));
        w.troops
            .iter_mut()
            .enumerate()
            .for_each(|(i, p)| shift(Kind::Troop, i, p.id, &mut p.x));
        w.enemies
            .iter_mut()
            .enumerate()
            .for_each(|(i, p)| shift(Kind::Enemy, i, p.id, &mut p.x));
        w.projectiles
            .iter_mut()
            .enumerate()
            .for_each(|(i, p)| shift(Kind::Projectile, i, p.id, &mut p.x));
        w.coins
            .iter_mut()
            .enumerate()
            .for_each(|(i, p)| shift(Kind::Coin, i, p.id, &mut p.x));
        self.saved = saved;
        self.saved_cam = cams.to_vec();
        for (c, &p) in cams.iter_mut().zip(self.cam.iter()) {
            *c = glide(p, *c, a);
        }
        self.active = true;
        self.saved_clock = w.time;
        let d = w.time - self.clock;
        if d > 0.0 && d < 0.5 {
            w.time = self.clock + d * a;
        }
    }

    /// Stellt Einheiten, Kameras und Uhr nach dem Zeichnen wieder her.
    pub fn restore(&mut self, w: &mut World, cams: &mut [f64]) {
        for &(kind, i, x) in &self.saved {
            match kind {
                Kind::Player => {
                    if let Some(u) = w.players.get_mut(i) {
                        u.x = x;
                    }
                }
                Kind::Troop => {
                    if let Some(u) = w.troops.get_mut(i) {
                        u.x = x;
                    }
                }
                Kind::Enemy => {
                    if let Some(u) = w.enemies.get_mut(i) {
                        u.x = x;
                    }
                }
                Kind::Projectile => {
                    if let Some(u) = w.projectiles.get_mut(i) {
                        u.x = x;
                    }
                }
                Kind::Coin => {
                    if let Some(u) = w.coins.get_mut(i) {
                        u.x = x;
                    }
                }
            }
        }
        self.saved.clear();
        if self.active {
            cams.copy_from_slice(&self.saved_cam);
            w.time = self.saved_clock;
            self.active = false;
        }
    }
}

#[cfg(test)]
#[path = "blend_tests.rs"]
mod tests;
