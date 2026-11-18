//! Port von `native/k3c-game/hud/drawstory.go`: Zeichnen der Geschichtszeile (`story.rs`, B-974): ein bis zwei Zeilen
//! mittig auf einer dunklen Fläche, Deckkraft nach dem Ablauf der Einblendung. Passt die längere Zeile nicht in die Zelle,
//! wird die Schrift kleiner (`fit_px`), nicht umbrochen und nicht gekürzt.

use k3c_platform::Canvas;

use super::Hud;
use super::draw::{BOX_ALPHA, BOX_PAD_X, BOX_PAD_Y, fit_px};
use super::fonts::{ROLE_STORY, font_px, rgb};
use super::layout::{GAP, ITEM_STORY, Items, Rect};
use super::paint::{TextSlot, Z_CONTENT};
use super::state::SeatState;
use super::story::StoryEntry;
use crate::art::{Font, LINE_SPACING, OUTLINE8};

/// StoryFit sind Schriftgröße und Maße der beiden Zeilen eines Eintrags, so dass die längere in die Breite `cell_w`
/// passt.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StoryFit {
    pub px: f64,
    pub wa: f64,
    pub ha: f64,
    pub wb: f64,
    pub hb: f64,
}

impl Hud {
    /// fit_story misst den Eintrag `e` in der Größe der Rolle (gemerkt je Spieler: einmal je Text, nicht je Bild) und
    /// rechnet Größe und Maße im Verhältnis herunter, wenn die längere Zeile nicht in `cell_w` passt.
    pub fn fit_story(&mut self, e: &StoryEntry, f: Option<&Font>, p: i64, k: f64, cell_w: f64) -> StoryFit {
        let base = font_px(ROLE_STORY, self.cells as i64) * k;
        let (wa, ha) = self.paint.measure_text(f, p, TextSlot::STORY_A, &e.a, base);
        let (wb, hb) = self.paint.measure_text(f, p, TextSlot::STORY_B, &e.b, base);
        let px = fit_px(base, wa.max(wb) + 2.0 * BOX_PAD_X * k, cell_w - 2.0 * GAP * k);
        let s = px / base;
        StoryFit {
            px,
            wa: wa * s,
            ha: ha * s,
            wb: wb * s,
            hb: hb * s,
        }
    }

    /// story_size trägt die Fläche des laufenden Eintrags des Spielers in das Layout ein (ohne Eintrag: nichts).
    pub fn story_size(&mut self, st: &SeatState, f: Option<&Font>, p: i64, k: f64, cell_w: f64, inp: &mut Items) {
        if st.story.showing.a.is_empty() {
            return;
        }
        let m = self.fit_story(&st.story.showing, f, p, k, cell_w);
        if m.wa.max(m.wb) > 0.0 {
            (inp[ITEM_STORY].w, inp[ITEM_STORY].h) = story_box(m.wa, m.ha, m.wb, m.hb, m.px, k);
        }
    }

    /// draw_story zeichnet den laufenden Eintrag auf seiner Fläche `r`, jede Zeile mittig.
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn draw_story(
        &mut self,
        dst: &mut dyn Canvas,
        st: &SeatState,
        mut f: Option<&mut Font>,
        p: i64,
        r: Rect,
        k: f64,
        cell_w: f64,
    ) {
        let a = st.story.alpha();
        if a <= 0.0 {
            return;
        }
        let e = &st.story.showing;
        self.paint.fill(dst, r, rgb(0, BOX_ALPHA * a));
        let y = r.y + BOX_PAD_Y * k;
        let m = self.fit_story(e, f.as_deref(), p, k, cell_w);
        dst.layer(Z_CONTENT);
        let clr = ROLE_STORY.clr;
        self.paint.outline.draw(
            dst,
            f.as_deref_mut(),
            &e.a,
            r.x + (r.w - m.wa) / 2.0,
            y,
            m.px,
            clr,
            &OUTLINE8,
            a,
        );
        if !e.b.is_empty() {
            let y2 = y + LINE_SPACING * m.px;
            self.paint
                .outline
                .draw(dst, f, &e.b, r.x + (r.w - m.wb) / 2.0, y2, m.px, clr, &OUTLINE8, a);
        }
    }
}

/// story_box ist die Fläche um zwei Zeilen der Breiten `wa` und `wb` (`hb` == 0: nur eine Zeile) in der Schriftgröße
/// `px`.
pub fn story_box(wa: f64, ha: f64, wb: f64, hb: f64, px: f64, k: f64) -> (f64, f64) {
    let h = if hb > 0.0 { LINE_SPACING * px + hb } else { ha };
    (wa.max(wb) + 2.0 * BOX_PAD_X * k, h + 2.0 * BOX_PAD_Y * k)
}
