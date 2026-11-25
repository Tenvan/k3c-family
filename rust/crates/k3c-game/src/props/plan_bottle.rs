//! Flaschenpost am Strand (GS4.4, B-980): Liegt die Flasche in der Simulation (`World::bottle.ashore`, Stufe der Tiefe 0),
//! zeichnet das Spiel sie an dieser Position mit Glitzer. Liegt keine Flasche, zeichnet es nichts. Ohne Grafik bleibt
//! eine schlanke Glasfläche als Platzhalter. Das Spiel ändert den Zustand der Sim nicht.

use k3c_sim::state::World;

use super::buildings::Look;
use super::plan::{Item, Planner, Shape, px, visible};
use crate::view::cell::Camera;

/// Asset der Flasche (Gruppe `objekte`, Animation `stand`, `data/paper.json` › `objekte` › `flaschenpost`).
const BOTTLE_ASSET: &str = "objekte/flaschenpost";
const BOTTLE_ANIM: &str = "stand";
/// Platzhalter ohne Grafik: Glas und Funke.
const GLASS: [u8; 4] = [120, 200, 230, 255];
const SPARKLE: [u8; 4] = [255, 250, 220, 255];
/// Warmer Schein am Ziel der Markierung: Durchmesser in px, je innerer Ring schwächer gestaffelt.
const GLOW: [u8; 4] = [255, 226, 122, 255];
const GLOW_RINGS: [f64; 3] = [150.0, 100.0, 56.0];
/// Zahl der Funken am Glitzer.
const SPARKLES: usize = 3;
/// Halbe Breite der Sichtprüfung in Units; die Flasche ist klein.
const VIEW_HALF: f64 = 1.0;

/// Die liegende Flasche der Welt `w`, `None` wenn keine liegt oder die Stufe nicht die oberste ist (B-971: nur Tiefe 0).
pub fn lying_bottle(w: &World) -> Option<f64> {
    if w.biome.depth != 0 {
        return None;
    }
    w.bottle.ashore
}

impl Planner {
    /// `bottle_glow`: weicher Schein um das markierte Ziel bei `x` (drei Scheiben, Deckkraft pulsiert mit der Uhr);
    /// außerhalb des Bildes nichts. Ohne Schrift.
    pub fn bottle_glow(&mut self, x: f64, k: &Camera, clock: f64) {
        if !visible(k, x, 3.0) {
            return;
        }
        let pulse = 0.5 + 0.5 * (clock * 2.0).sin();
        for (i, d) in GLOW_RINGS.iter().enumerate() {
            self.body.push(Item {
                shape: Shape::Circle,
                x,
                y: px(40.0),
                w: px(*d),
                clr: GLOW,
                alpha: (0.10 + 0.08 * i as f64 + 0.06 * pulse) as f32,
                ..Default::default()
            });
        }
    }

    /// `bottle`: liegt die Flasche, Bild am Strand (Rückfall: Glasfläche) und drei Funken, deren Deckkraft mit der Uhr
    /// pulsiert. Außerhalb des Bildes nichts.
    pub fn bottle(&mut self, w: &World, k: &Camera, clock: f64) {
        let Some(x) = lying_bottle(w) else {
            return;
        };
        if !visible(k, x, VIEW_HALF) {
            return;
        }
        if !self.sprite(Look::new(BOTTLE_ASSET, BOTTLE_ANIM), x, 0.0, 0) {
            Planner::rect(&mut self.body, x, 0.0, px(10.0), px(96.0), GLASS);
        }
        for i in 0..SPARKLES {
            let phase = i as f64 * 2.1;
            let a = (0.5 + 0.5 * (clock * 3.0 + phase).sin()) as f32;
            self.body.push(Item {
                shape: Shape::Circle,
                x: x + px((i as f64 - 1.0) * 14.0),
                y: px(60.0 + 12.0 * i as f64),
                w: px(3.0),
                clr: SPARKLE,
                alpha: a,
                ..Default::default()
            });
        }
    }
}

#[cfg(test)]
#[path = "plan_bottle_tests.rs"]
mod plan_bottle_tests;
