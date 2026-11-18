//! Platzhalter für Dinge späterer Sessions (Leitfaden § 7); die liefernde Session löscht sie.
#![allow(dead_code)]
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf RU12.2 und RU12.3

use k3c_platform::Canvas;

use super::Shared;
use super::keys::Note;
use super::texts::TextKey;
use crate::view::cell::Cell;
use crate::view::frame::Frame;
use crate::view::input::Input;
use crate::view::screen::{Screen, Step};
use crate::view::session::Session;

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

/// Stellvertreter der Screens aus RU12.2 und RU12.3.
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

pub fn new_testing(_m: &Shared) -> PendingScreen {
    unimplemented!("RU12: new_testing")
}
