//! Kennzahlen je Tiefe der Insel (BS11, B-642), verdichtet über die gültigen Läufe mit World.

use std::fmt::Write as _;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::datafiles;
use super::figures::{median_or_nil, round1};
use super::report_types::{Metrics, Report, read_go};
use super::summary::num;

/// Eine Zeile je Tiefe, je Boss und der Endboss.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IslandFigures {
    /// Gültige Insel-Läufe.
    pub runs: i64,
    pub depths: Vec<DepthFigures>,
    pub bosses: Vec<BossFigures>,
    pub endboss: BossFigures,
}

/// Anteil der Läufe, die die Tiefe erreichten (%), Median des Ankunftstags, Anteil mit Burgfall und Mittelwerte.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DepthFigures {
    pub depth: i64,
    pub arrived: f64,
    pub arrival_day_median: Option<f64>,
    pub castle_fell: f64,
    /// Gefallene Monarchen je Lauf (Mittel).
    pub downs: f64,
    pub hub_level: f64,
    pub fighters: f64,
}

/// Anteil der Läufe, die den Boss besiegten (%), und Median des Tags (`null`: nie besiegt).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BossFigures {
    pub boss: String,
    /// Anteil der Läufe in %, in denen der Boss erschien.
    pub spawned: f64,
    pub defeated: f64,
    pub day_median: Option<f64>,
}

/// Verdichtet die Insel-Läufe; `None` ohne einen gültigen Lauf mit Stufen.
pub(crate) fn island_figures_of(root: &Path, r: &Report) -> Option<IslandFigures> {
    let runs: Vec<&Metrics> = r
        .results
        .iter()
        .filter(|res| res.valid)
        .filter_map(|res| res.metrics.as_ref())
        .filter(|m| !m.island.stages.is_empty())
        .collect();
    let first = runs.first()?;
    let n = runs.len() as f64;
    let depths = first
        .island
        .stages
        .iter()
        .enumerate()
        .map(|(i, stage)| depth_figures(&runs, i, stage.depth))
        .collect();
    let endbosses = endboss_ids(root);
    let end: Vec<f64> = runs
        .iter()
        .filter_map(|m| m.island.endboss_day.map(|d| d as f64))
        .collect();
    Some(IslandFigures {
        runs: i64::try_from(runs.len()).unwrap_or(i64::MAX),
        depths,
        bosses: boss_figures(&runs, &endbosses),
        endboss: BossFigures {
            boss: "endboss".to_owned(),
            spawned: spawn_share(&runs, &endbosses),
            defeated: round1(100.0 * end.len() as f64 / n),
            day_median: median_or_nil(&end),
        },
    })
}

fn depth_figures(runs: &[&Metrics], i: usize, depth: i64) -> DepthFigures {
    let n = runs.len() as f64;
    let mut days = Vec::new();
    let (mut fell, mut downs, mut hub, mut fighters) = (0_i64, 0_i64, 0_i64, 0_i64);
    for stage in runs.iter().filter_map(|m| m.island.stages.get(i)) {
        days.extend(stage.arrival_day.map(|d| d as f64));
        fell += i64::from(stage.castle_fall_tick.is_some());
        downs = downs.wrapping_add(stage.downs);
        hub = hub.wrapping_add(stage.hub_level);
        fighters = fighters.wrapping_add(stage.fighters);
    }
    DepthFigures {
        depth,
        arrived: round1(100.0 * days.len() as f64 / n),
        arrival_day_median: median_or_nil(&days),
        castle_fell: round1(100.0 * fell as f64 / n),
        downs: round1(downs as f64 / n),
        hub_level: round1(hub as f64 / n),
        fighters: round1(fighters as f64 / n),
    }
}

/// Ein Eintrag je Boss-ID in der Reihenfolge des ersten Auftretens (über alle Läufe).
fn boss_figures(runs: &[&Metrics], endbosses: &[String]) -> Vec<BossFigures> {
    let mut ids: Vec<&str> = Vec::new();
    for b in runs.iter().flat_map(|m| &m.island.bosses) {
        if !ids.contains(&b.boss.as_str()) {
            ids.push(&b.boss);
        }
    }
    ids.iter()
        .map(|id| {
            let days: Vec<f64> = runs
                .iter()
                .filter_map(|m| m.island.bosses.iter().find(|b| b.boss == *id))
                .map(|b| b.day as f64)
                .collect();
            // Wie Go: die ID `endboss` steht für alle Endbosse der Daten.
            let own = [(*id).to_owned()];
            let want = if *id == "endboss" { endbosses } else { &own[..] };
            BossFigures {
                boss: (*id).to_owned(),
                spawned: spawn_share(runs, want),
                defeated: round1(100.0 * days.len() as f64 / runs.len() as f64),
                day_median: median_or_nil(&days),
            }
        })
        .collect()
}

pub(crate) fn day_text(d: Option<f64>) -> String {
    d.map_or_else(|| "–".to_owned(), num)
}

impl IslandFigures {
    /// Schreibt die Zeilen je Tiefe und je Boss.
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
        for x in self.bosses.iter().chain([&self.endboss]) {
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

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct EndBossFile {
    end_bosses: Vec<EndBoss>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct EndBoss {
    id: String,
}

/// IDs der Endbosse aus `data/bosses.json` (das Ereignis `bossSpawned` nennt die ID, nicht `endboss`); leer, wenn die
/// Datei fehlt oder nicht lesbar ist.
pub(crate) fn endboss_ids(root: &Path) -> Vec<String> {
    datafiles::read(root, "bosses.json")
        .and_then(|raw| read_go::<EndBossFile>(&raw))
        .map(|f| f.end_bosses.into_iter().map(|b| b.id).collect())
        .unwrap_or_default()
}

/// Anteil der Läufe in %, in denen einer der Bosse `want` erschien.
fn spawn_share(runs: &[&Metrics], want: &[String]) -> f64 {
    let n = runs
        .iter()
        .filter(|m| m.island.boss_spawns.iter().any(|b| want.contains(&b.boss)))
        .count();
    round1(100.0 * n as f64 / runs.len() as f64)
}
