//! Probe `--two`: zwei Bildhälften (`clip`), je Gerät ein Kästchen (links Tastatur, rechts das erste Pad), Text mit
//! Umlauten aus `public/schrift/k3c.ttf` (sonst Pixel-Schrift mit Warnung), Taste T oder Pad A spielt einen Klang.

use std::path::Path;

use k3c_platform::{AudioOut, Canvas, FontId, InputState, Key, PadAxis, PadButton, Rect, STEP, SoundId, log};

/// Kästchen-Tempo in logischen Pixeln je Sekunde.
const SPEED: f64 = 420.0;
const BOX: f32 = 80.0;
const HALF: f32 = 960.0;
const COLORS: [[f32; 4]; 2] = [[1.0, 0.78, 0.2, 1.0], [0.3, 0.8, 1.0, 1.0]];

/// Zwei Spieler, je eine Bildhälfte.
pub struct Two {
    /// Lage je Kästchen in seiner Hälfte (links oben), vorige und jetzige für die Interpolation.
    pos: [[f64; 2]; 2],
    prev: [[f64; 2]; 2],
    pub steps: u64,
    pub sounds: u64,
    quit: bool,
    full: bool,
    font: Option<Option<FontId>>,
    audio: Box<dyn AudioOut>,
    beep: Option<SoundId>,
}

impl Two {
    pub fn new(mut audio: Box<dyn AudioOut>) -> Self {
        let ogg = repo().join("public/audio/probe-a.ogg");
        let beep = match std::fs::read(&ogg)
            .map_err(|e| e.to_string())
            .and_then(|b| audio.load(&b))
        {
            Ok(s) => Some(s),
            Err(e) => {
                log::warn("🚫 Klang der Probe fehlt", &[("datei", &ogg.display()), ("grund", &e)]);
                None
            }
        };
        let start = [[440.0, 500.0]; 2];
        Self {
            pos: start,
            prev: start,
            steps: 0,
            sounds: 0,
            quit: false,
            full: false,
            font: None,
            audio,
            beep,
        }
    }

    /// Lage des Kästchens `i` nach dem Lauf (Tests, headless).
    pub fn box_pos(&self, i: usize) -> [f64; 2] {
        self.pos[i]
    }

    fn mv(&mut self, i: usize, dx: f64, dy: f64) {
        let p = &mut self.pos[i];
        p[0] = (p[0] + dx * SPEED * STEP).clamp(0.0, f64::from(HALF - BOX));
        p[1] = (p[1] + dy * SPEED * STEP).clamp(0.0, f64::from(1080.0 - BOX));
    }

    fn beep(&mut self) {
        if let Some(s) = self.beep {
            self.audio.play(s, 0.8);
        }
        self.sounds += 1;
    }

    /// Lädt die Spielschrift beim ersten Bild; fehlt sie, zeichnet die Probe mit der Pixel-Schrift.
    fn font(&mut self, c: &mut dyn Canvas) -> Option<FontId> {
        *self.font.get_or_insert_with(|| {
            let p = repo().join("public/schrift/k3c.ttf");
            match std::fs::read(&p).map_err(|e| e.to_string()).and_then(|b| c.font(&b)) {
                Ok(f) => Some(f),
                Err(e) => {
                    log::warn(
                        "🚫 Schrift fehlt, Pixel-Schrift",
                        &[("datei", &p.display()), ("grund", &e)],
                    );
                    None
                }
            }
        })
    }
}

impl k3c_platform::App for Two {
    fn step(&mut self, input: &InputState, _dt: f64) {
        self.prev = self.pos;
        self.steps += 1;
        self.quit |= input.just_pressed(Key::Escape);
        if input.just_pressed(Key::F) {
            self.full = !self.full;
        }
        let k = |a: Key, b: Key| f64::from(u8::from(input.held(b))) - f64::from(u8::from(input.held(a)));
        let (kx, ky) = (
            k(Key::A, Key::D) + k(Key::ArrowLeft, Key::ArrowRight),
            k(Key::W, Key::S),
        );
        self.mv(0, kx.clamp(-1.0, 1.0), ky);
        if input.just_pressed(Key::T) {
            self.beep();
        }
        if let Some(p) = input.pads().first() {
            let (px, py) = (p.axis(PadAxis::LX), p.axis(PadAxis::LY));
            let dx = px + f64::from(u8::from(p.held(PadButton::Right))) - f64::from(u8::from(p.held(PadButton::Left)));
            let dy = py + f64::from(u8::from(p.held(PadButton::Down))) - f64::from(u8::from(p.held(PadButton::Up)));
            self.mv(1, dx.clamp(-1.0, 1.0), dy.clamp(-1.0, 1.0));
            if p.just_pressed(PadButton::A) {
                self.beep();
            }
        }
    }

    fn draw(&mut self, c: &mut dyn Canvas, alpha: f64) {
        let font = self.font(c);
        for (i, &color) in COLORS.iter().enumerate() {
            let x0 = i as f32 * HALF;
            c.clip(Some(Rect::new(x0, 0.0, HALF, 1080.0)));
            c.layer(0);
            let bg = if i == 0 {
                [0.16, 0.2, 0.3, 1.0]
            } else {
                [0.2, 0.16, 0.26, 1.0]
            };
            c.rect(Rect::new(x0, 0.0, HALF, 1080.0), bg);
            let lerp = |k: usize| self.prev[i][k] + (self.pos[i][k] - self.prev[i][k]) * alpha;
            // Das Kästchen ragt absichtlich über die Mitte, wenn es am Rand steht: der Ausschnitt schneidet es ab.
            let (x, y) = (x0 + lerp(0) as f32 + 40.0, lerp(1) as f32);
            c.layer(1);
            c.rect(Rect::new(x, y, BOX, BOX), color);
            c.stroke_rect(Rect::new(x0 + 4.0, 4.0, HALF - 8.0, 1072.0), 4.0, color);
            let label = if i == 0 {
                "Spieler 1 · Tastatur (WASD, T)"
            } else {
                "Spieler 2 · Pad (Stick, A)"
            };
            match font {
                Some(f) => c.text_font(f, label, x0 + 40.0, 40.0, 40.0, [1.0; 4]),
                None => c.text(label, x0 + 40.0, 40.0, 28.0, [1.0; 4]),
            }
        }
        c.clip(None);
        c.layer(2);
        let line = "Grüße, ÄÖÜ äöü ß";
        match font {
            Some(f) => {
                let w = c.text_width(f, line, 56.0);
                c.text_font(f, line, 960.0 - w / 2.0, 980.0, 56.0, [1.0, 0.95, 0.7, 1.0]);
            }
            None => c.text(line, 600.0, 980.0, 48.0, [1.0, 0.95, 0.7, 1.0]),
        }
    }

    fn done(&self) -> bool {
        self.quit
    }

    fn fullscreen(&self) -> Option<bool> {
        Some(self.full)
    }
}

/// Wurzel des Repos (für Schrift und Klang).
pub fn repo() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../.."))
}
