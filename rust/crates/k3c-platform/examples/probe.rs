//! Probe: gelbes Quadrat und N bewegte Sprites (wie `.work/vergleich/src/rust-last`).
//!
//! `cargo run --release -p k3c-platform --example probe -- n=20000 [secs=10] [measure=probe.json]`
//! Backend über `WGPU_BACKEND` (vulkan, dx12). Titel zeigt Bilder/s und späte Bilder (> 25 ms).

use std::collections::HashMap;
use std::path::PathBuf;

use k3c_platform::{App, Canvas, Config, Rect, STEP, TexId, run_window};

const SIZE: u32 = 32;

struct Probe {
    n: usize,
    tick: u64,
    tex: Option<TexId>,
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
    fn step(&mut self, _dt: f64) {
        self.tick += 1;
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
    }

    fn done(&self) -> bool {
        false
    }
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
    let mut app = Probe { n, tick: 0, tex: None };
    if let Err(e) = run_window(&mut app, cfg) {
        eprintln!("Fehler: {e}");
        std::process::exit(1);
    }
}
