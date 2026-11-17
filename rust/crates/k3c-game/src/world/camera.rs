//! Port von `native/k3c-game/world/camera.go`.
//!
//! Kamera je Zelle (B-795/AC-01): Wie im Browser (src/scenes/GameScene.ts › aimAtStage, cameraFollow.ts) folgt jede
//! Kamera ihrem Monarchen weich und bleibt in den Grenzen der Stufe (setBounds); dazu eine kleine Totzone, damit kurze
//! Schritte nicht das ganze Bild schieben. Beim ersten Bild und beim Wechsel der Stufe oder Insel springt sie ans Ziel.

use k3c_det::{fmax, fmin};
use k3c_sim::state::World;

use crate::view::MAX_PLAYERS;
use crate::view::cell::{Camera, Cell};
use crate::view::frame::Frame;

/// DEAD_ZONE ist die halbe Breite der Totzone um die Kamera-Mitte in Units: Steht das Ziel darin, bleibt die Kamera.
pub const DEAD_ZONE: f64 = 1.0;
/// FOLLOW_RATE ist der Anteil des Abstands, den die Kamera je Update (30 Hz) aufholt: Phaser folgt mit lerp 0,1 je
/// Bild bei 60 Hz, das sind 1 − 0,9² = 0,19 je Update.
pub const FOLLOW_RATE: f64 = 0.19;
/// FADE_SECONDS ist die Dauer des Übergangs (Abblenden aus Schwarz) nach einem Stufen- oder Inselwechsel.
pub const FADE_SECONDS: f64 = 0.6;

/// StageKey erkennt die Stufe eines Spielers: Index, Tiefe und Level der Stufe, Seed ihrer Welt (Inselwechsel).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StageKey {
    pub index: i64,
    pub depth: i64,
    pub level: String,
    pub seed: String,
}

/// SeatState ist der Kamera-Zustand eines Spielers: zuletzt gesehene Stufe und Zeitpunkt (Frame.Clock) des Wechsels.
#[derive(Clone, Debug, Default)]
pub struct SeatState {
    pub key: StageKey,
    pub since: f64,
    pub known: bool,
}

/// Cameras ist der Zustand der Kamera-Regel je Spieler (`Seat::player`). Nur `follow` (Update) schreibt, Draw liest.
#[derive(Clone, Debug, Default)]
pub struct Cameras {
    pub seats: [SeatState; MAX_PLAYERS],
}

impl Cameras {
    /// Follow ist die Kamera-Regel (`view::CameraRule`): Ziel ist der Monarch, ohne ihn die Burg; ohne Welt bleibt die
    /// Kamera stehen. Beim ersten Bild (`prev.scale == 0`) und beim Stufenwechsel springt sie, sonst holt sie weich auf.
    pub fn follow(&mut self, f: &Frame, c: &Cell, prev: Camera) -> Camera {
        let mut k = c.camera;
        let Some(w) = f.world(c) else {
            if prev.scale != 0.0 {
                k.x = prev.x;
            }
            return k;
        };
        let jump = self.enter(f, c.player) || prev.scale == 0.0;
        let mut x = target_x(f, c, w);
        if !jump {
            x = chase(prev.x, x);
        }
        k.x = clamp_x(x, half_units(&k), w.width_units);
        k
    }

    /// Enter merkt sich die Stufe des Spielers p und meldet, ob sie neu ist (erstes Bild oder Wechsel); der Wechsel
    /// startet den Übergang zur Zeit `Frame::clock`.
    pub fn enter(&mut self, f: &Frame, p: i64) -> bool {
        if p < 0 || p as usize >= self.seats.len() {
            return true;
        }
        let key = key_of(f, p);
        let st = &mut self.seats[p as usize];
        let Some(key) = key else {
            return false;
        };
        if st.known && st.key == key {
            return false;
        }
        *st = SeatState {
            key,
            since: f.clock,
            known: true,
        };
        true
    }

    /// Fade liefert die Deckkraft des Übergangs für Spieler p zur Zeit clock (0 = kein Übergang).
    pub fn fade(&self, p: i64, clock: f64) -> f64 {
        if p < 0 || p as usize >= self.seats.len() || !self.seats[p as usize].known {
            return 0.0;
        }
        fade_alpha(clock - self.seats[p as usize].since)
    }
}

/// KeyOf liefert die Stufe des Spielers p im Frame.
pub fn key_of(f: &Frame, p: i64) -> Option<StageKey> {
    let seat = f.seat(p)?;
    let st = f.stage(seat.stage)?;
    let w = st.world.as_deref()?;
    Some(StageKey {
        index: st.index,
        depth: st.depth,
        level: st.level.clone(),
        seed: w.seed.clone(),
    })
}

/// TargetX ist das Ziel der Kamera: der Monarch des Spielers, sonst die Burg der Stufe.
pub fn target_x(f: &Frame, c: &Cell, w: &World) -> f64 {
    if let Some(m) = f.monarch(c.player) {
        return m.x;
    }
    w.castle.x
}

/// Chase führt die Kamera von x auf das Ziel t zu: in der Totzone bleibt sie, sonst holt sie `FOLLOW_RATE` des
/// Abstands zum Rand der Totzone auf.
pub fn chase(x: f64, t: f64) -> f64 {
    let d = t - x;
    if d > DEAD_ZONE {
        x + (d - DEAD_ZONE) * FOLLOW_RATE
    } else if d < -DEAD_ZONE {
        x + (d + DEAD_ZONE) * FOLLOW_RATE
    } else {
        x
    }
}

/// ClampX hält die Kamera-Mitte x so, dass die Zelle mit halber Breite half (Units) nicht über die Stufe 0 … width
/// hinausschaut; ist die Stufe schmaler als die Zelle, steht die Kamera in ihrer Mitte.
pub fn clamp_x(x: f64, half: f64, width: f64) -> f64 {
    if width <= 0.0 || half <= 0.0 {
        return x;
    }
    if width <= 2.0 * half {
        return width / 2.0;
    }
    fmin(fmax(x, half), width - half)
}

/// HalfUnits ist die halbe sichtbare Breite der Kamera in Units (0 ohne Maßstab).
pub fn half_units(k: &Camera) -> f64 {
    if k.scale <= 0.0 {
        return 0.0;
    }
    k.half / k.scale
}

/// FadeAlpha ist die Deckkraft des Übergangs elapsed Sekunden nach dem Wechsel: 1 → 0 linear über `FADE_SECONDS`.
#[allow(clippy::manual_range_contains)] // wie Go: NaN fällt nicht in die erste Bedingung
pub fn fade_alpha(elapsed: f64) -> f64 {
    if elapsed < 0.0 || elapsed >= FADE_SECONDS {
        return 0.0;
    }
    1.0 - elapsed / FADE_SECONDS
}

#[cfg(test)]
#[path = "camera_tests.rs"]
mod camera_tests;
