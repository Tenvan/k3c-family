//! Port von `native/k3c-game/game/layout.go`: Zellen des Split-Screens und Kamera-Führung.

use crate::view::MAX_PLAYERS;
use crate::view::cell::{Camera, Cell, Rectangle, new_camera, rect};
use crate::view::frame::Frame;
use crate::view::roles::CameraRule;

/// Sichtbare Welthöhe über dem Boden in Units im Look-Profil `go` (wie NV3: 420 px Welt bei 32 px je Unit); das Profil
/// `k2c` zeigt mehr (`data/look.json › viewUnits`). Der Maßstab einer Zelle folgt daraus: Höhe des Welt-Teils / Units.
pub const VIEW_UNITS: f64 = 420.0 / 32.0;

/// Lage der Zelle `i` von `n` im Fenster `win`: eine Zelle füllt das Fenster, zwei liegen übereinander (wie der
/// Browser, `src/scenes/layout.ts`); die untere bekommt bei ungerader Höhe den Rest.
pub fn cell_rect(win: Rectangle, n: usize, i: usize) -> Rectangle {
    if n <= 1 {
        return win;
    }
    let mid = win.min.y + win.dy() / 2;
    if i == 0 {
        rect(win.min.x, win.min.y, win.max.x, mid)
    } else {
        rect(win.min.x, mid, win.max.x, win.max.y)
    }
}

/// Maßstab der Zelle `c` in Bildpunkten je Unit bei einer Ansicht.
pub fn scale(c: &Cell) -> f64 {
    scale_n(c, 1)
}

/// Maßstab der Zelle `c`, wenn `n` Zellen im Fenster liegen: Höhe des Welt-Teils / `view_units` des Look-Profils
/// (RU55.2); im Split-Screen mal `split_scale · n`, nie unter der Mindestgröße des Monarchen (`min_figure_px`).
#[allow(clippy::cast_precision_loss)]
pub fn scale_n(c: &Cell, n: usize) -> f64 {
    k3c_data::look::current().cell_scale(c.world_rect().dy() as f64, n)
}

/// Baut die Zellen der Plätze in `f` (höchstens `MAX_PLAYERS`; Zelle i zeigt `f.seats[i]`) in `dst`, mit Lage und
/// Maßstab; `camera.x` ist 0, bis `Cameras::follow` es setzt. Mit Kapazität `MAX_PLAYERS` legt `cells` nichts an.
#[allow(clippy::cast_possible_wrap)]
pub fn cells(dst: &mut Vec<Cell>, win: Rectangle, f: &Frame) {
    dst.clear();
    let n = f.seats.len().min(MAX_PLAYERS);
    for (i, seat) in f.seats.iter().take(n).enumerate() {
        let mut c = Cell {
            index: i as i64,
            player: seat.player,
            rect: cell_rect(win, n, i),
            camera: Camera::default(),
        };
        c.camera = new_camera(c.rect, 0.0, scale_n(&c, n));
        dst.push(c);
    }
}

/// Legt die Zellen neu auf das Fenster `win`, falls es sich seit dem Update geändert hat (Fenstergröße zwischen zwei
/// Updates); das Welt-x der Kamera bleibt. Ohne Änderung bleibt die Kamera der Regel unberührt.
pub fn place(cells: &mut [Cell], win: Rectangle) {
    let n = cells.len();
    for (i, c) in cells.iter_mut().enumerate() {
        let r = cell_rect(win, n, i);
        if r != c.rect {
            c.rect = r;
            c.camera = new_camera(r, c.camera.x, scale_n(c, n));
        }
    }
}

/// Führt die Kameras der Zellen über die Updates: Zustand je Spieler (`Seat::player`), Regel aus dem Paket world,
/// feste Werte des Szenen-Schalters `--cam` je Zelle.
#[derive(Default)]
pub struct Cameras {
    /// `None` = folgt dem Monarchen, ohne Monarchen der Burg
    pub rule: Option<Box<dyn CameraRule>>,
    /// Welt-x je Zelle, NaN = Regel
    pub fixed: Vec<f64>,
    prev: [Camera; MAX_PLAYERS],
}

impl Cameras {
    /// Setzt die Kamera jeder Zelle (einmal je Update): die Regel mit der Kamera des Spielers aus dem vorigen Update,
    /// danach feste Werte; ein NaN- oder unendliches Ergebnis der Regel gilt nicht. Spieler ohne Zelle vergessen ihre
    /// Kamera (beim nächsten Auftauchen springt die Regel).
    pub fn follow(&mut self, cells: &mut [Cell], f: &Frame) {
        let mut next = [Camera::default(); MAX_PLAYERS];
        for c in cells.iter_mut() {
            let Some(p) = usize::try_from(c.player).ok().filter(|p| *p < MAX_PLAYERS) else {
                continue;
            };
            let prev = self.prev[p];
            c.camera.x = prev.x;
            match self.rule.as_mut() {
                None => c.camera.x = default_x(f, c),
                Some(rule) => {
                    let cam = rule.follow(f, c, prev);
                    if cam.x.is_finite() {
                        c.camera = cam;
                    }
                }
            }
            if let Some(&x) = usize::try_from(c.index).ok().and_then(|i| self.fixed.get(i))
                && !x.is_nan()
            {
                c.camera.x = x;
            }
            next[p] = c.camera;
        }
        self.prev = next;
    }
}

/// Welt-x der Kamera ohne Regel: Monarch, sonst Burg, sonst 0.
fn default_x(f: &Frame, c: &Cell) -> f64 {
    if let Some(m) = f.monarch(c.player) {
        return m.x;
    }
    f.world(c).map_or(0.0, |w| w.castle.x)
}

#[cfg(test)]
#[path = "layout_tests.rs"]
mod layout_tests;
