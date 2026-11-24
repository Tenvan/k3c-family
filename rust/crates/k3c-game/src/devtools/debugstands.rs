//! WD1.3 (B-1204): Anzeige der Waldstücke im Debug-Overlay (F10, Seite Geometrie): jedes Waldstück als Fläche über die
//! Länge des Stücks (gezeichnet grün schraffiert), die gerade markierbaren äußeren Bäume großer Stücke als Rahmen. Die
//! Geometrie kommt aus `k3c_sim::world::stands` (WD1.2), hier steht nur die Umrechnung in Fenster-Pixel. Der Umschalter
//! im Cheat-Fenster (F4, Taste `stands`) setzt `set_stands_on`; Standard ist an.

use std::sync::atomic::{AtomicBool, Ordering};

use k3c_sim::state::World;
use k3c_sim::world::stands::stands;
use k3c_sim::world::values_room::values;

use super::debuggeo::{GEO_STAND, GEO_STAND_TREE, Shape};
use crate::view::cell::Cell;

/// Höhe der Waldstück-Fläche über dem Boden in Units.
pub const STAND_HEIGHT: f64 = 3.5;
/// Höhe und halbe Breite des Rahmens um einen markierbaren Baum in Units.
pub const TREE_HEIGHT: f64 = 2.6;
pub const TREE_HALF: f64 = 0.5;
/// Abstand der Schraffur-Linien in Pixeln.
pub const HATCH_GAP: f64 = 9.0;

static ON: AtomicBool = AtomicBool::new(true);

/// stands_on sagt, ob das Overlay die Waldstücke zeigt.
pub fn stands_on() -> bool {
    ON.load(Ordering::Relaxed)
}

/// set_stands_on schaltet die Anzeige der Waldstücke.
pub fn set_stands_on(on: bool) {
    ON.store(on, Ordering::Relaxed);
}

/// stand_shapes hängt die sichtbaren Waldstücke (Fläche) und die markierbaren äußeren Bäume (Rahmen) an `dst`. Die
/// Fläche ist auf das Rechteck der Zelle beschnitten. Ohne native Regeln oder auf Tiefe > 0 gibt es keine Waldstücke.
pub fn stand_shapes(dst: &mut Vec<Shape>, w: &World, c: &Cell) {
    let v = values(w);
    let all = stands(w, &v);
    if all.is_empty() {
        return;
    }
    let k = &c.camera;
    #[allow(clippy::cast_precision_loss)]
    let (ground, left, right) = (c.ground_y() as f64, c.rect.min.x as f64, c.rect.max.x as f64);
    let top = c.screen_y(STAND_HEIGHT);
    for s in &all {
        let (x0, x1) = (k.screen_x(s.start - TREE_HALF), k.screen_x(s.end + TREE_HALF));
        if x1 < left || x0 > right {
            continue;
        }
        dst.push(Shape {
            kind: GEO_STAND,
            x0: x0.max(left),
            y0: top,
            x1: x1.min(right),
            y1: ground,
        });
        for id in &s.outer {
            let Some(n) = w.node(*id) else { continue };
            if !k.visible(n.x, TREE_HALF) {
                continue;
            }
            dst.push(Shape {
                kind: GEO_STAND_TREE,
                x0: k.screen_x(n.x - TREE_HALF),
                y0: c.screen_y(TREE_HEIGHT),
                x1: k.screen_x(n.x + TREE_HALF),
                y1: ground,
            });
        }
    }
}

/// hatch liefert die Schraffur des Rechtecks (x0,y0)–(x1,y1) als Linien von links unten nach rechts oben im Abstand
/// `gap` Pixel (45 Grad); jede Linie liegt ganz im Rechteck.
pub fn hatch(x0: f64, y0: f64, x1: f64, y1: f64, gap: f64) -> Vec<[f64; 4]> {
    let (w, h) = (x1 - x0, y1 - y0);
    if w <= 0.0 || h <= 0.0 || gap <= 0.0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    // Linie x - x0 + (y - y0) = d (Summe konstant), d läuft von 0 bis w + h.
    let mut d = gap;
    while d < w + h {
        let (xa, ya) = if d <= h { (x0, y0 + d) } else { (x0 + (d - h), y1) };
        let (xb, yb) = if d <= w { (x0 + d, y0) } else { (x1, y0 + (d - w)) };
        // Start unten links (xa, ya), Ende oben rechts (xb, yb): beide liegen auf dem Rand
        out.push([xa, ya, xb, yb]);
        d += gap;
    }
    out
}

#[cfg(test)]
#[path = "debugstands_tests.rs"]
mod debugstands_tests;
