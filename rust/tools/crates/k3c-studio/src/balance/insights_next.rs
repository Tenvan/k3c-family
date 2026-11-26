//! Tuning-Hinweise und Vorschlag für den nächsten Lauf (B-632).
//!
//! Bis RU70.17 stehen hier nur die Typen.

use serde::{Deserialize, Serialize};

use super::insights::Insights;
use super::insights_text::Hint;
use super::summary::Summary;

/// Befehl als Auftrag an Claude im Chat.
pub const WHERE_CHAT: &str = "chat";
/// Befehl für das Terminal im Checkout.
pub const WHERE_SHELL: &str = "shell";
/// Lauf-Spec der Balancing-Seite.
pub const WHERE_WORKBENCH: &str = "workbench";

/// Vorschlag: Schritte als Text, Seeds zum Nachspielen und kopierbare Befehle.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct NextRun {
    pub steps: Vec<String>,
    pub seeds: Vec<String>,
    pub commands: Vec<Command>,
}

/// Ein kopierbarer Aufruf; `place` (JSON `where`) sagt, wohin er gehört.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Command {
    pub label: String,
    #[serde(rename = "where")]
    pub place: String,
    pub text: String,
}

/// Stub RU70.17: noch keine Hinweise.
pub(crate) fn hints(_insights: &Insights) -> Vec<Hint> {
    Vec::new()
}

/// Stub RU70.17: noch kein Vorschlag.
pub(crate) fn next_run(_insights: &Insights, _summary: &Summary) -> NextRun {
    NextRun::default()
}
