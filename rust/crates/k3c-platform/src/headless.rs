//! Headless: Schleife ohne Fenster, Grafik, Tongerät und Gamepad-Abfrage, für Tests und Läufe ohne Anzeige.

use crate::runner::Runner;
use crate::ttf::Fonts;
use crate::{Affine, App, Canvas, Filter, FontId, InputScript, LOGICAL, Rect, STEP, TexId, Vertex};

/// Canvas, der nur zählt, was gezeichnet würde.
#[derive(Default, Debug, Clone)]
pub struct NullCanvas {
    pub textures: u32,
    pub sprites: u64,
    pub rects: u64,
    pub texts: u64,
    pub clips: u64,
    /// Dreiecksnetze (`mesh`).
    pub meshes: u64,
    /// Formen außer Rechtecken: Rahmen, Kreise, Linien, Polygone.
    pub shapes: u64,
    pub freed: u32,
    /// Geladene Schriften: misst wie im Fenster.
    fonts: Fonts,
}

impl Canvas for NullCanvas {
    fn size(&self) -> (f32, f32) {
        LOGICAL
    }

    fn texture_with(&mut self, _rgba: &[u8], _w: u32, _h: u32, _f: Filter) -> TexId {
        self.textures += 1;
        TexId(self.textures - 1)
    }

    fn free_texture(&mut self, _t: TexId) {
        self.freed += 1;
    }

    fn sprite(&mut self, _t: TexId, _src: Rect, _dst: Rect, _tint: [f32; 4]) {
        self.sprites += 1;
    }

    fn sprite_tf(&mut self, _t: TexId, _src: Rect, _tf: Affine, _tint: [f32; 4]) {
        self.sprites += 1;
    }

    fn mesh(&mut self, _t: TexId, _verts: &[Vertex], _idx: &[u16]) {
        self.meshes += 1;
    }

    fn stroke_rect(&mut self, _r: Rect, _w: f32, _color: [f32; 4]) {
        self.shapes += 1;
    }

    fn fill_circle(&mut self, _cx: f32, _cy: f32, _r: f32, _color: [f32; 4]) {
        self.shapes += 1;
    }

    fn stroke_circle(&mut self, _cx: f32, _cy: f32, _r: f32, _w: f32, _color: [f32; 4]) {
        self.shapes += 1;
    }

    fn stroke_line(&mut self, _x0: f32, _y0: f32, _x1: f32, _y1: f32, _w: f32, _color: [f32; 4]) {
        self.shapes += 1;
    }

    fn fill_path(&mut self, _points: &[[f32; 2]], _color: [f32; 4]) {
        self.shapes += 1;
    }

    fn rect(&mut self, _dst: Rect, _color: [f32; 4]) {
        self.rects += 1;
    }

    fn text(&mut self, _s: &str, _x: f32, _y: f32, _px: f32, _color: [f32; 4]) {
        self.texts += 1;
    }

    fn font(&mut self, ttf: &[u8]) -> Result<FontId, String> {
        self.fonts.load(ttf)
    }

    fn text_font(&mut self, _f: FontId, _s: &str, _x: f32, _y: f32, _px: f32, _color: [f32; 4]) {
        self.texts += 1;
    }

    fn text_width(&self, f: FontId, s: &str, px: f32) -> f32 {
        self.fonts.width(f, s, px)
    }

    fn text_height(&self, f: FontId, px: f32) -> f32 {
        self.fonts.height(f, px)
    }

    fn clip(&mut self, _area: Option<Rect>) {
        self.clips += 1;
    }
}

/// Führt höchstens `steps` Takte über den Schleifen-Kern aus (je Bild genau ein Takt, alpha 0); endet früher bei
/// `done()`. `script` liefert die Eingabe. Liefert die Zähler des [`NullCanvas`] über alle Bilder.
pub fn run_headless(app: &mut dyn App, steps: u32, script: Option<&InputScript>) -> NullCanvas {
    let mut canvas = NullCanvas::default();
    let mut r = Runner::new(app, script);
    for _ in 0..steps {
        if r.app().done() {
            break;
        }
        r.advance(STEP, &mut canvas);
    }
    canvas
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InputState;

    struct Count {
        steps: u32,
        limit: u32,
    }

    impl App for Count {
        fn step(&mut self, _input: &InputState, dt: f64) {
            assert!((dt - STEP).abs() < 1e-12);
            self.steps += 1;
        }
        fn draw(&mut self, c: &mut dyn Canvas, alpha: f64) {
            assert_eq!(alpha, 0.0);
            c.rect(Rect::default(), [1.0; 4]);
        }
        fn done(&self) -> bool {
            self.steps >= self.limit
        }
    }

    #[test]
    fn laeuft_bis_done_oder_schrittgrenze() {
        let mut a = Count { steps: 0, limit: 5 };
        assert_eq!(run_headless(&mut a, 100, None).rects, 5);
        let mut b = Count { steps: 0, limit: 100 };
        assert_eq!(run_headless(&mut b, 7, None).rects, 7);
    }
}
