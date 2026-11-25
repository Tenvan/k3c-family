//! Flaschenpost im HUD (GS4.7, B-980): Radar-Marker für die liegende Flasche und das markierte Ziel, Richtung des
//! Pfeils am Zellenrand und die Geschichtszeile nach dem Aufheben. Reine Rechnung aus Zustand und Kamera, ohne Text
//! und ohne Allokation je Frame; das Spiel ändert keinen Zustand der Sim. Die Markierung liegt auf der Stufe der Tiefe 0
//! (`World::bottle.target`); steht dort kein lokaler Spieler, kennt die Anzeige sie nicht.

use k3c_sim::state::World;

use k3c_platform::Canvas;

use super::Hud;
use super::banner::int_of;
use super::fonts::rgb;
use super::radar::{RadarKind, RadarMarker, unit_pos};
use super::state::SeatState;
use crate::view::bottle::target_of;
use crate::view::cell::{Camera, Cell};
use crate::view::frame::Frame;

/// Wohin der Pfeil einer Zelle zielt: `x` ist der Welt-Ort, auf den der Spieler zuhalten soll (das Ziel selbst oder der
/// nächste Übergang der eigenen Stufe), `dy` die Stufe des Ziels relativ zur eigenen (1 tiefer, -1 höher, 0 gleiche).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ArrowAim {
    pub x: f64,
    pub dy: i8,
}

/// `bottle_markers` hängt an die Radar-Marker der Welt `w` die liegende Flasche (nur Stufe der Tiefe 0) und das Ziel
/// dieser Stufe an. Ohne Zustand kommt nichts dazu.
pub fn bottle_markers(out: &mut Vec<RadarMarker>, w: &World, goal: Option<f64>) {
    let at = |kind: RadarKind, x: f64| RadarMarker {
        kind,
        pos: unit_pos(x, w.width_units),
        ..Default::default()
    };
    if let Some(x) = w.bottle.ashore.filter(|_| w.biome.depth == 0) {
        out.push(at(RadarKind::Bottle, x));
    }
    if let Some(x) = goal {
        out.push(at(RadarKind::Goal, x));
    }
}

/// `arrow_aim` rechnet, wohin der Pfeil des Monarchen bei `x` in der Stufe `stage` (Welt `w`) zeigt: gleiche Stufe → zum
/// Ziel; andere Stufe → zum nächsten Treppen-Übergang nach unten (Ziel tiefer) bzw. oben, ohne Übergang auf der Stelle.
pub fn arrow_aim(f: &Frame, stage: i64, w: &World, x: f64) -> Option<ArrowAim> {
    let t = target_of(f)?;
    let ts = t.stage as i64;
    if ts == stage {
        return Some(ArrowAim { x: t.x, dy: 0 });
    }
    let (dy, kind) = if ts > stage {
        (1, "stairsDown")
    } else {
        (-1, "stairsUp")
    };
    let gate = w
        .sites
        .iter()
        .filter(|s| s.kind == kind)
        .map(|s| s.x)
        .min_by(|a, b| (a - x).abs().total_cmp(&(b - x).abs()))
        .unwrap_or(x);
    Some(ArrowAim { x: gate, dy })
}

/// `arrow_dir` ist die Richtung des Pfeils in der Kamera `k` als (dx, dy) mit je -1, 0, 1 (dy: 1 = nach unten);
/// `None`, wenn nichts zu zeigen ist (Ziel in der eigenen Stufe im Bild).
pub fn arrow_dir(aim: Option<ArrowAim>, k: &Camera) -> Option<(i8, i8)> {
    let a = aim?;
    let (l, r) = k.span();
    let dx = if a.x < l {
        -1
    } else if a.x > r {
        1
    } else {
        0
    };
    (dx != 0 || a.dy != 0).then_some((dx, a.dy))
}

impl Hud {
    /// `bottle_story` reiht beim Aufheben der Flasche (`bottleFound`) die Zeilen des Eintrags `line` bei allen aktiven
    /// Spielern ein; ohne Zeilen (`line` < 0 oder keine Einträge) erscheint nichts. Das Laden eines Stands sendet das
    /// Ereignis nicht, die Zeilen kommen also nicht noch einmal (R8).
    pub fn bottle_story(&mut self, f: &Frame) {
        for e in &f.events {
            if e.type_() != "bottleFound" {
                continue;
            }
            let line = int_of(&e.data, "line");
            if let Some(lines) = usize::try_from(line).ok().and_then(|i| f.campaign.bottles.get(i)) {
                self.tell(lines, 0);
            }
        }
    }
}

/// Abstand des Pfeils vom Zellenrand und Schritt zwischen seinen Punkten in px bei 1080.
const ARROW_EDGE: f64 = 70.0;
const ARROW_STEP: f64 = 16.0;
/// Der Pfeil besteht aus drei wachsenden Punkten in Richtung des Ziels, dezent (Deckkraft) und ohne Schrift.
const ARROW_CLR: u32 = 0xffe27a;
const ARROW_ALPHA: f64 = 0.7;

impl Hud {
    /// `draw_bottle_arrow` zeichnet den Pfeil der Zelle `c` an den Rand, falls die Markierung läuft und das Ziel nicht im
    /// Bild ist: links oder rechts mittig, bei tieferem oder höherem Ziel unten bzw. oben (schräg, wenn der Übergang
    /// seitlich außerhalb liegt).
    pub fn draw_bottle_arrow(&mut self, dst: &mut dyn Canvas, st: &SeatState, c: &Cell, k: f64) {
        let Some((dx, dy)) = arrow_dir(st.arrow, &c.camera) else {
            return;
        };
        let (l, t) = (c.rect.min.x as f64, c.rect.min.y as f64);
        let (w, h) = (c.rect.dx() as f64, c.rect.dy() as f64);
        let edge = ARROW_EDGE * k;
        let ax = match dx {
            -1 => l + edge,
            1 => l + w - edge,
            _ => l + w / 2.0,
        };
        let ay = match dy {
            -1 => t + edge,
            1 => t + h - edge * 2.0,
            _ => t + h / 2.0,
        };
        let len = f64::from(dx * dx + dy * dy).sqrt();
        let (ux, uy) = (f64::from(dx) / len, f64::from(dy) / len);
        for i in 0..3 {
            let d = f64::from(i) * ARROW_STEP * k;
            let r = (3.0 + 1.5 * f64::from(i)) * k;
            self.paint
                .circle(dst, ax + ux * d, ay + uy * d, r, rgb(ARROW_CLR, ARROW_ALPHA));
        }
    }
}

#[cfg(test)]
#[path = "bottle_tests.rs"]
mod bottle_tests;
