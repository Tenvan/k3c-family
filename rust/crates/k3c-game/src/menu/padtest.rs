//! Port von `native/k3c-game/menu/padtest.go` (Screen): Tasten-Test (B-944), Seite unter Testing. Zeigt je Gerät live die
//! gedrückten Tasten (Standard-Belegung mit Xbox-Namen und roh), die Achsen und die zuletzt ausgelöste Aktion. So lässt
//! sich am Gerät prüfen, ob Taste und Aktion zusammenpassen. B geht zurück (wie in jedem Menü). Angezeigt werden die
//! Tastatur und die ersten [`PAD_SHOWN`] Pads. Den Rohzustand und die Pad-Quelle liefert `padtest_read`.

use std::cell::RefCell;
use std::collections::BTreeMap;

use k3c_platform::Canvas;

use super::Shared;
use super::draw::{Page, Row};
use super::keys::{Base, Handler, Key, run};
use super::layout::{MAT_BLUE, MAT_CORAL};
use super::padtest_read::{PadRaw, clip};
use super::texts::{TextKey, tr};
use crate::view::cell::Cell;
use crate::view::frame::{CueKind, Frame};
use crate::view::input::{DeviceID, DeviceKind, Input};
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::session::Session;

/// Angezeigte Pads (zwei Zeilen je Pad, dazu Tastatur und Zurück: höchstens `MAX_ROWS`).
pub const PAD_SHOWN: usize = 2;
/// Updates (30 Hz), die eine Aktion nach dem Auslösen stehen bleibt.
pub const HOLD_LAST: i64 = 45;
/// Zeichen des Gerätenamens in der Zeile.
const NAME_MAX: usize = 22;

/// Die Menü-Aktionen, Index = `Action as usize` (gleiche Reihenfolge wie in Go).
pub const ACTION_NAMES: [&str; 14] = [
    "Hoch",
    "Runter",
    "Links",
    "Rechts",
    "Bestätigen",
    "Zurück",
    "Menü",
    "View",
    "Seite zurück",
    "Seite vor",
    "Home",
    "Skill-Menü",
    "Vollbild",
    "Cheats",
];

/// PadTestScreen ist die Seite Tasten-Test.
pub struct PadTestScreen {
    pub base: Base,
    m: Shared,
    pub pg: RefCell<Page>,
    pub pads: Vec<PadRaw>,
    /// zuletzt ausgelöste Aktion je Gerät
    pub last: BTreeMap<DeviceID, &'static str>,
    /// Updates seit dem Auslösen
    pub age: BTreeMap<DeviceID, i64>,
    /// Text der letzten Anzeige (nur bei Änderung neu bauen)
    shown: String,
}

/// Die Seite wird aus dem Testing-Menü geöffnet (RU12.3, `testing.rs`).
pub fn new_pad_test(m: &Shared) -> PadTestScreen {
    PadTestScreen {
        base: Base::default(),
        m: m.clone(),
        pg: RefCell::new(Page::default()),
        pads: Vec::new(),
        last: BTreeMap::new(),
        age: BTreeMap::new(),
        shown: String::new(),
    }
}

impl PadTestScreen {
    /// Liefert die zuletzt ausgelöste Aktion des Geräts `d` ("–" ohne).
    fn action(&self, d: DeviceID) -> &'static str {
        self.last.get(&d).copied().unwrap_or("–")
    }

    /// Liefert die Zeilen (Beschriftung, Wert) der Anzeige.
    pub fn rows(&self) -> Vec<(String, String)> {
        let keys = DeviceID::new(DeviceKind::Keyboard, 0);
        let mut out = vec![("Tastatur".to_string(), format!("→ {}", self.action(keys)))];
        if self.pads.is_empty() {
            out.push(("Gamepad".to_string(), "keines verbunden".to_string()));
            return out;
        }
        for p in self.pads.iter().take(PAD_SHOWN) {
            let d = DeviceID::new(DeviceKind::Gamepad, p.id);
            out.push((
                format!("Pad {} · {}", p.id + 1, clip(&p.name, NAME_MAX)),
                format!("{} → {}", p.buttons, self.action(d)),
            ));
            out.push((format!("  roh {}", p.raw), p.axes.clone()));
        }
        out
    }

    fn text(&self) -> String {
        self.rows().iter().map(|(l, v)| format!("{l}|{v}\n")).collect()
    }

    /// Altert die Aktionen, löscht abgelaufene und merkt die neuen.
    fn track(&mut self, input: &Input) {
        for a in self.age.values_mut() {
            *a += 1;
        }
        let gone: Vec<DeviceID> = self
            .age
            .iter()
            .filter(|(_, a)| **a > HOLD_LAST)
            .map(|(d, _)| *d)
            .collect();
        for d in gone {
            self.age.remove(&d);
            self.last.remove(&d);
        }
        for di in &input.devices {
            for &a in &di.actions {
                if let Some(name) = ACTION_NAMES.get(a as usize) {
                    self.last.insert(di.device, name);
                    self.age.insert(di.device, 0);
                }
            }
        }
    }

    /// Baut die Zeilen der Seite; die Gerätetexte sind Entwickler-Anzeigen und nur deutsch.
    pub fn fill(&self, lang: &str) {
        let mut p = self.pg.borrow_mut();
        p.reset(&tr(lang, TextKey::TstPads));
        p.sub = "Tasten, Achsen und Aktion je Gerät live".to_string();
        for (label, value) in self.rows() {
            p.add(Row {
                label,
                value,
                mat: MAT_BLUE,
                ..Row::default()
            });
        }
        p.add(Row {
            label: tr(lang, TextKey::Back),
            mat: MAT_CORAL,
            ..Row::default()
        });
        p.sel = p.n as i64 - 1;
        if let Some(first) = self.pads.first() {
            p.note = format!(
                "Pad {}: {} · {}",
                first.id + 1,
                clip(&first.name, 40),
                clip(&first.sdl, 40)
            );
        }
        p.hint = tr(lang, TextKey::PadTestHint);
    }
}

impl Handler for PadTestScreen {
    fn base(&mut self) -> &mut Base {
        &mut self.base
    }

    /// B, Esc und A (auf Zurück, der einzigen Zeile) gehen zurück; alle anderen Tasten zeigt die Seite nur an.
    fn handle(&mut self, _d: DeviceID, k: Key, _s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        if matches!(k, Key::Back | Key::Home | Key::OK) {
            self.base.cue(CueKind::Back, -1, "");
            return (StepKind::Pop, None);
        }
        (StepKind::Stay, None)
    }
}

impl Screen for PadTestScreen {
    fn update(&mut self, input: &Input, s: &mut dyn Session) -> Step {
        self.pads = self.m.borrow().pads.read();
        self.track(input);
        let text = self.text();
        if text != self.shown {
            self.shown = text;
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
