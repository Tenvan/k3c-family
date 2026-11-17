//! Sheet: eine Animation als Streifen auf der GPU; Vorlage `native/k3c-game/art/sheet.go`.

use k3c_platform::{Affine, Canvas, Rect, TexId};

use super::SheetSpec;

/// Ein Frame eines Sheets: Teilbild `src` (Textur-Pixel) der Textur `tex` (statt `*ebiten.Image` als SubImage).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SheetFrame {
    pub tex: TexId,
    pub src: Rect,
}

/// Sheet ist eine Animation als Streifen auf der GPU; Frames sind Teilbilder desselben Bildes (Atlas, ein Zeichenaufruf
/// je Sheet reicht für die Stapelung). Nur lesen; Laden und Freigeben macht die Library.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sheet {
    pub asset: String,
    pub anim: String,
    pub frames: Vec<SheetFrame>,
    pub fps: f64,
    pub fw: i64,
    pub fh: i64,
    /// Fußpunkt als Anteil des Frames (von links oben); Rahmen-Einheiten je Bild-Pixel.
    pub origin_x: f64,
    pub origin_y: f64,
    pub units_per_px: f64,

    pub(super) img: Option<TexId>,
}

/// Ungefärbt: Go zeichnet ohne ColorScale.
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

impl Sheet {
    /// Liefert Bild i; der Index läuft zyklisch (auch negativ), ein leeres Sheet liefert `None`.
    pub fn frame(&self, i: i64) -> Option<&SheetFrame> {
        let n = self.frames.len() as i64;
        if n == 0 {
            return None;
        }
        self.frames.get((((i % n) + n) % n) as usize)
    }

    /// Liefert das Bild zur Zeit t (Sekunden) bei der Bildrate des Sheets.
    pub fn frame_at(&self, t: f64) -> Option<&SheetFrame> {
        if self.fps <= 0.0 {
            return self.frame(0);
        }
        self.frame((t * self.fps) as i64)
    }

    /// Zeichnet Bild i mit der Matrix g (Bild-Pixel → Ziel) auf die Zeichenfläche, ohne Allokation.
    /// Filter linear, keine Mipmaps wie Go (`drawOp`).
    pub fn draw(&self, dst: &mut dyn Canvas, i: i64, g: Affine) {
        let Some(f) = self.frame(i) else {
            return;
        };
        dst.sprite_tf(f.tex, f.src, g, WHITE);
    }

    /// Textur des ganzen Streifens (zum Freigeben durch die Library).
    pub fn texture(&self) -> Option<TexId> {
        self.img
    }
}

/// Legt das Bild aus vormultiplizierten RGBA-Pixeln an und schneidet die Frames aus.
pub fn upload(spec: &SheetSpec, pix: &[u8], canvas: &mut dyn Canvas) -> Sheet {
    let img = canvas.texture(pix, spec.w as u32, spec.h as u32);
    let frames = (0..spec.frames)
        .map(|i| SheetFrame {
            tex: img,
            src: Rect::new((i * spec.fw) as f32, 0.0, spec.fw as f32, spec.fh as f32),
        })
        .collect();
    Sheet {
        asset: spec.asset.clone(),
        anim: spec.anim.clone(),
        frames,
        fps: spec.fps,
        fw: spec.fw,
        fh: spec.fh,
        origin_x: spec.origin_x,
        origin_y: spec.origin_y,
        units_per_px: spec.units_per_px,
        img: Some(img),
    }
}
