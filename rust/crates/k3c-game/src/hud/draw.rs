//! Port von `native/k3c-game/hud/draw.go`: Zeichnen der HUD-Elemente einer Zelle: Größen messen, Lage über `layout`
//! (gemerkt, nur bei Änderung neu gerechnet), dann Fläche und Inhalt je Element.
//!
//! Die Geschichtszeile (`story_size`, `draw_story`) zeichnet `drawstory.rs`, Welt-Texte und Radar `drawworld.rs`.

use k3c_platform::Canvas;

use super::Hud;
use super::drawstock::Icons;
use super::fonts::{
    FontRole, ROLE_BANNER, ROLE_BOSS, ROLE_FIGHT, ROLE_JOIN, ROLE_PLAYER, ROLE_ROOM_INFO, ROLE_SHARED, ROLE_TRAVEL,
    Rgba, WHITE, font_px, rgb, scale_of,
};
use super::layout::*;
use super::paint::TextSlot;
use super::purse::{draw_purse, purse_side};
use super::radar::radar_size;
use super::state::SeatState;
use crate::art::Font;
use crate::view::cell::{Cell, Rectangle, stats_rect};

/// Stil der Elemente laut 🧑 (2026-10-10): Schwarz mit 55 % Deckkraft, Innenabstand 8/4 px, Breite in Stufen von 32 px,
/// damit wechselnde Texte (Uhr, Abklingzeit) das Layout nicht jedes Bild neu anstoßen.
pub const BOX_ALPHA: f64 = 0.55;
pub const BOX_PAD_X: f64 = 8.0;
pub const BOX_PAD_Y: f64 = 4.0;
pub const WIDTH_STEP: f64 = 32.0;

/// Maße des Boss-Balkens in px bei 1080 (HudScene.ts).
const BOSS_BAR_W: f64 = 640.0;
const BOSS_BAR_H: f64 = 22.0;

/// TextItem ist ein Element aus einem Text des Spielers.
struct TextItem {
    id: ItemId,
    slot: TextSlot,
    role: FontRole,
}

/// TEXT_ITEMS in der Reihenfolge von Go (`textItems`).
const TEXT_ITEMS: [TextItem; 8] = [
    TextItem {
        id: ITEM_BANNER,
        slot: TextSlot::BANNER,
        role: ROLE_BANNER,
    },
    TextItem {
        id: ITEM_SKILLS,
        slot: TextSlot::SKILL_BAR,
        role: ROLE_PLAYER,
    },
    TextItem {
        id: ITEM_CLOCK,
        slot: TextSlot::CLOCK,
        role: ROLE_SHARED,
    },
    TextItem {
        id: ITEM_FIGHT,
        slot: TextSlot::FIGHT,
        role: ROLE_FIGHT,
    },
    TextItem {
        id: ITEM_PLACE,
        slot: TextSlot::PLACE,
        role: ROLE_ROOM_INFO,
    },
    TextItem {
        id: ITEM_CITIZENS,
        slot: TextSlot::CIT,
        role: ROLE_SHARED,
    },
    TextItem {
        id: ITEM_EVENTS,
        slot: TextSlot::EVENTS,
        role: ROLE_BANNER,
    },
    TextItem {
        id: ITEM_TRAVEL,
        slot: TextSlot::TRAVEL,
        role: ROLE_TRAVEL,
    },
];

/// text_of ist der Text des Elements id im Zustand st (Go: `textItem.get`).
fn text_of(st: &SeatState, id: ItemId) -> &str {
    match id {
        ITEM_BANNER => &st.banner.showing,
        ITEM_SKILLS => &st.bar_s,
        ITEM_CLOCK => &st.clock_s,
        ITEM_FIGHT => &st.fight_s,
        ITEM_PLACE => &st.place_s,
        ITEM_CITIZENS => &st.cit_s,
        ITEM_EVENTS => &st.ev_s,
        ITEM_TRAVEL => &st.travel_s,
        _ => "",
    }
}

/// LayoutCache merkt das Layout einer Zelle zu seinen Eingaben.
#[derive(Clone, Debug, Default)]
pub struct LayoutCache {
    inp: Items,
    area: Rect,
    free: Rect,
    k: f64,
    out: Placed,
    set: bool,
}

/// rect_of rechnet ein ganzzahliges Fensterrechteck in px (Go: `rectOf`).
pub fn rect_of(r: Rectangle) -> Rect {
    Rect::new(r.min.x as f64, r.min.y as f64, r.dx() as f64, r.dy() as f64)
}

/// box_size ist die Größe der Fläche um einen Text der Größe (w, h) beim Maßstab k.
pub fn box_size(w: f64, h: f64, k: f64) -> (f64, f64) {
    if w <= 0.0 {
        return (0.0, 0.0);
    }
    let step = WIDTH_STEP * k;
    (
        ((w + 2.0 * BOX_PAD_X * k) / step).ceil() * step,
        h + 2.0 * BOX_PAD_Y * k,
    )
}

/// fit_px verkleinert die Schriftgröße px im Verhältnis, wenn ein Text der Breite w nicht in max_w passt (Meldung über
/// die Breite der Zelle, B-813); sonst bleibt sie. Stufen von 2 px (jede Größe ist eine eigene Schrift-Größe).
pub fn fit_px(px: f64, w: f64, max_w: f64) -> f64 {
    if w <= max_w || w <= 0.0 || max_w <= 0.0 {
        return px;
    }
    f64::max(2.0, (px * max_w / w / 2.0).floor() * 2.0)
}

/// space_w ist die Breite eines Leerzeichens in der Größe size (Abstand der Teile der Spielerkarte).
pub fn space_w(size: f64) -> f64 {
    size * 0.3
}

impl Hud {
    /// draw_items misst, legt ab und zeichnet die Elemente der Zelle c.
    pub fn draw_items(
        &mut self,
        dst: &mut dyn Canvas,
        st: &SeatState,
        c: &Cell,
        k: f64,
        mut f: Option<&mut Font>,
        icons: &Icons,
    ) {
        let (inp, px) = self.measure_items(st, c, k, f.as_deref(), icons);
        let pl = self.layout_for(c, &inp, k);
        let p = c.player;
        for (i, t) in TEXT_ITEMS.iter().enumerate() {
            if pl.ok[t.id] {
                let at = pl.at[t.id];
                self.text_box(dst, f.as_deref_mut(), at, text_of(st, t.id), px[i], t.role.clr, k);
            }
        }
        if pl.ok[ITEM_JOIN] {
            let size = font_px(ROLE_JOIN, self.cells as i64) * k;
            let s = std::mem::take(&mut self.join_s);
            self.text_box(dst, f.as_deref_mut(), pl.at[ITEM_JOIN], &s, size, ROLE_JOIN.clr, k);
            self.join_s = s;
        }
        if pl.ok[ITEM_PLAYER] {
            self.draw_card(dst, st, f.as_deref_mut(), icons, p, pl.at[ITEM_PLAYER], k);
        }
        if pl.ok[ITEM_SHARED] {
            let shared_px = font_px(ROLE_SHARED, self.cells as i64) * k;
            self.draw_stock(dst, st, f.as_deref_mut(), icons, p, pl.at[ITEM_SHARED], shared_px, k);
        }
        if pl.ok[ITEM_BOSS] {
            self.draw_boss(dst, st, f.as_deref_mut(), p, pl.at[ITEM_BOSS], k);
        }
        if pl.ok[ITEM_STORY] {
            self.draw_story(dst, st, f.as_deref_mut(), p, pl.at[ITEM_STORY], k, c.rect.dx() as f64);
        }
        if pl.ok[ITEM_RADAR] {
            self.draw_radar(dst, st, c, pl.at[ITEM_RADAR], k, f);
        }
    }

    /// measure_items misst die Texte und liefert die Größen der Elemente und die Schriftgrößen der Textzeilen.
    fn measure_items(
        &mut self,
        st: &SeatState,
        c: &Cell,
        k: f64,
        f: Option<&Font>,
        icons: &Icons,
    ) -> (Items, [f64; TEXT_ITEMS.len()]) {
        let (p, cells) = (c.player, self.cells as i64);
        let cell_w = c.rect.dx() as f64;
        let mut inp = Items::default();
        let mut px = [0.0; TEXT_ITEMS.len()];
        for (i, t) in TEXT_ITEMS.iter().enumerate() {
            let s = text_of(st, t.id);
            px[i] = font_px(t.role, cells) * k;
            let (mut w, mut ht) = self.paint.measure_text(f, p, t.slot, s, px[i]);
            if t.id == ITEM_BANNER {
                px[i] = fit_px(px[i], w + 2.0 * BOX_PAD_X * k, cell_w - 2.0 * GAP * k);
                (w, ht) = self.paint.measure_text(f, p, t.slot, s, px[i]);
            }
            (inp[t.id].w, inp[t.id].h) = box_size(w, ht, k);
        }
        if c.index == cells - 1 {
            let size = font_px(ROLE_JOIN, cells) * k;
            let (w, ht) = self.paint.measure_text(f, p, TextSlot::JOIN, &self.join_s, size);
            (inp[ITEM_JOIN].w, inp[ITEM_JOIN].h) = box_size(w, ht, k);
        }
        (inp[ITEM_PLAYER].w, inp[ITEM_PLAYER].h) = self.card_size(st, f, p, k);
        let shared_px = font_px(ROLE_SHARED, cells) * k;
        (inp[ITEM_SHARED].w, inp[ITEM_SHARED].h) = self.stock_size(st, f, icons, p, shared_px, k);
        (inp[ITEM_BOSS].w, inp[ITEM_BOSS].h) = self.boss_size(st, f, p, k, cell_w);
        self.story_size(st, f, p, k, cell_w, &mut inp);
        if st.width_units > 0.0 {
            (inp[ITEM_RADAR].w, inp[ITEM_RADAR].h) = radar_size(cell_w, k);
        }
        (inp, px)
    }

    /// layout_for liefert das Layout der Zelle aus dem Cache oder rechnet es neu. Die oberste Zelle lässt die große
    /// Anzeige oben rechts frei (`stats_rect`; Zelle 0 hat Oberkante und Breite des Fensters).
    pub fn layout_for(&mut self, c: &Cell, inp: &Items, k: f64) -> Placed {
        let area = rect_of(c.rect);
        let free = if c.index == 0 {
            rect_of(stats_rect(c.rect))
        } else {
            Rect::default()
        };
        let slot = usize::try_from(c.player).unwrap_or(0).min(self.lay.len() - 1);
        let lc = &mut self.lay[slot];
        if !lc.set || lc.inp != *inp || lc.area != area || lc.free != free || lc.k != k {
            let frees: &[Rect] = if free.w == 0.0 {
                &[]
            } else {
                std::slice::from_ref(&free)
            };
            (lc.inp, lc.area, lc.free, lc.k, lc.set) = (*inp, area, free, k, true);
            lc.out = layout(inp, area, frees, k);
        }
        lc.out
    }

    /// placed liefert die zuletzt gelegten Plätze der Elemente des Spielers p (None vor dem ersten Zeichnen). Für den
    /// Platz-Test des Standbilds (RU20.7); Go liest dafür `h.lay[p].out`.
    pub fn placed(&self, p: usize) -> Option<Placed> {
        self.lay.get(p).filter(|l| l.set).map(|l| l.out)
    }

    /// text_box zeichnet einen Text der Größe size auf seiner Fläche, bündig links.
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn text_box(
        &mut self,
        dst: &mut dyn Canvas,
        f: Option<&mut Font>,
        r: Rect,
        s: &str,
        size: f64,
        clr: Rgba,
        k: f64,
    ) {
        self.paint.fill(dst, r, rgb(0, BOX_ALPHA));
        self.paint
            .text(dst, f, s, r.x + BOX_PAD_X * k, r.y + BOX_PAD_Y * k, size, clr);
    }

    /// card_size: Geldsack (Kante doppelt so groß wie ein Ressourcen-Icon) und Gold nebeneinander, ohne Lebensbalken und
    /// Spielermarke (B-919).
    pub fn card_size(&mut self, st: &SeatState, f: Option<&Font>, p: i64, k: f64) -> (f64, f64) {
        let size = ROLE_PLAYER.px * k;
        let side = purse_side(font_px(ROLE_SHARED, self.cells as i64) * k);
        let (gw, th) = self.paint.measure_text(f, p, TextSlot::GOLD, &st.gold_s, size);
        let w = side + gw + 2.0 * space_w(size) + 2.0 * BOX_PAD_X * k;
        let step = WIDTH_STEP * k;
        ((w / step).ceil() * step, f64::max(th, side) + 2.0 * BOX_PAD_Y * k)
    }

    /// draw_card zeichnet die Spielerkarte: Geldsack in der Größe des Spielers (hervorgehoben nach einer Änderung des
    /// Goldes, B-398) und „Gold/Maximum“.
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn draw_card(
        &mut self,
        dst: &mut dyn Canvas,
        st: &SeatState,
        f: Option<&mut Font>,
        icons: &Icons,
        player: i64,
        r: Rect,
        k: f64,
    ) {
        let size = ROLE_PLAYER.px * k;
        self.paint.fill(dst, r, rgb(0, BOX_ALPHA));
        let side = purse_side(font_px(ROLE_SHARED, self.cells as i64) * k);
        let (mut x, y) = (r.x + BOX_PAD_X * k, r.y + BOX_PAD_Y * k);
        let inner = r.h - 2.0 * BOX_PAD_Y * k;
        let at = Rect::new(x, y + (inner - side) / 2.0, side, side);
        draw_purse(dst, &self.paint, icons.purse.as_ref(), at, st.purse, st.flash, k);
        x += side + space_w(size);
        let (_, th) = self
            .paint
            .measure_text(f.as_deref(), player, TextSlot::GOLD, &st.gold_s, size);
        self.paint
            .text(dst, f, &st.gold_s, x, y + (inner - th) / 2.0, size, WHITE);
    }

    /// boss_size: Titel und Leiste untereinander; ohne Boss 0.
    pub fn boss_size(&mut self, st: &SeatState, f: Option<&Font>, p: i64, k: f64, cell_w: f64) -> (f64, f64) {
        if !st.boss.ok {
            return (0.0, 0.0);
        }
        let (tw, th) = self
            .paint
            .measure_text(f, p, TextSlot::BOSS, &st.boss_s, ROLE_BOSS.px * k);
        let w = (f64::max(tw, BOSS_BAR_W * k) + 2.0 * BOX_PAD_X * k).min(cell_w - 48.0 * k);
        (w, th + 4.0 * k + BOSS_BAR_H * k + 2.0 * BOX_PAD_Y * k)
    }

    /// draw_boss zeichnet Name und Phase, darunter die Lebensleiste (kampfView.ts › drawBar).
    pub fn draw_boss(
        &mut self,
        dst: &mut dyn Canvas,
        st: &SeatState,
        f: Option<&mut Font>,
        player: i64,
        r: Rect,
        k: f64,
    ) {
        self.paint.fill(dst, r, rgb(0, BOX_ALPHA));
        let size = ROLE_BOSS.px * k;
        let (tw, th) = self
            .paint
            .measure_text(f.as_deref(), player, TextSlot::BOSS, &st.boss_s, size);
        self.paint.text(
            dst,
            f,
            &st.boss_s,
            r.x + (r.w - tw) / 2.0,
            r.y + BOX_PAD_Y * k,
            size,
            ROLE_BOSS.clr,
        );
        let bar = Rect::new(
            r.x + BOX_PAD_X * k,
            r.y + BOX_PAD_Y * k + th + 4.0 * k,
            r.w - 2.0 * BOX_PAD_X * k,
            BOSS_BAR_H * k,
        );
        self.paint.fill_top(dst, bar, rgb(0, 0.7));
        let inside = Rect::new(
            bar.x + 3.0 * k,
            bar.y + 3.0 * k,
            (bar.w - 6.0 * k) * st.boss.share,
            bar.h - 6.0 * k,
        );
        self.paint.fill_top(dst, inside, rgb(0xe63946, 1.0));
        self.paint.frame(dst, bar, 2.0 * k, WHITE);
    }

    /// draw_cell zeichnet die Anzeigen des Spielers der Zelle c (Go: `Draw` des Drawers): Vorlagen, Maßstab, Welt-Texte,
    /// die Elemente der Zelle, Seekarte und Menü. Invariante: Ab dem Ausleihen der Bibliothek (Schrift) ruft
    /// nichts im Zeichenweg `Hud::icon`/`Hud::font` (Doppelausleihe); Icons und Sheets der Karte stehen vorher bereit.
    pub fn draw_cell(&mut self, dst: &mut dyn Canvas, c: &Cell) {
        let Some(i) = usize::try_from(c.player)
            .ok()
            .filter(|i| self.seat(*i as i64).is_some())
        else {
            return;
        };
        // der Zustand wird für den Durchlauf herausgenommen (Draw ändert nur Messungen und Layout-Cache)
        let mut st = std::mem::take(&mut self.seats[i]);
        self.paint.stencils(dst);
        self.refresh_icon_cache();
        let k = scale_of(c, self.cells as i64);
        let icons = self.icons();
        let sea_art = st.sea.open.then(|| self.sea_art_of(&st.sea)).flatten();
        let lib = self.lib.clone();
        let mut guard = lib.as_ref().map(|l| l.borrow_mut());
        let mut font = guard.as_mut().map(|g| &mut g.font);
        self.draw_world_texts(dst, &st, c, k, font.as_deref_mut());
        self.draw_items(dst, &st, c, k, font.as_deref_mut(), &icons);
        self.draw_sea(dst, &mut st.sea, c, k, font.as_deref_mut(), sea_art);
        if st.menu.open {
            self.draw_menu(dst, &mut st, c, k, font);
        }
        self.seats[i] = st;
        dst.layer(0);
    }
}
