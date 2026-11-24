//! Spielstände und Testberichte als Dateien: Port von engine/store (atomic, backups, reports, restart, saves).
mod atomic;
#[cfg(test)]
mod atomic_tests;
mod backups;
#[cfg(test)]
mod backups_tests;
#[cfg(test)]
mod discard_tests;
mod reports;
#[cfg(test)]
mod reports_tests;
mod restart;
#[cfg(test)]
mod restart_tests;
mod saves;
mod saves_list;
#[cfg(test)]
mod saves_list_tests;
pub mod saves_route;
#[cfg(test)]
mod saves_route_tests;
mod stamp;
#[cfg(test)]
mod store_tests;
#[cfg(test)]
mod testutil;

pub use atomic::Fs;
pub use backups::{BACKUP_KEEP, Backup};
pub use reports::{MAX_REPORT_BYTES, MAX_REPORTS, Reports};
pub use restart::REMEMBERED_ROOMS_MAX_AGE;
pub use saves::{Header, MAX_SAVE_BYTES, Saves, is_slot, parse_save};
pub use saves_list::SaveInfo;
pub use saves_route::{ChronicleInfo, RouteIsland, route_of};
pub use stamp::{STAMP_LAYOUT, Stamp};
