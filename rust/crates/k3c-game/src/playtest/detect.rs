//! Marker-Taste erkennen: Pad Back (View) + Y, Tastatur F9 (B-1220). Die Kombination ist im Spiel sonst das Cheat-Menü
//! (`Action::Cheats`); im Spieltest geht sie nur an den Marker: `mask` nimmt Y und F9 aus der Eingabe des Spiels.

use std::collections::HashMap;

use k3c_platform::{InputState, Key, PadButton};

use crate::view::input::{DeviceID, DeviceKind};

/// Ein Auslöser: das Gerät und seine Reihenfolge (0 = Tastatur bzw. erstes Pad), falls kein Platz zugeordnet ist.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Trigger {
    pub device: DeviceID,
    pub ordinal: usize,
}

/// Flanken-Erkennung über die Takte.
#[derive(Debug, Default)]
pub struct Detector {
    pads: HashMap<u32, bool>,
    key: bool,
}

impl Detector {
    /// Liefert die Geräte, deren Marker-Taste in diesem Takt neu gedrückt wurde (Halten löst nicht erneut aus).
    pub fn poll(&mut self, input: &InputState) -> Vec<Trigger> {
        let mut out = Vec::new();
        let key = input.held(Key::F9);
        if key && !self.key {
            out.push(Trigger {
                device: DeviceID::new(DeviceKind::Keyboard, 0),
                ordinal: 0,
            });
        }
        self.key = key;
        for (i, p) in input.pads().iter().enumerate() {
            let now = p.held(PadButton::View) && p.held(PadButton::Y);
            if now && !self.pads.get(&p.id).copied().unwrap_or(false) {
                out.push(Trigger {
                    device: DeviceID::new(DeviceKind::Gamepad, i64::from(p.id)),
                    ordinal: i,
                });
            }
            self.pads.insert(p.id, now);
        }
        let ids: Vec<u32> = input.pads().iter().map(|p| p.id).collect();
        self.pads.retain(|id, _| ids.contains(id));
        out
    }
}

/// Die Eingabe für das Spiel: ohne Y (Pad) und F9, damit das Cheat-Menü im Spieltest nicht aufgeht.
pub fn mask(input: &InputState) -> InputState {
    let mut m = input.clone();
    m.keys.release(Key::F9);
    let ids: Vec<u32> = m.pads().iter().map(|p| p.id).collect();
    for id in ids {
        m.pad_mut(id).set_button(PadButton::Y, false);
    }
    m
}
