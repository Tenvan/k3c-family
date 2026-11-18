//! Spielstände und Testberichte als Dateien: Port von engine/store (Teil 1: atomic, backups, reports, restart).
mod atomic;
#[cfg(test)]
mod atomic_tests;
mod backups;
#[cfg(test)]
mod backups_tests;
mod reports;
#[cfg(test)]
mod reports_tests;
mod restart;
#[cfg(test)]
mod restart_tests;
mod saves;
mod stamp;
#[cfg(test)]
mod testutil;

pub use atomic::Fs;
pub use backups::{BACKUP_KEEP, Backup};
pub use reports::{MAX_REPORT_BYTES, MAX_REPORTS, Reports};
pub use restart::REMEMBERED_ROOMS_MAX_AGE;
pub use saves::{Saves, is_slot};
pub use stamp::{STAMP_LAYOUT, Stamp};
