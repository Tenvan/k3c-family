//! Spielfortschritt eines Insel-Laufs (B-648): Meilenstein-Leiter (Stufen und Bosse der Welt mit dem Tag des
//! Erreichens), Blocker-Satz, Konfiguration und ihre Entsprechung zum Stufen-Test. Steht im Summary, damit Markdown,
//! HTML und `result.json` aus derselben Quelle kommen und ein alter Bericht neu ausgewertet werden kann.

use std::path::Path;

use serde::{Deserialize, Serialize};

use super::datafiles;
use super::names::load_world;
use super::report_types::{Metrics, Report, RunResult, Scenario, nil_empty, read_go};

/// Urteil über das Ziel: `reached`, wenn alle Ziele mit Rolle goal erreicht sind.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct GoalView {
    pub reached: bool,
    pub hit: i64,
    pub total: i64,
}

/// Korridore mit Rolle guard, die im Korridor liegen.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct QualityView {
    pub passed: i64,
    pub total: i64,
}

/// Konfiguration des Ziel-Szenarios (ohne Seeds, die stehen im Summary).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct RunConfig {
    pub world: String,
    pub bot: String,
    pub players: i64,
    pub days: i64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub grade: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub helps: String,
}

/// Ein Meilenstein über alle Seeds: wie oft erreicht, Median des Tags und ein Pfeil gegen den letzten gleichen Lauf.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct MilestoneRow {
    pub id: String,
    pub label: String,
    pub reached: i64,
    pub runs: i64,
    pub median_day: Option<i64>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub arrow: String,
}

/// Spielfortschritt des Ziel-Szenarios.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Progress {
    pub config: RunConfig,
    pub test_match: String,
    #[serde(serialize_with = "nil_empty")]
    pub ladder: Vec<MilestoneRow>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub blocker: String,
}

/// Woran der Tag eines Meilensteins abgelesen wird.
enum Reach {
    Stage(usize),
    Boss(String),
    Endboss,
}

struct Milestone {
    id: String,
    label: String,
    reach: Reach,
}

impl Milestone {
    fn day(&self, m: &Metrics) -> Option<i64> {
        match &self.reach {
            Reach::Stage(i) => m.island.stages.get(*i).and_then(|s| s.arrival_day),
            Reach::Boss(id) => m.island.bosses.iter().find(|b| b.boss == *id).map(|b| b.day),
            Reach::Endboss => m.island.endboss_day,
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct BossFile {
    bosses: Vec<BossEntry>,
    end_bosses: Vec<BossEntry>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct BossEntry {
    id: String,
    name: String,
    depth: i64,
}

fn load_bosses(root: &Path) -> Result<BossFile, String> {
    let raw = datafiles::read(root, "bosses.json")?;
    read_go(&raw).map_err(|e| format!("bosses.json: {e}"))
}

/// Die Leiter der ersten Insel der Welt: Stufe erreicht, Miniboss der Stufe besiegt, am Ende der Endboss.
fn milestones(root: &Path, world_id: &str) -> Result<Vec<Milestone>, String> {
    let world = load_world(root, world_id)?;
    let Some(island) = world.islands.first() else {
        return Err(format!("welt \"{world_id}\" hat keine Insel"));
    };
    let file = load_bosses(root)?;
    let mut out = Vec::new();
    for (i, &depth) in island.depths.iter().enumerate() {
        out.push(Milestone {
            id: format!("stage:{i}"),
            label: format!("Stufe {} erreicht", i + 1),
            reach: Reach::Stage(i),
        });
        for b in file.bosses.iter().filter(|b| b.depth == depth) {
            out.push(Milestone {
                id: format!("boss:{}", b.id),
                label: format!("{} besiegt", b.name),
                reach: Reach::Boss(b.id.clone()),
            });
        }
        for b in file.end_bosses.iter().filter(|b| b.depth == depth) {
            out.push(Milestone {
                id: format!("endboss:{}", b.id),
                label: format!("{} (Endboss) besiegt", b.name),
                reach: Reach::Endboss,
            });
        }
    }
    Ok(out)
}

/// Unterer Median der Tage; `None` ohne Tage.
fn median_day(mut days: Vec<i64>) -> Option<i64> {
    days.sort_unstable();
    days.get(days.len().checked_sub(1)? / 2).copied()
}

fn same_scenario(a: &Scenario, b: &Scenario) -> bool {
    a.players == b.players
        && a.bot == b.bot
        && a.days == b.days
        && a.grade == b.grade
        && a.world == b.world
        && a.helps == b.helps
}

/// Wertet die Läufe des Ziel-Szenarios aus; `None`, wenn keine Läufe mit Welt dazu vorliegen.
pub(crate) fn progress_of(root: &Path, r: &Report, sc: &Scenario, seeds: &[String]) -> Option<Progress> {
    let defs = milestones(root, &sc.world).ok()?;
    let runs: Vec<(&RunResult, &Metrics)> = r
        .results
        .iter()
        .filter(|res| res.valid && same_scenario(&res.scenario, sc))
        .filter_map(|res| res.metrics.as_ref().map(|m| (res, m)))
        .collect();
    if runs.is_empty() {
        return None;
    }
    let ladder = defs.iter().map(|d| {
        let days: Vec<i64> = runs.iter().filter_map(|(_, m)| d.day(m)).collect();
        MilestoneRow {
            id: d.id.clone(),
            label: d.label.clone(),
            reached: days.len() as i64,
            runs: runs.len() as i64,
            median_day: median_day(days),
            arrow: String::new(),
        }
    });
    Some(Progress {
        config: RunConfig {
            world: sc.world.clone(),
            bot: sc.bot.clone(),
            players: sc.players,
            days: sc.days,
            grade: sc.grade.clone(),
            helps: sc.helps.clone(),
        },
        test_match: test_match(sc, seeds),
        ladder: ladder.collect(),
        blocker: blocker(&runs, &defs),
    })
}

/// Der erste Lauf, der die Leiter nicht schafft: wo er stehen blieb und was zu dem Zeitpunkt da war. Leer, wenn alle
/// Läufe die ganze Leiter schaffen.
fn blocker(runs: &[(&RunResult, &Metrics)], defs: &[Milestone]) -> String {
    for (res, m) in runs {
        let Some(next) = defs.iter().find(|d| d.day(m).is_none()) else {
            continue;
        };
        let last = defs.iter().rev().find_map(|d| d.day(m).map(|day| (d, day)));
        let mut out = match last {
            Some((d, day)) => format!("Stecken geblieben nach „{}“ (Tag {day})", d.label),
            None => "Stecken geblieben am Start".to_owned(),
        };
        out.push_str(&format!(
            ", nächster Meilenstein „{}“ nicht erreicht (Seed {}).",
            next.label, res.scenario.seed
        ));
        if let Some(d) = m.dawns.last() {
            let gold: i64 = d.gold.iter().sum();
            out.push_str(&format!(
                " Zu Tagesbeginn Tag {}: Hub {}, {} Kämpfer, Gold {gold}.",
                d.day, d.hub_level, d.fighters
            ));
        }
        if m.castle_fall_tick.is_some() {
            out.push_str(" Die Burg ist gefallen.");
        }
        return out;
    }
    String::new()
}

/// Länge des Stufen-Tests.
const FULLGAME_DAYS: i64 = 40;

/// Sagt, ob die Konfiguration einem Modus des Stufen-Tests entspricht, sonst die Abweichung.
fn test_match(sc: &Scenario, seeds: &[String]) -> String {
    let mut diff = Vec::new();
    if sc.world != "standard" {
        diff.push(format!("Welt {}", sc.world));
    }
    if sc.players != 1 {
        diff.push(format!("{} Spieler", sc.players));
    }
    if sc.bot != "wirtschaft" {
        diff.push(format!("Bot {}", sc.bot));
    }
    if sc.days != FULLGAME_DAYS {
        diff.push(format!("{} Tage", sc.days));
    }
    if !sc.grade.is_empty() {
        diff.push(format!("Grad {}", sc.grade));
    }
    if seeds.len() != 1 || seeds[0] != "k3c-dev" {
        diff.push("Seeds nicht nur k3c-dev".to_owned());
    }
    if !diff.is_empty() {
        return format!("weicht vom Stufen-Test ab: {}", diff.join(", "));
    }
    let mode = if sc.helps.is_empty() { "ohne" } else { &sc.helps };
    format!("entspricht dem Stufen-Test {mode}")
}

#[cfg(test)]
#[path = "progress_tests.rs"]
mod tests;
