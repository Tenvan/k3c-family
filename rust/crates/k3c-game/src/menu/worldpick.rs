//! Port von `native/k3c-game/menu/worldpick.go`: die Weltauswahl (B-938), erster Schritt von „Neues Spiel“. Links/Rechts
//! blättert durch die Welten der Session (`Session::world_list`), A wählt sie und öffnet die Lobby mit dem bestätigenden
//! Gerät als Spieler 1, B geht zurück ins Hauptmenü. Die Seite zeigt das große Weltbild mit dem Namen darüber (der Name
//! ist Schrift, nicht Teil des Bildes).

use std::cell::RefCell;

use k3c_platform::Canvas;

use super::Shared;
use super::draw::{Hero, Page};
use super::keys::{Base, Handler, Key, Note, cycle, run};
use super::lobby::{LobbyScreen, new_lobby};
use super::main_screen::by_hint;
use super::texts::{TextKey, tr};
use crate::view::cell::Cell;
use crate::view::frame::{CueKind, Frame};
use crate::view::input::{DeviceID, Input};
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::session::{Session, WorldInfo};

/// WorldPickScreen ist die Weltauswahl.
pub struct WorldPickScreen {
    pub base: Base,
    m: Shared,
    pub list: Vec<WorldInfo>,
    pub listed: bool,
    pub sel: i64,
    pub pg: RefCell<Page>,
}

/// Öffnet die Weltauswahl; das Gerät `_` wählt erst beim Bestätigen (Go: `newWorldPick(m, _)`).
pub fn new_world_pick(m: &Shared, _d: DeviceID) -> WorldPickScreen {
    WorldPickScreen {
        base: Base::default(),
        m: m.clone(),
        list: Vec::new(),
        listed: false,
        sel: 0,
        pg: RefCell::new(Page::default()),
    }
}

impl WorldPickScreen {
    /// Die Lobby der gewählten Welt mit dem Gerät `d` als Spieler 1 (B in der Lobby führt zurück zur Weltauswahl).
    pub fn lobby_of(&self, d: DeviceID) -> LobbyScreen {
        let mut lb = new_lobby(&self.m, d);
        lb.world = self.list[self.sel as usize].id.clone();
        lb
    }

    /// Baut die Seite: Titel, Weltbild mit Name, Hinweis.
    pub fn fill(&self, lang: &str) {
        let mut p = self.pg.borrow_mut();
        p.reset(&tr(lang, TextKey::LobbyTitle));
        p.sub = tr(lang, TextKey::World);
        if let Some(w) = self.list.get(self.sel as usize) {
            let name = if w.name.is_empty() { &w.id } else { &w.name };
            p.hero = Hero {
                on: true,
                name: name.clone(),
                art: self.m.borrow().world_asset(&w.id, true),
                left: self.list.len() > 1,
                right: self.list.len() > 1,
                dev: w.repo,
            };
        }
        p.note = self.base.note.text(lang);
        p.bad = self.base.note.bad;
        p.hint = by_hint(lang, TextKey::WorldHint, &self.base);
    }
}

impl Handler for WorldPickScreen {
    fn base(&mut self) -> &mut Base {
        &mut self.base
    }

    fn handle(&mut self, d: DeviceID, k: Key, _s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        match k {
            Key::Left | Key::Right => {
                let dir = if k == Key::Left { -1 } else { 1 };
                let next = cycle(self.sel, dir, self.list.len() as i64);
                if next != self.sel {
                    self.sel = next;
                    self.base.cue(CueKind::Move, -1, "");
                    self.base.ver += 1;
                }
            }
            Key::OK => {
                if let Some(w) = self.list.get(self.sel as usize).filter(|w| !w.error.is_empty()) {
                    // RU36.6: nicht ladbare Welt (Dev-Mode) zeigt ihren Grund statt der Lobby
                    let text = format!("{}: {}", w.id, w.error);
                    k3c_platform::log::info("🚫 Welt nicht ladbar", &[("welt", &w.id), ("grund", &w.error)]);
                    self.base.note = Note {
                        raw: text.clone(),
                        bad: true,
                        set: true,
                        ..Note::default()
                    };
                    self.base.cue(CueKind::Denied, -1, &text);
                    self.base.ver += 1;
                    return (StepKind::Stay, None);
                }
                let lb = self.lobby_of(d);
                k3c_platform::log::info("🚦 Welt gewählt", &[("welt", &lb.world), ("device", &d)]);
                self.base.cue(CueKind::Confirm, -1, "");
                return (StepKind::Push, Some(Box::new(lb)));
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

impl Screen for WorldPickScreen {
    /// Liest die Weltliste beim ersten Mal und bedient die Tasten.
    fn update(&mut self, input: &Input, s: &mut dyn Session) -> Step {
        if !self.listed {
            self.listed = true;
            self.list = s.world_list();
            if self.list.is_empty() {
                self.list = vec![WorldInfo {
                    id: "standard".to_string(),
                    name: "Standard".to_string(),
                    ..WorldInfo::default()
                }];
            }
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
