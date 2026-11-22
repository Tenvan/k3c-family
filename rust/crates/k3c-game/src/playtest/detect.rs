//! Marker-Taste erkennen: Pad Back (View) + Klick des linken Sticks (LS), Tastatur F (B-1220). Beide sind sonst nirgends
//! belegt; das Cheat-Menü (View + Y, F9) bleibt unberührt, das Spiel bekommt die Eingabe unverändert.

use std::collections::HashMap;

use k3c_platform::{InputState, Key, PadButton};

/// Marker-Taste der Tastatur; gleich `input::bindings::MARKER_KEY` (die Fachmodule kennen einander nicht, Importregel).
pub const MARKER_KEY: Key = Key::F;
/// Marker-Taste des Pads mit View: Klick des linken Sticks; gleich `input::bindings::MARKER_PAD`.
pub const MARKER_PAD: PadButton = PadButton::LS;

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
        let key = input.held(MARKER_KEY);
        if key && !self.key {
            out.push(Trigger {
                device: DeviceID::new(DeviceKind::Keyboard, 0),
                ordinal: 0,
            });
        }
        self.key = key;
        for (i, p) in input.pads().iter().enumerate() {
            let now = p.held(PadButton::View) && p.held(MARKER_PAD);
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
