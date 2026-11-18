//! Port von `native/k3c-game/actors/draw.go`.
//!
//! Zeichnen der Figuren einer Zelle: hinten nach vorn Truppen, Gegner, Monarchen (Reihenfolge der Simulation), nur
//! sichtbare. Draw ändert keinen Zustand der Simulation: Rig-Figuren zeichnen die Pose, die Listen im Takt gerechnet
//! hat (pose.rs), Sheets wählen nur ihr Bild. Beide mit der Deckkraft der Figur (fg.alpha).
//!
//! Abweichung von Go: Draw legt vor dem Zeichnen die noch fehlenden Rig-Actors an (`ensure_actors`, braucht die
//! Zeichenfläche) und nimmt darum `&mut self`; `ActorsDrawer` hüllt das in den `Drawer` ein.

use std::cell::RefCell;
use std::rc::Rc;

use k3c_platform::{Affine, Canvas};
use k3c_sim::state::World as SimWorld;

use super::Actors;
use super::anim::{anim_time, sheet_frame};
use super::figures::{Class, Figure, Key};
use super::markdraw::new_ink;
use super::marks::seated;
use crate::art::{actor_frame, rig_view};
use crate::view::cell::{Camera, Cell};
use crate::view::frame::Frame;
use crate::view::layer::{Drawer, Layer, Listener};

/// ActorsDrawer meldet `Actors` als Drawer (LayerActors) und Listener an; der Zustand liegt hinter `Rc<RefCell>`,
/// weil Draw Actors anlegt und Pose-Puffer schreibt.
pub struct ActorsDrawer {
    pub a: Rc<RefCell<Actors>>,
}

impl Drawer for ActorsDrawer {
    fn layer(&self) -> Layer {
        Layer::ACTORS
    }

    fn draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        self.a.borrow_mut().draw(dst, f, c);
    }
}

impl Listener for ActorsDrawer {
    fn listen(&mut self, f: &Frame) {
        self.a.borrow_mut().listen(f);
    }
}

impl Actors {
    /// Draw zeichnet die Figuren der Zelle c.
    pub fn draw(&mut self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        if self.cat.is_none() || self.lib.is_none() {
            return;
        }
        let Some(w) = f.world(c) else {
            return;
        };
        let Some(s) = f.seat(c.player) else {
            return;
        };
        if self.ink.is_none() {
            self.ink = Some(new_ink(dst));
        }
        self.ensure_actors(dst);
        for t in &w.troops {
            let k = Key {
                stage: s.stage,
                class: Class::Troop,
                id: t.id,
            };
            if self.draw_figure(dst, c, k, t.x)
                && let Some(fg) = self.figs.get(&k)
            {
                self.troop_marks(dst, c, fg, t);
            }
        }
        for e in &w.enemies {
            let k = Key {
                stage: s.stage,
                class: Class::Enemy,
                id: e.id,
            };
            if self.draw_figure(dst, c, k, e.x)
                && let Some(fg) = self.figs.get(&k)
            {
                self.enemy_marks(dst, c, fg, e);
            }
        }
        self.draw_players(dst, f, c, s.stage, w);
    }

    /// drawPlayers zeichnet die Monarchen der Zelle: Punkt, Figur, Balken. Freie Monarchen (kein Platz in
    /// Frame.Seats) bleiben unsichtbar.
    fn draw_players(&mut self, dst: &mut dyn Canvas, f: &Frame, c: &Cell, stage: i64, w: &SimWorld) {
        for p in &w.players {
            if !seated(&f.seats, p.index) {
                continue;
            }
            let k = Key {
                stage,
                class: Class::Player,
                id: p.id,
            };
            if let Some(fg) = self.figs.get(&k)
                && !fg.broken
            {
                self.player_marker(dst, c, fg, p);
            }
            if self.draw_figure(dst, c, k, p.x)
                && let Some(fg) = self.figs.get(&k)
            {
                self.player_marks(dst, c, fg, p, w);
            }
        }
    }

    /// drawFigure zeichnet die Figur k mit Fußpunkt bei Welt-x x, wenn sie in die Zelle ragt. Rückgabe: `true`, wenn
    /// sie zeichenbar ist (für ihre Anzeigen, auch außerhalb der Zelle), sonst `false` (unbekannt, Rig fehlt, noch
    /// ohne Pose).
    pub fn draw_figure(&mut self, dst: &mut dyn Canvas, c: &Cell, k: Key, x: f64) -> bool {
        let Some(fg) = self.figs.get(&k) else {
            return false;
        };
        if fg.broken {
            return false;
        }
        let y = c.screen_y(fg.lift);
        if fg.actor.is_none() {
            self.draw_sheet(dst, &c.camera, k, x, y);
            return true;
        }
        if self.painter.is_none() {
            return false;
        }
        let (size, frame) = rig_size(fg);
        if !c.camera.visible(x, (frame[0].abs() + frame[2].abs()) * size) {
            return true;
        }
        if !fg.posed {
            return false; // noch ohne Pose aus Listen oder Pose fehlerhaft (gemeldet)
        }
        let (alpha, left) = (fg.alpha, fg.left);
        let Some(lib) = self.lib.clone() else {
            return false;
        };
        let l = lib.borrow();
        let (Some(assets), Some(painter)) = (l.rig_assets(), self.painter.as_mut()) else {
            return true;
        };
        painter.alpha = alpha;
        painter.view = rig_view(&c.camera, x, y, size, frame, left);
        if let Some(a) = self.figs.get_mut(&k).and_then(|fg| fg.actor.as_mut()) {
            a.draw(painter, dst, assets);
        }
        true
    }

    /// drawSheet zeichnet eine Sheet-Figur: Fußpunkt aus dem Rahmen, gespiegelt im eigenen Bild, mit Deckkraft.
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_precision_loss
    )]
    fn draw_sheet(&self, dst: &mut dyn Canvas, k: &Camera, key: Key, x: f64, y: f64) {
        let (Some(fg), Some(lib)) = (self.figs.get(&key), &self.lib) else {
            return;
        };
        let l = lib.borrow();
        let Some(s) = l.sheet(&fg.look.asset, &fg.pose.anim) else {
            return;
        };
        if s.fh <= 0 {
            return;
        }
        let px = fg.look.height * k.scale / s.fh as f64; // Bildpunkte je Sheet-Pixel
        let (fw, fh) = (s.fw as f64, s.fh as f64);
        if !k.visible(x, fw * px / k.scale) {
            return;
        }
        let i = sheet_frame(
            anim_time(&fg.pose, self.now, fg.shift),
            s.fps,
            s.frames.len() as i64,
            fg.pose.dur > 0.0,
        );
        let mut g = Affine::IDENTITY;
        if fg.left {
            g.scale(-1.0, 1.0);
            g.translate(fw as f32, 0.0);
        }
        g.translate((-s.origin_x * fw) as f32, (-s.origin_y * fh) as f32);
        g.scale(px as f32, px as f32);
        g.translate(k.screen_x(x) as f32, y as f32);
        let al = fg.alpha as f32;
        if let Some(f) = s.frame(i) {
            dst.sprite_tf(f.tex, f.src, g, straight_tint(al));
        }
    }
}

/// Tönung eines Sprites mit Deckkraft `al`: gerade Farbe (weiß, Alpha), wie der Canvas-Shader sie erwartet; Go
/// skaliert die vormultiplizierte Farbe mit `ScaleAlpha`, das ist dieselbe Wirkung (B-1061).
pub fn straight_tint(al: f32) -> [f32; 4] {
    [1.0, 1.0, 1.0, al]
}

/// rigSize liefert Welt-Units je Rig-Einheit (Spielhöhe / Rahmenhöhe der eigenen Figur; beritten bleibt der Maßstab
/// des Reiters) und den Rahmen der gezeichneten Figur (beritten bzw. Vierbeiner der des Reittiers).
pub fn rig_size(fg: &Figure) -> (f64, [f64; 4]) {
    let Some(a) = fg.actor.as_ref() else {
        return (0.0, [0.0; 4]);
    };
    let mut frame = actor_frame(a);
    let Some(base) = a.rider.as_ref().or(a.mount.as_ref()) else {
        return (0.0, frame);
    };
    if fg.pose.gait.is_empty()
        && let Some(r) = &a.rider
    {
        frame = r.frame; // zu Fuß (Monarch tot oder beim Respawn)
    }
    if base.frame[3] <= 0.0 {
        return (0.0, frame);
    }
    (fg.look.height / base.frame[3], frame)
}

#[cfg(test)]
#[path = "draw_tests.rs"]
mod draw_tests;

#[cfg(test)]
mod tint_tests {
    use k3c_platform::{Affine, Canvas, Filter, FontId, Rect, TexId, Vertex};

    use super::straight_tint;
    use crate::actors::markdraw::{Anchor, Ink};
    use crate::art::{Sheet, SheetFrame};

    /// Zeichenfläche, die nur die Tönung der Sprites merkt.
    #[derive(Default)]
    struct Tints(Vec<[f32; 4]>);

    impl Canvas for Tints {
        fn size(&self) -> (f32, f32) {
            (1.0, 1.0)
        }
        fn texture_with(&mut self, _: &[u8], _: u32, _: u32, _: Filter) -> TexId {
            TexId(0)
        }
        fn free_texture(&mut self, _: TexId) {}
        fn sprite(&mut self, _: TexId, _: Rect, _: Rect, _: [f32; 4]) {}
        fn sprite_tf(&mut self, _: TexId, _: Rect, _: Affine, tint: [f32; 4]) {
            self.0.push(tint);
        }
        fn mesh(&mut self, _: TexId, _: &[Vertex], _: &[u16]) {}
        fn rect(&mut self, _: Rect, _: [f32; 4]) {}
        fn stroke_rect(&mut self, _: Rect, _: f32, _: [f32; 4]) {}
        fn fill_circle(&mut self, _: f32, _: f32, _: f32, _: [f32; 4]) {}
        fn stroke_circle(&mut self, _: f32, _: f32, _: f32, _: f32, _: [f32; 4]) {}
        fn stroke_line(&mut self, _: f32, _: f32, _: f32, _: f32, _: f32, _: [f32; 4]) {}
        fn fill_path(&mut self, _: &[[f32; 2]], _: [f32; 4]) {}
        fn text(&mut self, _: &str, _: f32, _: f32, _: f32, _: [f32; 4]) {}
        fn font(&mut self, _: &[u8]) -> Result<FontId, String> {
            Err("keine Schrift".to_string())
        }
        fn text_font(&mut self, _: FontId, _: &str, _: f32, _: f32, _: f32, _: [f32; 4]) {}
        fn text_width(&self, _: FontId, _: &str, _: f32) -> f32 {
            0.0
        }
    }

    /// B-1061: Die Tönung bei halber Deckkraft ist weiß mit Alpha 0,5, nicht 0,5 grau (zu dunkel); die Gefechtsmarken
    /// (`Ink::sprite`) tönen genauso.
    #[test]
    fn tint_is_straight() {
        assert_eq!(straight_tint(0.5), [1.0, 1.0, 1.0, 0.5]);
        assert_eq!(straight_tint(1.0), [1.0; 4]);
        let mut s = Sheet::default();
        s.frames = vec![SheetFrame {
            tex: TexId(0),
            src: Rect::new(0.0, 0.0, 8.0, 8.0),
        }];
        (s.fw, s.fh, s.units_per_px) = (8, 8, 1.0);
        let a = Anchor {
            sx: 0.0,
            sy: 0.0,
            s: 1.0,
        };
        let mut c = Tints::default();
        Ink::default().sprite(&mut c, a, &s, 0, (0.0, 0.0), 1.0, false, 0.5);
        assert_eq!(c.0, vec![[1.0, 1.0, 1.0, 0.5]]);
    }
}
