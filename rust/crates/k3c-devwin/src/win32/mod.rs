//! Win32-Schicht des Cheat-Fensters. Einzige Stelle im Rust-Workspace mit `unsafe` (RU56, rust-port.md § 12):
//! jeder Block trägt einen `// SAFETY:`-Kommentar, `tests/no_unsafe.rs` prüft beides.
#![allow(unsafe_code)]

mod app;
mod controls;
mod page;
mod pagedraw;
mod pageevents;
mod pagescroll;
pub mod pipe;
mod tabdraw;
mod tabs;
mod theme;
mod window;

/// run öffnet das Fenster und läuft die Nachrichtenschleife bis zum Schließen; liefert den Exit-Code.
pub fn run(args: &crate::args::Args) -> windows::core::Result<u8> {
    window::run(args)
}
