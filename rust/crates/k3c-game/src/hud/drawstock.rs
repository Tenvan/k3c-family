//! Port von `native/k3c-game/hud/drawstock.go`: Vorrats-Anzeige mit Icons (B-890): je Rohstoff Icon und Zahl in einer
//! Fläche, Skill-Punkte als Text dahinter. Ein voller Rohstoff steht in Orange. Fehlt das Icon (ohne Grafik), steht der
//! Name vor der Zahl.
//!
//! Abweichung von Go: `Hud::icon` liefert eine Kopie des ersten Bilds (`Icon`) statt eines Zeigers auf das Sheet, damit
//! die Bibliothek beim Zeichnen von Text (Schrift, veränderlich) nicht ausgeliehen bleibt; `Icons` sammelt die Icons
//! eines Zeichendurchlaufs vor dem Zugriff auf die Schrift.

use std::borrow::Cow;

use k3c_platform::{Affine, Canvas};

use super::Hud;
use super::draw::{BOX_ALPHA, BOX_PAD_X, BOX_PAD_Y, box_size, space_w};
use super::fonts::{ROLE_SHARED, Rgba, rgb};
use super::icons::{ICON_ANIM, StockPart, resource_icon};
use super::layout::Rect;
use super::paint::TextSlot;
use super::purse::PURSE_ASSET;
use super::state::SeatState;
use super::values::{RESOURCE_ORDER, res_name};
use crate::art::{Font, SheetFrame};
use crate::view::seal::SEAL_ART;

/// ICON_SCALE ist die Kantenlänge eines Icons als Vielfaches der Schriftgröße (lesbar aus TV-Abstand).
pub const ICON_SCALE: f64 = 1.5;

/// full_color ist die Farbe eines vollen Rohstoffs.
pub fn full_color() -> Rgba {
    rgb(0xf4a261, 1.0)
}

/// Icon ist das erste Bild eines Icon-Sheets samt Maßen.
#[derive(Clone, Copy, Debug)]
pub struct Icon {
    pub frame: SheetFrame,
    pub fw: i64,
    pub fh: i64,
}

/// Icons sind die Icons, die ein Zeichendurchlauf des HUD braucht (None ohne Grafik oder wenn nicht geladen).
#[derive(Clone, Copy, Debug, Default)]
pub struct Icons {
    pub purse: Option<Icon>,
    /// je Rohstoff in der Reihenfolge von `RESOURCE_ORDER`
    pub res: [Option<Icon>; RESOURCE_ORDER.len()],
}

impl Icons {
    /// of liefert das Icon des Rohstoffs res (None für "" und unbekannte).
    pub fn of(&self, res: &str) -> Option<Icon> {
        RESOURCE_ORDER.iter().position(|r| *r == res).and_then(|i| self.res[i])
    }
}

impl Hud {
    /// icon liefert das Icon asset (None ohne Grafik oder wenn es nicht geladen ist).
    pub fn icon(&self, asset: &str) -> Option<Icon> {
        if asset.is_empty() {
            return None;
        }
        // Beim Zeichnen hält `draw_cell` die Bibliothek (Schrift) geliehen: Tasten-Icons kommen dann aus dem Cache
        let Ok(lib) = self.lib.as_ref()?.try_borrow() else {
            return self.icon_cache.iter().find(|(a, _)| *a == asset).map(|(_, ic)| *ic);
        };
        let s = lib.sheet(asset, ICON_ANIM)?;
        Some(Icon {
            frame: *s.frame(0)?,
            fw: s.fw,
            fh: s.fh,
        })
    }

    /// refresh_icon_cache legt die Tasten-Icons für den Zeichendurchlauf zurück (vor dem Ausleihen der Bibliothek).
    pub fn refresh_icon_cache(&mut self) {
        let mut cache = std::mem::take(&mut self.icon_cache);
        cache.clear();
        cache.extend(
            super::icons::KEY_ASSETS
                .iter()
                .chain(SEAL_ART.iter().map(|(_, a)| a)) // Siegel des Grads (ECO1.2) ebenso
                .filter_map(|a| self.icon(a).map(|ic| (*a, ic))),
        );
        self.icon_cache = cache;
    }

    /// icons sammelt die Icons für einen Zeichendurchlauf.
    pub fn icons(&self) -> Icons {
        Icons {
            purse: self.icon(PURSE_ASSET),
            res: std::array::from_fn(|i| self.icon(resource_icon(RESOURCE_ORDER[i]))),
        }
    }

    /// part_label ist der Text eines Teils: die Zahl, ohne Icon mit dem Namen davor.
    fn part_label<'a>(&self, p: &'a StockPart, icons: &Icons) -> (Cow<'a, str>, Option<Icon>) {
        let ic = icons.of(&p.res);
        if ic.is_none() && !p.res.is_empty() {
            return (Cow::Owned(format!("{} {}", res_name(&self.lang, &p.res), p.text)), None);
        }
        (Cow::Borrowed(p.text.as_str()), ic)
    }

    /// stock_size ist die Größe der Vorrats-Fläche bei Schriftgröße px und Maßstab k.
    pub fn stock_size(
        &mut self,
        st: &SeatState,
        f: Option<&Font>,
        icons: &Icons,
        player: i64,
        px: f64,
        k: f64,
    ) -> (f64, f64) {
        if st.parts.is_empty() {
            return (0.0, 0.0);
        }
        let (mut w, mut ht) = (0.0, px * ICON_SCALE);
        for (i, p) in st.parts.iter().enumerate() {
            let (label, ic) = self.part_label(p, icons);
            let (tw, th) = self.paint.measure_text(f, player, TextSlot::stock(i), &label, px);
            if ic.is_some() {
                w += px * ICON_SCALE + space_w(px);
            }
            w += tw + 3.0 * space_w(px);
            ht = f64::max(ht, th);
        }
        box_size(w, ht, k)
    }

    /// draw_stock zeichnet die Vorrats-Fläche in r.
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn draw_stock(
        &mut self,
        dst: &mut dyn Canvas,
        st: &SeatState,
        mut f: Option<&mut Font>,
        icons: &Icons,
        player: i64,
        r: Rect,
        px: f64,
        k: f64,
    ) {
        self.paint.fill(dst, r, rgb(0, BOX_ALPHA));
        let (mut x, y) = (r.x + BOX_PAD_X * k, r.y + BOX_PAD_Y * k);
        let inner = r.h - 2.0 * BOX_PAD_Y * k;
        for (i, p) in st.parts.iter().enumerate() {
            let (label, ic) = self.part_label(p, icons);
            if let Some(ic) = ic {
                let s = px * ICON_SCALE;
                draw_icon(dst, &ic, x, y + (inner - s) / 2.0, s);
                x += s + space_w(px);
            }
            let (tw, th) = self
                .paint
                .measure_text(f.as_deref(), player, TextSlot::stock(i), &label, px);
            let clr = if p.full { full_color() } else { ROLE_SHARED.clr };
            self.paint
                .text(dst, f.as_deref_mut(), &label, x, y + (inner - th) / 2.0, px, clr);
            x += tw + 3.0 * space_w(px);
        }
    }
}

/// draw_icon zeichnet Bild 0 des Icons ic als Quadrat der Kante s mit der linken oberen Ecke (x, y).
pub fn draw_icon(dst: &mut dyn Canvas, ic: &Icon, x: f64, y: f64, s: f64) {
    if ic.fw <= 0 || ic.fh <= 0 {
        return;
    }
    dst.layer(super::paint::Z_CONTENT);
    let mut g = Affine::IDENTITY;
    g.scale((s / ic.fw as f64) as f32, (s / ic.fh as f64) as f32)
        .translate(x as f32, y as f32);
    dst.sprite_tf(ic.frame.tex, ic.frame.src, g, [1.0; 4]);
}
