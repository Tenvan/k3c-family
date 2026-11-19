//! Port von `native/k3c-game/world/world.go` (Paketkopf): zeichnet je Zelle Himmel, Kulisse (fern, mitte, nah), Boden,
//! Fluss und Licht und führt die Kamera (B-795). Rechnung (Kamera, Culling, Schichten, Fluss, Licht) liegt in reinen
//! Funktionen ohne Zeichenfläche; die Drawer sind dünn und legen je Bild nichts an.
//!

use std::cell::RefCell;
use std::rc::Rc;

use k3c_platform::Canvas;
use k3c_sim::state::World as SimWorld;

use crate::art::Library;
use crate::view::cell::{Camera, Cell, Rectangle};
use crate::view::frame::Frame;
use crate::view::layer::{Drawer, Layer};
use crate::view::roles::{AssetSource, CameraRule};

pub mod backdrop;
pub mod bank;
pub mod camera;
pub mod cull;
pub mod ground;
pub mod halo;
pub mod layers;
pub mod light;
pub mod lightdraw;
pub mod mirror;
pub mod river;
pub mod tint;
pub mod water;

use backdrop::{Looks, Rgba, load_looks};
use camera::Cameras;
use halo::{Halo, HaloMesh};
use light::{LightData, load_light};
use lightdraw::LightMesh;
use river::RiverTile;
use water::{RIVER_ENV, RiverMode, RiverWater, river_mode_of};

#[cfg(test)]
mod testutil;

/// World hält den gemeinsamen Zustand der Drawer: Daten (Paletten, Kulissen, Licht), Kamera-Zustand je Spieler und
/// die wiederverwendeten Zeichen-Puffer. Der weiße Bildpunkt und die `DrawImageOptions` von Go entfallen: Flächen
/// gehen als `Canvas::rect` hinaus.
pub struct World {
    pub lib: Option<Rc<Library>>,
    pub looks: Looks,
    pub light: LightData,
    pub cams: Cameras,
    pub river: RiverTile,
    /// Fluss mit Shader (water.rs)
    pub water: RiverWater,
    /// Fassung ohne gewählte Einstellung (K3C_RIVER)
    pub start: RiverMode,
    pub mesh: LightMesh,
    /// Höfe der Zelle und ihr Netz (RU55.5), je Bild wiederverwendet.
    pub halos: Vec<Halo>,
    pub halo_mesh: HaloMesh,
    /// Weißes 1×1-Bild für die Lichtkarte (Go: `w.pixel()`), beim ersten Zeichnen angelegt.
    pub light_pixel: Option<k3c_platform::TexId>,
}

/// Parts sind die Beiträge der Welt für game: die Drawer in Anmeldereihenfolge (fern, mitte, nah, Boden, Fluss,
/// Licht), genau eine Kamera-Regel und genau eine Asset-Quelle (Go: dieselbe Instanz des fernen Drawers erfüllt alle
/// drei Rollen, in Rust ein Trait-Objekt je Rolle über denselben Zustand).
pub struct Parts {
    pub drawers: Vec<Box<dyn Drawer>>,
    pub camera_rule: Option<Box<dyn CameraRule>>,
    pub asset_source: Option<Box<dyn AssetSource>>,
}

/// New liefert die Drawer der Welt: fern (mit Himmel), mitte, nah, Boden, Fluss und Licht. lib (Grafik) darf `None`
/// sein, dann bleiben die Kulissen leer. Der ferne Drawer führt zugleich die Kamera (`CameraRule`) und meldet die
/// Kulissen-Grafik je Level (`AssetSource`).
pub fn new(lib: Option<Rc<Library>>) -> Parts {
    register_look();
    let mut w = World::new(lib);
    w.start = river_mode_of(&std::env::var(RIVER_ENV).unwrap_or_default());
    w.water.init(w.start);
    let w = Rc::new(RefCell::new(w));
    let part = |layer, part| -> Box<dyn Drawer> {
        Box::new(LayerDrawer {
            w: Rc::clone(&w),
            layer,
            part,
        })
    };
    Parts {
        drawers: vec![
            part(Layer::FAR, Part::Far),
            part(Layer::MID, Part::Mid),
            part(Layer::NEAR, Part::Near),
            part(Layer::GROUND, Part::Ground),
            part(Layer::RIVER, Part::River),
            part(Layer::RIVER, Part::Light),
        ],
        camera_rule: Some(Box::new(Far { w: Rc::clone(&w) })),
        asset_source: Some(Box::new(Far { w })),
    }
}

/// Meldet `view` den Welt-Anteil des Look-Profils (RU55.1; Quelle `data/look.json`, Profil `K3C_LOOK`). Die Welt ist in
/// jedem Aufbau (Spiel, Wiedergabe, Standbild) das erste Modul, das den Look braucht; mehrfach aufrufen ist harmlos.
pub fn register_look() {
    crate::view::cell::set_world_share_source(|| k3c_data::look::current().world_share);
}

/// Part wählt die Zeichen-Methode eines Drawers (Go: Methodenwert `w.drawMid` …).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Far,
    Mid,
    Near,
    Ground,
    River,
    Light,
}

/// LayerDrawer zeichnet eine Ebene über eine Methode der Welt.
pub struct LayerDrawer {
    pub w: Rc<RefCell<World>>,
    pub layer: Layer,
    pub part: Part,
}

impl Drawer for LayerDrawer {
    fn layer(&self) -> Layer {
        self.layer
    }

    fn draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let mut w = self.w.borrow_mut();
        match self.part {
            Part::Far => w.draw_far(dst, f, c),
            Part::Mid => w.draw_mid(dst, f, c),
            Part::Near => w.draw_near(dst, f, c),
            Part::Ground => w.draw_ground(dst, f, c),
            Part::River => w.draw_river(dst, f, c),
            Part::Light => w.draw_light(dst, f, c),
        }
    }
}

/// Far ist die ferne Ebene mit Himmel; sie trägt die Kamera-Regel und die Asset-Quelle der Welt. Als Drawer meldet
/// sie sich über `LayerDrawer` (Part::Far); diese Hülle liefert nur die beiden Rollen.
pub struct Far {
    pub w: Rc<RefCell<World>>,
}

impl CameraRule for Far {
    /// Follow ist die Kamera-Regel (Totzone, Grenzen der Stufe, Sprung beim Stufenwechsel; camera.rs).
    fn follow(&mut self, f: &Frame, c: &Cell, prev: Camera) -> Camera {
        self.w.borrow_mut().cams.follow(f, c, prev)
    }
}

impl AssetSource for Far {
    /// Assets nennt die Kulissen-Grafik des Levels (alle Varianten der drei Schichten).
    fn assets(&self, level: &str, _w: &SimWorld) -> Vec<String> {
        self.w.borrow().looks.assets(level).unwrap_or_default()
    }
}

impl World {
    /// Baut die Welt aus den Daten (Kulissen, Licht); die Fluss-Fassung setzt `new`.
    pub fn new(lib: Option<Rc<Library>>) -> World {
        World {
            lib,
            looks: load_looks(),
            light: load_light(),
            cams: Cameras::default(),
            river: RiverTile::default(),
            water: RiverWater::default(),
            start: RiverMode::Tiles,
            mesh: LightMesh::default(),
            halos: Vec::with_capacity(16),
            halo_mesh: HaloMesh::default(),
            light_pixel: None,
        }
    }

    /// fill füllt das Rechteck (Fenster-px) mit der Farbe clr, skaliert mit dem Faktor k (0 … 1, Helligkeit) und der
    /// Deckkraft a. Der Canvas nimmt gerade Farben (nicht vormultipliziert).
    #[allow(clippy::too_many_arguments, clippy::cast_possible_truncation)]
    pub fn fill(&self, dst: &mut dyn Canvas, x: f64, y: f64, wd: f64, h: f64, clr: Rgba, k: f64, a: f64) {
        if wd <= 0.0 || h <= 0.0 || a <= 0.0 {
            return;
        }
        let ch = |v: u8| (f64::from(v) / 255.0 * k) as f32;
        dst.rect(
            k3c_platform::Rect::new(x as f32, y as f32, wd as f32, h as f32),
            [ch(clr[0]), ch(clr[1]), ch(clr[2]), a as f32],
        );
    }

    /// fillRect füllt ein ganzzahliges Rechteck.
    #[allow(clippy::cast_precision_loss)]
    pub fn fill_rect(&self, dst: &mut dyn Canvas, r: Rectangle, clr: Rgba, k: f64) {
        self.fill(
            dst,
            r.min.x as f64,
            r.min.y as f64,
            r.dx() as f64,
            r.dy() as f64,
            clr,
            k,
            1.0,
        );
    }
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod mod_tests;
