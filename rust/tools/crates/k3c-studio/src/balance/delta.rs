//! Delta-Bericht (B-640, BS7.2): Vergleich mit der Baseline eines Meilensteins, nach Werte-Gruppen geordnet. Freie
//! Gruppen erscheinen immer mit Ampel und Trend, eingefrorene nur, wenn eines ihrer Ziele kippt (Warnung); Hinweise
//! und Hebel nennt die Auswertung nur für freie Gruppen.

use serde::{Deserialize, Serialize};

use super::evaluate::{STATUS_NARROW, STATUS_OK};
use super::groups::GROUPS;
use super::hints::Lever;
use super::insights::{LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW, TargetInsight};

/// Trend einer Gruppe gegenüber der Baseline.
pub const TREND_BETTER: &str = "besser";
pub const TREND_WORSE: &str = "schlechter";
pub const TREND_SAME: &str = "gleich";

/// Eine Werte-Gruppe im Delta-Bericht.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupDelta {
    pub group: String,
    pub frozen: bool,
    /// Schlechteste Ampel der Ziele der Gruppe.
    pub light: String,
    pub trend: String,
    /// Ziele mit größerem Abstand zum Korridor.
    pub better: i64,
    /// Ziele mit kleinerem Abstand.
    pub worse: i64,
    /// Ziele der Gruppe im Lauf.
    pub targets: i64,
    /// Ziele, deren Pass gegenüber der Baseline wechselt.
    pub tipped: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub warning: String,
}

/// Ordnet die Ziele einer Auswertung nach Gruppen; der Aufrufer ruft es nur mit Baseline auf.
pub fn build_delta(targets: &[TargetInsight], frozen: &[String]) -> Vec<GroupDelta> {
    GROUPS
        .iter()
        .filter_map(|group| group_delta(group, targets, frozen))
        .collect()
}

/// `None`: Gruppe ohne Ziele, oder eingefroren und nichts kippt.
fn group_delta(group: &str, targets: &[TargetInsight], frozen: &[String]) -> Option<GroupDelta> {
    let mut d = GroupDelta {
        group: group.to_owned(),
        frozen: frozen.iter().any(|f| f == group),
        light: LIGHT_GREEN.to_owned(),
        ..GroupDelta::default()
    };
    let mut compared = 0;
    for t in targets.iter().filter(|t| t.group == group) {
        d.targets += 1;
        if light_rank(&t.light) > light_rank(&d.light) {
            d.light.clone_from(&t.light);
        }
        let Some(trend) = &t.trend else { continue };
        compared += 1;
        // unverändert zählt weder besser noch schlechter
        if trend.delta != 0.0 {
            if trend.better {
                d.better += 1;
            } else {
                d.worse += 1;
            }
        }
        if passed_status(&trend.before_status) != passed_status(&t.status) {
            d.tipped.push(t.name.clone());
        }
    }
    if d.targets == 0 || (d.frozen && d.tipped.is_empty()) {
        return None;
    }
    trend_of(&d, compared).clone_into(&mut d.trend);
    if d.frozen {
        d.warning = format!("eingefrorene Gruppe {group} kippt: {}", d.tipped.join(", "));
    }
    Some(d)
}

fn trend_of(d: &GroupDelta, compared: usize) -> &'static str {
    if compared == 0 || d.better == d.worse {
        TREND_SAME
    } else if d.better > d.worse {
        TREND_BETTER
    } else {
        TREND_WORSE
    }
}

fn passed_status(status: &str) -> bool {
    status == STATUS_OK || status == STATUS_NARROW
}

/// Rot vor gelb vor grau vor grün; unbekannt zählt wie grün (wie Go: Rang 0).
fn light_rank(light: &str) -> u8 {
    match light {
        LIGHT_GREY => 1,
        LIGHT_YELLOW => 2,
        LIGHT_RED => 3,
        _ => 0,
    }
}

/// Lässt die Hebel eingefrorener Gruppen weg.
pub fn free_levers(levers: &[Lever], frozen: &[String]) -> Vec<Lever> {
    levers.iter().filter(|l| !frozen.contains(&l.group)).cloned().collect()
}

#[cfg(test)]
#[path = "delta_tests.rs"]
mod tests;
