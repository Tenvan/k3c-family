//! Andenken der abgeschlossenen Inseln auf der Seekarte (GS3.8): die gezeichnete Trophäe samt Fahne (Rückfall, bis die
//! Assets aus GS3.2 da sind) und die Zeilen unter der Karte, wenn der Zeiger des Spielers auf einer abgeschlossenen Insel
//! steht: die erste `done`-Zeile der Insel und klein „<Tage> Tage · <n> Monarchen“. Die Texte baut `SeaView` bei der
//! Änderung der Karte; hier wird nur gemessen und gezeichnet (keine Allokation je Frame).

use k3c_platform::Canvas;

use super::Hud;
use super::drawsea::SeaSizes;
use super::fonts::{WHITE, rgb};
use super::layout::Rect;
use super::seekarte::{SeaNode, SeaView};
use crate::art::Font;
use crate::view::trophy::{Part, parts};

/// Höhe des Platzes der Andenken-Zeilen in Schriftgrößen `b`: zwei Zeilen in Namensgröße (GS3.8: nicht kleiner als die Inselnamen), immer reserviert, sobald eine abgeschlossene
/// Insel eine Zeile oder Zahlen trägt (die Tafel springt nicht, wenn der Zeiger wandert).
pub const MEMO_H: f64 = 2.5;

/// memo_of ist die Insel, auf der der Zeiger dieses Spielers steht und die ein Andenken trägt.
pub fn memo_of(v: &SeaView) -> Option<&SeaNode> {
    v.rows.iter().flat_map(|r| &r.nodes).find(|n| n.pointed)
}

impl Hud {
    /// measure_memo misst die breiteste Andenken-Zeile (bei Schriftgröße b) und merkt, ob es Zeilen gibt.
    pub fn measure_memo(&self, m: &mut SeaSizes, font: &Font, v: &SeaView, b: f64) {
        for n in v.rows.iter().flat_map(|r| &r.nodes) {
            if n.done_line.is_empty() && n.numbers.is_empty() {
                continue;
            }
            m.has_memo = true;
            m.memo_w = m
                .memo_w
                .max(font.measure(&n.done_line, b).0)
                .max(font.measure(&n.numbers, b).0);
        }
    }

    /// draw_trophy zeichnet die Trophäe mit Mitte cx, Standlinie by und Höhe h (mit Fahne, wenn `flag`).
    pub fn draw_trophy(&self, dst: &mut dyn Canvas, cx: f64, by: f64, h: f64, flag: bool) {
        let pt = &self.paint;
        parts(cx, by, h, flag, &mut |p| match p {
            Part::Rect(x, y, w, hh, c) => pt.fill_top(dst, Rect::new(x, y, w, hh), rgb(c, 1.0)),
            Part::Disc(x, y, r, c) => pt.circle(dst, x, y, r, rgb(c, 1.0)),
            Part::Ring(x, y, r, c) => pt.ring_at(dst, x, y, r, rgb(c, 1.0)),
        });
    }

    /// draw_memo zeichnet die Zeilen der Insel unter dem Zeiger ab (x, y): `done`-Zeile, darunter klein die Zahlen.
    pub fn draw_memo(&mut self, dst: &mut dyn Canvas, mut f: Option<&mut Font>, v: &SeaView, x: f64, y: f64, b: f64) {
        let Some(n) = memo_of(v) else { return };
        if !n.done_line.is_empty() {
            self.paint.text(dst, f.as_deref_mut(), &n.done_line, x, y, b, WHITE);
        }
        if !n.numbers.is_empty() {
            self.paint.text(dst, f, &n.numbers, x, y + 1.2 * b, b, WHITE);
        }
    }
}
