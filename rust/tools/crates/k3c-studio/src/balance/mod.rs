//! Balancing des Studios (Port von `tools/k3c-studio/internal/balance`, RU70.14 bis RU70.17): Berichte von
//! `k3c-game sim balance` lesen, bewerten und verdichten. Die Bots und die Läufe selbst rechnet `k3c-balance`.
//!
//! Daten (`data/`) liest [`datafiles`] von der Platte; alle Funktionen, die sie brauchen, nehmen die Repo-Wurzel.

pub mod baselines;
pub mod curves;
pub mod datafiles;
pub mod delta;
pub mod economy_measures;
pub mod evaluate;
pub mod figures;
pub mod figures_island;
pub mod groups;
pub mod hints;
pub mod insights;
pub mod insights_figures;
pub mod insights_files;
pub mod insights_levers;
pub mod insights_markdown;
pub mod insights_next;
pub mod insights_plain;
pub mod insights_progress;
pub mod insights_text;
pub mod names;
pub mod progress;
pub mod report_types;
pub mod scope;
pub mod seedmode;
pub mod ship_figures;
pub mod summary;
pub mod targets;
pub mod tune;
pub mod tuneparam;

#[cfg(test)]
#[path = "datafiles_tests.rs"]
mod datafiles_tests;
#[cfg(test)]
#[path = "figures_tests.rs"]
mod figures_tests;
#[cfg(test)]
#[path = "groups_tests.rs"]
mod groups_tests;
#[cfg(test)]
#[path = "names_tests.rs"]
mod names_tests;
#[cfg(test)]
#[path = "summary_tests.rs"]
mod summary_tests;
#[cfg(test)]
mod test_helpers;
