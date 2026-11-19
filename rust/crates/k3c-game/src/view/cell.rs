//! Port von `native/k3c-game/view/cell.go`.
//!
//! `image.Rectangle` und `image.Point` aus Go stehen hier als [`Rectangle`] und [`Point`] (ganze Fensterpixel, `i64`);
//! [`Rectangle::to_rect`] liefert das Rechteck der Plattform für `Canvas::clip`.

use std::sync::OnceLock;

use k3c_platform::Rect;

/// Point ist ein Punkt im Fenster in px (wie `image.Point`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

/// Rectangle ist ein Rechteck im Fenster in px (wie `image.Rectangle`): Min einschließlich, Max ausschließlich.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Rectangle {
    pub min: Point,
    pub max: Point,
}

/// rect ist `image.Rect`: legt das Rechteck aus zwei Ecken an und tauscht vertauschte Koordinaten.
pub fn rect(x0: i64, y0: i64, x1: i64, y1: i64) -> Rectangle {
    let (x0, x1) = if x0 > x1 { (x1, x0) } else { (x0, x1) };
    let (y0, y1) = if y0 > y1 { (y1, y0) } else { (y0, y1) };
    Rectangle {
        min: Point { x: x0, y: y0 },
        max: Point { x: x1, y: y1 },
    }
}

impl Rectangle {
    /// Breite in px (wie `Rectangle.Dx`).
    pub fn dx(&self) -> i64 {
        self.max.x - self.min.x
    }

    /// Höhe in px (wie `Rectangle.Dy`).
    pub fn dy(&self) -> i64 {
        self.max.y - self.min.y
    }

    /// to_rect liefert das Rechteck der Plattform (Fensterkoordinaten), z. B. für `Canvas::clip` beim Zeichnen einer Zelle.
    #[allow(clippy::cast_precision_loss)]
    pub fn to_rect(&self) -> Rect {
        Rect::new(self.min.x as f32, self.min.y as f32, self.dx() as f32, self.dy() as f32)
    }
}

/// Welt-Anteil im Look-Profil `go` (obere zwei Drittel wie NV3); gilt, solange keine Quelle gesetzt ist.
const GO_SHARE: f64 = 2.0 / 3.0;

static SHARE_SOURCE: OnceLock<fn() -> f64> = OnceLock::new();

/// Setzt die Quelle des Welt-Anteils (einmal; die Welt meldet sie beim Aufbau: Look-Profil aus `data/look.json`).
/// `view` kennt `k3c_data` nicht (Importregel), die Quelle ist darum eine Funktion von außen.
pub fn set_world_share_source(f: fn() -> f64) {
    let _ = SHARE_SOURCE.set(f);
}

/// WorldShare ist der Anteil der Welt an der Zellenhöhe (Boden-Linie an der Grenze), aus dem Look-Profil
/// (RU55.1): `k2c` 58 %, `go` obere zwei Drittel wie NV3; der Rest ist Ufer und Fluss.
pub fn world_share() -> f64 {
    SHARE_SOURCE.get().map_or(GO_SHARE, |f| f())
}

/// Camera bildet Welt-x (Units) auf Fenster-x (px) einer Zelle ab. Reine Rechnung; X setzt game je Update über die
/// CameraRule, Scale, Center und Half folgen aus der Lage der Zelle.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Camera {
    /// Welt-x in Units in der Mitte der Zelle (CameraRule bzw. --cam)
    pub x: f64,
    /// Bildpunkte je Unit
    pub scale: f64,
    /// Fenster-x der Zellenmitte in px
    pub center: f64,
    /// halbe Zellenbreite in px
    pub half: f64,
}

/// NewCamera legt die Kamera mit Welt-x x und Maßstab scale auf das Rechteck r.
#[allow(clippy::cast_precision_loss)]
pub fn new_camera(r: Rectangle, x: f64, scale: f64) -> Camera {
    let half = r.dx() as f64 / 2.0;
    Camera {
        x,
        scale,
        center: r.min.x as f64 + half,
        half,
    }
}

impl Camera {
    /// ScreenX liefert das Fenster-x des Welt-x wx (Boden-Ebene, Parallaxe 1).
    pub fn screen_x(&self, wx: f64) -> f64 {
        (wx - self.x) * self.scale + self.center
    }

    /// ParallaxX liefert das Fenster-x des Welt-x wx auf einer Ebene mit Scroll-Faktor factor (0 = steht, 1 = Boden).
    pub fn parallax_x(&self, wx: f64, factor: f64) -> f64 {
        (wx - self.x * factor) * self.scale + self.center
    }

    /// WorldX ist die Umkehrung von ScreenX.
    pub fn world_x(&self, sx: f64) -> f64 {
        if self.scale == 0.0 {
            return self.x;
        }
        (sx - self.center) / self.scale + self.x
    }

    /// Span liefert den sichtbaren Welt-Bereich in Units (links, rechts).
    pub fn span(&self) -> (f64, f64) {
        if self.scale == 0.0 {
            return (self.x, self.x);
        }
        let w = self.half / self.scale;
        (self.x - w, self.x + w)
    }

    /// Visible meldet, ob ein Objekt bei Welt-x x mit halber Breite half (Units) in die Zelle ragt.
    pub fn visible(&self, x: f64, half: f64) -> bool {
        let (l, r) = self.span();
        x + half >= l && x - half <= r
    }
}

/// Cell ist eine Zelle des Split-Screens: Rechteck im Fenster, Spieler und Kamera. Welt in den oberen zwei Dritteln,
/// Fluss im unteren Drittel.
/// Zelle i zeigt Frame.Seats[i]; Index und Player sind verschieden, sobald Spieler 0 fehlt.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cell {
    /// Lage: 0 = oben, 1 = unten (Zahl der Zellen = len(Frame.Seats)); -1 = WindowCell
    pub index: i64,
    /// lokaler Spieler (Seat.Player) dieser Zelle, nicht Index; -1 = keiner (Fenster)
    pub player: i64,
    /// Lage im Fenster in px
    pub rect: Rectangle,
    pub camera: Camera,
}

// Anzeige oben rechts (game: FPS bzw. im Aufnahme-Modus der Takt): Debug-Schrift statsW × statsH px, statsScale-fach,
// statsMargin px vom Rand.
const STATS_W: i64 = 76;
const STATS_H: i64 = 18;
const STATS_SCALE: i64 = 5;
const STATS_MARGIN: i64 = 16;

/// StatsRect ist die Fläche der Anzeige oben rechts im Fenster win; game zeichnet dort, die Drawer halten sie frei.
/// Sie liegt in der obersten Zelle (gleiche Oberkante und Breite wie das Fenster).
pub fn stats_rect(win: Rectangle) -> Rectangle {
    let (w, h) = (STATS_W * STATS_SCALE, STATS_H * STATS_SCALE);
    let x = win.max.x - w - STATS_MARGIN;
    rect(x, win.min.y + STATS_MARGIN, x + w, win.min.y + STATS_MARGIN + h)
}

/// StatsTextSize ist die Größe des Textbilds der Anzeige vor der Vergrößerung (Debug-Schrift) und der Faktor.
pub fn stats_text_size() -> (i64, i64, i64) {
    (STATS_W, STATS_H, STATS_SCALE)
}

/// statsLine ist die Höhe einer weiteren Textzeile der Debug-Schrift in px.
const STATS_LINE: i64 = 16;

/// StatsRectLines ist StatsRect mit lines Textzeilen (Leistungsanzeige, Settings.Stats: zwei Zeilen); die weiteren Zeilen
/// reichen nach unten über StatsRect hinaus. lines < 1 zählt als 1.
pub fn stats_rect_lines(win: Rectangle, lines: i64) -> Rectangle {
    let mut r = stats_rect(win);
    r.max.y += (lines.max(1) - 1) * STATS_LINE * STATS_SCALE;
    r
}

/// StatsTextSizeLines ist StatsTextSize für lines Textzeilen.
pub fn stats_text_size_lines(lines: i64) -> (i64, i64, i64) {
    let (w, h, scale) = stats_text_size();
    (w, h + (lines.max(1) - 1) * STATS_LINE, scale)
}

/// WindowCell ist die Zelle für LayerOverlay und Screens: das ganze Fenster, ohne Spieler.
pub fn window_cell(win: Rectangle) -> Cell {
    Cell {
        index: -1,
        player: -1,
        rect: win,
        camera: new_camera(win, 0.0, 1.0),
    }
}

impl Cell {
    /// GroundY ist das Fenster-y der Boden-Linie (Grenze Welt/Fluss).
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn ground_y(&self) -> i64 {
        // Go: int(float64(Dy)*WorldShare + 0.5) schneidet zur Null hin ab wie `as i64`.
        self.rect.min.y + (self.rect.dy() as f64 * world_share() + 0.5) as i64
    }

    /// WorldRect ist der Welt-Teil der Zelle (bis zur Boden-Linie).
    pub fn world_rect(&self) -> Rectangle {
        rect(self.rect.min.x, self.rect.min.y, self.rect.max.x, self.ground_y())
    }

    /// RiverRect ist der Teil der Zelle unter der Boden-Linie (Ufer und Fluss).
    pub fn river_rect(&self) -> Rectangle {
        rect(self.rect.min.x, self.ground_y(), self.rect.max.x, self.rect.max.y)
    }

    /// ScreenY liefert das Fenster-y einer Höhe h in Units über dem Boden.
    #[allow(clippy::cast_precision_loss)]
    pub fn screen_y(&self, h: f64) -> f64 {
        self.ground_y() as f64 - h * self.camera.scale
    }
}

#[cfg(test)]
#[path = "cell_tests.rs"]
mod cell_tests;
