//! Platzhalter-Zeichner der Wiedergabe: ein Rechteck je Einheit, bis `world` und `actors` (RU13) vorliegen.

use k3c_platform::{Canvas, Rect};
use k3c_sim::state::World;

/// Pixel je Unit (`UNIT_PX`).
pub const UNIT_PX: f32 = 32.0;

/// Breite und Höhe einer Bildhälfte in logischen Pixeln.
pub const CELL: (f32, f32) = (960.0, 1080.0);

/// Höhe der Grundlinie (obere Kante des Bodens) in der Bildhälfte.
const GROUND_Y: f32 = 800.0;

/// Sichtbare Breite einer Bildhälfte in Units.
pub fn view_units() -> f64 {
    f64::from(CELL.0 / UNIT_PX)
}

/// Kamera (linke Kante in Units) für den Spieler `index`: auf ihn zentriert, im Bereich der Welt.
pub fn camera_x(w: &World, index: i64) -> f64 {
    let x = w.players.iter().find(|p| p.index == index).map_or(w.hub_x, |p| p.x);
    let max = (w.width_units - view_units()).max(0.0);
    (x - view_units() / 2.0).clamp(0.0, max)
}

/// Zeichnet die Welt `w` mit der Kamera `cam` in die Bildhälfte ab `left`; liefert die Zahl der Rechtecke.
pub fn draw_cell(c: &mut dyn Canvas, w: &World, cam: f64, left: f32) -> u32 {
    let at = |x: f64| left + ((x - cam) as f32) * UNIT_PX;
    let mut n = 0;
    let mut rect = |c: &mut dyn Canvas, r: Rect, col: [f32; 4]| {
        c.rect(r, col);
        n += 1;
    };
    rect(c, Rect::new(left, 0.0, CELL.0, GROUND_Y), [0.10, 0.12, 0.20, 1.0]);
    rect(
        c,
        Rect::new(left, GROUND_Y, CELL.0, CELL.1 - GROUND_Y),
        [0.16, 0.30, 0.14, 1.0],
    );
    let mut unit = |c: &mut dyn Canvas, x: f64, w_px: f32, h_px: f32, col: [f32; 4]| {
        rect(c, Rect::new(at(x) - w_px / 2.0, GROUND_Y - h_px, w_px, h_px), col);
    };
    for p in &w.coins {
        unit(c, p.x, 10.0, 10.0, [0.95, 0.8, 0.2, 1.0]);
    }
    for p in &w.troops {
        unit(c, p.x, 22.0, 40.0, [0.3, 0.6, 0.9, 1.0]);
    }
    for p in &w.enemies {
        unit(c, p.x, 28.0, 48.0, [0.85, 0.2, 0.2, 1.0]);
    }
    for p in &w.projectiles {
        unit(c, p.x, 8.0, 8.0, [1.0, 1.0, 1.0, 1.0]);
    }
    for p in &w.players {
        unit(c, p.x, 26.0, 56.0, [0.95, 0.85, 0.3, 1.0]);
    }
    n
}
