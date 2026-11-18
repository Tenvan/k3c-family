//! Port von `native/k3c-game/menu/pause.go`: das Pause-Menü (Vorbild `src/scenes/pauseHub.ts`) auf dem Papier-Hintergrund:
//! Fortsetzen, Optionen, Tasten (Belegung, B-807), Spieler abmelden, Spiel verlassen. B und Menu setzen das Spiel fort
//! (game hebt die Pause auf, sobald der Stapel leer ist). „Spieler abmelden“ meldet den Spieler des bedienenden Geräts
//! ab und ist ausgegraut, solange nur einer spielt (der letzte Spieler bleibt, `Session::leave`). „Spiel verlassen“
//! speichert und beendet (`Session::close`) und führt ins Hauptmenü.

use std::cell::RefCell;

use k3c_platform::Canvas;

use super::Shared;
use super::art::PAUSE_ART;
use super::draw::{Page, Row};
use super::keys::{Base, Handler, Key, Note, device_name, move_sel, run};
use super::layout::{MAT_BLUE, MAT_CORAL, MAT_GOLD, MAT_GREEN, MAT_PURPLE, Material};
use super::main_screen::{MainScreen, by_hint, new_main};
use super::options::new_options;
use super::table::new_keys;
use super::texts::{TextKey, tr, tr1};
use crate::view::cell::Cell;
use crate::view::frame::{CueKind, Frame};
use crate::view::input::{DeviceID, Input};
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::session::Session;

/// Zeilen des Pause-Menüs von oben nach unten.
pub const PAUSE_RESUME: i64 = 0;
pub const PAUSE_OPTIONS: i64 = 1;
pub const PAUSE_KEYS: i64 = 2;
pub const PAUSE_REMOVE: i64 = 3;
pub const PAUSE_LEAVE: i64 = 4;
pub const PAUSE_ROWS: i64 = 5;

const PAUSE_LABELS: [TextKey; PAUSE_ROWS as usize] = [
    TextKey::Resume,
    TextKey::Options,
    TextKey::Keys,
    TextKey::Remove,
    TextKey::Leave,
];
const PAUSE_MATS: [Material; PAUSE_ROWS as usize] = [MAT_GOLD, MAT_PURPLE, MAT_GREEN, MAT_BLUE, MAT_CORAL];

/// PauseScreen ist das Pause-Menü über dem stehenden Spiel.
pub struct PauseScreen {
    pub base: Base,
    m: Shared,
    pub sel: i64,
    /// Welt des laufenden Spiels (kleines Bild auf „Fortsetzen“, B-938)
    pub world: String,
    pub pg: RefCell<Page>,
}

pub fn new_pause(m: &Shared) -> PauseScreen {
    PauseScreen {
        base: Base::default(),
        m: m.clone(),
        sel: 0,
        world: String::new(),
        pg: RefCell::new(Page::default()),
    }
}

impl PauseScreen {
    /// Meldet ausgegraute Einträge: Abmelden braucht zwei Spieler.
    pub fn off(&self, i: i64) -> bool {
        i == PAUSE_REMOVE && self.m.borrow().n_seats < 2
    }

    /// Löst den gewählten Eintrag aus.
    fn confirm(&mut self, d: DeviceID, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        let next: Box<dyn Screen> = match self.sel {
            PAUSE_RESUME => {
                self.base.cue(CueKind::Resume, -1, "");
                return (StepKind::Pop, None);
            }
            PAUSE_OPTIONS => Box::new(new_options(&self.m, true)),
            PAUSE_KEYS => Box::new(new_keys(&self.m)),
            PAUSE_REMOVE => return self.remove(d, s),
            _ => return (StepKind::Replace, Some(Box::new(self.leave(s)))),
        };
        self.base.cue(CueKind::Confirm, -1, "");
        (StepKind::Push, Some(next))
    }

    /// Meldet den Spieler des Geräts `d` ab und setzt das Spiel fort.
    fn remove(&mut self, d: DeviceID, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        let p = self.m.borrow().seat_of(d);
        if p < 0 {
            self.base.deny(TextKey::NoSeat, "");
            return (StepKind::Stay, None);
        }
        if let Err(e) = s.leave(p) {
            self.base.fail(&e);
            return (StepKind::Stay, None);
        }
        k3c_platform::log::info("👋 Spieler im Pause-Menü abgemeldet", &[("player", &p), ("device", &d)]);
        self.base.cue(CueKind::Resume, -1, "");
        (StepKind::Pop, None)
    }

    /// Speichert und beendet das Spiel und liefert das Hauptmenü, das die Pause ersetzt, mit schon gelesener
    /// Spielstand-Liste. Ein Fehler beim Speichern erscheint dort; das Spiel ist trotzdem beendet (`Session::close`).
    pub fn leave(&mut self, s: &mut dyn Session) -> MainScreen {
        if let Err(e) = s.close() {
            k3c_platform::log::warn("💥 Spiel beim Verlassen nicht sauber beendet", &[("err", &e)]);
            self.m.borrow_mut().notice = Note {
                raw: e.to_string(),
                bad: true,
                set: true,
                ..Note::default()
            };
        }
        k3c_platform::log::info("🛑 Spiel aus dem Pause-Menü verlassen", &[]);
        self.base.cue(CueKind::Confirm, -1, "");
        let mut next = new_main(&self.m);
        next.base.note = std::mem::take(&mut self.m.borrow_mut().notice);
        next.refresh(s); // das erste Bild zeigt schon den Spielstand (B-813)
        next
    }

    /// Baut die Texte des Pause-Menüs.
    pub fn fill(&self, lang: &str) {
        let mut p = self.pg.borrow_mut();
        p.reset(&tr(lang, TextKey::PauseTitle));
        for i in 0..PAUSE_ROWS {
            p.add(Row {
                label: tr(lang, PAUSE_LABELS[i as usize]),
                mat: PAUSE_MATS[i as usize],
                off: self.off(i),
                art: PAUSE_ART[i as usize].to_string(),
                ..Row::default()
            });
        }
        let m = self.m.borrow();
        if !self.world.is_empty() {
            let a = m.world_asset(&self.world, false);
            if !a.is_empty() {
                p.rows[PAUSE_RESUME as usize].art = a; // kleines Bild der Welt statt des Kriegers
            }
        }
        let pl = m.seat_of(self.base.by);
        if self.base.by_ok && pl >= 0 {
            p.rows[PAUSE_REMOVE as usize].value =
                tr1(lang, TextKey::Player, &(pl + 1).to_string()) + " · " + &device_name(lang, self.base.by);
        }
        p.sel = self.sel;
        p.note = self.base.note.text(lang);
        p.bad = self.base.note.bad;
        p.hint = by_hint(lang, TextKey::PauseHint, &self.base);
    }
}

impl Handler for PauseScreen {
    fn base(&mut self) -> &mut Base {
        &mut self.base
    }

    fn handle(&mut self, d: DeviceID, k: Key, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        match k {
            Key::Up | Key::Down => {
                let dir = if k == Key::Up { -1 } else { 1 };
                let off = |i: i64| self.off(i);
                let next = move_sel(self.sel, dir, PAUSE_ROWS, false, Some(&off));
                if next != self.sel {
                    self.sel = next;
                    self.base.cue(CueKind::Move, -1, "");
                }
                self.base.ver += 1;
            }
            Key::Home => {
                self.sel = PAUSE_RESUME;
                self.base.ver += 1;
            }
            Key::Back => {
                self.base.cue(CueKind::Resume, -1, "");
                return (StepKind::Pop, None);
            }
            Key::OK => return self.confirm(d, s),
            _ => {}
        }
        (StepKind::Stay, None)
    }
}

impl Screen for PauseScreen {
    /// Übernimmt die Plätze aus der Session (Abmelden) und bedient die Tasten.
    fn update(&mut self, input: &Input, s: &mut dyn Session) -> Step {
        let n = self.m.borrow().n_seats;
        self.m.borrow_mut().sync(&*s);
        if n != self.m.borrow().n_seats {
            self.base.ver += 1;
        }
        let w = s.world();
        if w != self.world {
            self.world = w;
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
