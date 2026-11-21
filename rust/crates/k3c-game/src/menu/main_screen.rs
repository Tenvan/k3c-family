//! Port von `native/k3c-game/menu/main_screen.go`: das Hauptmenü. „Weiter“ ist ausgegraut, solange es keinen ladbaren
//! Spielstand gibt; die Liste liest es beim ersten Update und nach jeder Rückkehr aus einem Untermenü (Laden kann
//! löschen). Bis dahin zeigt „Weiter“ „lädt ...“, nie „Kein Spielstand vorhanden“ (Draw vor dem ersten Update, B-813).
//! Unter dem Titel steht die Version.
//!
//! Abweichung: `Screen::draw` nimmt `&self`; Seite und Ausgang stehen darum in `RefCell` und `Cell`.

use std::cell::{Cell as StdCell, RefCell};

use k3c_platform::Canvas;

use super::Shared;
use super::art::MAIN_ART;
use super::draw::{Page, Row};
use super::keys::{Base, Handler, Key, Note, device_name, first, move_sel, run};
use super::layout::{MAT_BLUE, MAT_CORAL, MAT_GOLD, MAT_GREEN, MAT_PURPLE, MAT_RED, Material};
use super::options::new_options;
use super::saves::new_saves;
use super::table::new_licenses;
use super::testing::new_testing;
use super::texts::{TextKey, tr, tr1};
use super::version::version_text;
use super::worldpick::new_world_pick;
use crate::view::cell::Cell;
use crate::view::frame::{CueKind, Frame, Outcome};
use crate::view::input::{DeviceID, Input};
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::session::{SaveInfo, Session};

/// Einträge des Hauptmenüs von oben nach unten (B-800: Weiter, Neues Spiel, Laden, Optionen, Beenden; B-808: Lizenzen).
pub const MAIN_CONTINUE: i64 = 0;
pub const MAIN_NEW: i64 = 1;
pub const MAIN_LOAD: i64 = 2;
pub const MAIN_OPTIONS: i64 = 3;
pub const MAIN_TESTING: i64 = 4;
pub const MAIN_LICENSES: i64 = 5;
pub const MAIN_QUIT: i64 = 6;
pub const MAIN_COUNT: i64 = 7;

const MAIN_LABELS: [TextKey; 7] = [
    TextKey::Continue,
    TextKey::New,
    TextKey::Load,
    TextKey::Options,
    TextKey::Testing,
    TextKey::Licenses,
    TextKey::Quit,
];
const MAIN_SUBS: [TextKey; 7] = [
    TextKey::ContinueSub,
    TextKey::NewSub,
    TextKey::LoadSub,
    TextKey::OptionsSub,
    TextKey::TestingSub,
    TextKey::LicensesSub,
    TextKey::QuitSub,
];
const MAIN_MATS: [Material; 7] = [MAT_GOLD, MAT_GREEN, MAT_BLUE, MAT_PURPLE, MAT_BLUE, MAT_RED, MAT_CORAL];

/// MainScreen ist das Hauptmenü.
pub struct MainScreen {
    pub base: Base,
    m: Shared,
    pub sel: i64,
    pub listed: bool,
    /// neuester ladbarer Spielstand (Name leer: keiner)
    pub latest: SaveInfo,
    /// Ausgang des letzten Spiels, für den die Texte gelten (`Frame::outcome`)
    pub ended: StdCell<Outcome>,
    /// Karte Testing sichtbar (Entwickler-Build oder `Settings::dev_mode`, B-896)
    pub dev: bool,
    pub pg: RefCell<Page>,
}

pub fn new_main(m: &Shared) -> MainScreen {
    MainScreen {
        base: Base::default(),
        m: m.clone(),
        sel: 0,
        listed: false,
        latest: SaveInfo::default(),
        ended: StdCell::new(Outcome::None),
        dev: dev_build(),
        pg: RefCell::new(Page::default()),
    }
}

/// Meldet den Entwickler-Build: ohne Versionsangabe (`cargo run`, Tests).
pub fn dev_build() -> bool {
    super::version::is_unset()
}

impl MainScreen {
    /// Sucht den neuesten ladbaren Spielstand und wählt „Weiter“, wenn es ihn gibt, sonst „Neues Spiel“.
    pub fn refresh(&mut self, s: &mut dyn Session) {
        self.listed = true;
        self.latest = SaveInfo::default();
        match s.list() {
            Err(e) => {
                self.base.note = Note {
                    raw: e.to_string(),
                    bad: true,
                    set: true,
                    ..Note::default()
                };
            }
            Ok(list) => {
                if let Some(e) = list.into_iter().find(|e| e.loadable) {
                    self.latest = e;
                }
            }
        }
        if self.sel == MAIN_CONTINUE && self.off(MAIN_CONTINUE) {
            self.sel = MAIN_NEW;
        }
        self.base.ver += 1;
    }

    /// Meldet ausgegraute bzw. verborgene Einträge (Weiter ohne Spielstand, vor dem Lesen der Liste noch nicht; Testing
    /// ohne Entwickler-Modus). Die Navigation überspringt sie.
    pub fn off(&self, i: i64) -> bool {
        (i == MAIN_CONTINUE && self.listed && self.latest.name.is_empty()) || (i == MAIN_TESTING && !self.dev)
    }

    /// Löst den gewählten Eintrag aus. Untermenüs kommen über `Push`; danach ist die Spielstand-Liste neu zu lesen.
    fn confirm(&mut self, d: DeviceID, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        self.base.note = Note::default();
        let next: Box<dyn Screen> = match self.sel {
            MAIN_CONTINUE => {
                if let Err(e) = s.continue_game(&[d]) {
                    self.base.fail(&e);
                    self.listed = false;
                    return (StepKind::Stay, None);
                }
                k3c_platform::log::info("📂 Weiter mit dem neuesten Spielstand", &[("device", &d)]);
                self.base.cue(CueKind::Confirm, -1, "");
                return (StepKind::Play, None);
            }
            MAIN_NEW => Box::new(new_world_pick(&self.m, d)),
            MAIN_LOAD => {
                self.listed = false;
                Box::new(new_saves(&self.m))
            }
            MAIN_OPTIONS => Box::new(new_options(&self.m, false)),
            MAIN_TESTING => Box::new(new_testing(&self.m)),
            MAIN_LICENSES => Box::new(new_licenses(&self.m)),
            _ => {
                k3c_platform::log::info("🛑 Beenden aus dem Hauptmenü", &[]);
                self.base.cue(CueKind::Confirm, -1, "");
                return (StepKind::Quit, None);
            }
        };
        self.base.cue(CueKind::Confirm, -1, "");
        (StepKind::Push, Some(next))
    }

    /// Baut die Texte der Seite in der Sprache `lang`.
    pub fn fill(&self, lang: &str) {
        let mut p = self.pg.borrow_mut();
        p.reset(&tr(lang, TextKey::Title));
        p.sub = version_text(lang);
        for i in 0..MAIN_COUNT {
            if i == MAIN_TESTING && !self.dev {
                continue; // nur für Entwickler sichtbar
            }
            let u = i as usize;
            let mut r = Row {
                label: tr(lang, MAIN_LABELS[u]),
                value: tr(lang, MAIN_SUBS[u]),
                mat: MAIN_MATS[u],
                off: self.off(i),
                art: MAIN_ART[u].to_string(),
                ..Row::default()
            };
            if i == MAIN_CONTINUE {
                r.value = self.continue_value(lang);
            }
            p.add(r);
        }
        p.sel = self.sel;
        if !self.dev && self.sel > MAIN_TESTING {
            p.sel -= 1; // verborgene Karte Testing
        }
        let n = if self.base.note.set {
            self.base.note.clone()
        } else {
            outcome_note(self.ended.get())
        };
        p.note = n.text(lang);
        p.bad = n.bad;
        p.hint = by_hint(lang, TextKey::MainHint, &self.base);
    }

    /// Der Wert der Karte „Weiter“: Spielstand, „lädt“ oder „kein Spielstand“.
    fn continue_value(&self, lang: &str) -> String {
        if !self.latest.name.is_empty() {
            return tr1(lang, TextKey::ContinueSub, &self.latest.name)
                + " · "
                + &tr1(lang, TextKey::SavesLine, &self.latest.day.to_string());
        }
        if !self.listed {
            return tr(lang, TextKey::ContinueLoading);
        }
        tr(lang, TextKey::ContinueNone)
    }
}

impl Handler for MainScreen {
    fn base(&mut self) -> &mut Base {
        &mut self.base
    }

    fn handle(&mut self, d: DeviceID, k: Key, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        match k {
            Key::Up | Key::Down => {
                let dir = if k == Key::Up { -1 } else { 1 };
                let off = |i: i64| self.off(i);
                let next = move_sel(self.sel, dir, MAIN_COUNT, false, Some(&off));
                if next != self.sel {
                    self.sel = next;
                    self.base.cue(CueKind::Move, -1, "");
                }
                self.base.ver += 1;
            }
            Key::Home => {
                let off = |i: i64| self.off(i);
                self.sel = first(MAIN_COUNT, Some(&off));
                self.base.ver += 1;
            }
            Key::OK => return self.confirm(d, s),
            _ => {}
        }
        (StepKind::Stay, None)
    }
}

impl Screen for MainScreen {
    /// Liest bei Bedarf die Spielstände und bedient die Tasten.
    fn update(&mut self, input: &Input, s: &mut dyn Session) -> Step {
        if !self.listed {
            self.refresh(s);
        }
        let dev = dev_build() || s.settings().dev_mode;
        if dev != self.dev {
            self.dev = dev;
            self.base.ver += 1;
        }
        run(self, input, s)
    }

    /// Zeichnet das Hauptmenü; die Texte entstehen nur bei neuer Version oder Sprache.
    fn draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let lang = &f.settings.language;
        let stale = self.pg.borrow_mut().stale(self.base.ver, lang) || self.ended.get() != f.outcome;
        if stale {
            self.ended.set(f.outcome);
            self.fill(lang);
        }
        self.m.borrow_mut().paint.page(dst, c, &self.pg.borrow());
    }
}

/// Die Meldung zum Ausgang des letzten Spiels (leer, solange keiner feststeht).
pub fn outcome_note(o: Outcome) -> Note {
    match o {
        Outcome::Lost => Note {
            key: TextKey::Lost,
            set: true,
            bad: true,
            ..Note::default()
        },
        Outcome::Won => Note {
            key: TextKey::Won,
            set: true,
            ..Note::default()
        },
        Outcome::None => Note::default(),
    }
}

/// Die Hinweiszeile mit dem bedienenden Gerät (bei zwei Spielern sichtbar, wer das Menü führt).
pub fn by_hint(lang: &str, k: TextKey, b: &Base) -> String {
    let mut h = tr(lang, k);
    if b.by_ok {
        h += " · ";
        h += &tr1(lang, TextKey::By, &device_name(lang, b.by));
    }
    h
}
