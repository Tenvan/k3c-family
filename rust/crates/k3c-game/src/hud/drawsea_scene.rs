//! Port von `native/k3c-game/hud/drawsea_paper.go` (Teil 2 von 2): Szene des Seekarten-Dioramas: von hinten nach vorn je
//! Reihe Welle, Inseln und das fahrende Schiff, dann die vordere Welle, Rahmen und Texte. Das Schiff liegt an der
//! aktuellen Insel, Fahne = abgeschlossen, Nebel und blass = gesperrt. Gelber Rahmen: Cursor dieses Spielers; weißer
//! Rahmen: gewähltes Ziel (gilt für alle). Die Inseln einer Etappe stehen hintereinander, jede Reihe hinter ihrer
//! eigenen Welle.
//!
//! Abweichung von Go: Die bewegte vordere Welle der Fahrt (`SeaExtra::moving`) zeichnet es versetzt direkt auf das Ziel,
//! ohne Beschnitt (Go: Zwischenbild). `Canvas::clip` taugt nicht: Die Plattform sortiert nach (Ebene, Clip, Folge), die
//! Welle läge über dem Text, und `clip(None)` löschte den Clip des Aufrufers. RU23.5 entscheidet über den Beschnitt.

use k3c_platform::Canvas;

use super::Hud;
use super::drawsea::CLR_SEA_BACK;
use super::drawsea_paper::*;
use super::fonts::{WHITE, YELLOW, rgb};
use super::layout::Rect;
use super::seekarte::{SeaKind, SeaNode, SeaView};
use crate::art::Font;

/// sea_island_rect ist die Fläche der Miniatur s der Insel nd mit Mitte cx und Wasserlinie wy.
pub fn sea_island_rect(s: &SeaSheet, nd: &SeaNode, cx: f64, wy: f64, u: f64) -> Rect {
    let (w, ht) = (s.fw * u * nd.look.scale, s.fh * u * nd.look.scale);
    Rect::new(cx - s.origin_x * w, wy - s.origin_y * ht, w, ht)
}

/// sea_berth ist der Fußpunkt des Schiffs am Liegeplatz der Insel mit Fläche r, Mitte cx und Wasserlinie wy: links vor
/// der Miniatur.
pub fn sea_berth(r: Rect, cx: f64, wy: f64, u: f64) -> (f64, f64) {
    (cx - 0.46 * r.w, wy + 10.0 * u)
}

impl Hud {
    /// draw_sea_scene zeichnet das Diorama der Karte v in der Lage p, von hinten nach vorn.
    pub fn draw_sea_scene(
        &mut self,
        dst: &mut dyn Canvas,
        mut f: Option<&mut Font>,
        v: &mut SeaView,
        a: &SeaArt,
        p: &SeaPlan,
        x: &SeaExtra,
    ) {
        let (Some(back), Some(mid), Some(front), Some(ship)) = (a.back, a.mid, a.front, a.ship) else {
            return;
        };
        self.paint.fill_top(dst, p.box_, rgb(CLR_SEA_BACK, 0.85));
        let top = p.water - SEA_TOP_H * p.u;
        self.paint.fill_top(
            dst,
            Rect::new(p.box_.x, top, p.box_.w, p.front - top + p.u),
            rgb(CLR_SEA_SKY, 1.0),
        );
        self.draw_sea_heads(dst, f.as_deref_mut(), v, p);
        for lane in 0..p.lanes {
            let layer = if lane == 0 { &back } else { &mid };
            let wy = p.water_of(lane);
            self.draw_sea_strip(dst, layer, p.box_.x, wy - SEA_WAVE_UP * p.u, p.box_.w, p.u);
            for (i, r) in v.rows.iter().enumerate() {
                if let Some(nd) = r.nodes.get(lane) {
                    self.draw_sea_island(dst, a, v, (i, lane), nd, p.node_x(i, lane, r.nodes.len()), wy, p.u);
                }
            }
            if x.ship && x.lane == lane {
                self.draw_sea_sprite(dst, &ship, x.x, x.y, p.u, WHITE);
            }
        }
        self.draw_sea_front(dst, &front, p, x);
        let focus = self.draw_sea_frames(dst, v, a, p);
        let y = p.front + SEA_WAVE_UP * p.u;
        if let Some(n) = focus.and_then(|(i, j)| v.rows.get(i).and_then(|r| r.nodes.get(j))) {
            self.paint
                .text(dst, f.as_deref_mut(), &n.name, p.box_.x + p.pad, y, p.b, YELLOW);
        }
        self.draw_hint(dst, f, &mut v.line, &v.hint, p.box_.x + p.pad, y + 1.2 * p.b, 0.8 * p.b);
    }

    /// draw_sea_heads zeichnet Titel und die Beschriftung der Etappen über den Spalten.
    fn draw_sea_heads(&mut self, dst: &mut dyn Canvas, mut f: Option<&mut Font>, v: &SeaView, p: &SeaPlan) {
        self.paint.text(
            dst,
            f.as_deref_mut(),
            &v.title,
            p.box_.x + p.pad,
            p.box_.y + p.pad,
            p.b,
            YELLOW,
        );
        for (i, r) in v.rows.iter().enumerate() {
            let rx = p.x0 + (i as f64 + 0.08) * SEA_COL_W * p.u;
            self.paint.text(
                dst,
                f.as_deref_mut(),
                &r.title,
                rx,
                p.box_.y + p.pad + 1.4 * p.b,
                0.8 * p.b,
                WHITE,
            );
        }
    }

    /// draw_sea_front zeichnet die vordere Welle, bei der Fahrt versetzt (ohne Beschnitt, siehe Kopf).
    fn draw_sea_front(&self, dst: &mut dyn Canvas, front: &SeaSheet, p: &SeaPlan, x: &SeaExtra) {
        if x.moving {
            // breiter als die Tafel, damit der Versatz keine Lücke am Rand lässt
            let over = x.shift.abs() + p.u;
            self.draw_sea_strip(
                dst,
                front,
                p.box_.x - over + x.shift,
                p.front,
                p.box_.w + 2.0 * over,
                p.u,
            );
        } else {
            self.draw_sea_strip(dst, front, p.box_.x, p.front, p.box_.w, p.u);
        }
    }

    /// draw_sea_island zeichnet die Insel nd (Zeile, Reihe `at`) mit der Mitte cx auf die Wasserlinie wy: Schichten,
    /// Miniatur, Aufsatz, Zeichen und, liegt das Schiff an, das Schiff. Gesperrte Inseln sind blass.
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    fn draw_sea_island(
        &self,
        dst: &mut dyn Canvas,
        a: &SeaArt,
        v: &SeaView,
        at: (usize, usize),
        nd: &SeaNode,
        cx: f64,
        wy: f64,
        u: f64,
    ) {
        let Some([mini, top]) = a.island(v, at.0, at.1) else {
            return; // sea_art_of hat geprüft; ohne Sheet nichts
        };
        let (sc, alpha) = (u * nd.look.scale, if nd.kind == SeaKind::Locked { 0.5 } else { 1.0 });
        let r = sea_island_rect(mini, nd, cx, wy, u);
        for i in 0..nd.look.layers {
            let w = r.w * 0.06f64.mul_add(-(i as f64), 0.66);
            let clr = if i % 2 == 1 { CLR_SEA_SLAB_B } else { CLR_SEA_SLAB_A };
            let slab = Rect::new(
                cx - w / 2.0,
                r.y + r.h - sc + i as f64 * SEA_SLAB_H * sc,
                w,
                SEA_SLAB_H * sc,
            );
            self.paint.fill_top(dst, slab, rgb(clr, alpha));
        }
        let t = tint(alpha);
        self.draw_sea_sprite(dst, mini, cx, wy, sc, t);
        self.draw_sea_sprite(dst, top, cx + 0.42 * r.w, wy + 3.0 * sc, sc, t);
        match nd.kind {
            SeaKind::Done => {
                if let Some(flag) = &a.flag {
                    self.draw_sea_sprite(dst, flag, cx + 0.05 * r.w, wy - 2.0 * sc, sc, WHITE);
                }
            }
            SeaKind::Locked => {
                if let Some(fog) = &a.fog {
                    self.draw_sea_sprite(dst, fog, cx, wy - 0.2 * r.h, sc, tint(0.92));
                }
            }
            SeaKind::Target => {}
        }
        if nd.current
            && let Some(ship) = &a.ship
        {
            let (x, y) = sea_berth(r, cx, wy, u);
            self.draw_sea_sprite(dst, ship, x, y, u, WHITE);
        }
    }

    /// draw_sea_frames zeichnet die Rahmen über alle Schichten: weiß um das gewählte Ziel, gelb innen um den Cursor.
    /// Rückgabe: (Zeile, Reihe) der Insel, deren Name unten steht (Cursor, sonst die Wahl; None ohne beides).
    fn draw_sea_frames(&self, dst: &mut dyn Canvas, v: &SeaView, a: &SeaArt, p: &SeaPlan) -> Option<(usize, usize)> {
        let mut focus = None;
        for (i, row) in v.rows.iter().enumerate() {
            for (j, nd) in row.nodes.iter().enumerate() {
                if !nd.chosen && !nd.cursor {
                    continue;
                }
                let Some([mini, _]) = a.island(v, i, j) else {
                    continue;
                };
                let r = sea_island_rect(mini, nd, p.node_x(i, j, row.nodes.len()), p.water_of(j), p.u);
                let lw = (p.b * 0.08).max(2.0);
                if nd.chosen {
                    let o = Rect::new(r.x - 2.0 * lw, r.y - 2.0 * lw, r.w + 4.0 * lw, r.h + 4.0 * lw);
                    self.paint.frame(dst, o, lw, WHITE);
                }
                if nd.cursor {
                    let o = Rect::new(r.x - lw, r.y - lw, r.w + 2.0 * lw, r.h + 2.0 * lw);
                    self.paint.frame(dst, o, (p.b * 0.12).max(3.0), YELLOW);
                }
                if nd.cursor || focus.is_none() {
                    focus = Some((i, j));
                }
            }
        }
        focus
    }
}
