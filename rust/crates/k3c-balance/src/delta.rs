//! Delta-Bericht (Port von `engine/balance/delta.go`, B-640, BS7.2): Der Vergleich mit der Baseline eines Meilensteins, nach
//! Werte-Gruppen geordnet. Freie Gruppen erscheinen immer mit Ampel und Trend, eingefrorene nur, wenn eines ihrer Ziele kippt
//! (Warnung); Hinweise und Hebel nennt die Auswertung nur für freie Gruppen (`BuildInsights`, RU31).
//!
//! Go ruft `buildDelta` mit `[]TargetInsight`; dieses Crate hat den Typ erst mit RU31.1. Bis dahin liest `build_delta` die
//! Ziele über den Trait [`DeltaTarget`]; RU31.1 setzt ihn für `TargetInsight` um.

use serde::{Deserialize, Serialize};

use crate::evaluate::{STATUS_NARROW, STATUS_OK};
use crate::groups::GROUPS;
use crate::hints::Lever;
use crate::insights::{LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW};

/// Trend: der Abstand zum Korridor wurde größer.
pub const TREND_BETTER: &str = "besser";
/// Trend: der Abstand wurde kleiner.
pub const TREND_WORSE: &str = "schlechter";
/// Trend: unverändert.
pub const TREND_SAME: &str = "gleich";

/// Eine Werte-Gruppe im Delta-Bericht (Go: `GroupDelta`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupDelta {
    pub group: String,
    pub frozen: bool,
    /// Schlechteste Ampel der Ziele der Gruppe.
    pub light: String,
    /// `TREND_BETTER`, `TREND_WORSE` oder `TREND_SAME`.
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

/// Der Vergleich eines Ziels mit der Baseline (Go: `Trend`, die Felder, die `buildDelta` liest).
pub struct TrendFacts<'a> {
    pub delta: f64,
    pub better: bool,
    pub before_status: &'a str,
}

/// Ein Ziel einer Auswertung, soweit der Delta-Bericht es braucht (Go: `TargetInsight`).
pub trait DeltaTarget {
    fn name(&self) -> &str;
    fn group(&self) -> &str;
    fn light(&self) -> &str;
    fn status(&self) -> &str;
    /// `None` ohne Baseline-Wert für das Ziel.
    fn trend(&self) -> Option<TrendFacts<'_>>;
}

/// Ordnet die Ziele einer Auswertung nach Gruppen; der Aufrufer ruft es nur mit Baseline auf (Go: `buildDelta`).
pub fn build_delta<T: DeltaTarget>(targets: &[T], frozen: &[String]) -> Vec<GroupDelta> {
    let mut out = Vec::new();
    for g in GROUPS {
        let mut d = GroupDelta {
            group: g.to_string(),
            frozen: frozen.iter().any(|f| f == g),
            light: LIGHT_GREEN.into(),
            ..GroupDelta::default()
        };
        let mut compared = 0;
        for t in targets.iter().filter(|t| t.group() == g) {
            d.targets += 1;
            d.light = worse_light(&d.light, t.light()).to_string();
            let Some(tr) = t.trend() else { continue };
            compared += 1;
            if tr.delta == 0.0 {
                // unverändert zählt weder besser noch schlechter
            } else if tr.better {
                d.better += 1;
            } else {
                d.worse += 1;
            }
            if passed_status(tr.before_status) != passed_status(t.status()) {
                d.tipped.push(t.name().to_string());
            }
        }
        if d.targets == 0 || (d.frozen && d.tipped.is_empty()) {
            continue;
        }
        d.trend = trend_of(&d, compared).into();
        if d.frozen {
            d.warning = format!("eingefrorene Gruppe {g} kippt: {}", d.tipped.join(", "));
        }
        out.push(d);
    }
    out
}

fn trend_of(d: &GroupDelta, compared: i64) -> &'static str {
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

/// Rot vor gelb vor grau vor grün (Go: `worseLight`).
pub fn worse_light<'a>(a: &'a str, b: &'a str) -> &'a str {
    let rank = |l: &str| match l {
        LIGHT_GREY => 1,
        LIGHT_YELLOW => 2,
        LIGHT_RED => 3,
        _ => 0, // grün und unbekannt (Go: fehlender Schlüssel = 0)
    };
    if rank(b) > rank(a) { b } else { a }
}

/// Lässt die Hebel eingefrorener Gruppen weg (Go: `freeLevers`).
pub fn free_levers(levers: &[Lever], frozen: &[String]) -> Vec<Lever> {
    levers.iter().filter(|l| !frozen.contains(&l.group)).cloned().collect()
}
