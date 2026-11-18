//! Port von `native/k3c-game/game/`: die Spielschleife des nativen Spiels. RU26.1 legt Schalter, Modi, Menü-Stapel und
//! die Konstante `TPS` an; `game.go` (die Nabe) liefert RU26.4. Die Zeitsammlung `pacing.go` entfällt: Die Plattform
//! (`k3c_platform::Clock`, 30 Hz, höchstens 4 Takte je Bild, `alpha`) ersetzt sie; `deps.go` (Go-Modul-Pins) entfällt.

pub mod flags;
pub mod modes;
mod pending;
pub mod scene;
pub mod stack;

#[cfg(test)]
#[path = "pacing_tests.rs"]
mod pacing_tests;

pub use modes::Measure;
// RU30.2 tauscht diese Zeile gegen `script::parse_script` und löscht `pending.rs`.
pub use pending::parse_script;
pub use scene::{Args, Hold, Scene, USAGE, parse_args};
pub use stack::{Mode, Stack};

/// Takte der Simulation je Sekunde.
pub const TPS: i64 = 30;
