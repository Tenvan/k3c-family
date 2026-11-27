//! Planung des Repos (Port von `internal/planning`, Dateinamen wie Go): aktive und geplante Sprints mit ihren
//! Sessions (docs/sprints/), Tickets (docs/backlog/), Projekte (docs/projekte/) und die Dokumente Plan, Fragenkatalog
//! und Glossar. Die Dateien bleiben die Quelle; Format und Vorlagen stehen in docs/arbeitsweise.md.
//!
//! Stand RU69.8: Modell, Auflösen der IDs, `get`, `list`, `items`, Projekte lesen und der Unterbau der Schreibwege
//! (`ChangeSet`). Die schreibenden Werkzeuge folgen in RU69.9 und RU69.10.

pub mod create;
pub mod domains;
pub mod edit;
pub mod gitrepo;
pub mod items;
pub mod list;
#[allow(clippy::module_inception)] // Dateinamen wie Go (`planning.go`).
pub mod planning;
pub mod priority;
pub mod projects;
pub mod relations;
pub mod store;
pub mod store_remove;
pub mod tables;
pub mod ticketid;
pub mod worktree;

pub use create::get;
pub use items::{Item, MAX_ITEMS, items, load_list};
pub use list::{Filter, list};
pub use planning::{Data, Session, Sprint, Ticket, available, doc, load, parse_sprint, parse_ticket};
pub use projects::{Project, parse_project};

#[cfg(test)]
mod domains_tests;
#[cfg(test)]
mod gitrepo_tests;
#[cfg(test)]
mod items_tests;
#[cfg(test)]
mod list_tests;
#[cfg(test)]
mod planning_tests;
#[cfg(test)]
mod priority_tests;
#[cfg(test)]
mod projects_tests;
#[cfg(test)]
mod relations_tests;
#[cfg(test)]
mod resolve_slug_tests;
#[cfg(test)]
mod session_ids_tests;
#[cfg(test)]
mod store_tests;
#[cfg(test)]
mod testsupport;
