//! Cheat-Fenster `k3c-devwin`: zweites Win32-Fenster neben dem Rust-Spiel (RU56, B-1132).
//!
//! Die reine Logik (Maße, Farben, später Layout und Zustand) liegt in Modulen ohne Win32 und ist überall
//! testbar. `unsafe` und das `windows`-Crate gibt es nur im Modul `win32` (nur Windows).

pub mod style;

#[cfg(windows)]
pub mod win32;
