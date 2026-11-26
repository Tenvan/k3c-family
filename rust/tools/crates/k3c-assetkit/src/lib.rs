//! Asset-Logik des Studios (Port von `tools/assetkit`, RU67). Die Go-Fassung ist das Orakel; Verhalten und Dateiformate
//! bleiben gleich (Leitfaden `docs/standards/rust-werkzeuge.md`).
//!
//! Auf der Wurzel liegen die Namen des Go-Pakets `assets` (`k3c_assetkit::load` wie `assets.Load`):
//! - Slot-Register der Sounds (`data/sounds.json`): je Slot die Zuordnungen nach Präferenz, nur die erste ist aktiv.
//! - Stand des Registers mit Schreibregel und Beobachtung ([`stamp()`], [`watch`]).
//! - Zielmaß der Grafik-Slots ([`set_target_if`]).
//!
//! [`ticket`] (Go-Paket `ticket`) ist ein eigenes Modul: `k3c_assetkit::ticket::create`.

mod error;
mod fsutil;
mod gojson;
mod pngsize;
mod register;
mod stamp;
mod target;
pub mod ticket;

#[cfg(test)]
mod repo_tests;
#[cfg(test)]
mod testutil;
#[cfg(test)]
mod write_tests;

pub use error::{Error, Result};
pub use register::{
    CATEGORIES, Choice, KIND_GRAPHIC, KIND_SOUND, REGISTER_PATH, Rect, Register, STATUS_ASSIGNED, STATUS_GAP,
    STATUS_MISSING, Size, Slot, load, set_choices, status, write_atomic,
};
pub use stamp::{WatchHandle, find_root, set_choices_if, stamp, watch};
pub use target::{MAX_TARGET_UNITS, Units, set_target_if};
