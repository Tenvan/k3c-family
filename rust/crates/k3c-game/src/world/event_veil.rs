//! Schleier der Welt-Events (WE8.8, B-995): Sandsturm und Nebel legen in ihrer Nacht einen Schleier über die Oberfläche
//! (Tiefe 0). Das Spiel zeichnet nur; Art und Nacht kommen aus `World::night_events` und `cycle.phase` der Sim (WE8.7).
//! Zwei Schleier in derselben Nacht: der mit der größeren Deckkraft gilt, sie überlagern sich nicht.

use k3c_platform::{Canvas, Rect};
use k3c_sim::state::World as SimWorld;

use crate::view::cell::Cell;
use crate::view::frame::Frame;
use crate::world::World;
use crate::world::layers::stage_of;

/// Schleier des Sandsturms: warmer Sand, Deckkraft 0.22.
pub const SANDSTORM_VEIL: [f32; 4] = [0.85, 0.7, 0.4, 0.22];
/// Schleier des Nebels: heller, grauer Dunst, Deckkraft 0.3.
pub const FOG_VEIL: [f32; 4] = [0.82, 0.84, 0.86, 0.3];

/// Der Schleier der Stufe `w` in ihrer Nacht, `None` wenn keiner gilt (Tag, unter Tage, kein Event).
pub fn veil_of(w: &SimWorld) -> Option<[f32; 4]> {
    if w.biome.depth != 0 || w.cycle.phase != "night" {
        return None;
    }
    w.night_events
        .iter()
        .filter_map(|e| match e.id.as_str() {
            "sandstorm" => Some(SANDSTORM_VEIL),
            "fog" => Some(FOG_VEIL),
            _ => None,
        })
        .max_by(|a, b| a[3].total_cmp(&b[3]))
}

impl World {
    /// `draw_event_veil`: legt den Schleier der Nacht über die Zelle der Oberfläche; nach dem Wetter gerufen.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn draw_event_veil(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let Some(wd) = stage_of(f, c).and_then(|s| s.world.as_deref()) else {
            return;
        };
        let Some(col) = veil_of(wd) else {
            return;
        };
        let r = c.world_rect();
        dst.rect(Rect::new(r.min.x as f32, r.min.y as f32, r.dx() as f32, r.dy() as f32), col);
    }
}

#[cfg(test)]
#[path = "event_veil_tests.rs"]
mod event_veil_tests;
