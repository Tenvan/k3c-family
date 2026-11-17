//! Port von `native/k3c-game/actors/combatmarks.go`.
//!
//! Anzeigen der Kampfregeln an den Figuren (KS5, B-934, docs/rules/kampf.md): Krone am Kopf des Monarchen und auf dem
//! Kopf ihres Trägers (Gegner oder Mitspieler), Monarch ohne Krone, Banner am Monarchen, Schild des Gruppenführers und
//! die dezente Gruppenmarke der Truppe. Krone, Banner und Schild sind Papierschnitt-Bilder (B-940: icons/icon-krone,
//! objekte/banner-angriff und objekte/schild-gruppenfuehrer, Varianten je Spieler); die Gruppenmarke bleibt eine
//! Scheibe. Fliegende und liegende Kronen, Portal-HP und Zahlstellen zeichnet props.

use k3c_platform::{Affine, Canvas};
use k3c_sim::state::shared::CROWN_PLAYER;
use k3c_sim::state::{Player, Troop, World as SimWorld};

use super::Actors;
use super::anim::sheet_frame;
use super::figures::Figure;
use super::pending::{Anchor, BoxPx, Ink, LOAD_HEIGHT, LOAD_SIDE, load_offset, marks_anchor, player_color, tint};
use crate::art::Sheet;
use crate::view::cell::Cell;

// Assets der Anzeigen (data/paper.json › objekte) und die Animationen ihrer Varianten (Spieler 1 bis 4, 0 = ohne).
pub const CROWN_ASSET: &str = "icons/icon-krone";
pub const BANNER_ASSET: &str = "objekte/banner-angriff";
pub const SHIELD_ASSET: &str = "objekte/schild-gruppenfuehrer";

pub const VARIANT_ANIMS: [&str; 5] = [
    "idle",
    "idle@spieler1",
    "idle@spieler2",
    "idle@spieler3",
    "idle@spieler4",
];

/// variantAnim ist die Animation der Variante für Spieler n (1 bis 4); sonst die Grundvariante.
#[allow(clippy::cast_sign_loss)]
pub fn variant_anim(n: i64) -> &'static str {
    if n < 1 || n >= VARIANT_ANIMS.len() as i64 {
        return VARIANT_ANIMS[0];
    }
    VARIANT_ANIMS[n as usize]
}

// Maße in Spiel-Pixeln (wie marks.rs).
/// das 32-px-Icon ist größer als die alte Form; am Kopf auf 24 px verkleinert
pub const CROWN_SCALE: f64 = 0.75;
/// Fuß der Krone über der Oberkante der Figur
pub const CROWN_ABOVE: f64 = 10.0;

/// Deckkraft des Spieler-Punkts ohne Krone, relativ zu markerAlpha
pub const BARE_ALPHA: f64 = 0.3;

/// Abstand des Stocks in Vielfachen des Last-Versatzes (hinter dem Rücken)
pub const BANNER_OFFSET: f64 = 1.6;

/// seitlicher Versatz des Schilds
pub const SHIELD_SIDE: f64 = 16.0;
/// Mitte des 28-px-Bilds über seinem Fuß
pub const SHIELD_CENTER: f64 = 12.0;

pub const GROUP_DOT: f64 = 6.0;
pub const GROUP_DOT_ALPHA: f64 = 0.7;

/// markAssets nennt die Papier-Bilder der Anzeigen, die actors für jedes Level anmeldet.
pub const MARK_ASSETS: [&str; 3] = [BANNER_ASSET, CROWN_ASSET, SHIELD_ASSET];

impl Ink {
    /// sprite zeichnet Bild i des Sheets mit dem Fuß bei (ox, oy) px vom Bezugspunkt (oy nach oben), um scale
    /// vergrößert, bei flip gespiegelt, mit Deckkraft alpha. Die Matrix und die Farbskala von Go (`op`) liegen hier
    /// auf dem Stack.
    #[allow(
        clippy::too_many_arguments,
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss
    )]
    pub fn sprite(
        &self,
        dst: &mut dyn Canvas,
        a: Anchor,
        s: &Sheet,
        i: i64,
        (ox, oy): (f64, f64),
        scale: f64,
        flip: bool,
        alpha: f64,
    ) {
        let Some(f) = s.frame(i) else {
            return;
        };
        if s.fw <= 0 || s.fh <= 0 {
            return;
        }
        let (fw, fh, px) = (s.fw as f64, s.fh as f64, s.units_per_px * a.s * scale);
        let mut g = Affine::IDENTITY;
        if flip {
            g.scale(-1.0, 1.0);
            g.translate(fw as f32, 0.0);
        }
        g.translate((-s.origin_x * fw) as f32, (-s.origin_y * fh) as f32);
        g.scale(px as f32, px as f32);
        g.translate((a.sx + ox * a.s) as f32, (a.sy - oy * a.s) as f32);
        let al = alpha as f32;
        dst.sprite_tf(f.tex, f.src, g, [al, al, al, al]);
    }
}

/// crownCarrier: Der Monarch p trägt die Krone eines Mitspielers (CrownPlayer, Träger ist er).
pub fn crown_carrier(w: &SimWorld, p: &Player) -> bool {
    w.players.iter().any(|o| {
        o.crown
            .as_ref()
            .is_some_and(|c| c.state == CROWN_PLAYER && c.carrier == p.id)
    })
}

/// bannerFlip: Der Wimpel (im Bild nach rechts) zeigt vom Stock weg, den der Monarch hinter dem Rücken trägt.
pub fn banner_flip(left: bool) -> bool {
    load_offset(left) < 0.0
}

impl Actors {
    /// crown zeichnet das Kronen-Icon mit dem Fuß bei (cx, cy); alpha ist die Deckkraft der Figur.
    pub fn crown(&self, dst: &mut dyn Canvas, a: Anchor, (cx, cy): (f64, f64), alpha: f64) {
        let (Some(lib), Some(ink)) = (&self.lib, &self.ink) else {
            return;
        };
        let l = lib.borrow();
        if let Some(s) = l.sheet(CROWN_ASSET, VARIANT_ANIMS[0]) {
            ink.sprite(dst, a, s, 0, (cx, cy), CROWN_SCALE, false, alpha);
        }
    }

    /// playerMarks zeichnet Krone (am Kopf bzw. getragen) und Banner des Monarchen.
    pub fn player_marks(&self, dst: &mut dyn Canvas, c: &Cell, fg: &Figure, p: &Player, w: &SimWorld) {
        let Some(a) = marks_anchor(c, fg, p.x) else {
            return;
        };
        let top = self.top_px(fg);
        if p.crown.is_none() || crown_carrier(w, p) {
            self.crown(dst, a, (0.0, top + CROWN_ABOVE), fg.ui);
        }
        if p.banner != 0 {
            let (Some(lib), Some(ink)) = (&self.lib, &self.ink) else {
                return;
            };
            let l = lib.borrow();
            if let Some(s) = l.sheet(BANNER_ASSET, variant_anim(p.index + 1)) {
                #[allow(clippy::cast_possible_wrap)]
                let i = sheet_frame(self.now, s.fps, s.frames.len() as i64, false);
                let ox = load_offset(fg.left) * BANNER_OFFSET;
                ink.sprite(dst, a, s, i, (ox, 0.0), 1.0, banner_flip(fg.left), fg.ui);
            }
        }
    }

    /// leaderMarks zeichnet das Schild des Gruppenführers, sonst bei zugeordneter Truppe die dezente Gruppenmarke über
    /// dem Kopf. Gruppen gehören keinem Spieler (t.Group ist eine Truppen-ID): das Schild nutzt die Grundvariante.
    pub fn leader_marks(&self, dst: &mut dyn Canvas, a: Anchor, fg: &Figure, t: &Troop, top: f64) {
        let (Some(lib), Some(ink)) = (&self.lib, &self.ink) else {
            return;
        };
        if t.leader {
            let l = lib.borrow();
            if let Some(s) = l.sheet(SHIELD_ASSET, VARIANT_ANIMS[0]) {
                let ox = -load_offset(fg.left) * SHIELD_SIDE / LOAD_SIDE;
                ink.sprite(dst, a, s, 0, (ox, top * LOAD_HEIGHT - SHIELD_CENTER), 1.0, false, fg.ui);
            }
        } else if t.group != 0 {
            let b = BoxPx {
                cx: 0.0,
                cy: top + GROUP_DOT,
                w: GROUP_DOT,
                h: GROUP_DOT,
            };
            ink.oval(dst, a, b, tint(player_color(t.group), GROUP_DOT_ALPHA * fg.ui));
        }
    }
}

#[cfg(test)]
#[path = "combatmarks_tests.rs"]
mod combatmarks_tests;
