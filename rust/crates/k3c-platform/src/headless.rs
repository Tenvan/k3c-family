//! Headless: Schleife ohne Fenster und Grafik, für Tests und Läufe ohne Anzeige.

use crate::{App, Canvas, LOGICAL, Rect, STEP, TexId};

/// Canvas, der nur zählt, was gezeichnet würde.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct NullCanvas {
    pub textures: u32,
    pub sprites: u64,
    pub rects: u64,
    pub texts: u64,
}

impl Canvas for NullCanvas {
    fn size(&self) -> (f32, f32) {
        LOGICAL
    }

    fn texture(&mut self, _rgba: &[u8], _w: u32, _h: u32) -> TexId {
        self.textures += 1;
        TexId(self.textures - 1)
    }

    fn sprite(&mut self, _t: TexId, _src: Rect, _dst: Rect, _tint: [f32; 4]) {
        self.sprites += 1;
    }

    fn rect(&mut self, _dst: Rect, _color: [f32; 4]) {
        self.rects += 1;
    }

    fn text(&mut self, _s: &str, _x: f32, _y: f32, _px: f32, _color: [f32; 4]) {
        self.texts += 1;
    }
}

/// Führt höchstens `steps` Takte aus (je Takt `step` und `draw` mit alpha 0); endet früher bei `done()`.
/// Liefert die Zähler des [`NullCanvas`] über alle Bilder.
pub fn run_headless(app: &mut dyn App, steps: u32) -> NullCanvas {
    let mut canvas = NullCanvas::default();
    for _ in 0..steps {
        if app.done() {
            break;
        }
        app.step(STEP);
        app.draw(&mut canvas, 0.0);
    }
    canvas
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Count {
        steps: u32,
        limit: u32,
    }

    impl App for Count {
        fn step(&mut self, dt: f64) {
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
        assert_eq!(run_headless(&mut a, 100).rects, 5);
        let mut b = Count { steps: 0, limit: 100 };
        assert_eq!(run_headless(&mut b, 7).rects, 7);
    }
}
