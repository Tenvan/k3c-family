//! RU39 (vorläufige Messung, eigener Messcode): CPU-Zeit der Zeichner in der Wiedergabe gegen `NullCanvas`, ohne
//! Fenster und ohne GPU. Misst je Bild `step` (Kopie des Zustands) und `draw` (Befehlsliste aller Zeichner, zwei
//! Zellen) und wertet Mittel, Perzentile, längstes Bild und Ausreißer aus. Nur mit `--ignored`:
//!
//! `K3C_TRACE=<datei.jsonl.gz> K3C_GRAFIK_DIR=<public/grafik> cargo test --release -p k3c-game --test messung_cpu -- --ignored --nocapture`
//!
//! Aussagekraft: nur der CPU-Teil der Zeichner (Befehle erzeugen), nicht Textur-Upload, GPU, VSync oder Treiber;
//! auf einem unruhigen Rechner vorläufig. Allokationen je Bild sind nicht messbar, weil der Workspace
//! `unsafe_code = "forbid"` setzt (ein zählender `GlobalAlloc` braucht `unsafe`).

use std::path::{Path, PathBuf};
use std::time::Instant;

use k3c_game::replay::app::ReplayApp;
use k3c_game::replay::trace::load_stream;
use k3c_platform::{App, InputState, NullCanvas, STEP};

const WARMUP: usize = 60;
const BUDGET_MS: f64 = 1000.0 / 60.0;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn pct(sorted: &[f64], q: f64) -> f64 {
    sorted[(((sorted.len() - 1) as f64) * q).round() as usize]
}

struct Pass {
    /// Zeichenbefehle (alle Arten) und Sprites je Bild im Mittel
    befehle: f64,
    sprites: f64,
    frames: Vec<f64>,
    steps: Vec<f64>,
    draws: Vec<f64>,
}

fn run_pass(file: &Path, frames: usize) -> Pass {
    let t = load_stream(file).expect("Aufzeichnung laden");
    let cfg = k3c_game::still::config(&root(), None);
    let mut app = ReplayApp::with_config(t, Some(cfg));
    let (input, mut canvas) = (InputState::default(), NullCanvas::default());
    app.draw(&mut canvas, 0.0);
    for _ in 0..WARMUP {
        app.step(&input, STEP);
        app.draw(&mut canvas, 0.5);
    }
    let (c0, s0) = (calls(&canvas), canvas.sprites);
    let mut p = Pass {
        befehle: 0.0,
        sprites: 0.0,
        frames: Vec::with_capacity(frames),
        steps: Vec::with_capacity(frames),
        draws: Vec::with_capacity(frames),
    };
    for _ in 0..frames {
        let t0 = Instant::now();
        app.step(&input, STEP);
        let t1 = Instant::now();
        app.draw(&mut canvas, 0.5);
        let t2 = Instant::now();
        if app.done() {
            break;
        }
        p.steps.push((t1 - t0).as_secs_f64() * 1000.0);
        p.draws.push((t2 - t1).as_secs_f64() * 1000.0);
        p.frames.push((t2 - t0).as_secs_f64() * 1000.0);
    }
    let n = p.frames.len().max(1) as f64;
    p.befehle = (calls(&canvas) - c0) as f64 / n;
    p.sprites = (canvas.sprites - s0) as f64 / n;
    p
}

fn calls(c: &NullCanvas) -> u64 {
    c.sprites + c.rects + c.texts + c.meshes + c.shapes
}

fn line(name: &str, v: &[f64]) -> String {
    let mut s = v.to_vec();
    s.sort_unstable_by(f64::total_cmp);
    let mean = s.iter().sum::<f64>() / s.len() as f64;
    let med = pct(&s, 0.5);
    let lang = s.iter().filter(|&&x| x > med * 1.5).count();
    let halb = s.iter().filter(|&&x| x > BUDGET_MS / 2.0).count();
    let ueber = s.iter().filter(|&&x| x > BUDGET_MS).count();
    format!(
        "{name}: n={} mittel={mean:.4} p50={med:.4} p99={:.4} p99,9={:.4} max={:.4} max/p50={:.1} >1,5x-p50={lang} >8,3ms={halb} >16,7ms={ueber}",
        s.len(),
        pct(&s, 0.99),
        pct(&s, 0.999),
        s[s.len() - 1],
        s[s.len() - 1] / med,
    )
}

#[test]
#[ignore = "Messung: braucht K3C_TRACE und gerenderte Grafik (K3C_GRAFIK_DIR)"]
fn cpu_zeit_der_zeichner() {
    let file = std::env::var("K3C_TRACE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| root().join(".work/trace/messung-nacht.jsonl.gz"));
    let passes: usize = std::env::var("K3C_PASSES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3);
    let frames: usize = std::env::var("K3C_FRAMES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(9000);
    println!(
        "Datei: {} · Grafik: {:?}",
        file.display(),
        k3c_game::still::config(&root(), None).grafik_dir
    );
    for n in 1..=passes {
        let p = run_pass(&file, frames);
        println!(
            "Lauf {n}: je Bild {:.0} Zeichenbefehle, davon {:.0} Sprites",
            p.befehle, p.sprites
        );
        println!("  {}", line("Bild (step+draw)", &p.frames));
        println!("  {}", line("step", &p.steps));
        println!("  {}", line("draw", &p.draws));
        let third = p.frames.len() / 3;
        for (i, w) in p.frames.chunks(third.max(1)).take(3).enumerate() {
            println!("  {}", line(&format!("Drittel {}", i + 1), w));
        }
    }
}
