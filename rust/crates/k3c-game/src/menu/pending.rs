//! Platzhalter für Dinge späterer Sessions (Leitfaden § 7); die liefernde Session löscht sie.
#![allow(dead_code)]

use k3c_platform::Canvas;

use super::Shared;
use super::draw::{Hero, Painter};
use super::layout::{Geom, Rgba, rgb};
use crate::view::cell::Cell;
use crate::view::frame::Frame;
use crate::view::input::{DeviceID, Input};
use crate::view::screen::{Screen, Step};
use crate::view::session::Session;

/// RU9.2 (`backdrop.go`): Farbe des Schattens.
pub const COL_SHADOW: Rgba = rgb(0x001a_0f14);
/// RU9.4 (`main_screen.go`): Zahl der Karten des Hauptmenüs.
pub const MAIN_COUNT: i64 = 7;
/// RU12 (`pause.go`): Zahl der Zeilen des Pause-Menüs.
pub const PAUSE_ROWS: i64 = 5;

/// RU9.3 (`texts.go`): Schlüssel eines Textes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextKey(pub u16);

/// RU9.5 (`keys.go`): Meldung fürs nächste Hauptmenü.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Note {
    pub key: TextKey,
    pub arg: String,
    /// gesetzt: Text der Session, ersetzt `key`
    pub raw: String,
    /// Fehler (rot) statt Hinweis
    pub bad: bool,
    pub set: bool,
}

/// Stellvertreter der Screens aus RU9.2 bis RU9.5 und RU12.
#[derive(Default)]
pub struct PendingScreen {
    pub note: Note,
}

impl Screen for PendingScreen {
    fn update(&mut self, _input: &Input, _s: &mut dyn Session) -> Step {
        unimplemented!("RU9: Screen")
    }
    fn draw(&self, _dst: &mut dyn Canvas, _f: &Frame, _c: &Cell) {
        unimplemented!("RU9: Screen")
    }
}

pub fn new_main(_m: &Shared) -> PendingScreen {
    unimplemented!("RU9.4: new_main")
}
pub fn new_pause(_m: &Shared) -> PendingScreen {
    unimplemented!("RU12: new_pause")
}
pub fn new_saves(_m: &Shared) -> PendingScreen {
    unimplemented!("RU9.4: new_saves")
}
pub fn new_options(_m: &Shared, _dim: bool) -> PendingScreen {
    unimplemented!("RU9.5: new_options")
}
pub fn new_lobby(_m: &Shared, _d: DeviceID) -> PendingScreen {
    unimplemented!("RU9.4: new_lobby")
}
pub fn new_keys(_m: &Shared) -> PendingScreen {
    unimplemented!("RU9.5: new_keys")
}
pub fn new_licenses(_m: &Shared) -> PendingScreen {
    unimplemented!("RU12: new_licenses")
}

impl Painter {
    /// RU9.2 (`backdrop.go`): Papier-Hintergrund.
    pub fn backdrop(&mut self, _dst: &mut dyn Canvas, _g: &Geom) {
        unimplemented!("RU9.2: backdrop")
    }
    /// RU9.2 (`hero.go`): großes Weltbild der Weltauswahl.
    pub fn hero(&mut self, _dst: &mut dyn Canvas, _g: &Geom, _h: &Hero) {
        unimplemented!("RU9.2: hero")
    }
    /// RU9.2 (`art.go`): Illustration der Karte; `false` = nur Text.
    pub fn art_box(&mut self, _dst: &mut dyn Canvas, _asset: &str, _x: f64, _y: f64, _w: f64, _h: f64) -> bool {
        unimplemented!("RU9.2: art_box")
    }
    /// RU9.2 (`art.go`): ist das Sheet des Assets geladen?
    pub fn has_sheet(&self, _asset: &str) -> bool {
        unimplemented!("RU9.2: has_sheet")
    }
}
