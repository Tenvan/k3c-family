//! Port von `native/k3c-game/menu/testing.go`: Testing (B-896), Stresstest einstellen und starten. Zeilen: Vorgabe,
//! Spieler, Stufe, Seed, Gegner je Welle, Anstieg, Faktor, Ausbau, Truppen, Zeitraffer, Wellen, Dauer, Modus (normal
//! oder rein, B-921), Starten, Tasten-Test, Zurück (rollt wie die Spielstände, `MAX_ROWS` sichtbar). Links/Rechts ändert
//! am Rand begrenzt, A im Kreis; eine geänderte Zeile macht aus der Vorgabe „eigene Werte“. Starten merkt die Werte in den
//! Einstellungen (`Settings::stress`) und startet über `view::stress::StressStarter` (`Menus::starter`).
//!
//! Abweichung: Go prüft `s.(view.StressStarter)` an der Session; die Rust-`Session` hat keinen Weg zur Rolle, darum hält
//! `Menus` den Starter (`set_stress_starter`). Ohne Starter erscheint wie in Go `TstNoRunner`.

use std::cell::RefCell;

use k3c_platform::Canvas;

use super::Shared;
use super::draw::{Page, Row};
use super::keys::{Base, Handler, Key, Note, cycle, move_sel, run, window};
use super::layout::{MAT_BLUE, MAT_CORAL, MAT_GREEN, MAT_PURPLE, MAX_ROWS};
use super::main_screen::by_hint;
use super::padtest::new_pad_test;
use super::stress_presets::{StressPreset, load_stress_presets};
use super::texts::{TextKey, tr};
use crate::view::MAX_PLAYERS;
use crate::view::cell::Cell;
use crate::view::devkeys::TIMESCALES;
use crate::view::frame::{CueKind, Frame};
use crate::view::input::{DeviceID, Input};
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::session::{Fehler, Session};
use crate::view::stress::{GROWTH_EXP, GROWTH_LINEAR, STRESS_MAX_ENEMIES, StressTest};
use crate::view::stress_limits::{
    STRESS_MAX_BUILD, STRESS_MAX_DEPTH, STRESS_MAX_FACTOR, STRESS_MAX_MINUTES, STRESS_MAX_SEED, STRESS_MAX_TROOPS,
    STRESS_MAX_WAVES,
};

pub const TST_PRESET: usize = 0;
pub const TST_PLAYERS: usize = 1;
pub const TST_DEPTH: usize = 2;
pub const TST_SEED: usize = 3;
pub const TST_ENEMIES: usize = 4;
pub const TST_GROWTH: usize = 5;
pub const TST_FACTOR: usize = 6;
pub const TST_BUILD: usize = 7;
pub const TST_TROOPS: usize = 8;
pub const TST_TIMESCALE: usize = 9;
pub const TST_WAVES: usize = 10;
pub const TST_MINUTES: usize = 11;
pub const TST_MODE: usize = 12;
pub const TST_START: usize = 13;
pub const TST_PADS: usize = 14;
pub const TST_BACK: usize = 15;
pub const TST_ROWS: usize = 16;

const TST_LABELS: [TextKey; TST_ROWS] = [
    TextKey::TstPreset,
    TextKey::TstPlayers,
    TextKey::TstDepth,
    TextKey::Seed,
    TextKey::TstEnemies,
    TextKey::TstGrowth,
    TextKey::TstFactor,
    TextKey::TstBuild,
    TextKey::TstTroops,
    TextKey::TstTimescale,
    TextKey::TstWaves,
    TextKey::TstMinutes,
    TextKey::TstMode,
    TextKey::TstStart,
    TextKey::TstPads,
    TextKey::Back,
];

/// Schritt und Höchstwert der Zahl-Zeilen (Minimum 0, Spieler 1); `None` für Zeilen ohne Zahl.
fn tst_steps(r: usize) -> Option<(i64, i64)> {
    Some(match r {
        TST_PLAYERS => (1, MAX_PLAYERS as i64),
        TST_DEPTH => (1, STRESS_MAX_DEPTH),
        TST_SEED => (1, STRESS_MAX_SEED),
        TST_ENEMIES => (5, STRESS_MAX_ENEMIES),
        TST_BUILD => (1, STRESS_MAX_BUILD),
        TST_TROOPS => (10, STRESS_MAX_TROOPS),
        TST_WAVES => (1, STRESS_MAX_WAVES),
        TST_MINUTES => (5, STRESS_MAX_MINUTES),
        _ => return None,
    })
}

/// TestingScreen ist die Seite Testing; `cur` sind die eingestellten Werte, `preset` der Index der Vorgabe (-1 = eigene).
pub struct TestingScreen {
    pub base: Base,
    m: Shared,
    pub cur: StressTest,
    pub presets: Vec<StressPreset>,
    pub preset: i64,
    loaded: bool,
    pub sel: usize,
    pub pg: RefCell<Page>,
}

pub fn new_testing(m: &Shared) -> TestingScreen {
    TestingScreen {
        base: Base::default(),
        m: m.clone(),
        cur: StressTest::default(),
        presets: Vec::new(),
        preset: -1,
        loaded: false,
        sel: 0,
        pg: RefCell::new(Page::default()),
    }
}

impl TestingScreen {
    /// Liest die gemerkten Werte und die Vorgaben (ohne Vorgaben: Meldung, die Seite bleibt bedienbar).
    fn load(&mut self, s: &dyn Session) {
        self.loaded = true;
        self.cur = s.settings().normalized().stress;
        match load_stress_presets() {
            Ok(ps) => self.presets = ps,
            Err(_) => {
                self.base.note = Note {
                    key: TextKey::TstNoPresets,
                    bad: true,
                    set: true,
                    ..Note::default()
                };
                self.presets = Vec::new();
            }
        }
        let id = &self.cur.preset;
        self.preset = self
            .presets
            .iter()
            .position(|p| p.stress.preset == *id)
            .map_or(-1, |i| i as i64);
        self.base.ver += 1;
    }

    /// Ändert die Zeile `r` mit Taste `k`: links/rechts begrenzt, A im Kreis.
    fn change(&mut self, r: usize, k: Key) {
        let dir = if k == Key::Left { -1 } else { 1 };
        let round = k == Key::OK;
        if r == TST_PRESET {
            let n = self.presets.len() as i64 + 1;
            self.choose_preset(cycle_or_clamp(self.preset + 1, dir, n, round) - 1);
        } else {
            self.cur = change_stress(std::mem::take(&mut self.cur), r, dir, round);
            self.cur.preset.clear();
            self.preset = -1;
        }
        self.base.cue(CueKind::Move, -1, "");
        self.base.ver += 1;
    }

    /// Übernimmt die Werte der Vorgabe `i`; -1 (eigene) behält die Werte.
    fn choose_preset(&mut self, i: i64) {
        self.preset = i;
        match usize::try_from(i).ok().and_then(|i| self.presets.get(i)) {
            Some(p) => self.cur = p.stress.clone(),
            None => self.cur.preset.clear(),
        }
    }

    /// Merkt die Werte und startet den Test über den Starter; ohne ihn oder bei Fehler bleibt die Seite.
    fn start(&mut self, d: DeviceID, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        let mut set = s.settings().normalized();
        set.stress = self.cur.clone();
        if let Err(e) = s.set_settings(set) {
            k3c_platform::log::warn("💥 Einstellungen nicht gespeichert", &[("err", &e)]);
        }
        let res = self
            .m
            .borrow_mut()
            .starter
            .as_mut()
            .map(|st| st.start_stress(&self.cur, d));
        match res {
            None => {
                self.base.note = Note {
                    key: TextKey::TstNoRunner,
                    bad: true,
                    set: true,
                    ..Note::default()
                };
                self.base.ver += 1;
                return (StepKind::Stay, None);
            }
            Some(Err(e)) => {
                self.base.fail(&Fehler::Session(e));
                return (StepKind::Stay, None);
            }
            Some(Ok(())) => {}
        }
        let c = &self.cur;
        k3c_platform::log::info(
            "🚀 Stresstest aus dem Menü",
            &[
                ("vorgabe", &c.preset),
                ("spieler", &c.players),
                ("gegner", &c.enemies),
                ("anstieg", &c.growth),
                ("faktor", &c.factor),
                ("wellen", &c.waves),
                ("rein", &c.pure),
            ],
        );
        self.base.cue(CueKind::Confirm, -1, "");
        (StepKind::Play, None)
    }

    /// Baut die sichtbaren Zeilen (Fenster von `MAX_ROWS` um die Auswahl) in der Sprache `lang`.
    pub fn fill(&self, lang: &str) {
        let mut p = self.pg.borrow_mut();
        p.reset(&tr(lang, TextKey::Testing));
        p.sub = tr(lang, TextKey::TestingSub);
        let from = window(self.sel as i64, TST_ROWS as i64, MAX_ROWS as i64) as usize;
        for (r, label) in TST_LABELS.iter().enumerate().skip(from).take(MAX_ROWS) {
            let mut rw = Row {
                label: tr(lang, *label),
                mat: MAT_BLUE,
                ..Row::default()
            };
            match r {
                TST_START => rw.mat = MAT_GREEN,
                TST_PADS => rw.mat = MAT_PURPLE,
                TST_BACK => rw.mat = MAT_CORAL,
                _ => rw.value = format!("< {} >", self.value_text(lang, r)),
            }
            p.add(rw);
        }
        p.sel = (self.sel - from) as i64;
        p.note = self.base.note.text(lang);
        p.bad = self.base.note.bad;
        p.hint = by_hint(lang, TextKey::TestingHint, &self.base);
    }
}

impl Handler for TestingScreen {
    fn base(&mut self) -> &mut Base {
        &mut self.base
    }

    fn handle(&mut self, d: DeviceID, k: Key, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        match k {
            Key::Up | Key::Down => {
                let dir = if k == Key::Up { -1 } else { 1 };
                self.sel = move_sel(self.sel as i64, dir, TST_ROWS as i64, true, None) as usize;
                self.base.cue(CueKind::Move, -1, "");
                self.base.ver += 1;
            }
            Key::Back | Key::Home => {
                self.base.cue(CueKind::Back, -1, "");
                return (StepKind::Pop, None);
            }
            Key::Left | Key::Right | Key::OK => match self.sel {
                TST_START if k == Key::OK => return self.start(d, s),
                TST_PADS if k == Key::OK => {
                    self.base.cue(CueKind::Confirm, -1, "");
                    return (StepKind::Push, Some(Box::new(new_pad_test(&self.m))));
                }
                TST_BACK if k == Key::OK => {
                    self.base.cue(CueKind::Back, -1, "");
                    return (StepKind::Pop, None);
                }
                r if r < TST_START => self.change(r, k),
                _ => {}
            },
            Key::Delete | Key::None => {}
        }
        (StepKind::Stay, None)
    }
}

impl Screen for TestingScreen {
    fn update(&mut self, input: &Input, s: &mut dyn Session) -> Step {
        if !self.loaded {
            self.load(s);
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

/// Verschiebt `i` um `dir` unter `n` Werten: im Kreis (`round`) oder am Rand begrenzt.
pub fn cycle_or_clamp(i: i64, dir: i64, n: i64, round: bool) -> i64 {
    if round {
        return cycle(i, dir, n);
    }
    0.max((n - 1).min(i + dir))
}

/// Ändert den Wert der Zeile `r` (rein): Zahlen in Schritten, Anstieg kippt, Faktor in 1 (linear) bzw. 0,1
/// (exponentiell), Zeitraffer durch `TIMESCALES`.
pub fn change_stress(mut t: StressTest, r: usize, dir: i64, round: bool) -> StressTest {
    match r {
        TST_GROWTH => {
            t.growth = if t.growth == GROWTH_EXP {
                GROWTH_LINEAR
            } else {
                GROWTH_EXP
            }
            .to_string();
        }
        TST_FACTOR => {
            let step = if t.growth == GROWTH_EXP { 0.1 } else { 1.0 };
            t.factor = ((t.factor + dir as f64 * step) * 10.0).round() / 10.0;
            if round && t.factor > STRESS_MAX_FACTOR {
                t.factor = 0.0;
            }
        }
        TST_MODE => t.pure = !t.pure,
        TST_TIMESCALE => {
            let i = TIMESCALES.iter().position(|v| *v == t.timescale).unwrap_or(0);
            let j = cycle_or_clamp(i as i64, dir, TIMESCALES.len() as i64, round);
            t.timescale = TIMESCALES.get(j as usize).copied().unwrap_or(t.timescale);
        }
        _ => {
            if let (Some(st), Some(p)) = (tst_steps(r), stress_field(&mut t, r)) {
                let lo = i64::from(r == TST_PLAYERS);
                *p = step_int(*p, dir * st.0, lo, st.1, round);
            }
        }
    }
    t.normalized()
}

/// Verschiebt `v` um `d` in `lo` … `hi`: am Rand begrenzt oder (`round`) von `hi` nach `lo`.
pub fn step_int(v: i64, d: i64, lo: i64, hi: i64, round: bool) -> i64 {
    let n = v + d;
    if round && n > hi {
        return lo;
    }
    if round && n < lo {
        return hi;
    }
    lo.max(hi.min(n))
}

/// Das Zahl-Feld der Zeile `r` (Go liefert einen Zeiger; `None` für Zeilen ohne Zahl).
pub fn stress_field(t: &mut StressTest, r: usize) -> Option<&mut i64> {
    Some(match r {
        TST_PLAYERS => &mut t.players,
        TST_DEPTH => &mut t.depth,
        TST_SEED => &mut t.seed,
        TST_ENEMIES => &mut t.enemies,
        TST_BUILD => &mut t.build,
        TST_TROOPS => &mut t.troops,
        TST_WAVES => &mut t.waves,
        TST_MINUTES => &mut t.minutes,
        _ => return None,
    })
}
