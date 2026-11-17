//! Port von `native/k3c-game/view/screen.go`: Menü-Bildschirme und ihre Einstiege.

use k3c_platform::Canvas;

use super::cell::Cell;
use super::frame::{Cue, Frame};
use super::input::{DeviceID, Input};
use super::session::Session;

/// StepKind sagt game, was nach `Screen::update` geschieht.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepKind {
    /// Screen bleibt oben
    #[default]
    Stay,
    /// `Step::next` oben auflegen (Untermenü)
    Push,
    /// obersten Screen durch `Step::next` ersetzen
    Replace,
    /// obersten Screen schließen (zurück); leerer Stapel mit aktivem Spiel = weiterspielen
    Pop,
    /// alle Screens schließen, Spiel läuft (nach Session.New, Continue oder Load)
    Play,
    /// Programm beenden (game ruft vorher Session.Close)
    Quit,
}

/// Step ist das Ergebnis eines `Screen::update`.
#[derive(Default)]
pub struct Step {
    pub kind: StepKind,
    /// für Push und Replace
    pub next: Option<Box<dyn Screen>>,
    /// z. B. Move, Confirm; game legt sie in den nächsten Frame
    pub cues: Vec<Cue>,
}

/// Screen ist ein Menü-Bildschirm (menu). Er bekommt die Eingabe aller Geräte, solange er oben liegt, und bedient die
/// Session selbst (Neues Spiel, Laden, Löschen …). Gezeichnet wird er nach `Layer::OVERLAY` über das ganze Fenster; im
/// Pause-Menü liegt darunter das stehende Spiel des Frames.
pub trait Screen {
    fn update(&mut self, input: &Input, s: &mut dyn Session) -> Step;
    /// `c` = Fenster-Zelle
    fn draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell);
}

/// Menus liefert die Einstiege des Menüs: Main beim Start und nach Spielende, Pause bei Menu im Spiel. game ruft
/// `Session::pause(true)` beim Auflegen des Pause-Screens und `pause(false)`, sobald der Stapel leer ist (Pop, Play).
pub trait Menus {
    fn main(&mut self) -> Box<dyn Screen>;
    fn pause(&mut self) -> Box<dyn Screen>;
}

/// MenuPages ist eine optionale Rolle von Menus für den Aufnahme-Modus (--menu): `page` liefert das Untermenü `name`
/// ("saves", "options", "lobby", "keys", "licenses"; sonst `None`), `in_game` = über dem Pause-Menü, `by` = bedienendes
/// Gerät. game legt es über Main bzw. Pause auf; im Spiel führt der Weg dorthin über die Menü-Einträge.
pub trait MenuPages {
    fn page(&mut self, name: &str, in_game: bool, by: DeviceID) -> Option<Box<dyn Screen>>;
}
