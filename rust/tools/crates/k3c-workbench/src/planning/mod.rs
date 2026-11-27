//! Planung des Repos (Port von `internal/planning`, Dateinamen wie Go): aktive und geplante Sprints mit ihren
//! Sessions (docs/sprints/), Tickets (docs/backlog/), Projekte (docs/projekte/) und die Dokumente Plan, Fragenkatalog
//! und Glossar. Die Dateien bleiben die Quelle; Format und Vorlagen stehen in docs/arbeitsweise.md.
//!
//! Stand RU69.9: Modell, Auflösen der IDs, `get`, `list`, `items`, Projekte und die Schreibwege `set`, `section`,
//! `sections` samt Index, Fahrplan und Projekt-Tabellen. Anlegen, Löschen und Neuaufbau folgen in RU69.10.

pub mod create;
pub mod domains;
pub mod edit;
pub mod gitrepo;
pub mod index;
pub mod items;
pub mod list;
pub mod next_lanes;
#[allow(clippy::module_inception)] // Dateinamen wie Go (`planning.go`).
pub mod planning;
pub mod priority;
pub mod projects;
pub mod projects_tables;
pub mod queue;
pub mod relations;
pub mod roadmap_order;
pub mod roadmap_refresh;
pub mod store;
pub mod store_remove;
pub mod tables;
pub mod ticketid;
pub mod waves_spec;
pub mod worktree;

pub use create::get;
pub use edit::{section, sections, set};
pub use items::{Item, MAX_ITEMS, items, load_list};
pub use list::{Filter, list};
pub use planning::{Data, Session, Sprint, Ticket, available, doc, load, parse_sprint, parse_ticket};
pub use projects::{Project, parse_project};

#[cfg(test)]
mod domains_tests;
#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod gitrepo_tests;
#[cfg(test)]
mod items_tests;
#[cfg(test)]
mod list_tests;
#[cfg(test)]
mod placeholder_rows_tests;
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
mod review_fixes_tests;
#[cfg(test)]
mod roadmap_refresh_tests;
#[cfg(test)]
mod roadmap_tests;
#[cfg(test)]
mod session_ids_tests;
#[cfg(test)]
mod session_title_tests;
#[cfg(test)]
mod store_tests;
#[cfg(test)]
mod testhooks;
#[cfg(test)]
mod testsupport;
#[cfg(test)]
mod titel_tests;
#[cfg(test)]
mod wz54_tests;
#[cfg(test)]
mod wz55_tests;
