//! Gamepads über gilrs: Ereignisse je Bild abholen und in [`InputState`] pflegen. Einziger Ort mit `gilrs`.
//!
//! Jede neue Verbindung bekommt eine neue `id` (auch dasselbe Pad nach dem Wiederverbinden), wie eine neue
//! `ebiten.GamepadID`. Belegung nur mit Xbox-Namen; Trigger sind Achsen 0 … 1 und gelten als Taste über der
//! Schwelle von Ebitengine (`gamepaddb.ButtonPressedThreshold`).

use gilrs::{Axis, Button, EventType, GamepadId, Gilrs};

use crate::input::clamp_axis;
use crate::log;
use crate::{InputState, PadAxis, PadButton};

/// Ab diesem Wert (echt größer) gilt ein Trigger als gedrückt (Ebitengine: 30/255, Totzone von XInput).
pub const TRIGGER_PRESSED: f64 = 30.0 / 255.0;
/// Steuerkreuz als Achse (manche Treiber): ab diesem Betrag gedrückt.
const DPAD_PRESSED: f64 = 0.5;

/// Verbundene Gamepads und ihre Spiel-IDs.
pub struct Pads {
    gilrs: Option<Gilrs>,
    ids: Vec<(GamepadId, u32)>,
    next: u32,
    /// Die beim Start schon verbundenen Pads sind übernommen.
    started: bool,
}

impl Pads {
    /// Öffnet gilrs; ohne Gamepad-Unterstützung läuft alles ohne Pads weiter (Warnung).
    pub fn open() -> Self {
        let gilrs = match Gilrs::new() {
            Ok(g) => Some(g),
            Err(gilrs::Error::NotImplemented(g)) => {
                log::warn("🚫 Gamepads auf diesem System nicht unterstützt", &[]);
                Some(g)
            }
            Err(e) => {
                log::warn("🚫 Gamepads nicht verfügbar", &[("grund", &e)]);
                None
            }
        };
        Self {
            gilrs,
            ids: Vec::new(),
            next: 0,
            started: false,
        }
    }

    /// Holt alle Ereignisse seit dem letzten Bild ab; beim ersten Aufruf auch die schon verbundenen Pads.
    pub fn poll(&mut self, input: &mut InputState) {
        let Some(g) = self.gilrs.as_mut() else {
            return;
        };
        if !self.started {
            self.started = true;
            let present: Vec<GamepadId> = g
                .gamepads()
                .filter(|(_, p)| p.is_connected())
                .map(|(id, _)| id)
                .collect();
            for id in present {
                connect(&mut self.ids, &mut self.next, input, id);
            }
        }
        while let Some(ev) = g.next_event() {
            if matches!(ev.event, EventType::Connected) {
                connect(&mut self.ids, &mut self.next, input, ev.id);
                continue;
            }
            let Some(pos) = self.ids.iter().position(|(gid, _)| *gid == ev.id) else {
                continue;
            };
            let id = self.ids[pos].1;
            if matches!(ev.event, EventType::Disconnected) {
                disconnect(&mut self.ids, input, pos);
            } else if let Some(e) = PadEv::from_gilrs(ev.event) {
                apply(input, id, e);
            }
        }
    }
}

/// Verbindet das Gerät `gid` mit der nächsten Spiel-ID; schon bekannte Geräte bleiben, wie sie sind.
fn connect<G: PartialEq>(ids: &mut Vec<(G, u32)>, next: &mut u32, input: &mut InputState, gid: G) {
    if ids.iter().any(|(g, _)| *g == gid) {
        return;
    }
    let id = *next;
    *next += 1;
    ids.push((gid, id));
    input.pad_mut(id);
}

fn disconnect<G>(ids: &mut Vec<(G, u32)>, input: &mut InputState, pos: usize) {
    let (_, id) = ids.swap_remove(pos);
    input.disconnect(id);
}

/// Eingabe-Ereignis eines Pads ohne gilrs-Kennung (testbar ohne Gerät).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PadEv {
    Button(Button, bool),
    /// Analoger Wert einer Taste (Trigger) in 0 … 1.
    Value(Button, f32),
    Axis(Axis, f32),
}

impl PadEv {
    pub fn from_gilrs(ev: EventType) -> Option<Self> {
        Some(match ev {
            EventType::ButtonPressed(b, _) | EventType::ButtonRepeated(b, _) => Self::Button(b, true),
            EventType::ButtonReleased(b, _) => Self::Button(b, false),
            EventType::ButtonChanged(b, v, _) => Self::Value(b, v),
            EventType::AxisChanged(a, v, _) => Self::Axis(a, v),
            _ => return None,
        })
    }
}

/// Ein Ereignis auf das Pad `id` anwenden.
pub fn apply(input: &mut InputState, id: u32, ev: PadEv) {
    let pad = input.pad_mut(id);
    let analog = |b: PadButton| matches!(b, PadButton::LT | PadButton::RT);
    match ev {
        PadEv::Value(b @ (Button::LeftTrigger2 | Button::RightTrigger2), v) => {
            let (axis, button) = if b == Button::LeftTrigger2 {
                (PadAxis::LT, PadButton::LT)
            } else {
                (PadAxis::RT, PadButton::RT)
            };
            let v = clamp_axis(axis, f64::from(v));
            pad.set_axis(axis, v);
            pad.set_button(button, v > TRIGGER_PRESSED);
        }
        PadEv::Button(b, down) => {
            if let Some(pb) = map_button(b).filter(|p| !analog(*p)) {
                pad.set_button(pb, down);
            }
        }
        PadEv::Axis(a, v) => apply_axis(pad, a, f64::from(v)),
        PadEv::Value(..) => {}
    }
}

fn apply_axis(pad: &mut crate::PadState, a: Axis, v: f64) {
    match map_axis(a) {
        // gilrs zählt Y nach oben positiv, das Spiel (wie Ebitengine) nach unten.
        Some(ax @ (PadAxis::LY | PadAxis::RY)) => pad.set_axis(ax, clamp_axis(ax, -v)),
        Some(ax @ (PadAxis::LT | PadAxis::RT)) => {
            let v = clamp_axis(ax, v);
            pad.set_axis(ax, v);
            let b = if ax == PadAxis::LT {
                PadButton::LT
            } else {
                PadButton::RT
            };
            pad.set_button(b, v > TRIGGER_PRESSED);
        }
        Some(ax) => pad.set_axis(ax, clamp_axis(ax, v)),
        None => match a {
            Axis::DPadX => {
                pad.set_button(PadButton::Left, v < -DPAD_PRESSED);
                pad.set_button(PadButton::Right, v > DPAD_PRESSED);
            }
            Axis::DPadY => {
                pad.set_button(PadButton::Up, v > DPAD_PRESSED);
                pad.set_button(PadButton::Down, v < -DPAD_PRESSED);
            }
            _ => {}
        },
    }
}

/// gilrs-Taste → Xbox-Name (Positionen wie das Standard-Layout von Ebitengine).
pub fn map_button(b: Button) -> Option<PadButton> {
    Some(match b {
        Button::South => PadButton::A,
        Button::East => PadButton::B,
        Button::West => PadButton::X,
        Button::North => PadButton::Y,
        Button::LeftTrigger => PadButton::LB,
        Button::RightTrigger => PadButton::RB,
        Button::LeftTrigger2 => PadButton::LT,
        Button::RightTrigger2 => PadButton::RT,
        Button::Select => PadButton::View,
        Button::Start => PadButton::Menu,
        Button::Mode => PadButton::Home,
        Button::LeftThumb => PadButton::LS,
        Button::RightThumb => PadButton::RS,
        Button::DPadUp => PadButton::Up,
        Button::DPadDown => PadButton::Down,
        Button::DPadLeft => PadButton::Left,
        Button::DPadRight => PadButton::Right,
        _ => return None,
    })
}

/// gilrs-Achse → Achse des Spiels; Trigger als `LeftZ`/`RightZ` bei manchen Treibern.
pub fn map_axis(a: Axis) -> Option<PadAxis> {
    Some(match a {
        Axis::LeftStickX => PadAxis::LX,
        Axis::LeftStickY => PadAxis::LY,
        Axis::RightStickX => PadAxis::RX,
        Axis::RightStickY => PadAxis::RY,
        Axis::LeftZ => PadAxis::LT,
        Axis::RightZ => PadAxis::RT,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_BUTTONS: [Button; 20] = [
        Button::South,
        Button::East,
        Button::North,
        Button::West,
        Button::C,
        Button::Z,
        Button::LeftTrigger,
        Button::LeftTrigger2,
        Button::RightTrigger,
        Button::RightTrigger2,
        Button::Select,
        Button::Start,
        Button::Mode,
        Button::LeftThumb,
        Button::RightThumb,
        Button::DPadUp,
        Button::DPadDown,
        Button::DPadLeft,
        Button::DPadRight,
        Button::Unknown,
    ];

    #[test]
    fn jede_xbox_taste_hat_genau_eine_quelle() {
        for pb in PadButton::ALL {
            let n = ALL_BUTTONS.iter().filter(|b| map_button(**b) == Some(pb)).count();
            assert_eq!(n, 1, "{pb:?} hat {n} Quellen");
        }
    }

    #[test]
    fn jede_achse_hat_genau_eine_quelle() {
        let all = [
            Axis::LeftStickX,
            Axis::LeftStickY,
            Axis::LeftZ,
            Axis::RightStickX,
            Axis::RightStickY,
            Axis::RightZ,
            Axis::DPadX,
            Axis::DPadY,
            Axis::Unknown,
        ];
        for pa in PadAxis::ALL {
            assert_eq!(all.iter().filter(|a| map_axis(**a) == Some(pa)).count(), 1, "{pa:?}");
        }
    }

    #[test]
    fn neue_verbindung_neue_id_auch_beim_selben_pad() {
        let (mut ids, mut next, mut i) = (Vec::new(), 0, InputState::new());
        connect(&mut ids, &mut next, &mut i, 7u8);
        connect(&mut ids, &mut next, &mut i, 7u8);
        connect(&mut ids, &mut next, &mut i, 9u8);
        assert_eq!(
            ids,
            [(7, 0), (9, 1)],
            "erstes Pad hat id 0, doppelte Meldung zählt nicht"
        );
        disconnect(&mut ids, &mut i, 0);
        assert!(i.pad(0).is_none() && i.pad(1).is_some());
        connect(&mut ids, &mut next, &mut i, 7u8);
        assert_eq!(ids, [(9, 1), (7, 2)], "wieder verbunden: neue id");
        assert_eq!(i.pads().iter().map(|p| p.id).collect::<Vec<_>>(), [1, 2]);
    }

    #[test]
    fn trigger_ist_achse_und_ab_schwelle_taste() {
        let mut i = InputState::new();
        apply(&mut i, 0, PadEv::Value(Button::RightTrigger2, 0.1));
        let p = i.pad(0).unwrap();
        assert!((p.axis(PadAxis::RT) - 0.1).abs() < 1e-6);
        assert!(!p.held(PadButton::RT), "unter 30/255");
        apply(&mut i, 0, PadEv::Value(Button::RightTrigger2, 0.2));
        assert!(i.pad(0).unwrap().held(PadButton::RT));
    }

    #[test]
    fn stick_y_nach_unten_positiv_und_tasten() {
        let mut i = InputState::new();
        apply(&mut i, 3, PadEv::Axis(Axis::LeftStickY, 0.8));
        apply(&mut i, 3, PadEv::Button(Button::East, true));
        let p = i.pad(3).unwrap();
        assert!((p.axis(PadAxis::LY) + 0.8).abs() < 1e-6, "oben ist negativ");
        assert!(p.held(PadButton::B));
        apply(&mut i, 3, PadEv::Button(Button::East, false));
        apply(&mut i, 3, PadEv::Axis(Axis::DPadX, -1.0));
        let p = i.pad(3).unwrap();
        assert!(!p.held(PadButton::B) && p.held(PadButton::Left));
    }
}
