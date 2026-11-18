//! Port von `native/k3c-game/devtools/debuggeo.go`.
//!
//! Geometrie des Debug-Overlays (F10, Seite Geometrie, B-888): Rahmen um Figuren, Münzen und Bauplätze (dort die
//! Zahl-Reichweite aus data/economy.json), Linien an den Chunk-Grenzen und Weltkanten, Kamera-Mitte. Die Simulation ist
//! eindimensional (Welt-x in Units); die Höhen der Rahmen sind feste Anzeige-Werte. Reine Rechnung in Fenster-Pixeln.

use std::sync::OnceLock;

use k3c_sim::state::World;
use serde::Deserialize;

use crate::view::cell::Cell;

// Arten der Debug-Formen (Farbe).
pub const GEO_MONARCH: usize = 0;
pub const GEO_TROOP: usize = 1;
pub const GEO_ENEMY: usize = 2;
pub const GEO_COIN: usize = 3;
pub const GEO_SITE: usize = 4;
pub const GEO_CHUNK: usize = 5;
pub const GEO_EDGE: usize = 6;
pub const GEO_CAMERA: usize = 7;
pub const GEO_KINDS: usize = 8;

/// GEO_HEIGHT ist die Höhe der Rahmen je Art in Units (Linien: ganze Zelle).
pub const GEO_HEIGHT: [f64; GEO_KINDS] = [2.2, 1.6, 1.6, 0.4, 0.3, 0.0, 0.0, 0.0];

/// GEO_HALF ist die halbe Breite der Figuren-Rahmen in Units (Bauplätze: Zahl-Reichweite).
pub const GEO_HALF: [f64; GEO_KINDS] = [0.5, 0.4, 0.45, 0.2, 0.0, 0.0, 0.0, 0.0];

/// Shape ist eine Form in Fenster-Pixeln: Rahmen (`x0 != x1`) oder senkrechte Linie (`x0 == x1`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Shape {
    pub kind: usize,
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

#[derive(Deserialize)]
struct EconomyFile {
    #[serde(rename = "payRangeUnits", alias = "PayRangeUnits", default)]
    pay_range_units: f64,
}

/// pay_range ist die Zahl-Reichweite der Bauplätze (economy.json › payRangeUnits; Fehler oder Wert ≤ 0: 2).
pub fn pay_range() -> f64 {
    static R: OnceLock<f64> = OnceLock::new();
    *R.get_or_init(|| {
        k3c_data::Source::from_env()
            .read("economy.json")
            .ok()
            .and_then(|raw| serde_json::from_str::<EconomyFile>(&raw).ok())
            .map(|v| v.pay_range_units)
            .filter(|v| *v > 0.0)
            .unwrap_or(2.0)
    })
}

/// add hängt den Rahmen der Art `kind` bei Welt-x `x` mit halber Breite `half` an, wenn er sichtbar ist.
fn add(dst: &mut Vec<Shape>, c: &Cell, kind: usize, x: f64, half: f64) {
    let k = &c.camera;
    if !k.visible(x, half) {
        return;
    }
    #[allow(clippy::cast_precision_loss)]
    let ground = c.ground_y() as f64;
    dst.push(Shape {
        kind,
        x0: k.screen_x(x - half),
        y0: c.screen_y(GEO_HEIGHT[kind]),
        x1: k.screen_x(x + half),
        y1: ground,
    });
}

/// geometry hängt die sichtbaren Formen der Welt `w` in der Zelle `c` an `dst`.
pub fn geometry(dst: &mut Vec<Shape>, w: &World, c: &Cell) {
    for p in &w.players {
        add(dst, c, GEO_MONARCH, p.x, GEO_HALF[GEO_MONARCH]);
    }
    for t in &w.troops {
        add(dst, c, GEO_TROOP, t.x, GEO_HALF[GEO_TROOP]);
    }
    for e in &w.enemies {
        add(dst, c, GEO_ENEMY, e.x, GEO_HALF[GEO_ENEMY]);
    }
    for m in &w.coins {
        add(dst, c, GEO_COIN, m.x, GEO_HALF[GEO_COIN]);
    }
    let range = pay_range();
    for s in &w.sites {
        add(dst, c, GEO_SITE, s.x, range);
    }
    lines(dst, w, c);
}

/// lines hängt die Chunk-Grenzen, die Weltkanten und die Kamera-Mitte als Linien über die ganze Zelle an.
#[allow(clippy::cast_precision_loss)]
pub fn lines(dst: &mut Vec<Shape>, w: &World, c: &Cell) {
    let k = &c.camera;
    let (top, bottom) = (c.rect.min.y as f64, c.rect.max.y as f64);
    let mut line = |kind: usize, x: f64| {
        dst.push(Shape {
            kind,
            x0: x,
            y0: top,
            x1: x,
            y1: bottom,
        });
    };
    for ch in &w.level.chunks {
        let x = ch.start_units;
        if x > 0.0 && k.visible(x, 0.0) {
            line(GEO_CHUNK, k.screen_x(x));
        }
    }
    for x in [0.0, w.width_units] {
        if k.visible(x, 0.0) {
            line(GEO_EDGE, k.screen_x(x));
        }
    }
    line(GEO_CAMERA, k.center);
}
