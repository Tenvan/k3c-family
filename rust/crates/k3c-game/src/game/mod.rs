//! Port von `native/k3c-game/game/`: die Spielschleife des nativen Spiels. RU26.1 legt Schalter, Modi, Menü-Stapel und
//! die Konstante `TPS` an; `game.go` (die Nabe) liefert RU26.4. Die Zeitsammlung `pacing.go` entfällt: Die Plattform
//! (`k3c_platform::Clock`, 30 Hz, höchstens 4 Takte je Bild, `alpha`) ersetzt sie; `deps.go` (Go-Modul-Pins) entfällt.

pub mod blend;
pub mod flags;
pub mod layout;
pub mod modes;
pub mod packages;
pub mod parts;
mod pending;
pub mod scene;
pub mod shared;
pub mod stack;

#[cfg(test)]
#[path = "imports_tests.rs"]
mod imports_tests;

#[cfg(test)]
#[path = "pacing_tests.rs"]
mod pacing_tests;

pub use blend::Blend;
pub use layout::{Cameras, VIEW_UNITS, cell_rect, cells, place, scale};
pub use modes::Measure;
pub use packages::{Libs, packages};
pub use parts::{Part, Parts, assemble};
// RU30.2 tauscht `parse_script` gegen `script::parse_script`, RU30.1 `Loader`; danach entfällt `pending.rs`.
pub use pending::{Loader, parse_script};
pub use scene::{Args, Hold, Scene, USAGE, parse_args};
pub use stack::{Mode, Stack};

/// Takte der Simulation je Sekunde.
pub const TPS: i64 = 30;
