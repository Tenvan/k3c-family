//! Port von `native/k3c-game/menu/controls.go`: Optionen › Steuerung (B-893): eine Zeile je Spieler mit seinem
//! Eingabe-Profil (Tastatur WASD, Tastatur Pfeile, Gamepad). Links/Rechts bzw. A blättert; Tastatur-Profile überspringt
//! das Blättern, solange ein anderer Spieler eins hat (`Settings::next_profile`). Jede Änderung geht sofort über
//! `Session::set_settings`; game stellt die Tastatur im nächsten Update um. Grundlage jeder Änderung ist der aktuelle
//! Stand der Session.

use std::cell::RefCell;

use k3c_platform::Canvas;

use super::Shared;
use super::draw::{Page, Row};
use super::keys::{Base, Handler, Key, Note, move_sel, run};
use super::layout::{MAT_CORAL, MAT_PURPLE};
use super::main_screen::by_hint;
use super::texts::{TextKey, tr, tr1};
use crate::view::MAX_PLAYERS;
use crate::view::cell::Cell;
use crate::view::frame::{CueKind, Frame};
use crate::view::input::{DeviceID, Input};
use crate::view::profile::{PROFILE_ARROWS, PROFILE_WASD};
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::session::Session;
use crate::view::settings::Settings;

/// Die Zeile Zurück unter den Spielern.
pub const CTL_BACK: usize = MAX_PLAYERS;

/// ControlsScreen bearbeitet die Eingabe-Profile der Spieler.
pub struct ControlsScreen {
    pub base: Base,
    m: Shared,
    pub cur: Settings,
    loaded: bool,
    pub sel: usize,
    dim: bool,
    pub pg: RefCell<Page>,
}

pub fn new_controls(m: &Shared, dim: bool) -> ControlsScreen {
    ControlsScreen {
        base: Base::default(),
        m: m.clone(),
        cur: Settings::default(),
        loaded: false,
        sel: 0,
        dim,
        pg: RefCell::new(Page::default()),
    }
}

impl ControlsScreen {
    /// Übernimmt die neuen Profile über die Session; sie gelten auch, wenn das Speichern scheitert.
    fn apply(&mut self, s: &mut dyn Session, next: &Settings) {
        let next = next.normalized();
        if next == self.cur {
            return;
        }
        self.cur = next.clone();
        self.base.note = Note::default();
        self.base.cue(CueKind::Move, -1, "");
        self.base.ver += 1;
        k3c_platform::log::info(
            "🚦 Eingabe-Profil gewechselt",
            &[("player", &self.sel), ("profile", &next.controls[self.sel])],
        );
        if let Err(e) = s.set_settings(next) {
            k3c_platform::log::warn("💥 Einstellungen nicht gespeichert", &[("err", &e)]);
            self.base.note = Note {
                raw: e.to_string(),
                bad: true,
                set: true,
                ..Note::default()
            };
        }
    }

    /// Baut die Zeilen „Spieler 1  < Tastatur WASD >“ … und Zurück in der Sprache `lang`.
    pub fn fill(&self, lang: &str) {
        let mut p = self.pg.borrow_mut();
        p.reset(&tr(lang, TextKey::Controls));
        for i in 0..MAX_PLAYERS {
            p.add(Row {
                label: tr1(lang, TextKey::Player, &(i + 1).to_string()),
                value: format!("< {} >", profile_text(lang, &self.cur.controls[i])),
                mat: MAT_PURPLE,
                ..Row::default()
            });
        }
        p.add(Row {
            label: tr(lang, TextKey::Back),
            mat: MAT_CORAL,
            ..Row::default()
        });
        p.sel = self.sel as i64;
        p.dim = self.dim;
        p.note = self.base.note.text(lang);
        p.bad = self.base.note.bad;
        p.hint = by_hint(lang, TextKey::ControlsHint, &self.base);
    }
}

impl Handler for ControlsScreen {
    fn base(&mut self) -> &mut Base {
        &mut self.base
    }

    fn handle(&mut self, _d: DeviceID, k: Key, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        match k {
            Key::Up | Key::Down => {
                let dir = if k == Key::Up { -1 } else { 1 };
                self.sel = move_sel(self.sel as i64, dir, CTL_BACK as i64 + 1, true, None) as usize;
                self.base.cue(CueKind::Move, -1, "");
                self.base.ver += 1;
            }
            Key::Left | Key::Right | Key::OK => {
                if self.sel == CTL_BACK {
                    if k != Key::OK {
                        return (StepKind::Stay, None);
                    }
                    self.base.cue(CueKind::Back, -1, "");
                    return (StepKind::Pop, None);
                }
                let dir = if k == Key::Left { -1 } else { 1 };
                self.cur = s.settings().normalized();
                let mut next = self.cur.clone();
                next.controls[self.sel] = self.cur.next_profile(self.sel as i64, dir).to_string();
                self.apply(s, &next);
            }
            Key::Back | Key::Home => {
                self.base.cue(CueKind::Back, -1, "");
                return (StepKind::Pop, None);
            }
            _ => {}
        }
        (StepKind::Stay, None)
    }
}

impl Screen for ControlsScreen {
    fn update(&mut self, input: &Input, s: &mut dyn Session) -> Step {
        if !self.loaded {
            self.cur = s.settings().normalized();
            self.loaded = true;
            self.base.ver += 1;
        }
        run(self, input, s)
    }

    fn draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let lang = &f.settings.language;
        if self.pg.borrow_mut().stale(self.base.ver, lang) {
            self.fill(lang);
        }
        self.m.borrow_mut().paint.page(dst, c, &self.pg.borrow());
    }
}

/// Der Name des Eingabe-Profils `p`.
pub fn profile_text(lang: &str, p: &str) -> String {
    match p {
        PROFILE_WASD => tr(lang, TextKey::KeysWASD),
        PROFILE_ARROWS => tr(lang, TextKey::KeysArrows),
        _ => tr(lang, TextKey::Gamepad),
    }
}
