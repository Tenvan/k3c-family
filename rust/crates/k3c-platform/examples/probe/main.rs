//! Probe: gelbes Quadrat und N bewegte Sprites (wie `.work/vergleich/src/rust-last`).
//!
//! `cargo run --release -p k3c-platform --example probe -- n=20000 [secs=10] [measure=probe.json]`
//! Backend über `WGPU_BACKEND` (vulkan, dx12). Titel zeigt Bilder/s und späte Bilder (> 25 ms).

use std::collections::HashMap;
use std::path::PathBuf;

use k3c_platform::{
    Affine, App, AudioOut, Canvas, Config, FontId, InputState, Key, Rect, STEP, SoundId, TexId, Vertex, open_audio,
    run_window,
};

const SIZE: u32 = 32;

struct Probe {
    n: usize,
    tick: u64,
    tex: Option<TexId>,
    quit: bool,
    font: Option<Option<FontId>>,
    audio: Box<dyn AudioOut>,
    beep: Option<SoundId>,
}

/// Runder Fleck mit weichem Rand, vormultipliziertes Alpha.
fn blob() -> Vec<u8> {
    let mut px = vec![0u8; (SIZE * SIZE * 4) as usize];
    let c = (SIZE as f32 - 1.0) / 2.0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            let d = ((x as f32 - c).powi(2) + (y as f32 - c).powi(2)).sqrt();
            let a = (c - d).clamp(0.0, 1.0);
            let i = ((y * SIZE + x) * 4) as usize;
            px[i..i + 4].copy_from_slice(&[
                (255.0 * a) as u8,
                (200.0 * a) as u8,
                (150.0 * a) as u8,
                (255.0 * a) as u8,
            ]);
        }
    }
    px
}

impl App for Probe {
    fn step(&mut self, input: &InputState, _dt: f64) {
        self.tick += 1;
        self.quit |= input.just_pressed(Key::Escape);
        if input.just_pressed(Key::T)
            && let Some(s) = self.beep
        {
            self.audio.play(s, 0.8);
        }
    }

    fn draw(&mut self, c: &mut dyn Canvas, alpha: f64) {
        let tex = *self.tex.get_or_insert_with(|| c.texture(&blob(), SIZE, SIZE));
        let t = self.tick as f64 + alpha;
        let src = Rect::new(0.0, 0.0, SIZE as f32, SIZE as f32);
        for i in 0..self.n {
            let fi = i as f64;
            let x = (fi * 37.3 + t * 2.0 * (1.0 + (i % 5) as f64)) % 1920.0;
            let y = (fi * 13.7) % 1040.0;
            let tint = [0.5 + 0.25 * (i % 3) as f32, 0.8, 0.5 + 0.5 * (i % 7) as f32 / 6.0, 1.0];
            c.sprite(tex, src, Rect::new(x as f32, y as f32, 32.0, 32.0), tint);
        }
        let qx = (t * 16.0 % 1800.0) as f32;
        c.rect(Rect::new(qx, 400.0, 120.0, 120.0), [1.0, 0.78, 0.0, 1.0]);
        c.text(&format!("PROBE N{}", self.n), 60.0, 60.0, 84.0, [1.0; 4]);
        shapes(c, tex, t as f32);
        let ttf = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/schrift/k3c.ttf");
        let font = *self
            .font
            .get_or_insert_with(|| std::fs::read(&ttf).ok().and_then(|b| c.font(&b).ok()));
        if let Some(f) = font {
            c.text_font(
                f,
                "Grüße: ÄÖÜ äöü ß – Spieler 1",
                60.0,
                160.0,
                48.0,
                [1.0, 0.9, 0.6, 1.0],
            );
        }
    }

    fn done(&self) -> bool {
        self.quit
    }
}

/// Formen aus RU4.2: gedrehtes Sprite, Netz, Rahmen, Kreise, Linie, Polygon.
fn shapes(c: &mut dyn Canvas, tex: TexId, t: f32) {
    c.layer(1);
    let mut m = Affine::IDENTITY;
    let h = SIZE as f32 / 2.0;
    m.translate(-h, -h)
        .rotate(t * 0.05)
        .scale(4.0, 4.0)
        .translate(1500.0, 300.0);
    c.sprite_tf(tex, Rect::new(0.0, 0.0, SIZE as f32, SIZE as f32), m, [1.0; 4]);
    let s = SIZE as f32;
    let v = |x: f32, y: f32, k: f32| Vertex {
        pos: [1600.0 + x * 6.0, 600.0 + y * 6.0],
        uv: [x, y],
        color: [k, 1.0, 1.0 - k, 1.0],
    };
    c.mesh(
        tex,
        &[v(0.0, 0.0, 0.0), v(s, 0.0, 1.0), v(s, s, 0.5), v(0.0, s, 0.0)],
        &[0, 1, 2, 0, 2, 3],
    );
    c.stroke_rect(Rect::new(1200.0, 500.0, 300.0, 200.0), 6.0, [0.2, 1.0, 0.2, 1.0]);
    c.fill_circle(1300.0, 850.0, 60.0, [1.0, 0.2, 0.2, 0.6]);
    c.stroke_circle(1500.0, 850.0, 60.0, 5.0, [1.0; 4]);
    let a = t * 0.05;
    c.stroke_line(
        1700.0,
        850.0,
        1700.0 + 120.0 * a.cos(),
        850.0 + 120.0 * a.sin(),
        4.0,
        [1.0, 1.0, 0.0, 1.0],
    );
    let l = [
        [0.0, 0.0],
        [40.0, 0.0],
        [40.0, 80.0],
        [120.0, 80.0],
        [120.0, 120.0],
        [0.0, 120.0],
    ];
    let pts = l.map(|p| [p[0] + 1000.0, p[1] + 800.0]);
    c.fill_path(&pts, [0.4, 0.6, 1.0, 1.0]);
}

fn main() {
    let args: HashMap<String, String> = std::env::args()
        .skip(1)
        .filter_map(|s| s.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())))
        .collect();
    let n = args.get("n").and_then(|v| v.parse().ok()).unwrap_or(20_000);
    let cfg = Config {
        title: format!("k3c probe n={n} ({:.0} Hz)", 1.0 / STEP),
        measure: args.get("measure").map(PathBuf::from),
        max_seconds: args.get("secs").and_then(|v| v.parse().ok()),
        fullscreen: false,
    };
    let mut audio = open_audio();
    let ogg = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/audio/probe-a.ogg");
    let beep = std::fs::read(ogg).ok().and_then(|b| audio.load(&b).ok());
    let mut app = Probe {
        n,
        tick: 0,
        tex: None,
        quit: false,
        font: None,
        audio,
        beep,
    };
    if let Err(e) = run_window(&mut app, cfg) {
        eprintln!("Fehler: {e}");
        std::process::exit(1);
    }
}
