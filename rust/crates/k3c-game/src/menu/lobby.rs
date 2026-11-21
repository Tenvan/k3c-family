//! Port von `native/k3c-game/menu/lobby.go`: die Lobby (Vorbild `src/scenes/lobbyOptions.ts`, `localSlots.ts`). Wer
//! „Neues Spiel“ gewählt hat, ist Spieler 1. Ein weiteres Gerät tritt mit A bei (höchstens `MAX_PLAYERS`), Spieler 2
//! geht mit B, B von Spieler 1 bricht ab. Bedienen dürfen nur beigetretene Geräte. Grad, Ziel, Niederlage, Welt und Seed
//! wie der Anlegen-Dialog des Browsers; ein neuer Grad bringt seine Standard-Niederlage mit, solange sie niemand selbst
//! gewählt hat.
//!
//! Abweichung: `sim.StandardWorldID` und `sim.DefaultOptionsFor` gibt es in Rust erst mit RU14; die Standard-Welt steht
//! als Wert hier, die Standard-Niederlage liest `default_defeat` aus `data/difficulty.json`.

use std::cell::RefCell;
use std::sync::OnceLock;

use k3c_platform::Canvas;

use super::Shared;
use super::draw::{Page, Row};
use super::keys::{Base, Handler, Key, cycle, device_name, move_sel, run};
use super::layout::{MAT_BLUE, MAT_CORAL, MAT_GREEN};
use super::texts::{TextKey, tr, tr1};
use crate::view::MAX_PLAYERS;
use crate::view::cell::Cell;
use crate::view::frame::{CueKind, Frame};
use crate::view::input::{DeviceID, Input};
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::seal::grade_seal;
use crate::view::session::{NewGame, Session};
use k3c_data::difficulty::DifficultyFile;

/// Zeilen der Lobby von oben nach unten.
pub const LOBBY_GRADE: usize = 0;
pub const LOBBY_GOAL: usize = 1;
pub const LOBBY_DEFEAT: usize = 2;
pub const LOBBY_WORLD: usize = 3;
pub const LOBBY_SEED: usize = 4;
pub const LOBBY_START: usize = 5;
pub const LOBBY_CANCEL: usize = 6;
pub const LOBBY_ROWS: usize = 7;

/// Der höchste wählbare Seed; 0 heißt „Name des Spielstands“ (`NewGame::seed` leer).
pub const MAX_SEED: i64 = 999;

/// RU14 ersetzt dies durch `k3c_sim` (Go: `sim.StandardWorldID`, `engine/sim/island_worlds.go`).
const STANDARD_WORLD_ID: &str = "standard";

/// ohne dev (nur Dev-Mode, B-800 Nicht-Ziel)
const GRADES: [&str; 4] = ["easy", "normal", "hard", "ultra"];
const GRADE_TEXTS: [TextKey; 4] = [
    TextKey::GradeEasy,
    TextKey::GradeNormal,
    TextKey::GradeHard,
    TextKey::GradeUltra,
];
const GOALS: [&str; 5] = ["endboss", "gold", "days", "mineAll", "buildAll"];
const GOAL_TEXTS: [TextKey; 5] = [
    TextKey::GoalEndboss,
    TextKey::GoalGold,
    TextKey::GoalDays,
    TextKey::GoalMineAll,
    TextKey::GoalBuildAll,
];
const DEFEATS: [&str; 2] = ["resources", "stage"];
const DEFEAT_TEXTS: [TextKey; 2] = [TextKey::DefeatResources, TextKey::DefeatStage];
const LOBBY_LABELS: [TextKey; LOBBY_ROWS] = [
    TextKey::Grade,
    TextKey::Goal,
    TextKey::Defeat,
    TextKey::World,
    TextKey::Seed,
    TextKey::Start,
    TextKey::Cancel,
];
/// Zahl der Werte je Wahl-Zeile; die Welt wählt vorher die Weltauswahl (B-938).
const LOBBY_CHOICES: [i64; LOBBY_SEED] = [GRADES.len() as i64, GOALS.len() as i64, DEFEATS.len() as i64, 1];

/// LobbyScreen ist die Lobby eines neuen Spiels.
pub struct LobbyScreen {
    pub base: Base,
    m: Shared,
    pub devs: [DeviceID; MAX_PLAYERS],
    pub n: usize,
    pub row: usize,
    /// gewählter Index je Wahl-Zeile (Grad, Ziel, Niederlage; Welt immer 0)
    pub pick: [i64; LOBBY_SEED],
    /// Welt-ID aus der Weltauswahl
    pub world: String,
    pub seed: i64,
    defeat_set: bool,
    pub pg: RefCell<Page>,
}

/// Der Index der Standard-Niederlage des Grads (`data/difficulty.json`, unbekannter Grad wie `normal`).
pub fn default_defeat(grade: &str) -> i64 {
    static DIFFICULTY: OnceLock<Option<DifficultyFile>> = OnceLock::new();
    let d = DIFFICULTY.get_or_init(|| {
        let loaded = k3c_data::Source::from_env()
            .read(k3c_data::difficulty::FILE)
            .and_then(|raw| k3c_data::difficulty::load(&raw));
        loaded
            .map_err(|e| k3c_platform::log::error("❌ difficulty.json nicht lesbar", &[("err", &e)]))
            .ok()
    });
    let defeat = d
        .as_ref()
        .and_then(|d| d.grades.get(grade).or_else(|| d.grades.get("normal")))
        .map(|g| g.default_defeat.as_str())
        .unwrap_or_default();
    DEFEATS.iter().position(|x| *x == defeat).map_or(0, |i| i as i64)
}

/// Öffnet die Lobby mit dem Gerät `d` als Spieler 1 und den Standards (Normal, Endboss, Standard-Welt).
pub fn new_lobby(m: &Shared, d: DeviceID) -> LobbyScreen {
    let mut sc = LobbyScreen {
        base: Base::default(),
        m: m.clone(),
        devs: [DeviceID::default(); MAX_PLAYERS],
        n: 1,
        row: 0,
        pick: [0; LOBBY_SEED],
        world: STANDARD_WORLD_ID.to_string(),
        seed: 0,
        defeat_set: false,
        pg: RefCell::new(Page::default()),
    };
    sc.devs[0] = d;
    sc.pick[LOBBY_GRADE] = GRADES.iter().position(|g| *g == "normal").map_or(0, |i| i as i64);
    sc.pick[LOBBY_DEFEAT] = default_defeat(GRADES[sc.pick[LOBBY_GRADE] as usize]);
    sc
}

impl LobbyScreen {
    /// Der Spieler des Geräts `d` (`None` = nicht beigetreten).
    pub fn slot(&self, d: DeviceID) -> Option<usize> {
        self.devs[..self.n].iter().position(|x| *x == d)
    }

    /// Nimmt das Gerät `d` als nächsten Spieler auf.
    fn join(&mut self, d: DeviceID) {
        if self.n >= MAX_PLAYERS {
            self.base.deny(TextKey::TooMany, "");
            return;
        }
        self.devs[self.n] = d;
        self.n += 1;
        self.base.cue(CueKind::Joined, self.n as i64 - 1, "");
        self.base.ver += 1;
        k3c_platform::log::info(
            "👑 Spieler tritt der Lobby bei",
            &[("player", &(self.n - 1)), ("device", &d)],
        );
    }

    /// Lässt Spieler `p` gehen; die Späteren rücken auf.
    fn leave(&mut self, p: usize) {
        self.devs.copy_within(p + 1..self.n, p);
        self.n -= 1;
        self.base.cue(CueKind::Left, p as i64, "");
        self.base.ver += 1;
        k3c_platform::log::info("👋 Spieler verlässt die Lobby", &[("player", &p)]);
    }

    /// Wählt die Zeile darüber bzw. darunter (am Rand bleibt sie).
    fn move_row(&mut self, k: Key) {
        let dir = if k == Key::Up { -1 } else { 1 };
        let next = move_sel(self.row as i64, dir, LOBBY_ROWS as i64, false, None) as usize;
        if next != self.row {
            self.row = next;
            self.base.cue(CueKind::Move, -1, "");
        }
        self.base.ver += 1;
    }

    /// Ändert den Wert der Zeile (links/rechts begrenzt, A im Kreis) oder löst Start bzw. Abbrechen aus.
    fn change(&mut self, k: Key, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        let dir = if k == Key::Left { -1 } else { 1 };
        match self.row {
            LOBBY_START => {
                if k == Key::OK {
                    return self.start(s);
                }
                return (StepKind::Stay, None);
            }
            LOBBY_CANCEL => {
                if k == Key::OK {
                    self.base.cue(CueKind::Back, -1, "");
                    return (StepKind::Pop, None);
                }
                return (StepKind::Stay, None);
            }
            LOBBY_SEED => self.seed = step_value(self.seed, dir, MAX_SEED + 1, k == Key::OK),
            r => {
                let v = step_value(self.pick[r], dir, LOBBY_CHOICES[r], k == Key::OK);
                self.choose(r, v);
            }
        }
        self.base.cue(CueKind::Move, -1, "");
        self.base.ver += 1;
        (StepKind::Stay, None)
    }

    /// Setzt die Wahl-Zeile `r` auf Index `i`; ein neuer Grad bringt seine Standard-Niederlage mit.
    fn choose(&mut self, r: usize, i: i64) {
        self.pick[r] = i;
        match r {
            LOBBY_GRADE => {
                if !self.defeat_set {
                    self.pick[LOBBY_DEFEAT] = default_defeat(GRADES[i as usize]);
                }
            }
            LOBBY_DEFEAT => self.defeat_set = true,
            _ => {}
        }
    }

    /// Die Angaben des neuen Spiels (Name leer: local wählt spiel-N; Seed leer: Name als Seed).
    pub fn new_game(&self) -> NewGame {
        let mut g = NewGame {
            grade: GRADES[self.pick[LOBBY_GRADE] as usize].to_string(),
            goal: GOALS[self.pick[LOBBY_GOAL] as usize].to_string(),
            defeat: DEFEATS[self.pick[LOBBY_DEFEAT] as usize].to_string(),
            world: self.world.clone(),
            devices: self.devs[..self.n].to_vec(),
            ..NewGame::default()
        };
        if self.seed > 0 {
            g.seed = self.seed.to_string();
        }
        g
    }

    /// Legt das Spiel an; bei Fehler bleibt die Lobby mit Meldung.
    fn start(&mut self, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        let g = self.new_game();
        if let Err(e) = s.new_game(g.clone()) {
            self.base.fail(&e);
            return (StepKind::Stay, None);
        }
        k3c_platform::log::info(
            "🚀 Neues Spiel aus der Lobby",
            &[
                ("grad", &g.grade),
                ("ziel", &g.goal),
                ("welt", &g.world),
                ("seed", &g.seed),
                ("spieler", &g.devices.len()),
            ],
        );
        self.base.cue(CueKind::Confirm, -1, "");
        (StepKind::Play, None)
    }

    /// Baut die Texte der Lobby: Spieler-Zeile oben, Wahl-Zeilen mit „< Wert >“, Start und Abbrechen.
    pub fn fill(&self, lang: &str) {
        let mut p = self.pg.borrow_mut();
        p.reset(&tr(lang, TextKey::LobbyTitle));
        p.sub = self.players(lang);
        for (r, label) in LOBBY_LABELS.iter().enumerate() {
            let mut rw = Row {
                label: tr(lang, *label),
                mat: MAT_BLUE,
                ..Row::default()
            };
            match r {
                LOBBY_START => rw.mat = MAT_GREEN,
                LOBBY_CANCEL => rw.mat = MAT_CORAL,
                _ => rw.value = format!("< {} >", self.value_text(lang, r)),
            }
            if r == LOBBY_GRADE {
                rw.badge = grade_seal(GRADES[self.pick[r] as usize]); // Siegel des gewählten Grads (ECO1.2)
            }
            p.add(rw);
        }
        p.sel = self.row as i64;
        p.note = self.base.note.text(lang);
        p.bad = self.base.note.bad;
        p.hint = tr(lang, TextKey::LobbyHint);
    }

    /// Die Zeile der Plätze: „Spieler 1: Pad 1 · Spieler 2: frei: A drücken“.
    pub fn players(&self, lang: &str) -> String {
        let mut out = String::new();
        for i in 0..MAX_PLAYERS {
            if i > 0 {
                out += "   ·   ";
            }
            let who = if i < self.n {
                device_name(lang, self.devs[i])
            } else {
                tr(lang, TextKey::PlayerFree)
            };
            out += &tr1(lang, TextKey::Player, &(i + 1).to_string());
            out += ": ";
            out += &who;
        }
        out
    }

    /// Der angezeigte Wert der Wahl-Zeile `r`.
    fn value_text(&self, lang: &str, r: usize) -> String {
        let key = match r {
            LOBBY_GRADE => GRADE_TEXTS[self.pick[r] as usize],
            LOBBY_GOAL => GOAL_TEXTS[self.pick[r] as usize],
            LOBBY_DEFEAT => DEFEAT_TEXTS[self.pick[r] as usize],
            LOBBY_WORLD => return self.world.clone(),
            _ if self.seed == 0 => TextKey::SeedAuto,
            _ => return self.seed.to_string(),
        };
        tr(lang, key)
    }
}

/// Verschiebt `i` um `dir` unter `n` Werten: im Kreis (A) oder am Rand begrenzt (links/rechts).
pub fn step_value(i: i64, dir: i64, n: i64, round: bool) -> i64 {
    if round {
        return cycle(i, dir, n);
    }
    0.max((n - 1).min(i + dir))
}

impl Handler for LobbyScreen {
    fn base(&mut self) -> &mut Base {
        &mut self.base
    }

    fn handle(&mut self, d: DeviceID, k: Key, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        let Some(p) = self.slot(d) else {
            if k == Key::OK {
                self.join(d);
            }
            return (StepKind::Stay, None); // nicht beigetretene Geräte bedienen nichts
        };
        match k {
            Key::Back if p > 0 => self.leave(p),
            Key::Back | Key::Home => {
                self.base.cue(CueKind::Back, -1, "");
                return (StepKind::Pop, None);
            }
            Key::Up | Key::Down => self.move_row(k),
            Key::Left | Key::Right | Key::OK => return self.change(k, s),
            _ => {}
        }
        (StepKind::Stay, None)
    }
}

impl Screen for LobbyScreen {
    fn update(&mut self, input: &Input, s: &mut dyn Session) -> Step {
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
