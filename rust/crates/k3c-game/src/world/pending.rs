//! Platzhalter für Teile späterer Sessions (Leitfaden § 7); die liefernde Session löscht sie.

use std::rc::Rc;

use k3c_platform::Canvas;

use super::backdrop::{Looks, Rgba};
use super::camera::Cameras;
use super::light::LightData;
use super::lightdraw::LightMesh;
use super::river::RiverTile;
use crate::art::Library;
use crate::view::cell::Rectangle;

/// RU13.3: `World` (world.go); hier nur die Felder, die backdrop, camera, ground und layers lesen.
pub struct World {
    pub lib: Option<Rc<Library>>,
    pub looks: Looks,
    pub light: LightData,
    pub cams: Cameras,
    /// RU13.3: `World.mesh` (Puffer der Lichtkarte, lightdraw.go) und `World.river` (Wellen-Kachel, river.go)
    pub mesh: LightMesh,
    pub river: RiverTile,
}

impl World {
    /// RU13.3: `World.fill` (Fläche aus dem weißen Bildpunkt).
    #[allow(clippy::too_many_arguments)]
    pub fn fill(&self, _dst: &mut dyn Canvas, _x: f64, _y: f64, _w: f64, _h: f64, _clr: Rgba, _k: f64, _a: f64) {
        unimplemented!("RU13.3: world_fill")
    }

    /// RU13.3: `World.fillRect`.
    pub fn fill_rect(&self, _dst: &mut dyn Canvas, _r: Rectangle, _clr: Rgba, _k: f64) {
        unimplemented!("RU13.3: world_fill_rect")
    }
}
