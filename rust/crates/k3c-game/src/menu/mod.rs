//! Port von `native/k3c-game/menu/menu.go`: die Menü-Bildschirme des nativen Spiels (Sprint NG7, B-800; NG13):
//! Hauptmenü mit Version, Lobby für 1–2 Spieler, Spielstände, Optionen, Lizenzen, Pause und Tastenbelegung auf dem
//! Papier-Hintergrund; Spielende führt ins Hauptmenü. Online-Räume, Raum-Code, Werkzeug-Seiten und Dev-Optionen gibt es
//! nativ nicht.
//!
//! Aufbau: Jeder Bildschirm ist ein `view::Screen`; Untermenüs legt er über den Menü-Stapel von game auf
//! (`StepKind::Push`), zurück geht es mit `Pop`. Die Stapeltiefe ist höchstens zwei (Hauptmenü bzw. Pause und ein
//! Untermenü), Rückfragen liegen im Bildschirm selbst; Home (View + Menu, Pos1) ist darum im Untermenü dasselbe wie B.
//! `update` ändert den Zustand (reine Logik, Tabellen-Tests), `draw` baut daraus nur die Texte und zeichnet.
//!
//! Spielende: game schließt das Spiel `view::EndDelay` Takte nach `Frame::outcome` und legt das Hauptmenü auf; das
//! zeigt den Ausgang aus dem Frame. Plätze und Einstellungen liest menu aus der Session (`seats`, `settings`).
//!
//! Rust: Die Screens halten den gemeinsamen Zustand als [`Shared`] (`Rc<RefCell<Menus>>`): `Screen::draw` nimmt `&self`,
//! `Font::draw` aber `&mut self`, und der Painter cachet Bilder. Eine Borrow-Dauer reicht nie über einen Aufruf von
//! `update` oder einem anderen Screen.

pub mod art;
pub mod backdrop;
pub mod draw;
pub mod hero;
pub mod keys;
pub mod layout;
mod pending;
pub mod version;

#[cfg(test)]
mod art_tests;
#[cfg(test)]
mod keys_tests;
#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod paper_tests;
#[cfg(test)]
mod testutil;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use self::draw::Painter;
use self::keys::Note;
use crate::art::{Font, Library, WorldImages, load_font};
use crate::view::MAX_PLAYERS;
use crate::view::frame::Seat;
use crate::view::input::DeviceID;
use crate::view::keys::KeyTable;
use crate::view::screen::{MenuPages, Menus as MenusRole, Screen};
use crate::view::session::Session;

/// Menus hält, was die Bildschirme teilen: Zeichner, Tastenbelegung, Plätze des laufenden Spiels, Meldung fürs
/// Hauptmenü.
pub struct Menus {
    pub paint: Painter,
    /// Tastenbelegung (game baut sie aus input und hud); `None` = Seite ohne Zeilen
    pub keys: Option<KeyTable>,
    /// Plätze aus `Session::seats` beim letzten Update des Pause-Menüs
    pub seats: [Seat; MAX_PLAYERS],
    pub n_seats: usize,
    /// zeigt das nächste Hauptmenü (Fehler beim Verlassen)
    pub notice: Note,

    /// Bilder je Welt (`data/paper.json › weltbilder`)
    pub worlds: WorldImages,
    /// meldet, ob ein Weltbild ladbar ist (`Library::usable`; `None`: keins)
    pub usable: Option<Probe>,
    /// meldet, ob das Sheet eines Assets geladen ist (`None`: unbekannt, Bild gilt als da)
    pub has: Option<Probe>,
}

/// Eine Abfrage nach einem Asset (`usable`, `has`).
pub type Probe = Box<dyn Fn(&str) -> bool>;

/// Der gemeinsame Zustand der Screens.
pub type Shared = Rc<RefCell<Menus>>;

/// Baut den gemeinsamen Zustand: `lib` (Grafik) darf `None` sein, dann zeichnet die Pixel-Schrift; `keys` liefert die
/// Zeilen der Seite Tasten im Pause-Menü (B-807).
pub fn new_shared(lib: Option<Rc<RefCell<Library>>>, keys: Option<KeyTable>) -> Shared {
    let font: Option<Font> = match lib {
        Some(_) => None,
        None => load_font(Path::new(""), false).ok(),
    };
    let (mut usable, mut has): (Option<Probe>, Option<Probe>) = (None, None);
    if let Some(l) = &lib {
        let (l1, l2) = (Rc::clone(l), Rc::clone(l));
        usable = Some(Box::new(move |a| l1.borrow().usable(a)));
        has = Some(Box::new(move |a| {
            let l = l2.borrow();
            l.sheet(a, "idle").or_else(|| l.sheet(a, "stand")).is_some()
        }));
    }
    Rc::new(RefCell::new(Menus {
        paint: Painter::new(font, lib),
        keys,
        seats: [Seat::default(); MAX_PLAYERS],
        n_seats: 0,
        notice: Note::default(),
        worlds: WorldImages::from_data(),
        usable,
        has,
    }))
}

/// Liefert die Einstiege des Menüs (Go: `New`); `lib` (Grafik) darf `None` sein, dann zeichnet die Pixel-Schrift.
pub fn new(lib: Option<Rc<RefCell<Library>>>, keys: Option<KeyTable>) -> Box<dyn MenusRole> {
    Box::new(MenuSet {
        shared: new_shared(lib, keys),
    })
}

/// MenuSet erfüllt `view::Menus` und `view::MenuPages` (Go: `*menus`).
pub struct MenuSet {
    shared: Shared,
}

impl MenusRole for MenuSet {
    /// Das Hauptmenü; eine vorgemerkte Meldung (Fehler beim Verlassen) erscheint darin.
    fn main(&mut self) -> Box<dyn Screen> {
        let mut sc = pending::new_main(&self.shared);
        sc.note = std::mem::take(&mut self.shared.borrow_mut().notice);
        Box::new(sc)
    }

    /// Das Pause-Menü.
    fn pause(&mut self) -> Box<dyn Screen> {
        Box::new(pending::new_pause(&self.shared))
    }
}

impl MenuPages for MenuSet {
    /// Ein Untermenü für den Aufnahme-Modus: "saves", "options" (`in_game`: über der Pause), "lobby" (geführt von
    /// `by`), "keys" (Tastenbelegung) oder "licenses"; `None` für andere Namen.
    fn page(&mut self, name: &str, in_game: bool, by: DeviceID) -> Option<Box<dyn Screen>> {
        let m = &self.shared;
        Some(match name {
            "saves" => Box::new(pending::new_saves(m)),
            "options" => Box::new(pending::new_options(m, in_game)),
            "lobby" => Box::new(pending::new_lobby(m, by)),
            "keys" => Box::new(pending::new_keys(m)),
            "licenses" => Box::new(pending::new_licenses(m)),
            _ => return None,
        })
    }
}

impl Menus {
    /// Übernimmt die Plätze des laufenden Spiels aus der Session (ohne Listen zu halten).
    pub fn sync(&mut self, s: &dyn Session) {
        self.n_seats = 0;
        for st in s.seats() {
            if self.n_seats < MAX_PLAYERS {
                self.seats[self.n_seats] = st;
                self.n_seats += 1;
            }
        }
    }

    /// Liefert den Spieler des Geräts `d` aus den zuletzt übernommenen Plätzen (-1 ohne Platz).
    pub fn seat_of(&self, d: DeviceID) -> i64 {
        for st in &self.seats[..self.n_seats] {
            if st.device == d {
                return st.player;
            }
        }
        -1
    }
}
