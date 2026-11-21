//! Port von `native/k3c-game/menu/saves.go`: die Spielstand-Liste (Vorbild `src/scenes/saveMenu.ts`): neueste zuerst wie
//! `Session::list`. A lädt, View (Tab) fragt nach dem Löschen; die Rückfrage hat „Abbrechen“ vorgewählt, nur „Löschen“ + A
//! löscht, B bricht ab. Nicht ladbare Stände stehen markiert mit Grund in der Liste: Laden ist gesperrt, Löschen geht.
//!
//! Abweichung: Go formatiert die Zeit mit `Local()`; `std` kennt keine Ortszeit, darum rechnet [`format_saved_at`] mit dem
//! Versatz `Menus::utc_offset_min` (Standard 0; der Client setzt ihn aus der Plattform).

use std::cell::RefCell;
use std::time::{SystemTime, UNIX_EPOCH};

use k3c_platform::Canvas;

use super::Shared;
use super::draw::{Page, Row};
use super::keys::{Base, Handler, Key, Note, move_sel, run, window};
use super::layout::{MAT_BLUE, MAX_ROWS};
use super::main_screen::by_hint;
use super::texts::{TextKey, tr, tr1};
use crate::view::cell::Cell;
use crate::view::frame::{CueKind, Frame};
use crate::view::input::{DeviceID, Input};
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::seal::grade_seal;
use crate::view::session::{SaveInfo, Session};

/// SavesScreen ist die Spielstand-Liste.
pub struct SavesScreen {
    pub base: Base,
    m: Shared,
    pub list: Vec<SaveInfo>,
    pub listed: bool,
    pub sel: i64,
    pub asking: bool,
    /// in der Rückfrage ist „Löschen“ gewählt
    pub yes: bool,
    pub pg: RefCell<Page>,
}

pub fn new_saves(m: &Shared) -> SavesScreen {
    SavesScreen {
        base: Base::default(),
        m: m.clone(),
        list: Vec::new(),
        listed: false,
        sel: 0,
        asking: false,
        yes: false,
        pg: RefCell::new(Page::default()),
    }
}

impl SavesScreen {
    /// Liest die Spielstände neu; die Auswahl bleibt in der Liste.
    fn refresh(&mut self, s: &mut dyn Session) {
        self.listed = true;
        match s.list() {
            Ok(list) => self.list = list,
            Err(e) => {
                self.base.note = Note {
                    raw: e.to_string(),
                    bad: true,
                    set: true,
                    ..Note::default()
                };
                self.list = Vec::new();
            }
        }
        self.sel = 0.max(self.sel.min(self.list.len() as i64 - 1));
        self.base.ver += 1;
    }

    /// Lädt den gewählten Stand mit dem bestätigenden Gerät als Spieler 1 (weitere treten im Spiel mit A bei).
    fn load(&mut self, d: DeviceID, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        let Some(e) = self.list.get(self.sel as usize).cloned() else {
            return (StepKind::Stay, None);
        };
        if !e.loadable {
            self.base.note = Note {
                raw: e.problem.clone(),
                bad: true,
                set: true,
                ..Note::default()
            };
            self.base.cue(CueKind::Denied, -1, &e.problem);
            self.base.ver += 1;
            return (StepKind::Stay, None);
        }
        if let Err(err) = s.load(&e.name, &[d]) {
            self.base.fail(&err);
            return (StepKind::Stay, None);
        }
        k3c_platform::log::info(
            "📂 Spielstand aus dem Menü geladen",
            &[("save", &e.name), ("device", &d)],
        );
        self.base.cue(CueKind::Confirm, -1, "");
        (StepKind::Play, None)
    }

    /// Bedient die Rückfrage vor dem Löschen.
    fn answer(&mut self, k: Key, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        match k {
            Key::Up | Key::Down | Key::Left | Key::Right => {
                self.yes = !self.yes;
                self.base.cue(CueKind::Move, -1, "");
            }
            Key::OK => {
                self.asking = false;
                if self.yes {
                    self.remove(s);
                } else {
                    self.base.cue(CueKind::Back, -1, "");
                }
            }
            Key::Back | Key::Delete => {
                self.asking = false;
                self.base.cue(CueKind::Back, -1, "");
            }
            Key::Home => {
                self.asking = false;
                self.base.cue(CueKind::Back, -1, "");
                return (StepKind::Pop, None);
            }
            Key::None => {}
        }
        self.base.ver += 1;
        (StepKind::Stay, None)
    }

    /// Löscht den gewählten Stand und liest die Liste neu.
    fn remove(&mut self, s: &mut dyn Session) {
        let name = self.list[self.sel as usize].name.clone();
        if let Err(e) = s.delete(&name) {
            self.base.fail(&e);
            return;
        }
        k3c_platform::log::info("🧹 Spielstand aus dem Menü gelöscht", &[("save", &name)]);
        self.base.cue(CueKind::Confirm, -1, "");
        self.refresh(s);
        self.base.inform(TextKey::Deleted, &name);
    }

    /// Baut die sichtbaren Zeilen (rollend um die Auswahl) und die Rückfrage.
    pub fn fill(&self, lang: &str) {
        let mut p = self.pg.borrow_mut();
        p.reset(&tr(lang, TextKey::SavesTitle));
        let n = self.list.len();
        let from = window(self.sel, n as i64, MAX_ROWS as i64) as usize;
        let offset = self.m.borrow().utc_offset_min;
        for e in &self.list[from.min(n)..n.min(from + MAX_ROWS)] {
            let mut r = save_row(lang, e, offset);
            if e.loadable {
                r.art = self.m.borrow().world_asset(&e.world, false); // kleines Bild der Welt (B-938)
            }
            p.add(r);
        }
        p.sel = self.sel - from as i64;
        if n == 0 {
            p.sub = tr(lang, TextKey::SavesEmpty);
        }
        p.note = self.base.note.text(lang);
        p.bad = self.base.note.bad;
        p.hint = by_hint(lang, TextKey::SavesHint, &self.base);
        if self.asking {
            p.ask = tr1(lang, TextKey::DeleteAsk, &self.list[self.sel as usize].name);
            p.ask_rows = [tr(lang, TextKey::DeleteNo), tr(lang, TextKey::DeleteYes)];
            p.ask_sel = i64::from(self.yes);
            p.hint = tr(lang, TextKey::AskHint);
        }
    }
}

impl Handler for SavesScreen {
    fn base(&mut self) -> &mut Base {
        &mut self.base
    }

    fn handle(&mut self, d: DeviceID, k: Key, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        if self.asking {
            return self.answer(k, s);
        }
        match k {
            Key::Up | Key::Down => {
                let dir = if k == Key::Up { -1 } else { 1 };
                let next = move_sel(self.sel, dir, self.list.len() as i64, false, None);
                if next != self.sel {
                    self.sel = next;
                    self.base.cue(CueKind::Move, -1, "");
                }
                self.base.ver += 1;
            }
            Key::OK => return self.load(d, s),
            Key::Delete => {
                if !self.list.is_empty() {
                    self.asking = true;
                    self.yes = false;
                    self.base.ver += 1;
                }
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

impl Screen for SavesScreen {
    /// Liest die Liste beim ersten Mal und bedient die Tasten.
    fn update(&mut self, input: &Input, s: &mut dyn Session) -> Step {
        if !self.listed {
            self.refresh(s);
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

/// Die Karte eines Spielstands: Name links; rechts Tag, Stufen und Zeit bzw. der Grund, warum er nicht ladbar ist.
/// `utc_offset_min` ist der Versatz der Ortszeit zu UTC in Minuten.
pub fn save_row(lang: &str, e: &SaveInfo, utc_offset_min: i64) -> Row {
    let mut r = Row {
        label: e.name.clone(),
        mat: MAT_BLUE,
        ..Row::default()
    };
    if !e.loadable {
        r.value = e.problem.clone();
        r.off = true;
        return r;
    }
    r.badge = grade_seal(&e.grade); // alter Stand ohne Grad: Siegel des Standardgrads
    r.value = tr1(lang, TextKey::SavesLine, &e.day.to_string());
    for (i, d) in e.depths.iter().enumerate() {
        if i > 0 {
            r.value.push(',');
        } else {
            r.value += " · ";
            r.value += &tr(lang, TextKey::Depth);
            r.value.push(' ');
        }
        r.value += &d.to_string();
    }
    if let Some(t) = e.saved_at {
        r.value += " · ";
        r.value += &format_saved_at(t, utc_offset_min);
    }
    r
}

/// Formatiert `t` als „TT.MM. HH:MM“ in der Ortszeit mit dem Versatz `offset_min` (Minuten gegenüber UTC). Reine
/// Rechnung aus Tagen seit 1970 im gregorianischen Kalender; Zeiten vor 1970 gehen ebenso.
pub fn format_saved_at(t: SystemTime, offset_min: i64) -> String {
    let secs = match t.duration_since(UNIX_EPOCH) {
        Ok(d) => i64::try_from(d.as_secs()).unwrap_or(i64::MAX / 2),
        Err(e) => -i64::try_from(e.duration().as_secs()).unwrap_or(i64::MAX / 2),
    } + offset_min * 60;
    let days = secs.div_euclid(86_400);
    let rest = secs.rem_euclid(86_400);
    let (month, day) = month_day(days);
    format!("{day:02}.{month:02}. {:02}:{:02}", rest / 3600, rest % 3600 / 60)
}

/// Monat und Tag des Tages `days` seit 1970-01-01 (Zivilkalender nach H. Hinnant; das Jahr zeigt die Zeile nicht).
fn month_day(days: i64) -> (i64, i64) {
    let z = days + 719_468;
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    (month, day)
}
