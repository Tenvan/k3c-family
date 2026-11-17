//! Port von `native/k3c-game/input/source.go`.
//!
//! Eingabe-Quelle: liest den Rohzustand aus dem Plattform-Schnappschuss (`Source::poll`) und wertet ihn rein aus
//! (`poll_raw`). Geräte kommen und gehen: Ein Pad hat seinen tracker, solange seine Pad-ID verbunden ist; eine neue ID
//! ist ein neues Gerät.

use k3c_platform::{InputState, PadAxis, PadButton};

use super::bindings::{ACTION_COUNT, MENU_KEYS, layout};
use super::tracker::{KeyState, PadState, Tracker, any_key, key_command, key_held};
use crate::view::input::{DeviceID, DeviceInput, DeviceKind, Input, KeyboardSource, Source};

/// PadSlot ist der Zustand eines verbundenen Gamepads.
pub struct PadSlot {
    pub id: u32,
    pub t: Tracker,
}

/// InputSource hält Rohzustand, tracker und Puffer; `poll_raw` legt nach dem ersten Aufruf nichts mehr an (in Go
/// `source`).
#[derive(Default)]
pub struct InputSource {
    pub keys: KeyState,
    pub raw: Vec<PadState>,
    /// Tastatur (ein Gerät, Belegung layout)
    pub kb: Tracker,
    /// Index in LAYOUT_TABLE (set_keyboard)
    pub layout: i64,
    /// ein Menü ist offen: MENU_KEYS gelten zusätzlich (set_menu)
    pub menu: bool,
    pub pads: Vec<PadSlot>,
    /// Pads ohne Standard-Belegung, schon gemeldet
    pub warned: Vec<u32>,
    pub buf: Vec<DeviceInput>,
    pub n: usize,
}

/// New liefert die Eingabe-Quelle (in Go mit `*art.Library`, Eingabe zeichnet nichts und braucht sie nicht). Die
/// Quelle erfüllt `Source` und `KeyboardSource` (Belegung der Tastatur).
pub fn new() -> InputSource {
    InputSource::default()
}

impl KeyboardSource for InputSource {
    /// SetKeyboard stellt die Belegung der Tastatur auf das Tastatur-Profil p (`PROFILE_WASD` oder `PROFILE_ARROWS`,
    /// B-893). Ein Wechsel gilt ab dem nächsten Poll; schon gehaltene Tasten lösen erst nach dem Loslassen aus.
    fn set_keyboard(&mut self, p: &str) {
        let l = layout(p);
        if l != self.layout {
            self.layout = l;
            self.kb = Tracker::default();
        }
    }

    /// SetMenu meldet, ob ein Menü offen ist: dann gelten Pfeile, Enter, Leertaste und Esc unabhängig vom Profil
    /// (B-945). Beim Wechsel Menü ↔ Spiel beginnen alle Geräte neu: gehaltene Menü-Tasten lösen nichts aus und
    /// gehaltenes Zahlen oder Schlagen zählt erst nach dem Loslassen (sonst bestätigt Enter im Menü und zahlt danach
    /// im Spiel).
    fn set_menu(&mut self, open: bool) {
        if open == self.menu {
            return;
        }
        self.menu = open;
        self.kb = quiet_tracker();
        for ps in &mut self.pads {
            ps.t = quiet_tracker();
        }
    }
}

fn quiet_tracker() -> Tracker {
    Tracker {
        quiet: true,
        ..Default::default()
    }
}

impl Source for InputSource {
    /// Poll liest Tastatur und Gamepads aus dem Plattform-Schnappschuss (in Go aus Ebitengine). Das Ergebnis gilt bis
    /// zum nächsten Poll.
    fn poll(&mut self, raw: &InputState) -> Input {
        self.keys = raw.keys;
        self.raw.clear();
        for p in raw.pads() {
            let mut ps = PadState::default();
            read_pad(p, &mut ps);
            self.raw.push(ps);
        }
        let keys = self.keys;
        let mut pads = std::mem::take(&mut self.raw);
        let devices = self.poll_raw(&keys, &mut pads).to_vec();
        self.raw = pads;
        Input { devices }
    }
}

/// ReadPad liest den Rohzustand des Gamepads p nach ps. Das Plattform-Pad hat immer Standard-Belegung (Go:
/// `IsStandardGamepadLayoutAvailable`); der Zweig ohne sie bleibt erhalten, greift aber nie.
fn read_pad(p: &k3c_platform::PadState, ps: &mut PadState) {
    *ps = PadState {
        id: p.id,
        standard: true,
        ..Default::default()
    };
    if !ps.standard {
        return;
    }
    for b in PadButton::ALL {
        ps.buttons[b as usize] = p.held(b);
    }
    ps.x = p.axis(PadAxis::LX);
    ps.y = p.axis(PadAxis::LY);
}

impl InputSource {
    /// poll_raw (in Go `poll`) wertet den Rohzustand eines Updates aus: die Tastatur mit der Belegung des Profils,
    /// dann die Gamepads mit Standard-Belegung nach ID (pads wird dafür sortiert). Pads ohne Standard-Belegung meldet
    /// es einmal im Log und lässt sie weg.
    pub fn poll_raw(&mut self, k: &KeyState, pads: &mut [PadState]) -> &[DeviceInput] {
        self.n = 0;
        let kb = self.slot(DeviceID::new(DeviceKind::Keyboard, 0));
        self.buf[kb].command = key_command(self.layout, k);
        self.kb.mute(&mut self.buf[kb].command);
        let mut held = key_held(self.layout, k);
        if self.menu {
            for a in 0..ACTION_COUNT {
                held[a] = held[a] || any_key(k, &MENU_KEYS[a]);
            }
        }
        self.kb.step(&held, false, &mut self.buf[kb].actions);
        pads.sort_unstable_by_key(|p| p.id);
        self.forget(pads);
        for p in pads.iter() {
            if !p.standard {
                self.warn(p);
                continue;
            }
            let ti = self.tracker(p.id);
            let bi = self.slot(DeviceID::new(DeviceKind::Gamepad, i64::from(p.id)));
            let t = &mut self.pads[ti].t;
            let di = &mut self.buf[bi];
            di.command = t.pad_command(p);
            t.mute(&mut di.command);
            let held = t.pad_held(p);
            t.step(&held, true, &mut di.actions);
        }
        &self.buf[..self.n]
    }

    /// Forget entfernt die tracker und Meldungen getrennter Pads (Reihenfolge bleibt, Speicher wird
    /// wiederverwendet).
    fn forget(&mut self, pads: &[PadState]) {
        let gone = |id: u32| !pads.iter().any(|p| p.id == id);
        self.pads.retain(|ps| !gone(ps.id));
        self.warned.retain(|&id| !gone(id));
    }

    /// Tracker liefert den Index des tracker des Pads id; ein neues Pad bekommt einen frischen (erstes Update ohne
    /// Aktionen).
    fn tracker(&mut self, id: u32) -> usize {
        if let Some(i) = self.pads.iter().position(|ps| ps.id == id) {
            return i;
        }
        self.pads.push(PadSlot {
            id,
            t: Tracker::default(),
        });
        self.pads.len() - 1
    }

    /// Warn meldet ein Pad ohne Standard-Belegung einmal, solange es verbunden ist.
    fn warn(&mut self, p: &PadState) {
        if self.warned.contains(&p.id) {
            return;
        }
        self.warned.push(p.id);
        k3c_platform::log::warn(
            "🚫 Gamepad ohne Standard-Belegung ignoriert",
            &[("pad", &p.id), ("name", &p.name)],
        );
    }

    /// Slot liefert den Index des nächsten Eintrags des Puffers für Gerät d, geleert; die Aktionen behalten ihren
    /// Speicher.
    fn slot(&mut self, d: DeviceID) -> usize {
        if self.n == self.buf.len() {
            self.buf.push(DeviceInput::default());
        }
        let i = self.n;
        self.n += 1;
        let di = &mut self.buf[i];
        di.device = d;
        di.command = Default::default();
        di.actions.clear();
        i
    }
}

#[cfg(test)]
#[path = "source_tests.rs"]
mod source_tests;
