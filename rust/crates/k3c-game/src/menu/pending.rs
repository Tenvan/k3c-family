//! Platzhalter für Dinge späterer Sessions (Leitfaden § 7); die liefernde Session löscht sie.
#![allow(dead_code)]

use k3c_platform::Canvas;

use super::Shared;
use super::keys::Note;
use super::texts::TextKey;
use crate::view::cell::Cell;
use crate::view::frame::Frame;
use crate::view::input::{DeviceID, Input};
use crate::view::screen::{Screen, Step};
use crate::view::session::Session;

/// RU12 (`pause.go`): Zahl der Zeilen des Pause-Menüs.
pub const PAUSE_ROWS: i64 = 5;

/// RU12.3 (`licenses.go`): ein Werk mit Lizenzhinweis.
#[derive(Clone, Copy, Debug)]
pub struct License {
    pub part: TextKey,
    pub work: &'static str,
    pub holder: &'static str,
    pub license: &'static str,
    pub place: &'static str,
}

/// RU12.3 (`licenses.go`): die Liste der Werke.
pub fn licenses() -> &'static [License] {
    unimplemented!("RU12: licenses")
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

pub fn new_pause(_m: &Shared) -> PendingScreen {
    unimplemented!("RU12: new_pause")
}
pub fn new_saves(_m: &Shared) -> PendingScreen {
    unimplemented!("RU12: new_saves")
}
pub fn new_options(_m: &Shared, _dim: bool) -> PendingScreen {
    unimplemented!("RU9.5: new_options")
}
pub fn new_world_pick(_m: &Shared, _d: DeviceID) -> PendingScreen {
    unimplemented!("RU12: new_world_pick")
}
pub fn new_testing(_m: &Shared) -> PendingScreen {
    unimplemented!("RU12: new_testing")
}
