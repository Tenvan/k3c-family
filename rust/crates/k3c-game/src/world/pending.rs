//! Platzhalter für Teile späterer Sessions (Leitfaden § 7); die liefernde Session löscht sie.

use std::rc::Rc;

use k3c_platform::Canvas;
use k3c_sim::state::CycleInfo;

use super::backdrop::{Looks, Rgba};
use super::camera::Cameras;
use crate::art::Library;
use crate::view::cell::Rectangle;

/// RU13.2: `lightData` (light.go).
#[derive(Clone, Debug, Default)]
pub struct LightData;

impl LightData {
    /// Helligkeit der Kulisse (Go: `lightData.backdrop`).
    pub fn backdrop(&self, _c: &CycleInfo) -> f64 {
        unimplemented!("RU13.2: light_data_backdrop")
    }
}

/// RU13.3: `World` (world.go); hier nur die Felder, die backdrop, camera, ground und layers lesen.
pub struct World {
    pub lib: Option<Rc<Library>>,
    pub looks: Looks,
    pub light: LightData,
    pub cams: Cameras,
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
