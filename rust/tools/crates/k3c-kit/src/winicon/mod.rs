//! Fenster-Icon zur Laufzeit (`workkit/winicon`): ein Dev-Lauf hat keine Icon-Ressource in der EXE, das Fenster bliebe
//! beim Standard-Icon. Deshalb setzt der Start das Icon der eigenen sichtbaren Fenster selbst (`WM_SETICON`) aus dem
//! eingebetteten PNG. Außerhalb von Windows tun beide Funktionen nichts.

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::{handle, set};

/// Tut außerhalb von Windows nichts.
#[cfg(not(windows))]
pub fn set(_png: &[u8]) {}

/// Icon-Handles gibt es nur unter Windows.
#[cfg(not(windows))]
pub fn handle(_png: &[u8], _px: i32) -> isize {
    0
}

#[cfg(all(test, windows))]
mod winicon_tests;
