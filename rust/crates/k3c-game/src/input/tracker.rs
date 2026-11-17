//! Port von `native/k3c-game/input/tracker.go`.
//!
//! Reine Auswertung eines Geräts: Rohzustand (gehaltene Tasten, Stick) → Befehl und Menü-Aktionen. Der tracker hält
//! je Gerät, was über die Updates zählt (vorige Tasten, Haltedauer, Hysterese des Sticks, Kombi View + Menu).

pub use k3c_platform::KeyState;
use k3c_platform::{Key, PadButton};

use super::bindings::{
    ACTION_COUNT, ALL_ACTIONS, LAYOUT_TABLE, PAD_A, PAD_ACTIONS, PAD_LEFT, PAD_RIGHT, PAD_RT, PAD_SKILLS, PAD_X,
};
use crate::view::input::{Action, Command};

/// PAD_BUTTON_COUNT ist die Zahl der Standard-Tasten eines Gamepads (Index = `PadButton as usize`).
pub const PAD_BUTTON_COUNT: usize = PadButton::ALL.len();

/// PadState ist der Rohzustand eines Gamepads in einem Update.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PadState {
    pub id: u32,
    /// Standard-Belegung vorhanden; sonst wird das Pad ignoriert
    pub standard: bool,
    /// nur für die Meldung bei fehlender Standard-Belegung
    pub name: String,
    pub buttons: [bool; PAD_BUTTON_COUNT],
    /// linker Stick, -1 … 1 (Y negativ = oben)
    pub x: f64,
    pub y: f64,
}

// Schwellen des Sticks mit Hysterese: an ab der ersten, aus erst unter der zweiten Schwelle.
/// laufen (Totzone wie im Browser-Client)
pub const MOVE_ON: f64 = 0.25;
pub const MOVE_OFF: f64 = 0.15;
/// Menü-Richtung aus dem Stick
pub const NAV_ON: f64 = 0.5;
pub const NAV_OFF: f64 = 0.3;

// Takte bei 30 Updates je Sekunde (game.TPS).
/// Wiederholung einer gehaltenen Richtung ab 400 ms …
pub const REPEAT_DELAY: i64 = 12;
/// … dann alle 133 ms
pub const REPEAT_EVERY: i64 = 4;
/// Menu kurz: losgelassen nach höchstens 600 ms
pub const MENU_SHORT: i64 = 18;

/// Tracker ist der Zustand eines Geräts über die Updates.
#[derive(Clone, Debug, Default)]
pub struct Tracker {
    pub started: bool,
    /// Zahlen und Schlagen ohne Wirkung, bis beide losgelassen sind (nach einem Wechsel Menü ↔ Spiel)
    pub quiet: bool,
    pub prev: [bool; ACTION_COUNT],
    /// Updates in Folge gehalten, einschließlich des aktuellen
    pub hold: [i64; ACTION_COUNT],
    /// beim ersten Update schon gehalten: ohne Wirkung bis zum Loslassen
    pub ignore: [bool; ACTION_COUNT],
    /// View + Menu kam in diesem Halten: Loslassen löst weder Menu noch View aus
    pub chord: bool,
    /// Stick außerhalb der Totzone (Hysterese)
    pub moving: bool,
    /// Stick-Richtungen ↑ ↓ ← → (Hysterese)
    pub nav: [bool; 4],
}

/// Hysteresis meldet, ob ein Wert v ≥ 0 aktiv ist: einschalten über on, ausschalten erst unter off.
fn hysteresis(active: bool, v: f64, on: f64, off: f64) -> bool {
    if active {
        return v > off;
    }
    v > on
}

impl Tracker {
    /// StickMove liefert die Laufrichtung aus der Stick-Achse x wie der Browser (src/input/playerInput.ts › moveX): 0
    /// in der Totzone, sonst der Rohwert (begrenzt auf -1 … 1). Die Hysterese hält den Rohwert noch bis unter
    /// `MOVE_OFF`, damit der Monarch an der Schwelle nicht flackert; der Betrag liegt damit nie unter dem des Browsers.
    pub fn stick_move(&mut self, x: f64) -> f64 {
        let x = if x.is_nan() { 0.0 } else { x };
        let a = x.abs();
        self.moving = a >= MOVE_ON || hysteresis(self.moving, a, MOVE_ON, MOVE_OFF); // ab genau 0,25 wie der Browser
        if !self.moving {
            return 0.0;
        }
        a.min(1.0).copysign(x)
    }

    /// StickNav aktualisiert die Menü-Richtungen des Sticks (↑ ↓ ← →) mit Hysterese.
    pub fn stick_nav(&mut self, x: f64, y: f64) {
        for (i, v) in [-y, y, -x, x].into_iter().enumerate() {
            let v = if v.is_nan() { 0.0 } else { v };
            self.nav[i] = hysteresis(self.nav[i], v, NAV_ON, NAV_OFF);
        }
    }

    /// Mute nimmt Zahlen und Schlagen aus c, solange sie seit dem Wechsel Menü ↔ Spiel gehalten sind.
    pub fn mute(&mut self, c: &mut Command) {
        if !self.quiet {
            return;
        }
        if !c.pay && !c.attack {
            self.quiet = false;
            return;
        }
        c.pay = false;
        c.attack = false;
    }

    /// PadCommand liefert den Befehl eines Gamepads; das Steuerkreuz geht vor dem Stick, beide zugleich heben sich auf.
    pub fn pad_command(&mut self, p: &PadState) -> Command {
        let b = &p.buttons;
        let mut c = Command {
            move_: self.stick_move(p.x),
            sprint: b[PAD_RT as usize],
            pay: b[PAD_A as usize],
            attack: b[PAD_X as usize],
            ..Default::default()
        };
        if b[PAD_LEFT as usize] && !b[PAD_RIGHT as usize] {
            c.move_ = -1.0;
        } else if b[PAD_RIGHT as usize] && !b[PAD_LEFT as usize] {
            c.move_ = 1.0;
        }
        for (i, s) in PAD_SKILLS.iter().enumerate() {
            if b[*s as usize] {
                c.skill = i as i64 + 1;
                break;
            }
        }
        c
    }

    /// PadHeld liefert die gehaltenen Menü-Tasten eines Gamepads; Steuerkreuz und Stick zählen gleich.
    pub fn pad_held(&mut self, p: &PadState) -> [bool; ACTION_COUNT] {
        self.stick_nav(p.x, p.y);
        let mut held = [false; ACTION_COUNT];
        for (a, b) in PAD_ACTIONS.iter().enumerate() {
            if let Some(b) = b {
                held[a] = p.buttons[*b as usize];
            }
        }
        for (h, n) in held.iter_mut().zip(self.nav) {
            *h = *h || n; // Action::Up … Action::Right sind 0 … 3
        }
        held
    }

    /// Step wertet die gehaltenen Menü-Tasten eines Updates aus und hängt die Aktionen an out: Flanke beim Drücken,
    /// Wiederholung gehaltener Richtungen. pad schaltet die Kombi View + Menu ein (Menu und View erst beim
    /// Loslassen). Das erste Update eines Geräts löst nichts aus; dort schon gehaltene Tasten wirken erst nach dem
    /// Loslassen.
    pub fn step(&mut self, held: &[bool; ACTION_COUNT], pad: bool, out: &mut Vec<Action>) {
        if !self.started {
            self.started = true;
            self.prev = *held;
            self.ignore = *held;
            return;
        }
        if pad {
            self.cheat_chord(held, out);
            self.pad_chord(held, out);
        }
        for a in ALL_ACTIONS {
            if self.fires(a, held[a as usize], pad) {
                out.push(a);
            }
        }
    }

    /// Fires aktualisiert Haltedauer und Sperre der Aktion a und meldet, ob sie in diesem Update auslöst.
    fn fires(&mut self, a: Action, held: bool, pad: bool) -> bool {
        let i = a as usize;
        let pressed = held && !self.prev[i];
        self.prev[i] = held;
        if !held {
            self.hold[i] = 0;
            self.ignore[i] = false;
            return false;
        } else if pressed {
            self.hold[i] = 1;
        } else {
            self.hold[i] += 1;
        }
        if self.ignore[i] || (pad && (a == Action::Menu || a == Action::View || a == Action::Cheats)) {
            return false;
        }
        if pressed {
            return true;
        }
        let h = self.hold[i] - REPEAT_DELAY - 1;
        a <= Action::Right && h >= 0 && h % REPEAT_EVERY == 0
    }

    /// PadChord wertet View und Menu eines Gamepads aus: beide gehalten und eine gerade gedrückt = `Action::Home`;
    /// sonst Menu beim Loslassen nach kurzem Druck, View beim Loslassen. Nach der Kombi lösen beide bis zum Loslassen
    /// nichts aus.
    fn pad_chord(&mut self, held: &[bool; ACTION_COUNT], out: &mut Vec<Action>) {
        let (v, m) = (held[Action::View as usize], held[Action::Menu as usize]);
        let (pv, pm) = (self.prev[Action::View as usize], self.prev[Action::Menu as usize]);
        if home_chord(v, m, v && !pv, m && !pm) {
            out.push(Action::Home);
            self.chord = true;
        }
        if pm
            && !m
            && !self.chord
            && !self.ignore[Action::Menu as usize]
            && self.hold[Action::Menu as usize] <= MENU_SHORT
        {
            out.push(Action::Menu);
        }
        if pv && !v && !self.chord && !self.ignore[Action::View as usize] {
            out.push(Action::View);
        }
        if !v && !m {
            self.chord = false;
        }
    }

    /// CheatChord meldet View + Y als `Action::Cheats` (Cheat-Menü, B-888); View löst danach beim Loslassen nichts aus.
    fn cheat_chord(&mut self, held: &[bool; ACTION_COUNT], out: &mut Vec<Action>) {
        if held[Action::View as usize] && held[Action::Cheats as usize] && !self.prev[Action::Cheats as usize] {
            out.push(Action::Cheats);
            self.chord = true;
        }
    }
}

/// AnyKey meldet, ob eine der Tasten ks gehalten ist.
pub fn any_key(k: &KeyState, ks: &[Key]) -> bool {
    ks.iter().any(|&x| k.held(x))
}

/// KeyCommand liefert den Befehl der Belegung layout; beide Richtungen zugleich heben sich auf.
pub fn key_command(layout: i64, k: &KeyState) -> Command {
    let l = &LAYOUT_TABLE[layout as usize];
    let mut c = Command::default();
    if any_key(k, &l.left) {
        c.move_ -= 1.0;
    }
    if any_key(k, &l.right) {
        c.move_ += 1.0;
    }
    c.sprint = any_key(k, &l.sprint);
    c.pay = any_key(k, &l.pay);
    c.attack = any_key(k, &l.attack);
    for (i, ks) in l.skills.iter().enumerate() {
        if any_key(k, ks) {
            c.skill = i as i64 + 1;
            break;
        }
    }
    c
}

/// KeyHeld liefert die gehaltenen Menü-Tasten der Belegung layout.
pub fn key_held(layout: i64, k: &KeyState) -> [bool; ACTION_COUNT] {
    let mut held = [false; ACTION_COUNT];
    for (a, ks) in LAYOUT_TABLE[layout as usize].actions.iter().enumerate() {
        held[a] = any_key(k, ks);
    }
    held
}

/// HomeChord meldet „View + Menu gemeinsam“ (`Action::Home`): beide gehalten und mindestens eine in diesem Update
/// gedrückt, damit Halten nicht wiederholt auslöst.
pub fn home_chord(held_view: bool, held_menu: bool, just_view: bool, just_menu: bool) -> bool {
    held_view && held_menu && (just_view || just_menu)
}

#[cfg(test)]
#[path = "tracker_tests.rs"]
mod tracker_tests;
