//! Port von `native/k3c-game/menu/options.go`: Optionen (Entscheidung 🧑 2026-11-11, B-800, dazu Blitze und Anleitung,
//! B-809; Vorbild `src/scenes/optionsLogic.ts`): Lautstärke Musik und Effekte in Schritten von 10 %, Anzeige (Fenster,
//! rahmenlos, Vollbild, B-826), Sprache, Blitze, Anleitung, Entwicklermodus (Karte Testing, B-896), dazu die Unterseite
//! Steuerung (Eingabe-Profile je Spieler, B-893, `controls.rs`). Mehr als `MAX_ROWS` Zeilen rollen mit der Auswahl.
//! Links/Rechts ändert, A legt Schalter, Anzeige und Sprache um, Hoch/Runter springt am Ende zum Anfang. Jede Änderung
//! geht sofort über `Session::set_settings` (local speichert sie); ein Fehler betrifft nur das Speichern und erscheint
//! als Meldung. Grundlage jeder Änderung ist der aktuelle Stand der Session, nicht der beim Öffnen: F2/F3 (`stats`,
//! `river`) gehen nicht verloren. Anleitung wieder an setzt „Anleitung gesehen“ zurück.

use std::cell::RefCell;

use k3c_platform::Canvas;

use super::Shared;
use super::controls::new_controls;
use super::draw::{Page, Row};
use super::keys::{Base, Handler, Key, Note, move_sel, run, window};
use super::layout::{MAT_CORAL, MAT_PURPLE, MAX_ROWS};
use super::main_screen::by_hint;
use super::texts::{TextKey, tr};
use crate::view::cell::Cell;
use crate::view::frame::{CueKind, Frame};
use crate::view::input::{DeviceID, Input};
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::session::Session;
use crate::view::settings::{
    DEFAULT_LANGUAGE, LANGUAGE_ENGLISH, Settings, WINDOW_BORDERLESS, WINDOW_FRAMED, WINDOW_FULLSCREEN, WINDOW_MODES,
};

pub const OPT_MUSIC: usize = 0;
pub const OPT_EFFECTS: usize = 1;
pub const OPT_WINDOW: usize = 2;
pub const OPT_LANGUAGE: usize = 3;
pub const OPT_FLASHES: usize = 4;
pub const OPT_GUIDE: usize = 5;
pub const OPT_DEV_MODE: usize = 6;
pub const OPT_CONTROLS: usize = 7;
pub const OPT_BACK: usize = 8;
pub const OPT_ROWS: usize = 9;

/// Der Schritt der Lautstärke-Regler (0 … 1).
const VOLUME_STEP: f64 = 0.1;

const OPT_LABELS: [TextKey; OPT_ROWS] = [
    TextKey::Music,
    TextKey::Effects,
    TextKey::Display,
    TextKey::Language,
    TextKey::Flashes,
    TextKey::Guide,
    TextKey::DevMode,
    TextKey::Controls,
    TextKey::Back,
];

/// OptionsScreen bearbeitet die Einstellungen. `cur` ist der Stand, den das Menü kennt und schreibt.
pub struct OptionsScreen {
    pub base: Base,
    m: Shared,
    pub cur: Settings,
    loaded: bool,
    pub sel: usize,
    /// aus dem Pause-Menü: halbdunkel über dem stehenden Spiel
    dim: bool,
    pub pg: RefCell<Page>,
}

pub fn new_options(m: &Shared, dim: bool) -> OptionsScreen {
    OptionsScreen {
        base: Base::default(),
        m: m.clone(),
        cur: Settings::default(),
        loaded: false,
        sel: 0,
        dim,
        pg: RefCell::new(Page::default()),
    }
}

impl OptionsScreen {
    /// Übernimmt neue Einstellungen über die Session; die Werte gelten auch, wenn das Speichern scheitert.
    fn apply(&mut self, s: &mut dyn Session, next: &Settings) {
        let next = next.normalized();
        if next == self.cur {
            return;
        }
        self.cur = next.clone();
        self.base.note = Note::default();
        self.base.cue(CueKind::Move, -1, "");
        self.base.ver += 1;
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

    /// Baut die Zeilen „Musik  < 70 % >“, „Vollbild  an“ … in der Sprache `lang`.
    pub fn fill(&self, lang: &str) {
        let mut p = self.pg.borrow_mut();
        p.reset(&tr(lang, TextKey::Options));
        let from = window(self.sel as i64, OPT_ROWS as i64, MAX_ROWS as i64) as usize;
        for (i, label) in OPT_LABELS.iter().enumerate().skip(from).take(MAX_ROWS) {
            let mut r = Row {
                label: tr(lang, *label),
                value: option_value(lang, &self.cur, i),
                mat: MAT_PURPLE,
                ..Row::default()
            };
            if i == OPT_BACK {
                r.mat = MAT_CORAL;
            }
            p.add(r);
        }
        p.sel = (self.sel - from) as i64;
        p.dim = self.dim;
        p.note = self.base.note.text(lang);
        p.bad = self.base.note.bad;
        p.hint = by_hint(lang, TextKey::OptionsHint, &self.base);
    }
}

impl Handler for OptionsScreen {
    fn base(&mut self) -> &mut Base {
        &mut self.base
    }

    fn handle(&mut self, _d: DeviceID, k: Key, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        match k {
            Key::Up | Key::Down => {
                let dir = if k == Key::Up { -1 } else { 1 };
                self.sel = move_sel(self.sel as i64, dir, OPT_ROWS as i64, true, None) as usize;
                self.base.cue(CueKind::Move, -1, "");
                self.base.ver += 1;
            }
            Key::Left | Key::Right | Key::OK => {
                if self.sel == OPT_CONTROLS && k == Key::OK {
                    self.base.cue(CueKind::Confirm, -1, "");
                    return (StepKind::Push, Some(Box::new(new_controls(&self.m, self.dim))));
                }
                if self.sel == OPT_CONTROLS || (self.sel == OPT_BACK && k != Key::OK) {
                    return (StepKind::Stay, None);
                }
                if self.sel == OPT_BACK {
                    self.base.cue(CueKind::Back, -1, "");
                    return (StepKind::Pop, None);
                }
                self.cur = s.settings().normalized();
                let next = apply_option(&self.cur, self.sel, k);
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

impl Screen for OptionsScreen {
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

/// Ändert die Einstellung der Zeile `opt` mit Taste `k` (rein): Regler um ±10 % (A ändert nichts), Schalter und Sprache
/// kippen bei jeder Taste.
pub fn apply_option(s: &Settings, opt: usize, k: Key) -> Settings {
    let mut s = s.clone();
    let dir = if k == Key::Left { -VOLUME_STEP } else { VOLUME_STEP };
    match opt {
        OPT_MUSIC if k != Key::OK => s.music = step_volume(s.music, dir),
        OPT_EFFECTS if k != Key::OK => s.effects = step_volume(s.effects, dir),
        OPT_WINDOW => s.window = next_window(&s.window, if k == Key::Left { -1 } else { 1 }),
        OPT_LANGUAGE => s.language = other_language(&s.language),
        OPT_FLASHES => s.flashes_off = !s.flashes_off,
        OPT_GUIDE => {
            s.guide_off = !s.guide_off;
            if !s.guide_off {
                s.guide_seen = 0;
            }
        }
        OPT_DEV_MODE => s.dev_mode = !s.dev_mode,
        _ => {}
    }
    s
}

/// Verschiebt `v` um `d` und rundet auf ganze 10 % (0 … 1).
pub fn step_volume(v: f64, d: f64) -> f64 {
    (((v + d) * 10.0).round() / 10.0).clamp(0.0, 1.0)
}

/// Geht in `WINDOW_MODES` um `step` weiter (zyklisch: Fenster, rahmenlos, Vollbild, B-826).
pub fn next_window(cur: &str, step: i64) -> String {
    let n = WINDOW_MODES.len() as i64;
    for (i, m) in WINDOW_MODES.iter().enumerate() {
        if *m == cur {
            return WINDOW_MODES[((i as i64 + step + n) % n) as usize].to_string();
        }
    }
    WINDOW_FRAMED.to_string()
}

/// Der Name des Anzeige-Modus `w`.
pub fn window_text(lang: &str, w: &str) -> String {
    match w {
        WINDOW_BORDERLESS => tr(lang, TextKey::Borderless),
        WINDOW_FULLSCREEN => tr(lang, TextKey::Fullscreen),
        _ => tr(lang, TextKey::Windowed),
    }
}

/// Wechselt zwischen Deutsch und Englisch.
pub fn other_language(lang: &str) -> String {
    if lang == LANGUAGE_ENGLISH {
        DEFAULT_LANGUAGE
    } else {
        LANGUAGE_ENGLISH
    }
    .to_string()
}

/// Der angezeigte Wert der Zeile `opt`.
pub fn option_value(lang: &str, s: &Settings, opt: usize) -> String {
    let on_off = |v: bool| tr(lang, if v { TextKey::On } else { TextKey::Off });
    match opt {
        OPT_MUSIC => format!("< {} % >", (s.music * 100.0).round() as i64),
        OPT_EFFECTS => format!("< {} % >", (s.effects * 100.0).round() as i64),
        OPT_WINDOW => format!("< {} >", window_text(lang, &s.window)),
        OPT_LANGUAGE => format!("< {} >", tr(&s.language, TextKey::LangName)),
        OPT_FLASHES => on_off(!s.flashes_off),
        OPT_GUIDE => on_off(!s.guide_off),
        OPT_DEV_MODE => on_off(s.dev_mode),
        _ => String::new(),
    }
}
