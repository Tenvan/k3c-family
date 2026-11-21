//! Hub und Ausbau (Port von `engine/sim/hub_level.go`, `hub_validate.go`, `hub_validate_layout.go` und den Prüf-Funktionen
//! aus `world_hub_sites.go`). Go hat keine `hub.go`: Die Datei hält nur die Modul-Zeilen.

pub mod level;
pub mod sites;
pub mod validate;
pub mod validate_layout;

#[cfg(test)]
mod candidate_tests;
#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod level_tests;
#[cfg(test)]
mod sites_tests;
#[cfg(test)]
mod stairs_cost_tests;
#[cfg(test)]
mod step_tests;
#[cfg(test)]
mod testutil;
#[cfg(test)]
mod validate_tests;

pub use level::{
    add_cost, each_site, finish_upgrade, hub_gate, hub_level_of, hub_step_of, hub_steps, level_of, new_hub_site,
    next_level, pay_upgrade, site_cost, site_hub_level, step_upgrade, upgrade_job, upgrade_payable,
};
pub use validate::{
    HubFinding, HubReport, all_sites, parse_hub, required_site_kinds, validate_hub, validate_hub_data,
    validate_hub_data_in,
};
