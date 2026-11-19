//! Eingabe-Schnappschuss je Takt: Tastatur (gehalten + Flanken) und Gamepads (Xbox-Namen), dazu das
//! Eingabe-Skript für Läufe ohne Fenster. Kein winit- oder Gamepad-Typ hier; das Backend füllt den Zustand.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

macro_rules! key_enum {
    ($($k:ident),* $(,)?) => {
        /// Alle Tasten, die das Spiel nutzt (`native/k3c-game`: `ebiten.Key*`), Namen ohne `Key`-Präfix.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum Key { $($k),* }

        impl Key {
            /// Jede Taste genau einmal, in Deklarationsreihenfolge.
            pub const ALL: &'static [Key] = &[$(Key::$k),*];
        }
    };
}

key_enum!(
    A,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    Backspace,
    ControlRight,
    D,
    Delete,
    E,
    End,
    Enter,
    Escape,
    F,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Home,
    Insert,
    K,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    NumpadEnter,
    PageDown,
    PageUp,
    Q,
    R,
    S,
    ShiftLeft,
    ShiftRight,
    Space,
    T,
    Tab,
    W,
    Y,
    Z,
);

impl Key {
    const fn bit(self) -> u64 {
        1 << self as u32
    }
}

/// Gamepad-Tasten, nur mit Xbox-Namen (Regel: alle Pads zeigen Xbox-Symbole).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PadButton {
    A,
    B,
    X,
    Y,
    LB,
    RB,
    LT,
    RT,
    View,
    Menu,
    Home,
    LS,
    RS,
    Up,
    Down,
    Left,
    Right,
}

impl PadButton {
    pub const ALL: [PadButton; 17] = [
        Self::A,
        Self::B,
        Self::X,
        Self::Y,
        Self::LB,
        Self::RB,
        Self::LT,
        Self::RT,
        Self::View,
        Self::Menu,
        Self::Home,
        Self::LS,
        Self::RS,
        Self::Up,
        Self::Down,
        Self::Left,
        Self::Right,
    ];

    const fn bit(self) -> u32 {
        1 << self as u32
    }
}

/// Gamepad-Achsen: Sticks in [-1, 1] (rechts und unten positiv), Trigger in [0, 1].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PadAxis {
    LX,
    LY,
    RX,
    RY,
    LT,
    RT,
}

impl PadAxis {
    pub const ALL: [PadAxis; 6] = [Self::LX, Self::LY, Self::RX, Self::RY, Self::LT, Self::RT];
}

/// Tastatur: gehaltene Tasten und Flanken seit dem letzten Takt (Bitfelder, keine Allokation).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KeyState {
    held: u64,
    pressed: u64,
    released: u64,
}

impl KeyState {
    pub fn held(&self, k: Key) -> bool {
        self.held & k.bit() != 0
    }

    /// Seit dem letzten Takt gedrückt; auch, wenn die Taste schon wieder los ist (kurzer Tipp).
    pub fn just_pressed(&self, k: Key) -> bool {
        self.pressed & k.bit() != 0
    }

    pub fn just_released(&self, k: Key) -> bool {
        self.released & k.bit() != 0
    }

    pub fn press(&mut self, k: Key) {
        if !self.held(k) {
            self.pressed |= k.bit();
        }
        self.held |= k.bit();
    }

    pub fn release(&mut self, k: Key) {
        if self.held(k) {
            self.released |= k.bit();
        }
        self.held &= !k.bit();
    }

    /// Lässt alle Tasten los (Fokus verloren).
    pub fn release_all(&mut self) {
        self.released |= self.held;
        self.held = 0;
    }

    /// Setzt den gehaltenen Zustand als Ganzes; die Flanken ergeben sich aus dem Unterschied.
    fn set_held(&mut self, held: u64) {
        self.pressed |= held & !self.held;
        self.released |= self.held & !held;
        self.held = held;
    }

    fn clear_edges(&mut self) {
        self.pressed = 0;
        self.released = 0;
    }
}

/// Ein verbundenes Gamepad.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PadState {
    /// Neu je Verbindung (wie `ebiten.GamepadID`).
    pub id: u32,
    held: u32,
    pressed: u32,
    axes: [f64; 6],
}

impl PadState {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            held: 0,
            pressed: 0,
            axes: [0.0; 6],
        }
    }

    pub fn held(&self, b: PadButton) -> bool {
        self.held & b.bit() != 0
    }

    /// Seit dem letzten Takt gedrückt (auch bei kurzem Tipp).
    pub fn just_pressed(&self, b: PadButton) -> bool {
        self.pressed & b.bit() != 0
    }

    pub fn axis(&self, a: PadAxis) -> f64 {
        self.axes[a as usize]
    }

    pub fn set_button(&mut self, b: PadButton, down: bool) {
        if down && !self.held(b) {
            self.pressed |= b.bit();
        }
        if down {
            self.held |= b.bit();
        } else {
            self.held &= !b.bit();
        }
    }

    pub fn set_axis(&mut self, a: PadAxis, v: f64) {
        self.axes[a as usize] = v;
    }
}

/// Eingabe eines Takts. Wird über die ganze Laufzeit wiederverwendet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InputState {
    pub keys: KeyState,
    pads: Vec<PadState>,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn held(&self, k: Key) -> bool {
        self.keys.held(k)
    }

    pub fn just_pressed(&self, k: Key) -> bool {
        self.keys.just_pressed(k)
    }

    /// Verbundene Gamepads in Verbindungsreihenfolge.
    pub fn pads(&self) -> &[PadState] {
        &self.pads
    }

    pub fn pad(&self, id: u32) -> Option<&PadState> {
        self.pads.iter().find(|p| p.id == id)
    }

    /// Liefert das Pad `id`; verbindet es, falls es fehlt.
    pub fn pad_mut(&mut self, id: u32) -> &mut PadState {
        let i = match self.pads.iter().position(|p| p.id == id) {
            Some(i) => i,
            None => {
                self.pads.push(PadState::new(id));
                self.pads.len() - 1
            }
        };
        &mut self.pads[i]
    }

    pub fn disconnect(&mut self, id: u32) {
        self.pads.retain(|p| p.id != id);
    }

    /// Löscht die Flanken; die Schleife ruft das nach jedem Takt.
    pub fn end_step(&mut self) {
        self.keys.clear_edges();
        for p in &mut self.pads {
            p.pressed = 0;
        }
    }
}

/// Ein Eintrag des Eingabe-Skripts; er gilt ab `step` bis zum nächsten Eintrag.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptEntry {
    pub step: u64,
    #[serde(default)]
    pub keys: Vec<Key>,
    #[serde(default)]
    pub pads: Vec<ScriptPad>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptPad {
    pub id: u32,
    #[serde(default)]
    pub buttons: Vec<PadButton>,
    #[serde(default)]
    pub axes: BTreeMap<PadAxis, f64>,
}

/// Eingabe-Skript (JSON-Liste aus [`ScriptEntry`]) für Läufe ohne Fenster.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InputScript {
    entries: Vec<ScriptEntry>,
}

impl InputScript {
    pub fn new(mut entries: Vec<ScriptEntry>) -> Self {
        entries.sort_by_key(|e| e.step);
        Self { entries }
    }

    pub fn from_json(s: &str) -> Result<Self, String> {
        serde_json::from_str::<Vec<ScriptEntry>>(s)
            .map(Self::new)
            .map_err(|e| format!("Eingabe-Skript: {e}"))
    }

    /// Index des Eintrags, der in Takt `step` gilt.
    fn index_at(&self, step: u64) -> Option<usize> {
        self.entries.partition_point(|e| e.step <= step).checked_sub(1)
    }

    /// Setzt `input` auf den Eintrag von Takt `step`, wenn dort einer beginnt (sonst bleibt der Zustand).
    pub fn apply(&self, step: u64, input: &mut InputState) {
        let Some(e) = self.index_at(step).map(|i| &self.entries[i]) else {
            return;
        };
        if e.step != step {
            return;
        }
        input.keys.set_held(e.keys.iter().fold(0, |m, k| m | k.bit()));
        input.pads.retain(|p| e.pads.iter().any(|s| s.id == p.id));
        for sp in &e.pads {
            let pad = input.pad_mut(sp.id);
            for b in PadButton::ALL {
                pad.set_button(b, sp.buttons.contains(&b));
            }
            for a in PadAxis::ALL {
                pad.set_axis(a, clamp_axis(a, sp.axes.get(&a).copied().unwrap_or(0.0)));
            }
        }
    }
}

/// Begrenzt einen Achsenwert auf seinen Bereich (Sticks [-1, 1], Trigger [0, 1]); NaN wird 0.
pub fn clamp_axis(a: PadAxis, v: f64) -> f64 {
    if v.is_nan() {
        return 0.0;
    }
    match a {
        PadAxis::LT | PadAxis::RT => v.clamp(0.0, 1.0),
        _ => v.clamp(-1.0, 1.0),
    }
}

#[cfg(test)]
#[path = "input_tests.rs"]
mod tests;
