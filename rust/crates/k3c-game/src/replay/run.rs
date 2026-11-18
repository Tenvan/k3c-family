//! Läufe des Unterbefehls `replay`: ohne Fenster (Prüfwert) und im Fenster (Messung).

use std::path::{Path, PathBuf};

use k3c_platform::{App, Config, InputState, NullCanvas, STEP, run_window};

use super::app::ReplayApp;
use super::cli::{DEFAULT_SIZE, Mode, Options, WARMUP_STEPS};
use super::frames::FrameLog;
use super::report::{Metered, Report};
use super::trace;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// FNV-1a 64 über Bytes, fortlaufend.
pub fn fnv(mut h: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        h = (h ^ u64::from(b)).wrapping_mul(FNV_PRIME);
    }
    h
}

/// Ergebnis des Laufs ohne Fenster.
pub struct Headless {
    /// FNV-1a 64 über alle Zeilen der Datei (je mit `\n`) und die Zahl der Zeichenbefehle je Bild.
    pub checksum: u64,
    pub steps: u64,
    pub report: Metered,
}

/// Grafik der Wiedergabe: aus dem Arbeitsordner (`public/grafik`, Rigs, Schrift; `K3C_GRAFIK_DIR` gilt vor).
pub fn art_config() -> crate::art::Config {
    let root = std::env::current_dir().unwrap_or_default();
    crate::still::config(&root, None)
}

/// Zeichenbefehle des Canvas bis jetzt (alle Arten).
fn calls(c: &NullCanvas) -> u64 {
    c.sprites + c.rects + c.texts + c.meshes + c.shapes
}

/// Spielt die Aufzeichnung ohne Fenster ab: ein Bild je Takt (Zwischenbild bei alpha 0,5) plus das erste Bild, mit
/// der Grafik aus dem Arbeitsordner (fehlt sie, zeichnen die Module ohne Sheets).
pub fn run_headless(lines: &[String], measure: u64) -> Result<Headless, String> {
    run_headless_with(lines, measure, Some(art_config()))
}

/// Wie `run_headless` mit der Grafik aus `cfg` (`None`: ohne Grafik).
pub fn run_headless_with(lines: &[String], measure: u64, cfg: Option<crate::art::Config>) -> Result<Headless, String> {
    let t = trace::parse(lines).map_err(|e| e.to_string())?;
    let mut h = FNV_OFFSET;
    for l in lines {
        h = fnv(fnv(h, l.as_bytes()), b"\n");
    }
    let mut app = Metered::new(ReplayApp::with_config(t, cfg), WARMUP_STEPS, measure);
    let (input, mut canvas) = (InputState::default(), NullCanvas::default());
    app.draw(&mut canvas, 0.0);
    let mut seen = calls(&canvas);
    h = fnv(h, &seen.to_le_bytes());
    let mut steps = 0;
    while !app.done() {
        app.step(&input, STEP);
        app.draw(&mut canvas, 0.5);
        let n = calls(&canvas);
        h = fnv(h, &(n - seen).to_le_bytes());
        seen = n;
        steps += 1;
    }
    Ok(Headless {
        checksum: h,
        steps,
        report: app,
    })
}

/// Zielpfad des Berichts.
pub fn report_path(o: &Options, r: &Report) -> PathBuf {
    o.out.clone().unwrap_or_else(|| {
        let name: String = r
            .szene
            .split_whitespace()
            .next()
            .unwrap_or("replay")
            .trim_start_matches("szene=")
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '_' })
            .collect();
        Path::new("reports").join(format!("replay-{name}.json"))
    })
}

fn write_report(o: &Options, r: &Report) -> Result<(), String> {
    let path = report_path(o, r);
    if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    let json = serde_json::to_string_pretty(r).map_err(|e| e.to_string())?;
    std::fs::write(&path, json + "\n").map_err(|e| format!("{}: {e}", path.display()))?;
    println!("Bericht: {}", path.display());
    Ok(())
}

/// Schreibt die Zeilen der Messung je Bild (nach der Messung).
fn write_frames(app: &Metered) -> Result<(), String> {
    if let Some(log) = &app.log {
        log.write().map_err(|e| format!("{}: {e}", log.path.display()))?;
        println!("Bilder: {} ({} verworfen)", log.path.display(), log.dropped);
    }
    Ok(())
}

/// Messung im Fenster: Puffer je Bild vor dem Start, Bericht und Zeilen danach.
fn run_window_mode(o: &Options) -> Result<(), String> {
    let t = trace::load_stream(&o.file).map_err(|e| format!("{}: {e}", o.file.display()))?;
    let measure = o.measure.unwrap_or(u64::MAX - WARMUP_STEPS);
    let log = o.frames.as_deref().map(|p| FrameLog::new(p, measure as f64 * STEP));
    let mut app = Metered::new(
        ReplayApp::with_config(t, Some(art_config())),
        if o.measure.is_some() { WARMUP_STEPS } else { u64::MAX },
        measure,
    )
    .with_frames(log);
    // Der Bericht nennt die echte Fenstergröße; --size wirkt erst mit B-1069
    app.size = DEFAULT_SIZE;
    if o.size != DEFAULT_SIZE {
        // k3c-platform öffnet das Fenster fest mit 1280x720 (logisch); --size steht nur im Bericht (B-1069)
        eprintln!(
            "⚠ replay: --size {}x{} nur im Bericht, das Fenster hat 1280x720",
            o.size.0, o.size.1
        );
    }
    let cfg = Config {
        title: format!("k3c replay ({:.0} Hz)", 1.0 / STEP),
        measure: None,
        max_seconds: None,
        fullscreen: false,
    };
    run_window(&mut app, cfg)?;
    if o.measure.is_some() {
        write_report(o, &app.report("wgpu"))?;
        write_frames(&app)?;
    }
    Ok(())
}

/// Führt `replay` nach den Optionen aus.
pub fn run(o: &Options) -> Result<(), String> {
    match o.mode {
        Mode::Headless => {
            let lines = trace::read_lines(&o.file).map_err(|e| format!("{}: {e}", o.file.display()))?;
            let hl = run_headless(&lines, o.measure.unwrap_or(u64::MAX - WARMUP_STEPS))?;
            if o.measure.is_some() {
                write_report(o, &hl.report.report("headless"))?;
            }
            println!("headless: {} Takte", hl.steps);
            println!("{:016x}", hl.checksum);
        }
        Mode::Window => run_window_mode(o)?,
    }
    Ok(())
}
