//! Kennzahlen je Tiefe der Insel im Bericht (Port von `engine/balance/figures_island.go`, BS11, B-642): verdichtet über die
//! gültigen Läufe mit Welt.

use std::fmt::Write;

use serde::{Deserialize, Serialize};

use crate::figures::round1;
use crate::island_world::shared;
use crate::metrics::Metrics;
use crate::pending::median_or_nil;
use crate::run::Report;
use crate::summary_text::num;

/// Eine Zeile je Tiefe, je Boss und der Endboss (Go: `IslandFigures`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct IslandFigures {
    /// Gültige Insel-Läufe.
    pub runs: i64,
    pub depths: Vec<DepthFigures>,
    pub bosses: Vec<BossFigures>,
    pub endboss: BossFigures,
}

/// Anteil der Läufe, die die Tiefe erreichten (`arrived` in %), Median des Ankunftstags und Anteil der Läufe, in denen die
/// Burg der Stufe fiel (`castle_fell` in %; Go: `DepthFigures`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DepthFigures {
    pub depth: i64,
    pub arrived: f64,
    pub arrival_day_median: Option<f64>,
    pub castle_fell: f64,
    /// Gefallene Monarchen je Lauf (Mittel), Ereignis `crownFallen`.
    pub downs: f64,
    /// Hub-Stufe am Ende (Mittel).
    pub hub_level: f64,
    /// Kämpfer am Ende (Mittel).
    pub fighters: f64,
}

/// Anteil der Läufe, die den Boss besiegten (`defeated` in %), und Median des Tags (null: nie besiegt; Go: `BossFigures`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BossFigures {
    pub boss: String,
    /// Anteil der Läufe in %, in denen der Boss erschien (`bossSpawned`).
    pub spawned: f64,
    pub defeated: f64,
    pub day_median: Option<f64>,
}

/// Verdichtet die Insel-Läufe; `None` ohne einen gültigen Lauf mit Stufen (Go: `islandFiguresOf`).
pub fn island_figures_of(r: &Report) -> Option<IslandFigures> {
    let runs: Vec<&Metrics> = r
        .results
        .iter()
        .filter(|res| res.valid)
        .filter_map(|res| res.metrics.as_ref())
        .filter(|m| !m.stages.is_empty())
        .collect();
    if runs.is_empty() {
        return None;
    }
    let n = runs.len() as f64;
    let depths = (0..runs[0].stages.len()).map(|i| depth_figures(&runs, i, n)).collect();
    let end: Vec<f64> = runs.iter().filter_map(|m| m.endboss_day).map(|d| d as f64).collect();
    Some(IslandFigures {
        runs: runs.len() as i64,
        depths,
        bosses: boss_figures(&runs),
        endboss: BossFigures {
            boss: "endboss".into(),
            spawned: spawn_share(&runs, "endboss"),
            defeated: round1(100.0 * end.len() as f64 / n),
            day_median: median_or_nil(&end),
        },
    })
}

/// Die Zeile der Stufe `i` über alle Läufe.
fn depth_figures(runs: &[&Metrics], i: usize, n: f64) -> DepthFigures {
    let mut days: Vec<f64> = Vec::new();
    let (mut fell, mut downs, mut hub, mut fighters) = (0, 0, 0, 0);
    for m in runs {
        let Some(s) = m.stages.get(i) else {
            continue;
        };
        if let Some(d) = s.arrival_day {
            days.push(d as f64);
        }
        fell += i64::from(s.castle_fall_tick.is_some());
        downs += s.downs;
        hub += s.hub_level;
        fighters += s.fighters;
    }
    DepthFigures {
        depth: runs[0].stages[i].depth,
        arrived: round1(100.0 * days.len() as f64 / n),
        arrival_day_median: median_or_nil(&days),
        castle_fell: round1(100.0 * fell as f64 / n),
        downs: round1(downs as f64 / n),
        hub_level: round1(hub as f64 / n),
        fighters: round1(fighters as f64 / n),
    }
}

/// Ein Eintrag je Boss-ID in der Reihenfolge des ersten Auftretens (über alle Läufe; Go: `bossFigures`).
fn boss_figures(runs: &[&Metrics]) -> Vec<BossFigures> {
    let mut ids: Vec<&str> = Vec::new();
    for b in runs.iter().flat_map(|m| &m.bosses) {
        if !ids.contains(&b.boss.as_str()) {
            ids.push(&b.boss);
        }
    }
    ids.into_iter()
        .map(|id| {
            let days: Vec<f64> = runs
                .iter()
                .filter_map(|m| m.bosses.iter().find(|b| b.boss == id))
                .map(|b| b.day as f64)
                .collect();
            BossFigures {
                boss: id.to_string(),
                spawned: spawn_share(runs, id),
                defeated: round1(100.0 * days.len() as f64 / runs.len() as f64),
                day_median: median_or_nil(&days),
            }
        })
        .collect()
}

fn day_text(d: Option<f64>) -> String {
    d.map_or_else(|| "–".to_string(), num)
}

impl IslandFigures {
    /// Schreibt die Zeilen je Tiefe und je Boss (Go: `markdown`).
    pub fn markdown(&self, b: &mut String) {
        b.push_str("\n| Tiefe | Erreicht % | Ankunftstag (Median) | Burg gefallen % | gefallene Monarchen (Mittel) | Hub-Stufe Ende | Kämpfer Ende |\n|---|---|---|---|---|---|---|\n");
        for d in &self.depths {
            let _ = writeln!(
                b,
                "| {} | {} | {} | {} | {} | {} | {} |",
                d.depth,
                num(d.arrived),
                day_text(d.arrival_day_median),
                num(d.castle_fell),
                num(d.downs),
                num(d.hub_level),
                num(d.fighters)
            );
        }
        b.push_str("\n| Boss | Erschienen % | Besiegt % | Tag (Median) |\n|---|---|---|---|\n");
        for x in self.bosses.iter().chain(std::iter::once(&self.endboss)) {
            let _ = writeln!(
                b,
                "| {} | {} | {} | {} |",
                x.boss,
                num(x.spawned),
                num(x.defeated),
                day_text(x.day_median)
            );
        }
    }
}

/// Die IDs der Endbosse aus data/bosses.json (das Ereignis `bossSpawned` nennt die ID, nicht „endboss“; Go: `endbossIDs`).
fn endboss_ids() -> Vec<String> {
    shared().map_or_else(
        |_| Vec::new(),
        |sh| sh.data.bosses.end_bosses.iter().map(|b| b.id.clone()).collect(),
    )
}

/// Anteil der Läufe in %, in denen der Boss erschien; `id` „endboss“ steht für alle Endbosse der Daten.
fn spawn_share(runs: &[&Metrics], id: &str) -> f64 {
    let want = if id == "endboss" {
        endboss_ids()
    } else {
        vec![id.to_string()]
    };
    let n = runs
        .iter()
        .filter(|m| m.boss_spawns.iter().any(|b| want.contains(&b.boss)))
        .count();
    round1(100.0 * n as f64 / runs.len() as f64)
}
