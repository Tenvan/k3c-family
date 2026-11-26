//! Windows-Aufrufe des Studios; anderswo ohne Wirkung (Ziel ist der PC).

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::{attach_console, shell_open};

/// Nur Windows: die Release-EXE hat keine eigene Konsole.
#[cfg(not(windows))]
pub fn attach_console() {}

/// Nur Windows.
#[cfg(not(windows))]
pub fn shell_open(_url: &str) -> Result<(), String> {
    Err("Adressen öffnen geht nur unter Windows".to_owned())
}
