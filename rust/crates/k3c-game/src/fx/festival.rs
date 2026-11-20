//! Himmels-Effekt des Fests (RU62.13, B-1002): Solange `World::festival` gilt, steigen in Sengoku Laternen auf, am Nil
//! stehen Fackeln am Ufer und treiben Blüten auf dem Wasser. Der Effekt kommt aus `World::festival` (Sim, RU62.12, Datei
//! `data/festival.json › worlds`); Welten ohne Eintrag haben keinen Zustand und feiern ohne Himmels-Effekt. Gezeichnet
//! wird nur an der Oberfläche und nur, wenn der Hub im Bild ist. Alles ist reine Rechnung aus der Uhr des Frames (keine
//! Zufallsquelle); die Bilder `objekte/fest-*` ersetzen die Formen, sobald ihre Sheets geladen sind.

use k3c_platform::{Affine, Canvas};
use k3c_sim::state::World;

use super::paint::Stencils;
use crate::art::{Library, Sheet};
use crate::view::cell::Cell;

pub const LANTERN_ASSET: &str = "objekte/fest-laterne";
pub const BLOSSOM_ASSET: &str = "objekte/fest-bluete";
pub const TORCH_ASSET: &str = "objekte/fest-fackel";
const ANIM: &str = "stand";
/// Wie weit der Hub (Units) vom Bildrand entfernt sein darf, damit der Effekt läuft.
pub const HUB_VIEW: f64 = 14.0;
pub const LANTERNS: usize = 14;
pub const BLOSSOMS: usize = 16;
const SPREAD: f64 = 10.0;
const LANTERN_CLR: [u8; 3] = [255, 196, 92];
const FLAME_CLR: [u8; 4] = [255, 150, 50, 255];
const BLOSSOM_CLR: [u8; 4] = [255, 183, 197, 255];

fn frac(v: f64) -> f64 {
    v - v.floor()
}

/// Was der Himmels-Effekt der Welt zeigt (`effect` aus `World::festival`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sky {
    None,
    Lanterns,
    Torches,
}

/// `sky_of`: der Himmels-Effekt der Stufe; unter Tage, ohne Fest oder bei unbekanntem Effekt nichts.
pub fn sky_of(w: &World) -> Sky {
    if w.biome.depth != 0 {
        return Sky::None;
    }
    match w.festival.as_ref().map(|f| f.effect.as_str()) {
        Some("lanterns") => Sky::Lanterns,
        Some("torches") => Sky::Torches,
        _ => Sky::None,
    }
}

/// `lantern`: Laterne `i` zur Zeit `clock` als (Welt-x, Höhe in Units über dem Boden, Deckkraft); sie steigt in 20 s auf und
/// blendet an den Enden ein und aus.
#[allow(clippy::cast_precision_loss)]
pub fn lantern(i: usize, clock: f64, hub_x: f64) -> (f64, f64, f64) {
    let k = i as f64;
    let p = frac(k * 0.381_966_011 + clock * 0.05);
    let x = hub_x + (frac(k * 0.618_033_988_7) - 0.5) * 2.0 * SPREAD + (clock * 0.7 + k).sin() * 0.4;
    (x, 1.5 + p * 8.0, (std::f64::consts::PI * p).sin())
}

/// `torch_xs`: Orte der Fackeln am Ufer (die Inselränder und beiderseits des Hubs).
pub fn torch_xs(width: f64, hub_x: f64) -> [f64; 4] {
    [1.5, hub_x - 9.0, hub_x + 9.0, (width - 1.5).max(1.5)]
}

/// `blossom`: Blüte `i` als (Welt-x, Tiefe unter der Boden-Linie 0 … 1); sie treibt langsam nach rechts.
#[allow(clippy::cast_precision_loss)]
pub fn blossom(i: usize, clock: f64, hub_x: f64) -> (f64, f64) {
    let k = i as f64;
    let x = hub_x - 20.0 + frac(k * 0.618_033_988_7 + clock * 0.01) * 40.0;
    (x, 0.1 + frac(k * 0.43) * 0.5)
}

/// `flicker`: Flackern der Flamme einer Fackel (0,85 … 1,15).
#[allow(clippy::cast_precision_loss)]
pub fn flicker(i: usize, clock: f64) -> f64 {
    1.0 + 0.15 * (clock * 9.0 + i as f64 * 1.7).sin()
}

/// `hub_in_view`: liegt der Hub im Bild der Zelle.
pub fn hub_in_view(w: &World, c: &Cell) -> bool {
    c.camera.visible(w.hub_x, HUB_VIEW)
}

/// Ein Sprite des Sheets, Mitte unten bei (x, y), Breite `w` in Fenster-px.
#[allow(clippy::cast_possible_truncation)]
fn sprite(dst: &mut dyn Canvas, sh: &Sheet, clock: f64, at: (f64, f64), w: f64, alpha: f64) -> bool {
    let Some(fr) = sh.frame_at(clock) else {
        return false;
    };
    if sh.fw <= 0 || w <= 0.0 {
        return false;
    }
    let k = w / sh.fw as f64;
    let mut g = Affine::IDENTITY;
    g.scale(k as f32, k as f32)
        .translate((at.0 - w / 2.0) as f32, (at.1 - sh.fh as f64 * k) as f32);
    dst.sprite_tf(fr.tex, fr.src, g, [1.0, 1.0, 1.0, alpha.clamp(0.0, 1.0) as f32]);
    true
}

/// `draw`: der Himmels-Effekt der Stufe in der Zelle `c`; ohne Fest, unter Tage oder ohne Hub im Bild nichts.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn draw(st: &Stencils, dst: &mut dyn Canvas, w: &World, c: &Cell, clock: f64, lib: Option<&Library>) {
    let sky = sky_of(w);
    if sky == Sky::None || !hub_in_view(w, c) {
        return;
    }
    let (ground, s) = (c.ground_y() as f64, c.camera.scale);
    let sheet = |a: &str| lib.and_then(|l| l.sheet(a, ANIM));
    if sky == Sky::Lanterns {
        let sh = sheet(LANTERN_ASSET);
        for i in 0..LANTERNS {
            let (wx, h, a) = lantern(i, clock, w.hub_x);
            let (x, y) = (c.camera.screen_x(wx), ground - h * s);
            if !sh.is_some_and(|sh| sprite(dst, sh, clock, (x, y + 0.4 * s), 0.75 * s, a)) {
                let [r, g, b] = LANTERN_CLR;
                st.circle(dst, x, y, 0.3 * s, [r, g, b, (a * 230.0) as u8]);
                st.ring_at(dst, x, y, 0.55 * s, [255, 210, 120, (a * 90.0) as u8]);
            }
        }
        return;
    }
    let torch = sheet(TORCH_ASSET);
    for (i, wx) in torch_xs(w.width_units, w.hub_x).into_iter().enumerate() {
        let (x, f) = (c.camera.screen_x(wx), flicker(i, clock));
        if !torch.is_some_and(|sh| sprite(dst, sh, clock + i as f64 * 0.3, (x, ground), 0.6 * s, 1.0)) {
            st.pixel_at(
                dst,
                x - 0.05 * s,
                ground - 1.2 * s,
                0.1 * s,
                1.2 * s,
                [102, 57, 49, 255],
            );
            st.circle(dst, x, ground - 1.35 * s, 0.2 * s * f, FLAME_CLR);
        }
    }
    let petal = sheet(BLOSSOM_ASSET);
    let river = (c.rect.max.y as f64 - ground).max(0.0);
    for i in 0..BLOSSOMS {
        let (wx, d) = blossom(i, clock, w.hub_x);
        let (x, y) = (c.camera.screen_x(wx), ground + d * river);
        if !petal.is_some_and(|sh| sprite(dst, sh, clock + i as f64 * 0.2, (x, y + 0.1 * s), 0.5 * s, 0.9)) {
            st.circle(dst, x, y, 0.1 * s, BLOSSOM_CLR);
        }
    }
}

#[cfg(test)]
#[path = "festival_tests.rs"]
mod festival_tests;
