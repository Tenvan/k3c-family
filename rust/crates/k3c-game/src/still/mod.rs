//! Standbild (RU13.6): `k3c-game still <zustand> --out <png> [--grafik <ordner>]` lädt einen Go-Zustand
//! (`testdata/rust/bild/<name>.state.json.gz`), zeichnet ihn mit den Ebenen des Spiels in ein Bild im Speicher und
//! schreibt ein PNG. Das Werkzeug öffnet kein Fenster und braucht kein GPU-Backend.
//!
//! `--props --simview <props-name.simview.json>` zeichnet nur Boden und Bauten (`PROPS_LAYERS`, RU16.6); die Sim-Antworten
//! kommen als `FixedSimView` aus der Datei, bis RU15 und RU18 die Sim-Funktionen liefern.
//!
//! `--hud --simview <hud-name.simview.json> --frame <hud-name.frame.json>` zeichnet nur die Anzeigen je Hälfte
//! (`HUD_LAYERS`, RU20.7): Sitze, Geräte, Sprache und Meldungen kommen aus der Frame-Datei.
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
use crate::view::layer::{Drawer, Listener};

pub mod compose;
pub mod frame;
mod inflate;
pub mod pngio;
pub mod raster;
mod raster_mirror;
#[cfg(test)]
mod raster_tests;
pub mod simview;
pub mod state;

pub use compose::{Builder, HUD_LAYERS, LAYERS, PROPS_LAYERS, Part, Setup};
pub use frame::{FrameExtras, FrameFile};
pub use raster::Image;
pub use simview::FixedSimView;
pub use state::{StillState, load_state, parse_state};

/// Umgebungsvariable des Grafik-Ordners (gerenderte Grafik, nicht eingecheckt).
pub const GRAFIK_ENV: &str = "K3C_GRAFIK_DIR";

/// Schalter von `k3c-game still`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    pub state: PathBuf,
    pub out: PathBuf,
    pub grafik: Option<PathBuf>,
    /// Sim-Antworten der Bauten (`--simview`), nötig mit `--props`
    pub simview: Option<PathBuf>,
    /// Nur Boden und Bauten zeichnen (`--props`, RU16.6)
    pub props: bool,
    /// Nur die Anzeigen je Hälfte zeichnen (`--hud`, RU20.7), nötig mit `--simview` und `--frame`
    pub hud: bool,
    /// Sitze, Sprache, Eingabe-Profile und Meldungen des Frames (`--frame`, RU20.7)
    pub frame: Option<PathBuf>,
}

/// Liest die Schalter: `<zustand> --out <png> [--grafik <ordner>] [--props --simview <json>]` oder
/// `[--hud --simview <json> --frame <json>]`.
pub fn parse_args(args: &[String]) -> Result<Options, String> {
    let (mut state, mut out, mut grafik, mut simview, mut props) = (None, None, None, None, false);
    let (mut hud, mut frame) = (false, None);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut value = |name: &str| it.next().cloned().ok_or(format!("{name} braucht einen Wert"));
        match a.as_str() {
            "--out" => out = Some(PathBuf::from(value("--out")?)),
            "--grafik" => grafik = Some(PathBuf::from(value("--grafik")?)),
            "--simview" => simview = Some(PathBuf::from(value("--simview")?)),
            "--frame" => frame = Some(PathBuf::from(value("--frame")?)),
            "--props" => props = true,
            "--hud" => hud = true,
            s if !s.starts_with("--") && state.is_none() => state = Some(PathBuf::from(s)),
            s => return Err(format!("unbekannter Schalter {s}")),
        }
    }
    Ok(Options {
        state: state.ok_or("Aufruf: k3c-game still <zustand> --out <png> [--grafik <ordner>]")?,
        out: out.ok_or("--out <png> fehlt")?,
        grafik,
        simview,
        props,
        hud,
        frame,
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
        rig_dir: root.join("public/rig"),
        font_path: root.join("public/schrift/k3c.ttf"),
        strict_font: true,
        // Maßstäbe aus data/paper.json des Repos (nicht vom Arbeitsordner abhängig); fehlt die Datei, gilt überall 1
        scales: std::fs::read(root.join("data/paper.json"))
            .map_err(|e| e.to_string())
            .and_then(|raw| parse_scales(&raw))
            .unwrap_or_default(),
        // Größenstufen aus data/sizes.json des Repos, nur im Look-Profil k2c
        sizes: k3c_data::look::current()
            .sizes
            .then(|| {
                std::fs::read_to_string(root.join("data/sizes.json"))
                    .ok()
                    .and_then(|r| k3c_data::sizes::load(&r).ok())
            })
            .flatten(),
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
        simview: s.simview.as_ref(),
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

/// Zeichenliste der Bauten je Zelle (Planer mit geladener Grafik wie in `PROPS_LAYERS`, Körper und Oberes nacheinander).
/// Für Tests: Wo Rust etwas zeichnen will, ohne ein Bild zu vergleichen (Positionen, Textfelder).
pub fn props_items(s: &StillState, cfg: &Config) -> Result<Vec<Vec<crate::props::plan::Item>>, String> {
    use crate::props::listen::Infos;
    use crate::props::plan::{Planner, Sheets};
    use std::rc::Rc;
    let view = s.simview.as_ref().ok_or("props braucht die Sim-Antworten")?;
    let mut canvas = raster::SoftCanvas::new(s.size.0, s.size.1);
    let setup = Setup {
        cfg: cfg.clone(),
        level: &s.level,
        world: &s.world,
        simview: s.simview.as_ref(),
    };
    let lib = compose::props_library(&setup, &mut canvas)?;
    let (frame, cells) = s.frame();
    let mut infos = Infos::default();
    infos.listen(&frame, view);
    let w = &s.world;
    let mut out = Vec::new();
    for (i, c) in cells.iter().enumerate() {
        let mut p = Planner::new(Some(lib.clone() as Rc<dyn Sheets>));
        p.viewer = w.players.iter().position(|m| m.index == s.seats[i].monarch);
        p.plan(Some(w), infos.info(0), &c.camera, s.clock, view);
        out.push(p.body.iter().chain(&p.top).cloned().collect());
    }
    Ok(out)
}

/// Plätze einer Zelle: Index der Zelle, Rechtecke der gezeigten Elemente nach Element-Index.
pub type HudCellLayout = (i64, Vec<(usize, crate::hud::layout::Rect)>);

/// Plätze der Elemente des HUD je Zelle nach dem Zeichnen (`Hud::placed`): je Zelle (Index der Zelle, Rechtecke der
/// gezeigten Elemente nach Element-Index). Für den Platz-Test gegen `hud-<name>.layout.json` (RU20.7).
pub fn hud_layout(s: &StillState, cfg: &Config) -> Result<Vec<HudCellLayout>, String> {
    use k3c_platform::Canvas;
    let mut canvas = raster::SoftCanvas::new(s.size.0, s.size.1);
    let setup = Setup {
        cfg: cfg.clone(),
        level: &s.level,
        world: &s.world,
        simview: s.simview.as_ref(),
    };
    let hud = compose::hud_drawer(&setup, &mut canvas)?;
    let (frame, cells) = s.frame();
    hud.borrow_mut().listen(&frame);
    let mut out = Vec::new();
    for c in &cells {
        canvas.clip(Some(c.rect.to_rect()));
        hud.borrow().draw(&mut canvas, &frame, c);
        canvas.clip(None);
        let p = usize::try_from(c.player).map_err(|e| e.to_string())?;
        let placed = hud.borrow().hud().borrow().placed(p).ok_or("HUD ohne Plätze")?;
        let items = (0..placed.ok.len())
            .filter(|i| placed.ok[*i])
            .map(|i| (i, placed.at[i]))
            .collect();
        out.push((c.index, items));
    }
    Ok(out)
}

/// Führt `k3c-game still` aus.
pub fn run(o: &Options) -> Result<(), String> {
    let s = load_state(&o.state)?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let s = match &o.simview {
        Some(p) => s.with_simview(FixedSimView::load(p)?),
        None => s,
    };
    let s = match &o.frame {
        Some(p) => s.with_frame(FrameFile::load(p)?)?,
        None => s,
    };
    let cfg = config(&root, o.grafik.as_deref());
    let img = if o.props {
        render_with(&s, &cfg, PROPS_LAYERS)?
    } else if o.hud {
        render_with(&s, &cfg, HUD_LAYERS)?
    } else {
        render(&s, &cfg)?
    };
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
        assert!(!o.props && o.simview.is_none());
        let o = parse_args(&args(&["a.gz", "--out", "b.png", "--props", "--simview", "s.json"])).unwrap();
        assert!(o.props);
        assert_eq!(o.simview, Some(PathBuf::from("s.json")));
        let o = parse_args(&args(&[
            "a.gz",
            "--out",
            "b.png",
            "--hud",
            "--simview",
            "s.json",
            "--frame",
            "f.json",
        ]))
        .unwrap();
        assert!(o.hud && !o.props);
        assert_eq!(o.frame, Some(PathBuf::from("f.json")));
        assert!(parse_args(&args(&["a.gz"])).is_err());
        assert!(parse_args(&args(&["--out", "b.png"])).is_err());
        assert!(parse_args(&args(&["a.gz", "--out"])).is_err());
        assert!(parse_args(&args(&["a.gz", "--out", "b", "--x"])).is_err());
    }
}
