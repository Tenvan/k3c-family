//! Port des Abschnitts `padRaw` von `native/k3c-game/menu/padtest.go`: der Rohzustand eines Pads, die Quelle der Pads
//! ([`PadSource`], in Go die globale Funktion `readPads`) und die Hilfen zum Formatieren.
//!
//! Abweichung: Go liest Ebitengine direkt (Gerätename, SDL-Kennung, rohe Tasten und Achsen). `k3c-platform` liefert nur
//! die Standard-Belegung; [`InputPads`] setzt darum Name `"Pad <id>"` und lässt SDL-Kennung und rohe Tasten leer (`–`).

use std::cell::RefCell;
use std::rc::Rc;

use k3c_platform::{InputState, PadAxis, PadButton, PadState};

/// Achsen darunter gelten als 0.
pub const AXIS_ZERO: f64 = 0.05;

/// Die Tasten der Standard-Belegung mit Xbox-Namen, in der Reihenfolge von Go (`padButtons`).
pub const PAD_BUTTONS: [(PadButton, &str); 17] = [
    (PadButton::A, "A"),
    (PadButton::B, "B"),
    (PadButton::X, "X"),
    (PadButton::Y, "Y"),
    (PadButton::LB, "LB"),
    (PadButton::RB, "RB"),
    (PadButton::LT, "LT"),
    (PadButton::RT, "RT"),
    (PadButton::View, "View"),
    (PadButton::Menu, "Menu"),
    (PadButton::LS, "LS"),
    (PadButton::RS, "RS"),
    (PadButton::Up, "↑"),
    (PadButton::Down, "↓"),
    (PadButton::Left, "←"),
    (PadButton::Right, "→"),
    (PadButton::Home, "Guide"),
];

/// PadRaw ist der Rohzustand eines Pads: Gerät, Standard-Tasten, Achsen und rohe Tasten.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PadRaw {
    pub id: i64,
    pub name: String,
    pub sdl: String,
    pub standard: bool,
    /// gedrückte Tasten der Standard-Belegung, Xbox-Namen
    pub buttons: String,
    /// Sticks der Standard-Belegung bzw. rohe Achsen
    pub axes: String,
    /// rohe Tasten (Nummern)
    pub raw: String,
}

/// PadSource liefert die verbundenen Pads dieses Updates (Go: `readPads`; Tests setzen eine Schein-Quelle).
pub trait PadSource {
    fn read(&self) -> Vec<PadRaw>;
}

/// NoPads ist die Standard-Quelle ohne Pads.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoPads;

impl PadSource for NoPads {
    fn read(&self) -> Vec<PadRaw> {
        Vec::new()
    }
}

/// InputPads liefert die Pads aus dem Eingabe-Schnappschuss der Plattform. Die Schleife behält einen Klon (`Clone`
/// teilt den Schnappschuss) und ruft je Takt [`InputPads::snapshot`].
#[derive(Clone, Debug, Default)]
pub struct InputPads {
    snap: Rc<RefCell<Vec<PadRaw>>>,
}

impl InputPads {
    pub fn new() -> Self {
        Self::default()
    }

    /// Übernimmt die Pads des Takts aus `input`.
    pub fn snapshot(&self, input: &InputState) {
        *self.snap.borrow_mut() = input.pads().iter().map(pad_raw).collect();
    }
}

impl PadSource for InputPads {
    fn read(&self) -> Vec<PadRaw> {
        self.snap.borrow().clone()
    }
}

/// Baut den Rohzustand eines Pads der Plattform.
fn pad_raw(p: &PadState) -> PadRaw {
    let down: Vec<String> = PAD_BUTTONS
        .iter()
        .filter(|(b, _)| p.held(*b))
        .map(|(_, n)| n.to_string())
        .collect();
    let a = |x: PadAxis| axis(p.axis(x));
    PadRaw {
        id: i64::from(p.id),
        name: format!("Pad {}", p.id),
        sdl: join_or(&[], " "),
        standard: true,
        buttons: join_or(&down, " "),
        axes: format!(
            "LS {} {} · RS {} {}",
            a(PadAxis::LX),
            a(PadAxis::LY),
            a(PadAxis::RX),
            a(PadAxis::RY)
        ),
        raw: join_or(&[], " "),
    }
}

/// Formatiert einen Achsenwert (Totzone 0).
pub fn axis(v: f64) -> String {
    if v > -AXIS_ZERO && v < AXIS_ZERO {
        return "0".to_string();
    }
    format!("{v:.2}")
}

/// Verbindet `s` mit `sep`; leer ergibt "–".
pub fn join_or(s: &[String], sep: &str) -> String {
    if s.is_empty() {
        return "–".to_string();
    }
    s.join(sep)
}

/// Kürzt `s` auf `n` Zeichen (nicht Bytes: Umlaute).
pub fn clip(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}
