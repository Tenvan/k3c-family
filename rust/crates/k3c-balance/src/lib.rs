//! Balancing und Testläufe: Port von engine/balance.

pub mod base64;
pub mod baselines;
pub mod bot_abstieg;
pub mod bot_boss;
pub mod bot_clear;
pub mod bot_learn;
pub mod bot_material;
pub mod bot_wirtschaft;
pub mod bots;
pub mod crown_bot;
pub mod curves;
pub mod delta;
pub mod economy_measures;
pub mod evaluate;
pub mod fehler;
pub mod figures;
pub mod figures_island;
pub mod groups;
pub mod helps;
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
pub mod island_world;
pub mod metrics;
pub mod metrics_island;
pub mod parallel;

pub mod prices;
pub mod progress;
pub mod replay;
pub mod report_html;
pub mod run;
pub mod scope;
pub mod seedmode;
pub mod sensitivity;
pub mod sha256;
pub mod ship_need;
pub mod summary;
pub mod summary_text;
pub mod targets;
#[cfg(test)]
mod testutil;
pub mod tune;
pub mod tune_diff;
pub mod tuneparam;
pub mod variant;
pub mod vary;

#[cfg(test)]
mod balance_tests;
#[cfg(test)]
mod baselines_tests;
#[cfg(test)]
mod bot_abstieg_tests;
#[cfg(test)]
mod bot_boss_tests;
#[cfg(test)]
mod bot_clear_tests;
#[cfg(test)]
mod bot_learn_tests;
#[cfg(test)]
mod bot_perf_tests;
#[cfg(test)]
mod bots_tests;
#[cfg(test)]
mod crown_bot_tests;
#[cfg(test)]
mod delta_tests;
#[cfg(test)]
mod economy_tests;
#[cfg(test)]
mod evaluate_tests;
#[cfg(test)]
mod figures_tests;
#[cfg(test)]
mod fullgame_tests;
#[cfg(test)]
mod groups_tests;
#[cfg(test)]
mod helps_tests;
#[cfg(test)]
mod hints_tests;
#[cfg(test)]
mod insights_levers_tests;
#[cfg(test)]
mod insights_plain_tests;
#[cfg(test)]
mod insights_tests;
#[cfg(test)]
mod island_world_tests;
#[cfg(test)]
mod milestone_tests;
#[cfg(test)]
mod mix_tests;
#[cfg(test)]
mod parallel_tests;
#[cfg(test)]
mod profiles_tests;
#[cfg(test)]
mod progress_tests;
#[cfg(test)]
mod replay_tests;
#[cfg(test)]
mod report_html_tests;
#[cfg(test)]
mod rustref_tests;
#[cfg(test)]
mod seedmode_tests;
#[cfg(test)]
mod sensitivity_tests;
#[cfg(test)]
mod sha256_tests;
#[cfg(test)]
mod ship_need_tests;
#[cfg(test)]
mod summary_tests;
#[cfg(test)]
mod targets_tests;
#[cfg(test)]
mod tune_tests;
#[cfg(test)]
mod variant_tests;

pub use fehler::Fehler;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_baut() {}
}
