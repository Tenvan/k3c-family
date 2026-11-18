//! Port von `native/k3c-game/menu/layout.go`.
//!
//! Lage und Farben der Menü-Seiten als reine Rechnung, Vorbild `src/scenes/pauseHub.ts` und `paperCard.ts`: Karten aus
//! Papier mit Schlagschatten auf Nachtblau, Beschriftung auf einem Leinen-Schild. Alle Maße folgen der Fensterhöhe,
//! damit 720p und 1080p gleich aussehen.

use crate::view::cell::Rectangle;

/// Rgba ist Gos `color.RGBA`: vormultipliziert, je Kanal ein `u8`.
pub type Rgba = [u8; 4];

/// MAX_ROWS ist die Höchstzahl sichtbarer Zeilen einer Seite (Lobby: 7; längere Listen rollen).
pub const MAX_ROWS: usize = 7;

/// Material sind die drei Töne eines Papiers (wie `MATERIAL` in `paperCard.ts`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Material {
    pub licht: Rgba,
    pub basis: Rgba,
    pub schatten: Rgba,
}

/// Gos `rgb`: 0xRRGGBB, deckend.
pub const fn rgb(v: u32) -> Rgba {
    [(v >> 16) as u8, (v >> 8) as u8, v as u8, 255]
}

const fn mat(licht: u32, basis: u32, schatten: u32) -> Material {
    Material {
        licht: rgb(licht),
        basis: rgb(basis),
        schatten: rgb(schatten),
    }
}

pub const MAT_GOLD: Material = mat(0xffe36e, 0xf2c33a, 0xc48a1e);
pub const MAT_GREEN: Material = mat(0x5fd18a, 0x2f9a5c, 0x1d6640);
pub const MAT_BLUE: Material = mat(0x8796f2, 0x4f5fd0, 0x33368f);
pub const MAT_RED: Material = mat(0xe2574c, 0xb8312f, 0x7c1f2a);
pub const MAT_PURPLE: Material = mat(0xa865b8, 0x7c3a8a, 0x5a2466);
pub const MAT_CORAL: Material = mat(0xe8a070, 0xc0703c, 0x8a4a24);
pub const MAT_CLOTH: Material = mat(0xaaa4b2, 0x857f8e, 0x625c6b);
pub const MAT_LINEN: Material = mat(0xf3ead4, 0xe0d2b0, 0xbba98a);

pub const COL_NIGHT: Rgba = rgb(0x0b1026);
/// über dem stehenden Spiel (Pause), vormultipliziert
pub const COL_DIM: Rgba = [0, 0, 0, 160];
/// Schrift auf Leinen
pub const COL_LINE: Rgba = rgb(0x2a1d2e);
/// Schrift ausgegraut
pub const COL_LINE_OFF: Rgba = rgb(0x7a7060);
pub const COL_TITLE: Rgba = rgb(0xffe36e);
pub const COL_TEXT: Rgba = rgb(0xf3ead4);
pub const COL_BAD: Rgba = rgb(0xff7a6e);
/// Leinen-Schild der gewählten Karte, goldhell
pub const COL_PICKED: Rgba = rgb(0xfff4c2);

/// Der weiche Schlagschatten (wie `SHADOW_LAYERS`): Versatz in px bei 1080p und Deckkraft.
pub const SHADOW_LAYERS: [(f64, f64); 3] = [(15.0, 0.1), (9.0, 0.15), (4.0, 0.24)];

/// Geom sind die Maße einer Seite im Fenster `win`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Geom {
    pub win: Rectangle,
    /// Maßstab gegenüber 1080p
    pub k: f64,
    pub title_y: f64,
    pub sub_y: f64,
    pub row_x: f64,
    pub row_w: f64,
    pub row_top: f64,
    pub row_h: f64,
    pub row_gap: f64,
    pub note_y: f64,
    pub hint_y: f64,
    pub title_px: f64,
    pub row_px: f64,
    pub small_px: f64,
}

/// Rechnet die Maße der Seite für das Fenster `win`.
#[allow(clippy::cast_precision_loss)]
pub fn geom_of(win: Rectangle) -> Geom {
    let (h, w) = (win.dy() as f64, win.dx() as f64);
    let (x0, y0) = (win.min.x as f64, win.min.y as f64);
    let row_w = (0.6 * w).min(1.4 * h);
    Geom {
        win,
        k: h / 1080.0,
        title_y: y0 + 0.07 * h,
        sub_y: y0 + 0.15 * h,
        row_w,
        row_x: x0 + (w - row_w) / 2.0,
        row_top: y0 + 0.21 * h,
        row_h: 0.078 * h,
        row_gap: 0.014 * h,
        note_y: y0 + 0.865 * h,
        hint_y: y0 + 0.93 * h,
        title_px: 0.065 * h,
        row_px: 0.04 * h,
        small_px: 0.034 * h,
    }
}

impl Geom {
    /// Das Rechteck `(x, y, w, h)` der sichtbaren Zeile `i` (0 = oben).
    #[allow(clippy::cast_precision_loss)]
    pub fn row_rect(&self, i: i64) -> (f64, f64, f64, f64) {
        (
            self.row_x,
            self.row_top + i as f64 * (self.row_h + self.row_gap),
            self.row_w,
            self.row_h,
        )
    }

    /// Der Rand zwischen Karte und Leinen-Schild.
    pub fn strip_inset(&self) -> f64 {
        (10.0 * self.k).max(3.0)
    }

    /// Der Kasten der Rückfrage `(x, y, w, h)` in der Mitte des Fensters.
    #[allow(clippy::cast_precision_loss)]
    pub fn ask_rect(&self) -> (f64, f64, f64, f64) {
        let (w, h) = (self.row_w * 1.05, 3.6 * self.row_h);
        (
            self.win.min.x as f64 + (self.win.dx() as f64 - w) / 2.0,
            self.win.min.y as f64 + (self.win.dy() as f64 - h) / 2.0,
            w,
            h,
        )
    }

    /// Breite des Auswahl-Rahmens um eine Karte (kleiner als der Zeilenabstand: berührt keine Nachbarin).
    pub fn frame_width(&self) -> f64 {
        0.9 * self.row_gap
    }
}

/// Die Vergrößerung der Debug-Schrift (16 px hoch) für die Schriftgröße `size`, mindestens 1.
pub fn text_scale(size: f64) -> f64 {
    (size / 16.0).round().max(1.0)
}

/// Liefert `c` mit Deckkraft `a` (vormultipliziert); `as u8` schneidet ab wie Gos `uint8(...)`.
pub fn fade(c: Rgba, a: f64) -> Rgba {
    [
        (f64::from(c[0]) * a) as u8,
        (f64::from(c[1]) * a) as u8,
        (f64::from(c[2]) * a) as u8,
        (255.0 * a) as u8,
    ]
}
