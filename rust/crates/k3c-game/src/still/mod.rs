//! Standbild (RU13.6): `k3c-game still <zustand> --out <png> [--grafik <ordner>]` lädt einen Go-Zustand
//! (`testdata/rust/bild/<name>.state.json.gz`), zeichnet ihn mit den Ebenen des Spiels in ein Bild im Speicher und
//! schreibt ein PNG. Das Werkzeug öffnet kein Fenster und braucht kein GPU-Backend.
//!
//! Schnittstelle für spätere Sprints (RU16, RU20, RU9, RU23, RU30):
//! - Ebenen sind `compose::Builder` in `compose::LAYERS` (Name, Funktion); ein Builder legt die Bibliothek seines
//!   Moduls an, lädt dessen Assets (`AssetSource`) und liefert Drawer und Listener als `Part`.
//! - `render_with(state, cfg, layers)` zeichnet mit einer beliebigen Liste von Ebenen (Tests nehmen einzelne);
//!   `render` nimmt `LAYERS`.
//! - Der Rasterer (`raster::SoftCanvas`) ist eine vollständige `Canvas`: Sprites, Rechtecke, Netze, Formen und Schrift.
//!   Er zeichnet in Aufrufreihenfolge wie Go (siehe `raster`); `layer()` der Aufrufer ist ohne Wirkung.
//! - Der Zustand (`state::StillState`) trägt Sim-Welt, Plätze, Kamera je Zelle und Bildgröße; das Dateiformat schreibt
//!   `native/k3c-game/world/rustref_test.go` (Go-Referenz und Bild am selben Zustand).

use std::path::{Path, PathBuf};

use crate::art::{Config, parse_scales};
use crate::view::layer::Drawer;

pub mod compose;
mod inflate;
pub mod pngio;
pub mod raster;
#[cfg(test)]
mod raster_tests;
pub mod state;

pub use compose::{Builder, LAYERS, Part, Setup};
pub use raster::Image;
pub use state::{StillState, load_state, parse_state};

/// Umgebungsvariable des Grafik-Ordners (gerenderte Grafik, nicht eingecheckt).
pub const GRAFIK_ENV: &str = "K3C_GRAFIK_DIR";

/// Schalter von `k3c-game still`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    pub state: PathBuf,
    pub out: PathBuf,
    pub grafik: Option<PathBuf>,
}

/// Liest die Schalter: `<zustand> --out <png> [--grafik <ordner>]`.
pub fn parse_args(args: &[String]) -> Result<Options, String> {
    let (mut state, mut out, mut grafik) = (None, None, None);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut value = |name: &str| it.next().cloned().ok_or(format!("{name} braucht einen Wert"));
        match a.as_str() {
            "--out" => out = Some(PathBuf::from(value("--out")?)),
            "--grafik" => grafik = Some(PathBuf::from(value("--grafik")?)),
            s if !s.starts_with("--") && state.is_none() => state = Some(PathBuf::from(s)),
            s => return Err(format!("unbekannter Schalter {s}")),
        }
    }
    Ok(Options {
        state: state.ok_or("Aufruf: k3c-game still <zustand> --out <png> [--grafik <ordner>]")?,
        out: out.ok_or("--out <png> fehlt")?,
        grafik,
    })
}

/// Pfade der Grafik: `grafik` (sonst `K3C_GRAFIK_DIR`, sonst `public/grafik`), Rigs und Schrift aus dem Repo `root`.
pub fn config(root: &Path, grafik: Option<&Path>) -> Config {
    let grafik_dir = grafik
        .map(Path::to_path_buf)
        .or_else(|| {
            std::env::var(GRAFIK_ENV)
                .ok()
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
        })
        .unwrap_or_else(|| root.join("public/grafik"));
    Config {
        grafik_dir,
        rig_dir: root.join("native/k3c-rig/assets"),
        font_path: root.join("public/schrift/k3c.ttf"),
        strict_font: true,
        // Maßstäbe aus data/paper.json des Repos (nicht vom Arbeitsordner abhängig); fehlt die Datei, gilt überall 1
        scales: std::fs::read(root.join("data/paper.json"))
            .map_err(|e| e.to_string())
            .and_then(|raw| parse_scales(&raw))
            .unwrap_or_default(),
    }
}

/// Zeichnet den Stand mit den Ebenen des Spiels.
pub fn render(s: &StillState, cfg: &Config) -> Result<Image, String> {
    render_with(s, cfg, LAYERS)
}

/// Zeichnet den Stand mit den genannten Ebenen: je Zelle (beschnitten auf die Zelle) alle Drawer nach Ebene.
pub fn render_with(s: &StillState, cfg: &Config, layers: &[(&str, Builder)]) -> Result<Image, String> {
    let mut canvas = raster::SoftCanvas::new(s.size.0, s.size.1);
    let setup = Setup {
        cfg: cfg.clone(),
        level: &s.level,
        world: &s.world,
    };
    let mut parts = Vec::new();
    for (name, build) in layers {
        parts.push(build(&setup, &mut canvas).map_err(|e| format!("{name}: {e}"))?);
    }
    let (frame, cells) = s.frame();
    for p in &mut parts {
        for l in &mut p.listeners {
            l.listen(&frame);
        }
    }
    let mut drawers: Vec<&dyn Drawer> = parts.iter().flat_map(|p| p.drawers.iter().map(AsRef::as_ref)).collect();
    drawers.sort_by_key(|d| d.layer());
    for c in &cells {
        use k3c_platform::Canvas;
        canvas.clip(Some(c.rect.to_rect()));
        for d in &drawers {
            d.draw(&mut canvas, &frame, c);
        }
        canvas.clip(None);
    }
    Ok(canvas.finish())
}

/// Führt `k3c-game still` aus.
pub fn run(o: &Options) -> Result<(), String> {
    let s = load_state(&o.state)?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let img = render(&s, &config(&root, o.grafik.as_deref()))?;
    pngio::write_png(&o.out, &img)?;
    k3c_platform::log::info(
        "💾 Standbild geschrieben",
        &[("datei", &o.out.display()), ("stand", &s.name)],
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn schalter() {
        let o = parse_args(&args(&["a.gz", "--out", "b.png", "--grafik", "g"])).unwrap();
        assert_eq!(o.state, PathBuf::from("a.gz"));
        assert_eq!(o.out, PathBuf::from("b.png"));
        assert_eq!(o.grafik, Some(PathBuf::from("g")));
        assert!(parse_args(&args(&["a.gz"])).is_err());
        assert!(parse_args(&args(&["--out", "b.png"])).is_err());
        assert!(parse_args(&args(&["a.gz", "--out"])).is_err());
        assert!(parse_args(&args(&["a.gz", "--out", "b", "--x"])).is_err());
    }
}
