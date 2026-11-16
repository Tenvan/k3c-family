//! Geladene Assets auf der Zeichenfläche: Texturen hochladen, Frame wählen, Sprite mit Fußpunkt und Spiegelung setzen.

use k3c_platform::{Canvas, Rect, TexId};

use crate::art::{Asset, Frame, Image};

/// Ein Asset mit seinen hochgeladenen Sheets.
pub struct Gpu {
    tex: Vec<TexId>,
    asset: Asset,
}

/// Straight-Alpha (PNG) in vormultipliziertes Alpha (Pipeline der Plattform).
fn premultiply(img: &Image) -> Vec<u8> {
    let mut out = img.rgba.clone();
    for px in out.as_chunks_mut::<4>().0 {
        let a = u16::from(px[3]);
        for c in &mut px[..3] {
            *c = ((u16::from(*c) * a + 127) / 255) as u8;
        }
    }
    out
}

impl Gpu {
    pub fn upload(c: &mut dyn Canvas, asset: Asset) -> Self {
        let tex = asset
            .sheets
            .iter()
            .map(|s| c.texture(&premultiply(s), s.width, s.height))
            .collect();
        Self { tex, asset }
    }

    /// Frame der Animation `anim` zur Zeit `ticks` (Takte) samt Rahmen; unbekannte Animation → erste des Assets.
    pub fn pick(&self, anim: &str, ticks: f64) -> Option<(TexId, &Frame, [f64; 4])> {
        let r = self
            .asset
            .animationen
            .get(anim)
            .or_else(|| self.asset.animationen.values().next())?;
        let n = r.frames.max(1);
        let i = (ticks.max(0.0) / crate::demo::world::TICKS * r.fps) as usize % n;
        let f = self.asset.frames.get(r.start + i)?;
        Some((*self.tex.get(f.sheet)?, f, r.rahmen))
    }

    /// Zeichnet die Figur mit Fußpunkt `foot`; `s` verkleinert den Rahmen, `mirror` spiegelt um den Fußpunkt.
    pub fn draw(&self, c: &mut dyn Canvas, anim: &str, ticks: f64, foot: (f32, f32), s: f32, mirror: bool) {
        let Some((t, f, rahmen)) = self.pick(anim, ticks) else {
            return;
        };
        sprite_at(c, t, f, rahmen, foot, s, mirror);
    }
}

/// Ziel-Rechteck eines Frames mit Rahmen `[x, y, b, h]` relativ zum Fußpunkt, skaliert und optional gespiegelt.
pub fn target(rahmen: [f64; 4], foot: (f32, f32), s: f32, mirror: bool) -> Rect {
    let [rx, ry, rw, rh] = rahmen.map(|v| v as f32 * s);
    let x = if mirror { foot.0 - rx - rw } else { foot.0 + rx };
    Rect::new(x, foot.1 + ry, rw, rh)
}

pub fn sprite_at(c: &mut dyn Canvas, t: TexId, f: &Frame, rahmen: [f64; 4], foot: (f32, f32), s: f32, mirror: bool) {
    let [x, y, w, h] = f.rect.map(|v| v as f32);
    let src = if mirror {
        Rect::new(x + w, y, -w, h)
    } else {
        Rect::new(x, y, w, h)
    };
    c.sprite(t, src, target(rahmen, foot, s, mirror), [1.0; 4]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vormultiplizieren_skaliert_farbe_mit_alpha() {
        let img = Image {
            width: 2,
            height: 1,
            rgba: vec![200, 100, 50, 128, 10, 20, 30, 255],
        };
        assert_eq!(premultiply(&img), [100, 50, 25, 128, 10, 20, 30, 255]);
    }

    #[test]
    fn spiegeln_dreht_um_den_fusspunkt() {
        let rahmen = [-60.0, -100.0, 200.0, 100.0];
        assert_eq!(
            target(rahmen, (500.0, 800.0), 0.5, false),
            Rect::new(470.0, 750.0, 100.0, 50.0)
        );
        assert_eq!(
            target(rahmen, (500.0, 800.0), 0.5, true),
            Rect::new(430.0, 750.0, 100.0, 50.0)
        );
    }
}
