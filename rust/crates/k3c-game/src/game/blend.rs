//! Port von `native/k3c-game/game/blend.go`: Zwischenbilder (B-943).
//!
//! Die Simulation taktet mit 30 Hz, gezeichnet wird mit der Bildrate des Bildschirms. `Blend` hält die Lage der
//! Einheiten und der Kameras vom Takt davor und schiebt sie in `draw` um den Anteil `alpha` zwischen den Takten auf
//! die aktuelle Lage (`apply`); `restore` stellt den Frame danach wieder her. Die Darstellung läuft eine Takt-Länge
//! hinter der Simulation.
//!
//! Abweichung von Go (mit Default entschieden): `alpha` kommt immer als Parameter aus der Zeitsammlung der Plattform
//! (`k3c_platform::Clock`); `stamp` und `alpha()` der Wanduhr entfallen. `glide` und `JUMP_UNITS` stammen aus
//! `replay::blend` (RU38.2, eine Kopie); dessen `Blend` arbeitet auf einer einzelnen Welt ohne Frame und Zellen und
//! bleibt für die Wiedergabe unverändert (Umstellung: Ticket im Ergebnis der Session RU26.2).
//!
//! Besitz: `Stage::world` ist ein `Rc<World>`. Verschoben wird mit `Rc::get_mut` (der Frame hält die einzige
//! Referenz); teilt die Sitzung die Welt, klont `Rc::make_mut` sie einmal für diesen Frame, danach ist sie eindeutig.

use std::rc::Rc;

use k3c_sim::state::World;

pub use crate::replay::blend::{JUMP_UNITS, glide};

use crate::view::MAX_PLAYERS;
use crate::view::cell::Cell;
use crate::view::frame::Frame;

/// Arten der Einheiten im Schlüssel von `Blend::prev` (wie `kind*` in Go).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotArt {
    Player = 1,
    Troop,
    Enemy,
    Projectile,
    Coin,
}

const ARTEN: [SlotArt; 5] = [
    SlotArt::Player,
    SlotArt::Troop,
    SlotArt::Enemy,
    SlotArt::Projectile,
    SlotArt::Coin,
];

/// Eine verschobene Einheit: Art, Stufe, Listenindex und x der Simulation.
pub type Saved = (SlotArt, i64, usize, f64);

/// Schlüssel Stufe, Art, ID (wie `key` in Go).
#[allow(clippy::cast_sign_loss)]
fn key(stage: i64, art: SlotArt, id: i64) -> u64 {
    ((stage as u64) << 48) | ((art as u64) << 40) | u64::from(id as u32)
}

/// Zahl der Einheiten der Art `art` in `w`.
fn slot_len(w: &World, art: SlotArt) -> usize {
    match art {
        SlotArt::Player => w.players.len(),
        SlotArt::Troop => w.troops.len(),
        SlotArt::Enemy => w.enemies.len(),
        SlotArt::Projectile => w.projectiles.len(),
        SlotArt::Coin => w.coins.len(),
    }
}

/// ID und x der Einheit `i` der Art `art`.
fn slot(w: &World, art: SlotArt, i: usize) -> Option<(i64, f64)> {
    match art {
        SlotArt::Player => w.players.get(i).map(|u| (u.id, u.x)),
        SlotArt::Troop => w.troops.get(i).map(|u| (u.id, u.x)),
        SlotArt::Enemy => w.enemies.get(i).map(|u| (u.id, u.x)),
        SlotArt::Projectile => w.projectiles.get(i).map(|u| (u.id, u.x)),
        SlotArt::Coin => w.coins.get(i).map(|u| (u.id, u.x)),
    }
}

/// Schreibzugriff auf das x der Einheit `i` der Art `art`.
fn slot_x(w: &mut World, art: SlotArt, i: usize) -> Option<&mut f64> {
    match art {
        SlotArt::Player => w.players.get_mut(i).map(|u| &mut u.x),
        SlotArt::Troop => w.troops.get_mut(i).map(|u| &mut u.x),
        SlotArt::Enemy => w.enemies.get_mut(i).map(|u| &mut u.x),
        SlotArt::Projectile => w.projectiles.get_mut(i).map(|u| &mut u.x),
        SlotArt::Coin => w.coins.get_mut(i).map(|u| &mut u.x),
    }
}

/// Welt der Stufe zum Verschieben: `Rc::get_mut`, sonst einmal `Rc::make_mut` (Klon für diesen Frame).
fn world_mut(w: &mut Rc<World>) -> &mut World {
    Rc::make_mut(w)
}

/// Zustand der Zwischenbilder; die Puffer werden wiederverwendet (keine Allokation je Bild).
pub struct Blend {
    on: bool,
    /// Schlüssel → x beim Takt davor, nach Schlüssel sortiert
    prev: Vec<(u64, f64)>,
    cam: [f64; MAX_PLAYERS],
    cam_known: [bool; MAX_PLAYERS],
    clock: f64,
    saved: Vec<Saved>,
    saved_cam: [f64; MAX_PLAYERS],
    saved_clock: f64,
    cam_active: bool,
}

impl Blend {
    pub fn new(on: bool) -> Blend {
        Blend {
            on,
            prev: Vec::with_capacity(256),
            cam: [0.0; MAX_PLAYERS],
            cam_known: [false; MAX_PLAYERS],
            clock: 0.0,
            saved: Vec::with_capacity(256),
            saved_cam: [0.0; MAX_PLAYERS],
            saved_clock: 0.0,
            cam_active: false,
        }
    }

    /// Gleiten an (`false`: `capture`, `apply` und `restore` tun nichts).
    pub fn is_on(&self) -> bool {
        self.on
    }

    /// Merkt vor dem nächsten Takt die Lage des Frames `f` (der Stand vor dem Takt) und der Zellen `cells`.
    pub fn capture(&mut self, f: &Frame, cells: &[Cell]) {
        if !self.on {
            return;
        }
        self.prev.clear();
        for st in &f.stages {
            let Some(w) = st.world.as_deref() else {
                continue;
            };
            for art in ARTEN {
                for i in 0..slot_len(w, art) {
                    if let Some((id, x)) = slot(w, art, i) {
                        self.prev.push((key(st.index, art, id), x));
                    }
                }
            }
        }
        self.prev.sort_unstable_by_key(|e| e.0);
        self.cam_known = [false; MAX_PLAYERS];
        for c in cells {
            if let Some(i) = cam_slot(c) {
                self.cam[i] = c.camera.x;
                self.cam_known[i] = true;
            }
        }
        self.clock = f.clock;
    }

    /// x der Einheit beim Takt davor.
    fn prev_x(&self, k: u64) -> Option<f64> {
        self.prev.binary_search_by_key(&k, |e| e.0).ok().map(|i| self.prev[i].1)
    }

    /// Schiebt Einheiten, Kameras und Uhr von `f` bzw. `cells` auf den Anteil `a`; `restore` macht es rückgängig.
    pub fn apply(&mut self, f: &mut Frame, cells: &mut [Cell], a: f64) {
        if !self.on || a >= 1.0 {
            return;
        }
        self.saved.clear();
        self.cam_active = false;
        for st in &mut f.stages {
            if let Some(w) = st.world.as_mut() {
                let idx = st.index;
                self.shift_world(world_mut(w), idx, a);
            }
        }
        for c in cells.iter_mut() {
            if let Some(i) = cam_slot(c).filter(|i| self.cam_known[*i]) {
                self.saved_cam[i] = c.camera.x;
                c.camera.x = glide(self.cam[i], c.camera.x, a);
            }
        }
        self.cam_active = true;
        let d = f.clock - self.clock;
        self.saved_clock = f.clock;
        if d > 0.0 && d < 0.5 {
            f.clock = self.clock + d * a;
        }
    }

    /// Schiebt die Einheiten der Welt `w` der Stufe `stage` auf den Anteil `a` und merkt die Werte der Simulation.
    fn shift_world(&mut self, w: &mut World, stage: i64, a: f64) {
        for art in ARTEN {
            for i in 0..slot_len(w, art) {
                let Some((id, x)) = slot(w, art, i) else {
                    continue;
                };
                let Some(prev) = self.prev_x(key(stage, art, id)) else {
                    continue;
                };
                let g = glide(prev, x, a);
                if g != x
                    && let Some(px) = slot_x(w, art, i)
                {
                    self.saved.push((art, stage, i, x));
                    *px = g;
                }
            }
        }
    }

    /// Stellt Einheiten, Kameras und Uhr nach dem Zeichnen wieder her.
    pub fn restore(&mut self, f: &mut Frame, cells: &mut [Cell]) {
        for &(art, stage, i, x) in &self.saved {
            let world = f
                .stages
                .iter_mut()
                .find(|s| s.index == stage)
                .and_then(|s| s.world.as_mut());
            if let Some(px) = world.and_then(|w| slot_x(world_mut(w), art, i)) {
                *px = x;
            }
        }
        self.saved.clear();
        if !self.cam_active {
            return;
        }
        for c in cells.iter_mut() {
            if let Some(i) = cam_slot(c).filter(|i| self.cam_known[*i]) {
                c.camera.x = self.saved_cam[i];
            }
        }
        f.clock = self.saved_clock;
        self.cam_active = false;
    }
}

/// Platz der Zelle in den Kamera-Feldern (`index` 0 bis `MAX_PLAYERS - 1`).
fn cam_slot(c: &Cell) -> Option<usize> {
    usize::try_from(c.index).ok().filter(|i| *i < MAX_PLAYERS)
}

#[cfg(test)]
#[path = "blend_tests.rs"]
mod blend_tests;
