//! Port von `native/k3c-game/hud/drawglyph.go`.
//!
//! Zeichnen der Tasten-Glyphen (B-911): mit geladenem Papierschnitt-Icon die Xbox-Taste als Bild bzw. die leere
//! Tastenkappe mit der Beschriftung in Spielschrift darauf; ohne Icon malt das HUD die Glyphe selbst (Kreis, Kappe,
//! Steuerkreuz) wie vor den Icons. Die breite Kappe wächst mit der Beschriftung (drei Streifen, Ecken unverzerrt); passt
//! die Beschriftung auch dann nicht, wird die Schrift kleiner, nie abgeschnitten. Nur Xbox-Symbole und Tastenkappen
//! (AC-02), kein Zweig für andere Gerätefamilien.
//!
//! Öffentliche Schnittstelle (für `keyline.rs`, Skill-Menü und Seekarte, RU23): `Hud::glyph_width` misst eine Glyphe
//! der Höhe `gh` (Beschriftung im Element `slot` des Spielers `p`), `Hud::draw_glyph` zeichnet sie links an `x`, mittig
//! an `y_mid` und liefert dieselbe Breite. Beide wählen selbst zwischen Icon und gemalter Glyphe. Alles liegt auf der
//! Inhalts-Ebene (`Z_CONTENT`).
//!
//! Abweichung von Go: Die breite Kappe schneidet Rust nicht in drei Bilder (`cap_cut`), sondern zeichnet drei
//! `src`-Rechtecke derselben Textur (`Painter::stamp` je Teil), das Ergebnis ist gleich. `Hud::cap_of`/`cap_cut` entfallen.

use k3c_platform::{Canvas, Rect as PRect};

use super::Hud;
use super::drawstock::{Icon, draw_icon};
use super::fonts::{BLACK, Rgba, WHITE};
use super::glyphs::{CAP_WIDE, Glyph, glyph_colors};
use super::layout::Rect;
use super::paint::{Painter, TextSlot};
use crate::art::Font;

/// Maße der Kappen je Glyph-Höhe (Bild 32 px hoch): Schriftgröße der Beschriftung (Textfläche 21 px hoch), Textfläche
/// der schmalen Kappe (23 px breit), Rand der breiten Kappe links und rechts zusammen (Kante 48 − 41 px und etwas Luft),
/// ihre Breite von (wie das Bild) bis (gestreckt) und die Mitte der Textfläche von oben (y 3,5 … 25; darunter liegt der
/// Kappenrand). `PAINT_LABEL_K` ist die Schriftgröße in der gemalten Kappe (ohne Icon, wie vor den Icons).
pub const GLYPH_LABEL_K: f64 = 0.54;
pub const PAINT_LABEL_K: f64 = 0.5;
pub const CAP_NARROW_TEXT: f64 = 23.0 / 32.0;
pub const CAP_WIDE_MARGIN: f64 = 12.0 / 32.0;
pub const CAP_WIDE_MIN: f64 = 48.0 / 32.0;
pub const CAP_WIDE_MAX: f64 = 3.25;
pub const CAP_TEXT_MID: f64 = 14.25 / 32.0;
/// Ecke der breiten Kappe in Bild-Pixeln, bleibt beim Strecken unverzerrt
pub const CAP_CORNER_PX: f64 = 4.0;

/// is_key_glyph: g ist eine Tastatur-Taste (Kappe mit Beschriftung).
pub fn is_key_glyph(g: &Glyph) -> bool {
    g.key.starts_with("key:")
}

/// is_shoulder: name ist eine Schultertaste (gemalt als Kappe mit ihrem Namen).
pub fn is_shoulder(name: &str) -> bool {
    matches!(name, "LB" | "RB" | "LT" | "RT")
}

/// pad_name ist der Teil des Schlüssels nach dem Doppelpunkt (Go: `strings.Cut`, ohne Doppelpunkt leer).
fn pad_name(g: &Glyph) -> &str {
    g.key.split_once(':').map_or("", |(_, n)| n)
}

/// colors: Füllung und Schrift der Glyph-Art name (None: unbekannt).
fn colors(name: &str) -> Option<[Rgba; 2]> {
    glyph_colors().into_iter().find(|(n, _)| *n == name).map(|(_, c)| c)
}

/// cap_box ist die Breite der Tastenkappe der Höhe gh und die Schriftgröße ihrer Beschriftung, die bei gh·`GLYPH_LABEL_K`
/// tw breit ist: Die schmale Kappe bleibt quadratisch, die breite wächst bis `CAP_WIDE_MAX`; reicht das nicht,
/// schrumpft die Schrift auf die Textfläche. Mit der Debug-Schrift (fixed, feste Größe) wächst die breite Kappe
/// unbegrenzt.
pub fn cap_box(gh: f64, tw: f64, wide: bool, fixed: bool) -> (f64, f64) {
    let mut px = gh * GLYPH_LABEL_K;
    if !wide {
        let a = gh * CAP_NARROW_TEXT;
        if tw > a && !fixed {
            px *= a / tw;
        }
        return (gh, px);
    }
    let m = gh * CAP_WIDE_MARGIN;
    let mut w = f64::max(gh * CAP_WIDE_MIN, tw + m);
    if !fixed && w > gh * CAP_WIDE_MAX {
        w = gh * CAP_WIDE_MAX;
        px *= (w - m) / tw;
    }
    (w, px)
}

/// is_fixed: die Schrift ist die Debug-Schrift mit fester Größe (Go: `f.Fallback`).
pub fn is_fixed(f: Option<&Font>) -> bool {
    f.is_some_and(|f| f.fallback)
}

impl Hud {
    /// glyph_icon liefert das geladene Papierschnitt-Icon von g (None: nicht geladen oder ohne Bild, dann wird gemalt).
    pub fn glyph_icon(&self, g: &Glyph) -> Option<Icon> {
        self.icon(&g.icon)
    }

    /// glyph_width ist die Breite der Glyph g bei Höhe gh, wie `draw_glyph` sie zeichnet; die Beschriftung misst es im
    /// Element slot des Spielers p.
    pub fn glyph_width(&mut self, f: Option<&Font>, p: i64, slot: TextSlot, g: &Glyph, gh: f64) -> f64 {
        let name = pad_name(g);
        let icon = self.glyph_icon(g).is_some();
        if is_key_glyph(g) && icon {
            let (tw, _) = self.paint.measure_text(f, p, slot, &g.label, gh * GLYPH_LABEL_K);
            return cap_box(gh, tw, g.icon == CAP_WIDE, is_fixed(f)).0;
        }
        if is_key_glyph(g) {
            let (tw, _) = self.paint.measure_text(f, p, slot, &g.label, gh * PAINT_LABEL_K);
            return f64::max(gh, tw + gh * 0.5);
        }
        if is_shoulder(name) && !icon {
            let (tw, _) = self.paint.measure_text(f, p, slot, name, gh * PAINT_LABEL_K);
            return f64::max(gh, tw + gh * 0.5);
        }
        gh
    }

    /// draw_glyph zeichnet eine Glyph der Höhe gh, links an x, mittig an y_mid, und liefert ihre Breite: mit geladenem
    /// Icon das Papierschnitt-Bild (`icon_glyph`), sonst gemalt (`paint_glyph`).
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn draw_glyph(
        &mut self,
        dst: &mut dyn Canvas,
        f: Option<&mut Font>,
        p: i64,
        slot: TextSlot,
        g: &Glyph,
        x: f64,
        y_mid: f64,
        gh: f64,
    ) -> f64 {
        let Some(ic) = self.glyph_icon(g) else {
            return self.paint_glyph(dst, f, p, slot, g, x, y_mid, gh);
        };
        let (mut tw, mut th) = (0.0, 0.0);
        if is_key_glyph(g) {
            (tw, th) = self
                .paint
                .measure_text(f.as_deref(), p, slot, &g.label, gh * GLYPH_LABEL_K);
        }
        self.icon_glyph(dst, f, &ic, g, x, y_mid, gh, tw, th)
    }

    /// icon_glyph zeichnet g mit dem Icon ic: die Xbox-Taste als Quadrat, die Tastenkappe mit der Beschriftung (dunkel,
    /// ohne Kontur) mittig in ihrer Textfläche. tw und th sind die Maße der Beschriftung bei gh·`GLYPH_LABEL_K`.
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn icon_glyph(
        &mut self,
        dst: &mut dyn Canvas,
        f: Option<&mut Font>,
        ic: &Icon,
        g: &Glyph,
        x: f64,
        y_mid: f64,
        gh: f64,
        tw: f64,
        th: f64,
    ) -> f64 {
        let y = y_mid - gh / 2.0;
        if !is_key_glyph(g) {
            draw_icon(dst, ic, x, y, gh);
            return gh;
        }
        let (w, px) = cap_box(gh, tw, g.icon == CAP_WIDE, is_fixed(f.as_deref()));
        self.draw_cap(dst, ic, x, y, w, gh);
        let s = px / (gh * GLYPH_LABEL_K);
        let clr = colors("key").map_or(BLACK, |c| c[1]);
        self.paint.label(
            dst,
            f,
            &g.label,
            x + (w - tw * s) / 2.0,
            y + gh * CAP_TEXT_MID - th * s / 2.0,
            px,
            clr,
        );
        w
    }

    /// draw_cap zeichnet die Kappe ic auf (x, y, w, gh): in ihrer eigenen Breite als ein Bild, breiter in drei Streifen
    /// (Ecken unverzerrt, Mitte gestreckt), je ein `src`-Rechteck derselben Textur.
    pub fn draw_cap(&mut self, dst: &mut dyn Canvas, ic: &Icon, x: f64, y: f64, w: f64, gh: f64) {
        if ic.fw as f64 <= 2.0 * CAP_CORNER_PX || ic.fh <= 0 {
            return;
        }
        let (tex, b) = (ic.frame.tex, ic.frame.src);
        let pt = &self.paint;
        let s = gh / ic.fh as f64;
        if w <= ic.fw as f64 * s + 0.5 {
            pt.stamp(dst, tex, b, Rect::new(x, y, w, gh), WHITE);
            return;
        }
        let k = CAP_CORNER_PX as f32;
        let parts = [
            PRect::new(b.x, b.y, k, b.h),
            PRect::new(b.x + k, b.y, b.w - 2.0 * k, b.h),
            PRect::new(b.x + b.w - k, b.y, k, b.h),
        ];
        let c = CAP_CORNER_PX * s;
        pt.stamp(dst, tex, parts[0], Rect::new(x, y, c, gh), WHITE);
        pt.stamp(dst, tex, parts[1], Rect::new(x + c, y, w - 2.0 * c, gh), WHITE);
        pt.stamp(dst, tex, parts[2], Rect::new(x + w - c, y, c, gh), WHITE);
    }

    /// paint_glyph malt eine Glyph ohne Icon: Kreis mit Buchstabe (A, X, Y) bzw. Menu-Strichen, Kappe mit Beschriftung
    /// (Schultertasten, Tastatur), Steuerkreuz (UP, DOWN).
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn paint_glyph(
        &mut self,
        dst: &mut dyn Canvas,
        f: Option<&mut Font>,
        p: i64,
        slot: TextSlot,
        g: &Glyph,
        x: f64,
        y_mid: f64,
        gh: f64,
    ) -> f64 {
        let name = pad_name(g);
        let pad = colors("pad").unwrap_or([WHITE; 2]);
        if is_key_glyph(g) || is_shoulder(name) {
            let (cols, label) = if is_key_glyph(g) {
                (colors("key").unwrap_or(pad), g.label.as_str())
            } else {
                (pad, name)
            };
            let (tw, th) = self
                .paint
                .measure_text(f.as_deref(), p, slot, label, gh * PAINT_LABEL_K);
            let w = f64::max(gh, tw + gh * 0.5);
            let pt = &mut self.paint;
            pt.fill_top(dst, Rect::new(x, y_mid - gh / 2.0, w, gh), cols[0]);
            pt.frame(dst, Rect::new(x, y_mid - gh / 2.0, w, gh), 2.0, BLACK);
            pt.text(
                dst,
                f,
                label,
                x + (w - tw) / 2.0,
                y_mid - th / 2.0,
                gh * PAINT_LABEL_K,
                cols[1],
            );
            return w;
        }
        if name == "UP" || name == "DOWN" {
            paint_d_pad(dst, &self.paint, name == "UP", x, y_mid, gh);
            return gh;
        }
        let cols = colors(name).unwrap_or(pad);
        self.paint.circle(dst, x + gh / 2.0, y_mid, gh / 2.0, BLACK);
        self.paint.circle(dst, x + gh / 2.0, y_mid, gh / 2.0 - 2.0, cols[0]);
        if name == "MENU" {
            for dy in [-0.18, 0.0, 0.18] {
                let r = Rect::new(x + gh * 0.28, y_mid + dy * gh - 1.5, gh * 0.44, 3.0);
                self.paint.fill_top(dst, r, cols[1]);
            }
            return gh;
        }
        let (tw, th) = self.paint.measure_text(f.as_deref(), p, slot, name, gh * 0.6);
        self.paint
            .text(dst, f, name, x + (gh - tw) / 2.0, y_mid - th / 2.0, gh * 0.6, cols[1]);
        gh
    }
}

/// paint_d_pad malt das Steuerkreuz der Höhe gh mit der Marke oben (up) oder unten.
pub fn paint_d_pad(dst: &mut dyn Canvas, pt: &Painter, up: bool, x: f64, y_mid: f64, gh: f64) {
    let s = gh / 3.0;
    let pad = colors("pad").unwrap_or([WHITE; 2]);
    pt.fill_top(dst, Rect::new(x + s - 2.0, y_mid - gh / 2.0, s + 4.0, gh), BLACK);
    pt.fill_top(dst, Rect::new(x - 2.0, y_mid - s / 2.0 - 2.0, gh + 4.0, s + 4.0), BLACK);
    pt.fill_top(dst, Rect::new(x + s, y_mid - gh / 2.0 + 2.0, s, gh - 4.0), pad[0]);
    pt.fill_top(dst, Rect::new(x, y_mid - s / 2.0, gh, s), pad[0]);
    let ty = if up {
        y_mid - gh / 2.0 + 5.0
    } else {
        y_mid + s / 2.0 + 1.0
    };
    pt.fill_top(dst, Rect::new(x + s + 3.0, ty, s - 6.0, s - 6.0), pad[1]);
}
