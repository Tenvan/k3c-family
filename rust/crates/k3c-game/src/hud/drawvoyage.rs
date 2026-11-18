//! Port von `native/k3c-game/hud/drawvoyage.go`: Zeichnen der Schiffsfahrt im Vollbild (B-977, `VoyageDrawer`): Papier
//! über das ganze Bild, darauf das Diorama der Seekarte (`drawsea_paper.rs`) ohne Cursor, das Schiff auf dem Bogen von der
//! verlassenen zur Zielinsel, die vordere Welle bewegt, die verlassene Insel mit Fahne, sobald das Schiff abgelegt hat.
//! Nach der Fahrzeit liegt das Schiff am Ziel und schaukelt weiter.
//!
//! Abweichungen von Go: (1) `Hud` liegt hinter `RefCell`, darum steht der Drawer (`VoyageDrawer`, `&self`) auf
//! `HudDrawer` und leiht `Hud` aus. (2) Go beschneidet die bewegte vordere Welle über ein `SubImage` (`seaClip`). In Rust
//! entfällt das Zwischenbild: `drawsea_paper.rs` stutzt die Kacheln der Welle selbst auf den Ausschnitt (Ziel- und
//! Quellrechteck, `draw_sea_strip_in`); `Canvas::clip` taugt nicht, weil die Plattform je Ebene nach Ausschnitt sortiert
//! (die Welle läge über dem Text). Die Zeichenbefehle bleiben in Go-Reihenfolge. (3) Die Sheets für die Liegeplätze
//! liefert `SeaArt` (Go liest sie über `seaSheet`).

use k3c_platform::Canvas;

use super::Hud;
use super::HudDrawer;
use super::drawsea_paper::{SeaArt, SeaExtra, SeaPlan, sea_paper_plan};
use super::drawsea_scene::{sea_berth, sea_island_rect};
use super::fonts::{REF_HEIGHT, rgb};
use super::layout::Rect;
use super::seekarte::{SeaKind, SeaView};
use super::voyage::{SeaSpot, VoyageView, voyage_ease, voyage_lane, voyage_pos, voyage_shift};
use crate::art::voyage as look_of_voyage;
use crate::view::frame::Frame;
use crate::view::voyage::{Voyage, VoyageDrawer};

/// Papier um die Tafel, wie das Ladebild
const CLR_VOYAGE_PAPER: u32 = 0xee_e2c6;

impl Hud {
    /// voyage_ready meldet, ob die Fahrt `v` zu zeichnen ist: Schrift, alle Assets der Karte und beide Inseln auf der
    /// Route (`VoyageDrawer`; sonst zeigt game den Ladebildschirm, B-977/R5).
    pub fn voyage_ready(&mut self, f: &Frame, v: &Voyage) -> bool {
        if self.font().is_none() || self.sea_map.is_none() {
            return false;
        }
        let mut y = std::mem::take(&mut self.voy);
        let mut ok = y.build(&self.lang, &f.route, v);
        if ok {
            ok = self.sea_art_of(&y.sea).is_some_and(|a| {
                self.sea_isl = a.isl; // Puffer der Insel-Sheets zurück
                true
            });
        }
        self.voy = y;
        ok
    }

    /// draw_voyage zeichnet die Fahrt `v` über das ganze Bild `dst`; false = nichts gezeichnet (Schrift oder Asset
    /// fehlt, Karte nicht baubar). Erst alle Prüfungen, dann Zeichnen.
    pub fn draw_voyage(&mut self, dst: &mut dyn Canvas, f: &Frame, v: &Voyage) -> bool {
        if self.font().is_none() {
            return false;
        }
        let mut y = std::mem::take(&mut self.voy);
        let ok = self.draw_voyage_with(dst, &mut y, f, v);
        self.voy = y;
        ok
    }

    fn draw_voyage_with(&mut self, dst: &mut dyn Canvas, y: &mut VoyageView, f: &Frame, v: &Voyage) -> bool {
        if !y.build(&self.lang, &f.route, v) {
            return false;
        }
        let Some(a) = self.sea_art_of(&y.sea) else {
            return false;
        };
        self.paint.stencils(dst);
        let (w, h) = dst.size();
        let cell = Rect::new(0.0, 0.0, f64::from(w), f64::from(h));
        let look = look_of_voyage();
        let k = look.scale * cell.h / REF_HEIGHT;
        if y.sea.sizes_k != k {
            let sizes = self.measure_sea(&mut y.sea, self.font().as_deref(), 28.0 * k);
            (y.sea.sizes, y.sea.sizes_k) = (sizes, k);
        }
        // mittig statt etwas hoch
        let p = sea_paper_plan(
            &y.sea.sizes,
            y.sea.rows.len(),
            Rect::new(cell.x, cell.y + 40.0 * k, cell.w, cell.h),
            k,
        );
        let s = voyage_ease(v.elapsed, v.duration);
        let (from, to) = (y.from, y.to);
        // am Liegeplatz noch ohne Fahne
        y.node_mut(from).kind = if s >= look.flag_at {
            SeaKind::Done
        } else {
            SeaKind::Target
        };
        let (ax, ay) = berth_of(&a, &y.sea, from, &p);
        let (bx, by) = berth_of(&a, &y.sea, to, &p);
        let mut x = SeaExtra {
            ship: true,
            lane: voyage_lane(from.lane, to.lane),
            moving: true,
            shift: voyage_shift(look.wave * p.u, look.wave_hz, v.elapsed),
            ..SeaExtra::default()
        };
        (x.x, x.y) = voyage_pos(
            ax,
            ay,
            bx,
            by,
            look.arc * p.u,
            look.bob * p.u,
            look.bob_hz,
            v.elapsed,
            v.duration,
        );
        self.paint.fill(dst, cell, rgb(CLR_VOYAGE_PAPER, 1.0));
        let lib = self.lib.clone();
        let mut guard = lib.as_ref().map(|l| l.borrow_mut());
        let font = guard.as_mut().map(|g| &mut g.font);
        self.draw_sea_scene(dst, font, &mut y.sea, &a, &p, &x);
        self.sea_isl = a.isl; // Puffer der Insel-Sheets zurück, je Bild wiederverwendet
        dst.layer(0);
        true
    }
}

/// berth_of ist der Liegeplatz des Schiffs an der Insel am Platz `s` in der Lage `p`; ohne Sheet die Mitte der Insel
/// (`sea_art_of` hat geprüft).
fn berth_of(a: &SeaArt, v: &SeaView, s: SeaSpot, p: &SeaPlan) -> (f64, f64) {
    let (cx, wy) = (p.node_x(s.col, s.lane, s.n), p.water_of(s.lane));
    match a.island(v, s.col, s.lane) {
        Some([mini, _]) => sea_berth(
            sea_island_rect(mini, &v.rows[s.col].nodes[s.lane], cx, wy, p.u),
            cx,
            wy,
            p.u,
        ),
        None => (cx, wy),
    }
}

impl VoyageDrawer for HudDrawer {
    fn voyage_ready(&self, f: &Frame, v: &Voyage) -> bool {
        self.hud.try_borrow_mut().is_ok_and(|mut h| h.voyage_ready(f, v))
    }

    fn draw_voyage(&self, dst: &mut dyn Canvas, f: &Frame, v: &Voyage) -> bool {
        self.hud.try_borrow_mut().is_ok_and(|mut h| h.draw_voyage(dst, f, v))
    }
}
