//! Planung des Repos (Port von `internal/planning`, Dateinamen wie Go): aktive und geplante Sprints mit ihren
//! Sessions (docs/sprints/), Tickets (docs/backlog/), Projekte (docs/projekte/) und die Dokumente Plan, Fragenkatalog
//! und Glossar. Die Dateien bleiben die Quelle; Format und Vorlagen stehen in docs/arbeitsweise.md.
//!
//! Stand RU69.11: Modell, Auflösen der IDs, `get`, `list`, `items`, Projekte, die Schreibwege `set`, `section`,
//! `sections`, `create`, `delete` samt Index, Fahrplan und Projekt-Tabellen, der Neuaufbau der Index-Dateien, die
//! Sprint-Zusammenfassungen, `next`, `waves`, `queue` und `check`.

pub mod check;
pub mod create;
pub mod digest;
pub mod digest_tables;
pub mod domains;
pub mod edit;
pub mod gitrepo;
pub mod index;
pub mod items;
pub mod list;
mod mergebase;
pub mod next;
pub mod next_lanes;
pub mod next_origin;
#[allow(clippy::module_inception)] // Dateinamen wie Go (`planning.go`).
pub mod planning;
pub mod priority;
pub mod projects;
pub mod projects_tables;
pub mod queue;
pub mod relations;
pub mod roadmap_order;
pub mod roadmap_refresh;
pub mod similar;
pub mod store;
pub mod store_remove;
pub mod store_resolve;
pub mod tables;
pub mod ticketid;
pub mod ticketnr;
mod untar;
pub mod waves;
mod waves_nodes;
pub mod waves_spec;
pub mod worktree;

pub use check::{CheckIn, check};
pub use create::{NewDoc, create, delete, get};
pub use digest::{DigestIn, digest};
pub use edit::{section, sections, set};
pub use index::rebuild_index;
pub use items::{Item, MAX_ITEMS, items, load_list};
pub use list::{Filter, list};
pub use next::{NextFilter, next};
pub use planning::{Data, Session, Sprint, Ticket, available, doc, load, parse_sprint, parse_ticket};
pub use projects::{Project, parse_project};
pub use queue::{QueueFilter, queue};
pub use waves::{WavesFilter, waves};

#[cfg(test)]
mod check_tests;
#[cfg(test)]
mod create_cases_tests;
#[cfg(test)]
mod create_fixtures;
#[cfg(test)]
mod create_tests;
#[cfg(test)]
mod digest_tests;
#[cfg(test)]
mod domains_tests;
#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod gitrepo_tests;
#[cfg(test)]
mod index_tests;
#[cfg(test)]
mod items_tests;
#[cfg(test)]
mod list_tests;
#[cfg(test)]
mod next_fixtures;
#[cfg(test)]
mod next_lanes_tests;
#[cfg(test)]
mod next_origin_tests;
#[cfg(test)]
mod next_restbefunde_tests;
#[cfg(test)]
mod next_tests;
#[cfg(test)]
mod numbers_tests;
#[cfg(test)]
mod placeholder_rows_tests;
#[cfg(test)]
mod planning_tests;
#[cfg(test)]
mod priority_tests;
#[cfg(test)]
mod projects_tests;
#[cfg(test)]
mod projects_write_tests;
#[cfg(test)]
mod queue_tests;
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
mod waves_tests;
#[cfg(test)]
mod write_tests;
#[cfg(test)]
mod wz54_tests;
#[cfg(test)]
mod wz55_tests;
