//! Startet die Spiel-EXE (`k3c-game sim …`, `docs/standards/k3c-game-laeufe.md`) als Kindprozess des Studios (Port von
//! `internal/gameexe`): EXE des Checkouts suchen und auf Frische prüfen (`find`), JSON-Zeilen lesen (`event`), Lauf
//! starten und abbrechen (`run`).

mod event;
mod find;
mod run;

#[cfg(test)]
mod argv_tests;
#[cfg(test)]
mod event_tests;
#[cfg(test)]
mod find_tests;

pub use event::{Event, parse_event};
pub use find::{BUILD_COMMAND, find, locate, stale};
pub use run::{Cancel, Error, GRACE, OnEvent, OnLine, Options, argv_for, call, never, run, stream};
