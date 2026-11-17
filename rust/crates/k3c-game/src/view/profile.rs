//! Port von `native/k3c-game/view/profile.go`.
//!
//! Eingabe-Profile (B-893): Jeder Spieler wählt in Optionen › Steuerung sein Gerät, Tastatur mit WASD, Tastatur mit
//! Pfeiltasten oder Gamepad. Die Tastatur ist ein Gerät (DeviceID{DeviceKeyboard, 0}); ihre Belegung folgt dem
//! Tastatur-Profil (KeyboardProfile). Höchstens ein Spieler hat ein Tastatur-Profil. Die Spielernummer folgt weiter dem
//! Beitritt in der Lobby; das Profil bestimmt die Belegung und die Tasten-Hinweise des Geräts (ProfileOf).

use super::MAX_PLAYERS;
use super::input::{DeviceID, DeviceKind};
use super::settings::Settings;

// Werte von Settings.Controls.
/// Tastatur, Bewegung mit A/D
pub const PROFILE_WASD: &str = "wasd";
/// Tastatur, Bewegung mit ←/→
pub const PROFILE_ARROWS: &str = "pfeile";
/// Gamepad (Xbox-Belegung)
pub const PROFILE_GAMEPAD: &str = "gamepad";

/// Profiles sind die Profile in der Reihenfolge der Auswahl.
pub const PROFILES: [&str; 3] = [PROFILE_WASD, PROFILE_ARROWS, PROFILE_GAMEPAD];

/// IsKeyboardProfile meldet, ob p ein Tastatur-Profil ist.
pub fn is_keyboard_profile(p: &str) -> bool {
    p == PROFILE_WASD || p == PROFILE_ARROWS
}

/// Steuerung je Spieler (`Settings.Controls`).
pub type Controls = [String; MAX_PLAYERS];

/// defaultControls: Spieler 1 Tastatur (WASD), Spieler 2 … 4 Gamepad.
pub(crate) fn default_controls() -> Controls {
    let mut c: Controls = std::array::from_fn(|_| PROFILE_GAMEPAD.to_string());
    c[0] = PROFILE_WASD.to_string();
    c
}

/// normalizeControls setzt unbekannte Werte auf den Standard des Platzes und jedes weitere Tastatur-Profil auf Gamepad.
pub(crate) fn normalize_controls(mut c: Controls) -> Controls {
    let def = default_controls();
    let mut keyboard = false;
    for (i, slot) in c.iter_mut().enumerate() {
        let mut p = slot.as_str();
        if p != PROFILE_WASD && p != PROFILE_ARROWS && p != PROFILE_GAMEPAD {
            p = &def[i];
            if is_keyboard_profile(p) && keyboard {
                p = PROFILE_GAMEPAD;
            }
        } else if is_keyboard_profile(p) && keyboard {
            p = PROFILE_GAMEPAD;
        }
        keyboard = keyboard || is_keyboard_profile(p);
        *slot = p.to_string();
    }
    c
}

impl Settings {
    /// KeyboardProfile ist die Belegung der Tastatur: das Tastatur-Profil eines Spielers, ohne eins ProfileWASD (die
    /// Tastatur bleibt für Menüs und Beitritt bedienbar).
    pub fn keyboard_profile(&self) -> &str {
        for p in &self.controls {
            if is_keyboard_profile(p) {
                return p;
            }
        }
        PROFILE_WASD
    }

    /// ProfileOf ist das Profil des Geräts d: Gamepad für Pads, sonst das Tastatur-Profil (KeyboardProfile).
    pub fn profile_of(&self, d: DeviceID) -> &str {
        if d.kind == DeviceKind::Gamepad {
            return PROFILE_GAMEPAD;
        }
        self.keyboard_profile()
    }

    /// NextProfile liefert das Profil, das Spieler player beim Blättern um dir (±1, im Kreis) bekommt. Tastatur-Profile
    /// überspringt es, solange ein anderer Spieler eins hat (höchstens ein Tastatur-Spieler, B-893).
    pub fn next_profile(&self, player: i64, dir: i64) -> &'static str {
        if player < 0 || player >= MAX_PLAYERS as i64 {
            return PROFILE_GAMEPAD;
        }
        let mut taken = false;
        for (i, p) in self.controls.iter().enumerate() {
            taken = taken || (i as i64 != player && is_keyboard_profile(p));
        }
        let n = PROFILES.len() as i64;
        let mut cur = 0;
        for (i, p) in PROFILES.iter().enumerate() {
            if *p == self.controls[player as usize] {
                cur = i as i64;
            }
        }
        for step in 1..=n {
            let next = PROFILES[(((cur + step * dir) % n + n) % n) as usize];
            if !taken || !is_keyboard_profile(next) {
                return next;
            }
        }
        PROFILE_GAMEPAD
    }
}

#[cfg(test)]
#[path = "profile_tests.rs"]
mod profile_tests;
