//! Gemeinsame Test-Helfer des Pakets input (in Go lagen sie in bindings_test.go und source_test.go).

use k3c_platform::{Key, PadButton};

use super::source::InputSource;
use super::tracker::{KeyState, PadState};
use crate::view::input::{DeviceID, DeviceKind, Input};

/// key_state liefert eine Tastatur mit den gehaltenen Tasten ks.
pub fn key_state(ks: &[Key]) -> KeyState {
    let mut k = KeyState::default();
    for &x in ks {
        k.press(x);
    }
    k
}

/// pad_state liefert ein Pad mit Standard-Belegung, den gehaltenen Tasten bs und dem Stick in Ruhe.
pub fn pad_state(id: u32, bs: &[PadButton]) -> PadState {
    let mut p = PadState {
        id,
        standard: true,
        ..Default::default()
    };
    for &b in bs {
        p.buttons[b as usize] = true;
    }
    p
}

pub const KEYS1: DeviceID = DeviceID::new(DeviceKind::Keyboard, 0);

pub fn pad_id(id: i64) -> DeviceID {
    DeviceID::new(DeviceKind::Gamepad, id)
}

pub fn device_ids(input: &Input) -> Vec<DeviceID> {
    input.devices.iter().map(|di| di.device).collect()
}

/// poll liefert die Eingabe eines Updates (Go: `s.poll(k, pads)` mit `slices.Clone(pads)`).
pub fn poll(s: &mut InputSource, k: &KeyState, pads: &[PadState]) -> Input {
    let mut v = pads.to_vec();
    Input {
        devices: s.poll_raw(k, &mut v).to_vec(),
    }
}
