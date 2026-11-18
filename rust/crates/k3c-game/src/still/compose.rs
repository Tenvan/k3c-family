//! Zusammenbau der Ebenen des Standbilds. Jede Ebene ist ein `Builder`: er legt die Grafik-Bibliothek seines
//! Fachmoduls an, lädt die Assets, die das Modul im Level nennt (`AssetSource`), und liefert die Drawer und Listener
//! als `Part`. Neue Ebenen (props, fx, hud) tragen sich in `LAYERS` ein, mehr braucht es nicht.
//!
//! Anpassung statt Umbau (RU13.6): `world` hält die Bibliothek als `Rc<Library>`, `actors` als `Rc<RefCell<Library>>`.
//! Beide Module laden darum je eine eigene Bibliothek mit ihren Assets auf dieselbe Zeichenfläche.

use std::cell::RefCell;
use std::rc::Rc;

use k3c_platform::Canvas;
use k3c_sim::state::World as SimWorld;

use super::simview::FixedSimView;
use crate::actors::{self, draw::ActorsDrawer};
use crate::art::{Config, Levels, Library};
use crate::props::{self, Props};
use crate::view::frame::Frame;
use crate::view::layer::{Drawer, Layer, Listener};
use crate::view::roles::AssetSource;
use crate::world;

/// Was ein Builder braucht: Pfade der Grafik, Level und Welt des Standes.
pub struct Setup<'a> {
    pub cfg: Config,
    pub level: &'a str,
    pub world: &'a SimWorld,
    /// Antworten der Sim-Funktionen für `props` (RU16.6, Orakel); ohne sie lässt sich `props` nicht bauen
    pub simview: Option<&'a FixedSimView>,
}

/// Beitrag einer Ebene: Drawer (nach Ebene sortiert beim Zeichnen) und Listener (je Frame einmal, vor dem Zeichnen).
pub struct Part {
    pub name: &'static str,
    pub drawers: Vec<Box<dyn Drawer>>,
    pub listeners: Vec<Box<dyn Listener>>,
}

/// Baut eine Ebene und lädt ihre Grafik auf `canvas`.
pub type Builder = fn(&Setup, &mut dyn Canvas) -> Result<Part, String>;

/// Die Ebenen des Standbilds in Anmeldereihenfolge. Spätere Sprints (Gebäude, Effekte, Anzeigen) ergänzen hier.
pub const LAYERS: &[(&str, Builder)] = &[("world", build_world), ("actors", build_actors)];

/// Die Ebenen des Standbilds der Bauten (`still --props`, RU16.6): Boden der Welt (ohne Fluss und Licht) und props.
/// Die Figuren fehlen: das Go-Referenzbild (`props/rustref_test.go`) zeichnet sie auch nicht.
pub const PROPS_LAYERS: &[(&str, Builder)] = &[("ground", build_ground), ("props", build_props)];

fn levels(s: &Setup, assets: Vec<String>) -> Levels {
    Levels::from([(s.level.to_string(), assets)])
}

/// Welt: Himmel, Kulissen, Boden, Fluss, Licht.
pub fn build_world(s: &Setup, canvas: &mut dyn Canvas) -> Result<Part, String> {
    let assets = world::new(None)
        .asset_source
        .map(|a| a.assets(s.level, s.world))
        .unwrap_or_default();
    let mut lib = Library::new(Config {
        rig_dir: std::path::PathBuf::new(),
        ..s.cfg.clone()
    })
    .map_err(|e| e.to_string())?;
    lib.set_levels(&levels(s, assets), canvas).map_err(|e| e.to_string())?;
    let parts = world::new(Some(Rc::new(lib)));
    Ok(Part {
        name: "world",
        drawers: parts.drawers,
        listeners: Vec::new(),
    })
}

/// Figuren: Monarchen, Truppen, Gegner und ihre Anzeigen.
pub fn build_actors(s: &Setup, canvas: &mut dyn Canvas) -> Result<Part, String> {
    let assets = actors::new(None).assets(s.level, s.world);
    let mut lib = Library::new(s.cfg.clone()).map_err(|e| e.to_string())?;
    lib.load_rigs(canvas).map_err(|e| e.to_string())?;
    lib.set_levels(&levels(s, assets), canvas).map_err(|e| e.to_string())?;
    let a = Rc::new(RefCell::new(actors::new(Some(Rc::new(RefCell::new(lib))))));
    a.borrow_mut().check_rigs(canvas);
    Ok(Part {
        name: "actors",
        drawers: vec![Box::new(ActorsDrawer { a: Rc::clone(&a) })],
        listeners: vec![Box::new(ActorsDrawer { a })],
    })
}

/// Boden: die Ebenen der Welt bis `Layer::GROUND` (Himmel, Kulissen, Boden), ohne Fluss, Fackeln und Licht.
pub fn build_ground(s: &Setup, canvas: &mut dyn Canvas) -> Result<Part, String> {
    let mut part = build_world(s, canvas)?;
    part.drawers.retain(|d| d.layer() <= Layer::GROUND);
    part.name = "ground";
    Ok(part)
}

/// Drawer und Listener teilen sich den Zustand von `props` (wie `ActorsDrawer`).
struct PropsDrawer(Rc<RefCell<Props>>);

impl Drawer for PropsDrawer {
    fn layer(&self) -> Layer {
        self.0.borrow().layer()
    }
    fn draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &crate::view::cell::Cell) {
        self.0.borrow().draw(dst, f, c);
    }
}

impl Listener for PropsDrawer {
    fn listen(&mut self, f: &Frame) {
        self.0.borrow_mut().listen(f);
    }
}

/// Bibliothek der Bauten: lädt die Assets, die `props` im Level zeichnen kann.
pub fn props_library(s: &Setup, canvas: &mut dyn Canvas) -> Result<Rc<RefCell<Library>>, String> {
    let view = s.simview.ok_or(NO_SIMVIEW)?.clone();
    let assets = props::new(None, Box::new(view)).assets(s.level, s.world);
    let mut lib = Library::new(s.cfg.clone()).map_err(|e| e.to_string())?;
    lib.set_levels(&levels(s, assets), canvas).map_err(|e| e.to_string())?;
    Ok(Rc::new(RefCell::new(lib)))
}

const NO_SIMVIEW: &str = "props braucht die Sim-Antworten (--simview <props-name.simview.json>)";

/// Bauten, Natur und Objekte (`props`) mit den Sim-Antworten des Standes.
pub fn build_props(s: &Setup, canvas: &mut dyn Canvas) -> Result<Part, String> {
    let view = s.simview.ok_or(NO_SIMVIEW)?.clone();
    let lib = props_library(s, canvas)?;
    let p = Rc::new(RefCell::new(props::new(Some(lib), Box::new(view))));
    Ok(Part {
        name: "props",
        drawers: vec![Box::new(PropsDrawer(Rc::clone(&p)))],
        listeners: vec![Box::new(PropsDrawer(p))],
    })
}
