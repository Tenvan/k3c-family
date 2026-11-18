//! Port von `native/k3c-game/game/overlay.go`: Anzeige oben rechts in `view::cell::stats_rect`: mit der Schrift des
//! Spiels (`art::Font`) rechtsbündig auf einer dunklen Fläche, die nur so groß ist wie der Text; beide Zeilen bleiben in
//! `stats_rect` (NG13: die vergrößerte Debug-Schrift verdeckte die Kopfzeile des HUD). Ohne Schriftdatei (oder mit
//! Rückfall-Schrift) wie vorher die Pixel-Schrift der Plattform, vergrößert gezeichnet. Im Fenster die FPS (Text
//! höchstens viermal je Sekunde Spielzeit neu), im Aufnahme-Modus der Takt statt der FPS, damit das Bild nicht von der
//! Wanduhr abhängt. Mit `Settings::stats` (F2, B-816) kommt eine zweite Zeile dazu: Rechenzeit von Update und Draw in ms
//! (Mittel seit dem letzten Text). Die Wanduhr läuft nur dann, ausgeschaltet kostet die Anzeige keine Zeitnahme; im
//! Aufnahme-Modus stehen an ihrer Stelle feste Striche.
//!
//! Abweichungen: Go zeichnet die Pixel-Schrift in ein eigenes Bild und skaliert es; Rust zeichnet Fläche (`Canvas::rect`)
//! und Text (`Canvas::text`, Zeichenhöhe 12 · Maßstab) in Go-Reihenfolge (erst Fläche, dann Text). Die FPS kommt als
//! Parameter von `draw` (Go: `ebiten.ActualFPS`), die Schrift als `Option<&mut Font>` (sie gehört der Bibliothek, Go
//! teilte den Zeiger).

use std::fmt::Write;

use k3c_platform::{Canvas, Rect};

use crate::art::{Font, LINE_SPACING};
use crate::view::cell::{Rectangle, rect, stats_rect, stats_text_size_lines};
use crate::view::frame::Frame;

use super::clock::clock;

/// Sekunden Spielzeit zwischen zwei FPS-Texten. Lage und Größe: `stats_rect_lines`.
pub const OVERLAY_INTERVAL: f64 = 0.25;

/// Fläche hinter dem Text: Schwarz mit 170 von 255 Alpha.
const OVERLAY_BACK: [f32; 4] = [0.0, 0.0, 0.0, 170.0 / 255.0];

/// Farbe des Texts der Anzeige.
const STATS_COLOR: [u8; 4] = [255, 255, 255, 255];

/// Rand um den Text je px Schriftgröße.
pub const STATS_PAD: f64 = 0.2;

/// Zeichenhöhe der Pixel-Schrift in der 16er-Zeile der Debug-Schrift.
const DEBUG_PX: f32 = 12.0;

/// Mittelt Zeiten in Millisekunden.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Avg {
    pub sum: f64,
    pub n: i64,
}

impl Avg {
    pub fn add(&mut self, sec: f64) {
        self.sum += sec * 1000.0;
        self.n += 1;
    }

    /// Liefert das Mittel seit dem letzten Aufruf (0 ohne Werte) und beginnt neu.
    pub fn take(&mut self) -> f64 {
        if self.n == 0 {
            return 0.0;
        }
        let v = self.sum / self.n as f64;
        *self = Avg::default();
        v
    }
}

/// RectF ist ein Rechteck in Fenster-Pixeln.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RectF {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl RectF {
    fn of(r: Rectangle) -> Self {
        Self {
            x: r.min.x as f64,
            y: r.min.y as f64,
            w: r.dx() as f64,
            h: r.dy() as f64,
        }
    }
}

/// stats_px ist die Schriftgröße der Anzeige: zwei Zeilen samt Rand passen in die Höhe von `StatsRect` r.
pub fn stats_px(r: RectF) -> f64 {
    r.h / (2.0 * LINE_SPACING + 2.0 * STATS_PAD)
}

/// stats_box ist die Fläche hinter dem Text: rechts oben in r, so breit wie die längste der `lines` Zeilen (Breiten
/// `tw`, Zeilenhöhe `th`) plus Rand (rein, getestet).
pub fn stats_box(r: RectF, tw: [f64; 2], th: f64, lines: i64, px: f64) -> RectF {
    let mut w = tw[0];
    if lines == 2 {
        w = w.max(tw[1]);
    }
    let pad = STATS_PAD * px;
    let (bw, bh) = (w + 2.0 * pad, lines as f64 * th + 2.0 * pad);
    RectF {
        x: r.x + r.w - bw,
        y: r.y,
        w: bw,
        h: bh,
    }
}

/// stats_text liefert die Zeilen der Anzeige: FPS bzw. Takt (`recording`), mit `on` dazu Update und Draw in ms (im
/// Aufnahme-Modus Striche); l2 ist sonst leer.
pub fn stats_text(recording: bool, on: bool, fps: f64, tick: u64, upd: f64, drw: f64) -> (String, String) {
    let (mut l1, mut l2) = (String::new(), String::new());
    stats_text_into(&mut l1, &mut l2, recording, on, fps, tick, upd, drw);
    (l1, l2)
}

/// Wie `stats_text`, schreibt aber in die (geleerten) Puffer `l1` und `l2`: nach dem ersten Text ohne Allokation.
#[allow(clippy::too_many_arguments)] // eine Zeile je Wert der Anzeige, wie Go `statsText`
pub fn stats_text_into(
    l1: &mut String,
    l2: &mut String,
    recording: bool,
    on: bool,
    fps: f64,
    tick: u64,
    upd: f64,
    drw: f64,
) {
    l1.clear();
    l2.clear();
    // String-Schreiben schlägt nie fehl
    let _ = if recording {
        write!(l1, "T {tick:7}")
    } else {
        write!(l1, "{fps:4.0} FPS")
    };
    match (on, recording) {
        (true, true) => l2.push_str("U --- D ---"),
        (true, false) => {
            let _ = write!(l2, "U{upd:4.1} D{drw:4.1}");
        }
        _ => {}
    }
}

/// StatsOverlay zeichnet die Leistungsanzeige (Go: `overlay`).
pub struct StatsOverlay {
    /// Schrift des Spiels (nicht der Rückfall) steht beim Zeichnen zur Verfügung; sonst Pixel-Schrift
    use_font: bool,
    /// aktueller Text (Zeile 1 und 2)
    text1: [String; 2],
    /// gemessene Breiten der Zeilen bei `px` (Zwischenspeicher)
    tw: [f64; 2],
    /// gemessene Zeilenhöhe und die Größe, für die `tw` und `th` gelten
    th: f64,
    px: f64,
    /// true: FPS, false: Takt (Aufnahme-Modus)
    fps: bool,
    next: f64,
    tick: u64,
    /// Zeilen des aktuellen Texts (0: noch keiner)
    lines: i64,
    /// Messmodus: keine eigene Zeitmessung, sonst verfälscht die Leistungsanzeige die Messung
    pub measuring: bool,
    /// Wanduhr in Sekunden, nur für die Leistungsanzeige (`clock`)
    pub now: Box<dyn Fn() -> f64>,
    /// Rechenzeit von Update und Draw seit dem letzten Text
    pub upd: Avg,
    pub drw: Avg,
}

impl StatsOverlay {
    /// Baut die Anzeige; `font` None oder Rückfall zeichnet mit der Pixel-Schrift.
    pub fn new(fps: bool, font: Option<&Font>) -> Self {
        Self {
            use_font: font.is_some_and(|f| !f.fallback),
            text1: [String::new(), String::new()],
            tw: [0.0; 2],
            th: 0.0,
            px: 0.0,
            fps,
            next: 0.0,
            tick: u64::MAX,
            lines: 0,
            measuring: false,
            now: Box::new(clock),
            upd: Avg::default(),
            drw: Avg::default(),
        }
    }

    /// timing meldet, ob Update und Draw gemessen werden: nur mit Leistungsanzeige und nicht im Aufnahme-Modus (dort
    /// hängt das Bild nicht von der Wanduhr ab).
    pub fn timing(&self, f: &Frame) -> bool {
        f.settings.stats && !f.recording && !self.measuring
    }

    /// time_update führt `run` aus und rechnet die Zeit in `upd` ein, aber nur bei `timing`; sonst fragt es die Wanduhr
    /// nie ab (Go: `Game.advanceTimed`, Zweig `stats.timing`).
    pub fn time_update<R>(&mut self, f: &Frame, run: impl FnOnce() -> R) -> R {
        if !self.timing(f) {
            return run();
        }
        let t0 = (self.now)();
        let r = run();
        self.upd.add((self.now)() - t0);
        r
    }

    /// due meldet, ob der Text neu entsteht: beim Wechsel der Zeilenzahl, dann nach Spielzeit (FPS) bzw. Takt.
    fn due(&self, f: &Frame, lines: i64) -> bool {
        if lines != self.lines {
            return true;
        }
        if self.fps {
            return f.clock >= self.next;
        }
        f.tick != self.tick
    }

    /// Setzt den Text bei Bedarf neu und zeichnet ihn in `stats_rect_lines`; `fps` ist die gemessene Bildrate.
    pub fn draw(&mut self, dst: &mut dyn Canvas, f: &Frame, fps: f64, font: Option<&mut Font>) {
        let lines = if f.settings.stats { 2 } else { 1 };
        if self.due(f, lines) {
            (self.next, self.tick, self.lines) = (f.clock + OVERLAY_INTERVAL, f.tick, lines);
            let (u, d) = (self.upd.take(), self.drw.take());
            let [t1, t2] = &mut self.text1;
            stats_text_into(t1, t2, !self.fps, lines == 2, fps, f.tick, u, d);
            self.px = 0.0;
        }
        let (w, h) = dst.size();
        let win = rect(0, 0, w as i64, h as i64);
        match font {
            Some(font) if self.use_font => self.draw_font(dst, font, win, lines),
            _ => self.draw_pixel(dst, win, lines),
        }
    }

    /// Zeichnet den Text mit der Schrift des Spiels rechtsbündig in `stats_rect`; gemessen wird nur bei neuem Text oder
    /// neuer Größe.
    fn draw_font(&mut self, dst: &mut dyn Canvas, font: &mut Font, win: Rectangle, lines: i64) {
        let r = RectF::of(stats_rect(win));
        let px = stats_px(r);
        if self.px != px {
            self.px = px;
            (self.tw[0], self.th) = font.measure(&self.text1[0], px);
            self.tw[1] = font.measure(&self.text1[1], px).0;
        }
        let b = stats_box(r, self.tw, self.th, lines, px);
        dst.rect(Rect::new(b.x as f32, b.y as f32, b.w as f32, b.h as f32), OVERLAY_BACK);
        let pad = STATS_PAD * px;
        for i in 0..lines as usize {
            let x = b.x + b.w - pad - self.tw[i];
            font.draw(dst, &self.text1[i], x, b.y + pad + i as f64 * self.th, px, STATS_COLOR);
        }
    }

    /// Ohne Schrift: Pixel-Schrift auf dunkler Fläche, im Maßstab `k` der Anzeige (Go: Textbild, vergrößert).
    fn draw_pixel(&self, dst: &mut dyn Canvas, win: Rectangle, lines: i64) {
        let (iw, ih, k) = stats_text_size_lines(lines);
        let r = stats_rect(win);
        let (x, y, k) = (r.min.x as f32, r.min.y as f32, k as f32);
        dst.rect(Rect::new(x, y, iw as f32 * k, ih as f32 * k), OVERLAY_BACK);
        dst.text(&self.text1[0], x + 4.0 * k, y + k, DEBUG_PX * k, [1.0; 4]);
        if lines == 2 {
            dst.text(&self.text1[1], x + 4.0 * k, y + 17.0 * k, DEBUG_PX * k, [1.0; 4]);
        }
    }
}

#[cfg(test)]
#[path = "overlay_tests.rs"]
mod overlay_tests;
