//! Port von `native/k3c-game/hud/drawworld.go`: Zeichnen an Orten der Welt (Aktions-Hinweis und geführte erste Nacht über
//! dem Monarchen bzw. Objekt, Berufe und Händler als Labels) und des Radars. Welt-Texte erscheinen in fester Größe
//! (Browser: scaleTexts gleicht den Zoom aus).
//!
//! Abweichung von Go: Die Schrift kommt als `Option<&mut Font>` herein (`draw_cell` hält die Bibliothek geliehen);
//! `guide_layout` steht hier statt in `draw.rs` (dort war sie bis RU23.5 vorgezogen).

use k3c_platform::Canvas;

use super::Hud;
use super::draw::{BOX_ALPHA, BOX_PAD_X, BOX_PAD_Y};
use super::fonts::{ROLE_CITIZEN, ROLE_HINT, ROLE_PLAYER, Rgba, font_px, rgb};
use super::hints::{GuideIcon, HINT_ABOVE_UNITS, HINT_TOP_FREE_REF_PIXEL, guide_height, guide_image};
use super::layout::Rect;
use super::paint::{MAX_LABELS, TextSlot};
use super::radar::{RadarKind, radar_view};
use super::state::{Label, PLAYER_COLORS, SeatState};
use crate::art::Font;
use crate::view::cell::Cell;

/// Radar-Farben (radarView.ts).
const RADAR_CASTLE_C: u32 = 0xffd166;
const RADAR_PORTAL_C: u32 = 0xb388ff;
const RADAR_EXIT_C: u32 = 0x6bd96b;
const RADAR_ENEMY_C: u32 = 0xff4d4d;

/// Farben der Bilder der Anleitung: Münze (Gold wie das Münzbild) und Papiermond auf Nachtblau (paperBackdrop.ts).
const COIN_FILL: u32 = 0xffd23c;
const COIN_RIM: u32 = 0xc48a1e;
const MOON_LIGHT: u32 = 0xf3ead4;
const MOON_NIGHT: u32 = 0x0b1026;

/// LabelSpot ist ein sichtbares Label: Index in `SeatState::labels` und Fenster-Punkt.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LabelSpot {
    pub i: usize,
    pub x: f64,
    pub y: f64,
}

/// screen_at ist der Fenster-Punkt `h` Units über dem Boden bei Welt-x `x` in der Zelle; oben bleibt Platz frei (Home-
/// Button des Browsers, hier die oberen Zeilen). None außerhalb der Zelle.
pub fn screen_at(c: &Cell, x: f64, h: f64, k: f64) -> Option<(f64, f64)> {
    let sx = c.camera.screen_x(x);
    if sx < c.rect.min.x as f64 || sx > c.rect.max.x as f64 {
        return None;
    }
    Some((
        sx,
        (c.rect.min.y as f64 + HINT_TOP_FREE_REF_PIXEL * k).max(c.screen_y(h)),
    ))
}

/// visible_labels hängt an `dst` die Lage der Labels, die in der Zelle sichtbar sind, in Reihenfolge von `labels`
/// (Händler zuerst), höchstens `MAX_LABELS`: Das Budget zählt erst nach der Sichtprüfung.
pub fn visible_labels(mut dst: Vec<LabelSpot>, labels: &[Label], c: &Cell, k: f64) -> Vec<LabelSpot> {
    for (i, l) in labels.iter().enumerate() {
        if dst.len() >= MAX_LABELS {
            break;
        }
        if let Some((x, y)) = screen_at(c, l.x, l.y, k) {
            dst.push(LabelSpot { i, x, y });
        }
    }
    dst
}

/// guide_layout legt Bild und Text eines Hinweises der Breite `tw` und Höhe `th` mittig über (x, y) (Unterkante): mit
/// Bild steht links ein Quadrat der Texthöhe, dann ein Abstand, dann der Text; ohne Bild nur der Text (rein, getestet).
pub fn guide_layout(x: f64, y: f64, tw: f64, th: f64, icon: bool, k: f64) -> (Rect, f64, f64) {
    let ty = y - th;
    if !icon {
        return (Rect::default(), x - tw / 2.0, ty);
    }
    let (s, gap_x) = (th, 8.0 * k);
    let left = x - (s + gap_x + tw) / 2.0;
    (Rect::new(left, ty, s, s), left + s + gap_x, ty)
}

impl Hud {
    /// draw_world_texts zeichnet Labels, Aktions-Hinweis und Hinweis der ersten Nacht der Zelle.
    pub fn draw_world_texts(
        &mut self,
        dst: &mut dyn Canvas,
        st: &SeatState,
        c: &Cell,
        k: f64,
        mut f: Option<&mut Font>,
    ) {
        let mut spots = std::mem::take(&mut self.spots);
        spots.clear();
        spots = visible_labels(spots, &st.labels, c, k);
        for (n, sp) in spots.iter().enumerate() {
            let l = &st.labels[sp.i];
            if l.dot != 0 {
                self.paint.circle(dst, sp.x, sp.y, 12.0 * k * 0.5, rgb(l.dot, 1.0));
            }
            let size = ROLE_CITIZEN.px * k * 0.75;
            let at = (sp.x, sp.y - 10.0 * k);
            self.centered(
                dst,
                f.as_deref_mut(),
                c.player,
                TextSlot::label(n),
                &l.text,
                at,
                size,
                ROLE_CITIZEN.clr,
                k,
            );
        }
        self.spots = spots;
        if !st.hint_s.is_empty()
            && let Some((x, y)) = screen_at(c, st.x, HINT_ABOVE_UNITS, k)
        {
            let size = ROLE_HINT.px * k;
            self.centered(
                dst,
                f.as_deref_mut(),
                c.player,
                TextSlot::HINT,
                &st.hint_s,
                (x, y),
                size,
                ROLE_HINT.clr,
                k,
            );
        }
        if !st.guide_s.is_empty()
            && let Some((x, y)) = screen_at(c, st.guide.x, guide_height(&st.guide), k)
        {
            self.draw_guide(dst, f, c.player, st, (x, y), k);
        }
    }

    /// draw_guide zeichnet den Hinweis der ersten Nacht: Bild (Münze, Mond) und Text, mit der Debug-Schrift auf dunkler
    /// Fläche.
    pub fn draw_guide(
        &mut self,
        dst: &mut dyn Canvas,
        f: Option<&mut Font>,
        p: i64,
        st: &SeatState,
        at: (f64, f64),
        k: f64,
    ) {
        let size = ROLE_PLAYER.px * k;
        let (tw, th) = self
            .paint
            .measure_text(f.as_deref(), p, TextSlot::GUIDE, &st.guide_s, size);
        if tw == 0.0 {
            return;
        }
        let icon = guide_image(st.guide.id, self.paint.ready());
        let (img, tx, ty) = guide_layout(at.0, at.1, tw, th, icon != GuideIcon::None, k);
        if f.as_deref().is_some_and(|f| f.fallback) {
            let r = Rect::new(
                tx - BOX_PAD_X * k,
                ty - BOX_PAD_Y * k,
                tw + 2.0 * BOX_PAD_X * k,
                th + 2.0 * BOX_PAD_Y * k,
            );
            self.paint.fill(dst, r, rgb(0, BOX_ALPHA));
        }
        self.draw_icon(dst, icon, img);
        self.paint.text(dst, f, &st.guide_s, tx, ty, size, ROLE_PLAYER.clr);
    }

    /// draw_icon zeichnet das Bild der Anleitung in `r`: Münze mit Rand und Glanz, Mond als Sichel auf einer Nachtscheibe.
    pub fn draw_icon(&self, dst: &mut dyn Canvas, icon: GuideIcon, r: Rect) {
        let pt = &self.paint;
        let (cx, cy, rad) = (r.x + r.w / 2.0, r.y + r.h / 2.0, r.w / 2.0);
        match icon {
            GuideIcon::Coin => {
                pt.circle(dst, cx, cy, rad, rgb(COIN_RIM, 1.0));
                pt.circle(dst, cx, cy, rad * 0.78, rgb(COIN_FILL, 1.0));
                pt.circle(dst, cx - rad * 0.25, cy - rad * 0.25, rad * 0.22, rgb(0xffffff, 0.6));
            }
            GuideIcon::Moon => {
                pt.circle(dst, cx, cy, rad, rgb(MOON_NIGHT, 0.9));
                pt.circle(dst, cx, cy, rad * 0.72, rgb(MOON_LIGHT, 1.0));
                pt.circle(dst, cx - rad * 0.3, cy - rad * 0.18, rad * 0.62, rgb(MOON_NIGHT, 1.0));
            }
            GuideIcon::None => {}
        }
    }

    /// centered zeichnet einen Text mittig über `at` = (x, y) (Unterkante); mit der Debug-Schrift auf dunkler Fläche.
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn centered(
        &mut self,
        dst: &mut dyn Canvas,
        f: Option<&mut Font>,
        p: i64,
        slot: TextSlot,
        s: &str,
        at: (f64, f64),
        size: f64,
        clr: Rgba,
        k: f64,
    ) {
        let (w, ht) = self.paint.measure_text(f.as_deref(), p, slot, s, size);
        if w == 0.0 {
            return;
        }
        let (tx, ty) = (at.0 - w / 2.0, at.1 - ht);
        if f.as_deref().is_some_and(|f| f.fallback) {
            let r = Rect::new(
                tx - BOX_PAD_X * k,
                ty - BOX_PAD_Y * k,
                w + 2.0 * BOX_PAD_X * k,
                ht + 2.0 * BOX_PAD_Y * k,
            );
            self.paint.fill(dst, r, rgb(0, BOX_ALPHA));
        }
        self.paint.text(dst, f, s, tx, ty, size, clr);
    }

    /// draw_radar zeichnet Leiste, Marker und Kamera-Ausschnitt.
    pub fn draw_radar(
        &mut self,
        dst: &mut dyn Canvas,
        st: &SeatState,
        c: &Cell,
        r: Rect,
        k: f64,
        f: Option<&mut Font>,
    ) {
        if !st.biomes_s.is_empty() {
            // Biome der Insel über der Leiste, mittig (LA1.3)
            let size = font_px(ROLE_HINT, self.cells as i64) * k;
            let (w, h) = self
                .paint
                .measure_text(f.as_deref(), c.player, TextSlot::BIOMES, &st.biomes_s, size);
            let box_r = Rect::new(
                r.x + (r.w - w) / 2.0 - BOX_PAD_X * k,
                r.y - h - 2.0 * BOX_PAD_Y * k - 3.0 * k,
                w + 2.0 * BOX_PAD_X * k,
                h + 2.0 * BOX_PAD_Y * k,
            );
            if let Some(ic) = self.icon(st.seal) {
                // Siegel des Schwierigkeitsgrads links neben der Zeile (ECO1.2), so hoch wie die Zeile
                let s = box_r.h;
                super::drawstock::draw_icon(dst, &ic, box_r.x - s - 2.0 * k, box_r.y, s);
            }
            self.text_box(dst, f, box_r, &st.biomes_s, size, ROLE_HINT.clr, k);
        }
        let pt = &self.paint;
        pt.fill(dst, r, rgb(0, 0.5));
        pt.frame(dst, r, 1.0, rgb(0xffffff, 0.35));
        let y = r.y + r.h / 2.0;
        for m in &st.radar {
            let x = r.x + m.pos * r.w;
            let tri = Rect::new(x - 6.0 * k, y - 7.0 * k, 12.0 * k, 13.0 * k);
            match m.kind {
                RadarKind::Player => {
                    let a = if m.down { 0.4 } else { 1.0 };
                    let mut rad = 6.0 * k;
                    if m.own {
                        rad = 8.0 * k;
                        pt.ring_at(dst, x, y, rad + k, rgb(0xffffff, a));
                    }
                    let clr = PLAYER_COLORS[m.index.rem_euclid(PLAYER_COLORS.len() as i64) as usize];
                    pt.circle(dst, x, y, rad, rgb(clr, a));
                }
                RadarKind::Enemy => pt.circle(dst, x, y, 3.0 * k, rgb(RADAR_ENEMY_C, 1.0)),
                RadarKind::Castle => pt.fill_top(
                    dst,
                    Rect::new(x - 5.0 * k, y - 5.0 * k, 10.0 * k, 10.0 * k),
                    rgb(RADAR_CASTLE_C, 1.0),
                ),
                RadarKind::Portal => pt.triangle(dst, tri, rgb(RADAR_PORTAL_C, 1.0)),
                RadarKind::Exit | RadarKind::Stairs => pt.triangle(dst, tri, rgb(RADAR_EXIT_C, 1.0)),
            }
        }
        let (from, to) = radar_view(&c.camera, st.width_units);
        let view = Rect::new(r.x + from * r.w, r.y + 1.0, f64::max(2.0, (to - from) * r.w), r.h - 2.0);
        pt.frame(dst, view, 2.0 * k, rgb(0xffffff, 0.9));
    }
}
