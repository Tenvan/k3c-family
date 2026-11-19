//! Zellen und Kameras der Wiedergabe nach `native/k3c-game/game/layout.go` (nur gelesen, kein Port: die Spielschleife
//! portiert RU26). Zwei Zellen liegen übereinander, jede Kamera folgt über die Kamera-Regel der Welt (`world::Far`).

use crate::view::MAX_PLAYERS;
use crate::view::cell::{Camera, Cell, Rectangle, new_camera, rect};
use crate::view::frame::Frame;
use crate::view::roles::CameraRule;

/// Sichtbare Welthöhe über dem Boden in Units (Go: `ViewUnits`).
pub const VIEW_UNITS: f64 = 420.0 / 32.0;

/// Lage der Zelle i von n im Fenster win: eine füllt das Fenster, zwei liegen übereinander (Go: `CellRect`).
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

/// Maßstab der Zelle in Bildpunkten je Unit (Go: `Scale`).
#[allow(clippy::cast_precision_loss)]
pub fn scale(c: &Cell) -> f64 {
    crate::game::layout::scale_n(c, 1)
}

/// Baut die Zellen der Plätze in `f` nach `dst` (Go: `Cells`); `Camera.x` ist 0, bis `Cameras::follow` es setzt.
pub fn cells(dst: &mut Vec<Cell>, win: Rectangle, f: &Frame) {
    dst.clear();
    let n = f.seats.len().min(MAX_PLAYERS);
    for i in 0..n {
        let mut c = Cell {
            index: i as i64,
            player: f.seats[i].player,
            rect: cell_rect(win, n, i),
            ..Cell::default()
        };
        c.camera = new_camera(c.rect, 0.0, crate::game::layout::scale_n(&c, n));
        dst.push(c);
    }
}

/// Kameras über die Takte: Regel der Welt, Kamera des Spielers aus dem vorigen Takt (Go: `Cameras`).
#[derive(Default)]
pub struct Cameras {
    prev: [Camera; MAX_PLAYERS],
}

impl Cameras {
    /// Setzt die Kamera jeder Zelle einmal je Takt (Go: `Cameras.Follow`, ohne feste Werte von `--cam`).
    pub fn follow(&mut self, rule: &mut Option<Box<dyn CameraRule>>, cells: &mut [Cell], f: &Frame) {
        let mut next = [Camera::default(); MAX_PLAYERS];
        for c in cells.iter_mut() {
            let Ok(p) = usize::try_from(c.player) else { continue };
            if p >= MAX_PLAYERS {
                continue;
            }
            let prev = self.prev[p];
            c.camera.x = prev.x;
            match rule.as_mut() {
                None => c.camera.x = default_x(f, c),
                Some(r) => {
                    let cam = r.follow(f, c, prev);
                    if cam.x.is_finite() {
                        c.camera = cam;
                    }
                }
            }
            next[p] = c.camera;
        }
        self.prev = next;
    }
}

/// Welt-x ohne Regel: Monarch, sonst Burg, sonst 0 (Go: `defaultX`).
fn default_x(f: &Frame, c: &Cell) -> f64 {
    if let Some(m) = f.monarch(c.player) {
        return m.x;
    }
    f.world(c).map_or(0.0, |w| w.castle.x)
}
