//! Platzhalter für Teile späterer Sessions (Leitfaden § 7); RU13.4 und RU13.5 löschen sie.

use std::collections::BTreeMap;

use k3c_data::{Fehler, Source};
use k3c_platform::Canvas;
use k3c_sim::state::World as SimWorld;

use super::Actors;
use crate::art::Actor;
use crate::view::cell::Cell;
use crate::view::frame::Frame;
use crate::view::layer::{Drawer, Layer, Listener};

/// RU13.4: catalog.go `unitPx` (Spiel-Pixel je Unit).
pub const UNIT_PX: f64 = 32.0;

/// RU13.5: marks.go `markAssets`.
pub const MARK_ASSETS: [&str; 0] = [];

/// RU13.4: catalog.go `Look`.
#[derive(Clone, Debug, Default)]
pub struct Look {
    pub sheet: bool,
}

/// RU13.4: catalog.go `Catalog`.
#[derive(Clone, Debug, Default)]
pub struct Catalog {
    pub troops: BTreeMap<String, Look>,
    pub enemies: BTreeMap<String, Look>,
    pub bosses: BTreeMap<String, Look>,
}

impl Catalog {
    /// RU13.4: `Catalog.Player`.
    pub fn player(&self, _i: i64) -> Look {
        unimplemented!("RU13.4: catalog_player")
    }

    /// RU13.4: `Catalog.SheetAssets`.
    pub fn sheet_assets(&self) -> Vec<String> {
        unimplemented!("RU13.4: catalog_sheet_assets")
    }

    /// RU13.4: levels.go `Catalog.LevelSheets`.
    pub fn level_sheets(&self, _w: &SimWorld) -> Vec<String> {
        unimplemented!("RU13.4: catalog_level_sheets")
    }
}

/// RU13.4: catalog.go `LoadCatalog`.
pub fn load_catalog(_src: &Source) -> Result<Catalog, Fehler> {
    unimplemented!("RU13.4: load_catalog")
}

/// RU13.4: figures.go `key` (stabile ID einer Figur).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key;

/// RU13.4: figures.go `warnKey`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct WarnKey;

/// RU13.4: figures.go `figure`.
#[derive(Clone, Debug, Default)]
pub struct Figure;

/// RU13.5: markdraw.go `ink`.
#[derive(Clone, Debug, Default)]
pub struct Ink;

impl Actors {
    /// RU13.4: figures.go `newActor`; liefert den Actor und ob die Rig-Datei fehlt.
    pub fn new_actor(&self, _l: &Look, _player: bool) -> (Option<Actor>, bool) {
        unimplemented!("RU13.4: actors_new_actor")
    }
}

impl Drawer for Actors {
    /// actors.go `Layer`; hier, damit der Drawer als Ganzes ersetzt werden kann.
    fn layer(&self) -> Layer {
        Layer::ACTORS
    }

    /// RU13.4: draw.go `Draw`.
    fn draw(&self, _dst: &mut dyn Canvas, _f: &Frame, _c: &Cell) {
        unimplemented!("RU13.4: actors_draw")
    }
}

impl Listener for Actors {
    /// RU13.4: figures.go `Listen`.
    fn listen(&mut self, _f: &Frame) {
        unimplemented!("RU13.4: actors_listen")
    }
}
