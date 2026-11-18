//! Platzhalter für Dinge späterer Sessions (Leitfaden § 7); die liefernde Session löscht sie.
#![allow(dead_code)]

use k3c_platform::Canvas;

use super::Shared;
use super::keys::Note;
use crate::view::cell::Cell;
use crate::view::frame::Frame;
use crate::view::input::{DeviceID, Input};
use crate::view::screen::{Screen, Step};
use crate::view::session::Session;

/// RU9.4 (`main_screen.go`): Zahl der Karten des Hauptmenüs.
pub const MAIN_COUNT: i64 = 7;
/// RU12 (`pause.go`): Zahl der Zeilen des Pause-Menüs.
pub const PAUSE_ROWS: i64 = 5;

/// RU9.3 (`texts.go`): Schlüssel eines Textes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextKey(pub u16);

/// RU9.3: Texte, die `keys.rs` nennt.
pub const TXT_PAD: TextKey = TextKey(1);
pub const TXT_KEYBOARD: TextKey = TextKey(2);

/// RU9.3 (`texts.go`): Text `k` in der Sprache `lang`.
pub fn tr(_lang: &str, _k: TextKey) -> String {
    unimplemented!("RU9.3: tr")
}

/// RU9.3 (`texts.go`): Text `k` mit eingesetztem Platzhalter.
pub fn tr1(_lang: &str, _k: TextKey, _arg: &str) -> String {
    unimplemented!("RU9.3: tr1")
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
