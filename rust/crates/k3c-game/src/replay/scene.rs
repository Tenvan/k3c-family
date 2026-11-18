//! Die Zeichner des Spiels in der Wiedergabe (RU39.1): Welt, Bauten, Figuren, Effekte und HUD in der Anmeldereihenfolge
//! von `native/k3c-game/game/parts.go` (world, props, actors, fx, hud; Werkzeuge, Menü und Ton fehlen), je Zelle nach
//! Ebene stabil sortiert wie `drawGame` in `game.go`. Gelesen, nicht portiert: die Spielschleife portiert RU26.
//!
//! Anpassung statt Umbau (wie `still::compose`): `world` hält die Bibliothek als `Rc<Library>`, die übrigen Module als
//! `Rc<RefCell<Library>>`. Die Welt bekommt eine eigene Bibliothek, Bauten, Figuren, Effekte und HUD teilen sich eine
//! (wie die eine Bibliothek in Go). Fehlt Grafik, zeichnen die Module ohne Sheets weiter (Meldung, kein Abbruch), damit
//! die Wiedergabe auch ohne `task gen:paper` (Tests, CI) läuft.
//!
//! Sim-Naht: Bauten und HUD bekommen `NeutralSimView` (neutrale Werte, kein Sim-Aufruf); die Effekte die Standard-Quelle
//! der Auren (`fx::default_aura_source`, leer bis RU18). Ton bleibt aus.

use std::cell::RefCell;
use std::rc::Rc;

use k3c_platform::Canvas;
use k3c_sim::state::World as SimWorld;

use super::neutral::NeutralSimView;
use crate::actors::{self, draw::ActorsDrawer};
use crate::art::{Config, Levels, Library};
use crate::view::cell::Cell;
use crate::view::frame::Frame;
use crate::view::layer::{Drawer, Layer, Listener};
use crate::view::roles::{AssetSource, CameraRule};
use crate::{fx, hud, props, world};

/// Teilt den Zustand eines Moduls zwischen Drawer und Listener (Go: dieselbe Instanz in beiden Rollen).
struct Shared<T>(Rc<RefCell<T>>);

impl<T: Drawer> Drawer for Shared<T> {
    fn layer(&self) -> Layer {
        self.0.borrow().layer()
    }
    fn draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        self.0.borrow().draw(dst, f, c);
    }
}

impl<T: Listener> Listener for Shared<T> {
    fn listen(&mut self, f: &Frame) {
        self.0.borrow_mut().listen(f);
    }
}

/// Die Zeichner der Wiedergabe: Drawer nach Ebene sortiert, Listener in Anmeldereihenfolge, die Kamera-Regel der Welt.
pub struct Scene {
    /// Name des Moduls und Drawer, stabil nach Ebene sortiert
    pub drawers: Vec<(&'static str, Box<dyn Drawer>)>,
    pub listeners: Vec<Box<dyn Listener>>,
    pub camera_rule: Option<Box<dyn CameraRule>>,
    /// Meldungen beim Aufbau (fehlende Grafik)
    pub notes: Vec<String>,
}

fn levels(level: &str, assets: Vec<String>) -> Levels {
    Levels::from([(level.to_string(), assets)])
}

/// Öffnet eine Bibliothek und lädt die Assets des Levels; ohne Grafik bleibt sie leer, ohne Schrift fehlt sie.
fn library(
    cfg: Option<&Config>,
    level: &str,
    assets: Vec<String>,
    rigs: bool,
    c: &mut dyn Canvas,
    notes: &mut Vec<String>,
) -> Option<Library> {
    let cfg = cfg?;
    let cfg = if rigs {
        cfg.clone()
    } else {
        Config {
            rig_dir: std::path::PathBuf::new(),
            ..cfg.clone()
        }
    };
    let mut lib = match Library::new(cfg) {
        Ok(l) => l,
        Err(e) => {
            notes.push(format!("Bibliothek: {e}"));
            return None;
        }
    };
    if rigs && let Err(e) = lib.load_rigs(c) {
        notes.push(format!("Rigs: {e}"));
    }
    if let Err(e) = lib.set_levels(&levels(level, assets), c) {
        notes.push(format!("Grafik: {e}"));
    }
    Some(lib)
}

impl Scene {
    /// Baut die Zeichner für das Level `level` der Welt `w` und lädt ihre Grafik auf `c`; `cfg` `None`: ohne Grafik.
    pub fn build(cfg: Option<&Config>, level: &str, w: &SimWorld, c: &mut dyn Canvas) -> Scene {
        let mut notes = Vec::new();
        let world_assets = world::new(None)
            .asset_source
            .map(|a| a.assets(level, w))
            .unwrap_or_default();
        let wlib = library(cfg, level, world_assets, false, c, &mut notes).map(Rc::new);
        let parts = world::new(wlib);
        let mut assets = props::new(None, Box::new(NeutralSimView)).assets(level, w);
        assets.extend(actors::new(None).assets(level, w));
        assets.extend(fx::new(None, fx::default_aura_source()).assets(level, w));
        assets.extend(hud::new(None, Box::new(NeutralSimView)).assets(level, w));
        assets.sort();
        assets.dedup();
        let lib = library(cfg, level, assets, true, c, &mut notes).map(|l| Rc::new(RefCell::new(l)));
        let p = Rc::new(RefCell::new(props::new(lib.clone(), Box::new(NeutralSimView))));
        let a = Rc::new(RefCell::new(actors::new(lib.clone())));
        a.borrow_mut().check_rigs(c);
        let x = Rc::new(RefCell::new(fx::new(lib.clone(), fx::default_aura_source())));
        let h = Rc::new(RefCell::new(hud::new(lib, Box::new(NeutralSimView))));
        let mut drawers: Vec<(&'static str, Box<dyn Drawer>)> =
            parts.drawers.into_iter().map(|d| ("world", d)).collect();
        drawers.push(("props", Box::new(Shared(Rc::clone(&p)))));
        drawers.push(("actors", Box::new(ActorsDrawer { a: Rc::clone(&a) })));
        drawers.push(("fx", Box::new(Shared(Rc::clone(&x)))));
        drawers.push(("hud", Box::new(Shared(Rc::clone(&h)))));
        drawers.sort_by_key(|(_, d)| d.layer());
        drawers.retain(|(_, d)| d.layer().per_cell());
        Scene {
            drawers,
            listeners: vec![
                Box::new(Shared(p)),
                Box::new(ActorsDrawer { a }),
                Box::new(Shared(x)),
                Box::new(Shared(h)),
            ],
            camera_rule: parts.camera_rule,
            notes,
        }
    }

    /// Gibt den Frame einmal je Takt an alle Listener.
    pub fn listen(&mut self, f: &Frame) {
        for l in &mut self.listeners {
            l.listen(f);
        }
    }

    /// Zeichnet je Zelle (beschnitten auf die Zelle, wie `SubImage` in Go) alle Drawer nach Ebene, hebt den Ausschnitt am
    /// Ende auf; liefert die Zahl der Drawer-Aufrufe.
    pub fn draw(&self, c: &mut dyn Canvas, f: &Frame, cells: &[Cell]) -> u32 {
        let mut n = 0;
        for cell in cells {
            c.clip(Some(cell.rect.to_rect()));
            for (_, d) in &self.drawers {
                d.draw(c, f, cell);
                n += 1;
            }
        }
        c.clip(None);
        n
    }
}
