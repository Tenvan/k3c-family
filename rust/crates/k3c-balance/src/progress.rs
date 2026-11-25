//! Spielfortschritt eines Insel-Laufs (Port von `engine/balance/progress.go`, B-648): Meilenstein-Leiter (Stufen und Bosse
//! der Welt mit dem Tag des Erreichens), Blocker-Satz, Konfiguration und ihre Entsprechung zum Stufen-Test. Steht im
//! Summary, damit Markdown, HTML und result.json aus derselben Quelle kommen und ein alter Bericht neu ausgewertet werden
//! kann.

use serde::{Deserialize, Serialize};

use crate::fehler::Fehler;
use crate::pending::{Metrics, load_world, shared};
use crate::run::{Report, RunResult, Scenario};

/// Länge des Stufen-Tests (Go: `fullgameDays`, `fullgame_test.go`).
pub const FULLGAME_DAYS: i64 = 40;

/// Das Urteil über das Ziel: `reached`, wenn alle Ziele mit Rolle goal erreicht sind.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct GoalView {
    pub reached: bool,
    pub hit: i64,
    pub total: i64,
}

/// Zählt die Korridore mit Rolle guard, die im Korridor liegen.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct QualityView {
    pub passed: i64,
    pub total: i64,
}

/// Die Konfiguration des Ziel-Szenarios (ohne Seeds, die stehen im Summary).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RunConfig {
    pub world: String,
    pub bot: String,
    pub players: i64,
    pub days: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub grade: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub helps: String,
}

/// Ein Meilenstein über alle Seeds: wie oft erreicht, Median des Tags und (im Vergleich mit dem letzten Lauf gleicher
/// Konfiguration) ein Pfeil.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MilestoneRow {
    pub id: String,
    pub label: String,
    pub reached: i64,
    pub runs: i64,
    pub median_day: Option<i64>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub arrow: String,
}

/// Der Spielfortschritt des Ziel-Szenarios.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub config: RunConfig,
    pub test_match: String,
    pub ladder: Vec<MilestoneRow>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub blocker: String,
}

/// Woran ein Meilenstein erkannt wird (Go: `milestoneDef.day`, dort ein Funktionswert).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MilestoneKind {
    /// Stufe mit diesem Index erreicht.
    Stage(usize),
    /// Miniboss mit dieser ID besiegt.
    Boss(String),
    /// Endboss der Insel besiegt.
    Endboss,
}

/// Ein Meilenstein der Leiter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneDef {
    pub id: String,
    pub label: String,
    pub kind: MilestoneKind,
}

impl MilestoneDef {
    /// Der Tag, an dem der Lauf den Meilenstein erreicht hat; `None`, wenn nicht (Go: `day(m)`).
    pub fn day(&self, m: &Metrics) -> Option<i64> {
        match &self.kind {
            MilestoneKind::Stage(i) => m.stages.get(*i).and_then(|s| s.arrival_day),
            MilestoneKind::Boss(id) => m.bosses.iter().find(|b| b.boss == *id).map(|b| b.day),
            MilestoneKind::Endboss => m.endboss_day,
        }
    }
}

/// Baut die Leiter der ersten Insel der Welt: Stufe erreicht, Miniboss der Stufe besiegt, am Ende der Endboss.
pub fn milestone_defs(world_id: &str) -> Result<Vec<MilestoneDef>, Fehler> {
    let world = load_world(world_id)?;
    let Some(isl) = world.islands.first() else {
        return Err(Fehler::Sim(format!("welt {world_id:?} hat keine Insel")));
    };
    let bosses = &shared()?.data.bosses;
    let deepest = isl.depths.iter().max().copied();
    let mut defs = Vec::new();
    for (i, &depth) in isl.depths.iter().enumerate() {
        defs.push(MilestoneDef {
            id: format!("stage:{i}"),
            label: format!("Stufe {} erreicht", i + 1),
            kind: MilestoneKind::Stage(i),
        });
        for b in bosses.bosses.iter().filter(|b| b.depth == depth) {
            defs.push(MilestoneDef {
                id: format!("boss:{}", b.id),
                label: format!("{} besiegt", b.name),
                kind: MilestoneKind::Boss(b.id.clone()),
            });
        }
        for b in &bosses.end_bosses {
            // Nur der Endboss der Insel: das Feld `endBoss`, sonst der der tiefsten Stufe (B-981).
            let own = b.id == isl.end_boss || isl.end_boss.is_empty() && b.depth == depth;
            if Some(depth) == deepest && own {
                defs.push(MilestoneDef {
                    id: format!("endboss:{}", b.id),
                    label: format!("{} (Endboss) besiegt", b.name),
                    kind: MilestoneKind::Endboss,
                });
            }
        }
    }
    Ok(defs)
}

/// Unterer Median der Tage; `None` ohne Tage (Go: `medianDay`).
pub fn median_day(days: &[i64]) -> Option<i64> {
    if days.is_empty() {
        return None;
    }
    let mut s = days.to_vec();
    s.sort_unstable();
    Some(s[(s.len() - 1) / 2])
}

/// Wertet die Läufe des Ziel-Szenarios aus; `None`, wenn keine Läufe mit Welt dazu vorliegen (Go: `progressOf`).
pub fn progress_of(r: &Report, sc: &Scenario, seeds: &[String]) -> Option<Progress> {
    let defs = milestone_defs(&sc.world).ok()?;
    let runs: Vec<&RunResult> = r
        .results
        .iter()
        .filter(|res| res.valid && res.metrics.is_some() && same_scenario(&res.scenario, sc))
        .collect();
    if runs.is_empty() {
        return None;
    }
    let ladder = defs
        .iter()
        .map(|d| {
            let days: Vec<i64> = runs
                .iter()
                .filter_map(|res| res.metrics.as_ref().and_then(|m| d.day(m)))
                .collect();
            MilestoneRow {
                id: d.id.clone(),
                label: d.label.clone(),
                reached: days.len() as i64,
                runs: runs.len() as i64,
                median_day: median_day(&days),
                arrow: String::new(),
            }
        })
        .collect();
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
        ladder,
        blocker: blocker(&runs, &defs),
    })
}

fn same_scenario(a: &Scenario, b: &Scenario) -> bool {
    a.players == b.players
        && a.bot == b.bot
        && a.days == b.days
        && a.grade == b.grade
        && a.world == b.world
        && a.helps == b.helps
}

/// Beschreibt den ersten Lauf, der den letzten Meilenstein nicht erreicht: wo er stehen blieb und was zu dem Zeitpunkt da
/// war. Leer, wenn alle Läufe die ganze Leiter schaffen.
pub fn blocker(runs: &[&RunResult], defs: &[MilestoneDef]) -> String {
    for res in runs {
        let Some(m) = res.metrics.as_ref() else { continue };
        let mut last: Option<(&MilestoneDef, i64)> = None;
        let mut next: Option<&MilestoneDef> = None;
        for d in defs {
            match d.day(m) {
                Some(day) => last = Some((d, day)),
                None if next.is_none() => next = Some(d),
                None => {}
            }
        }
        let Some(next) = next else { continue };
        let mut b = match last {
            Some((d, day)) => format!("Stecken geblieben nach „{}“ (Tag {day})", d.label),
            None => "Stecken geblieben am Start".to_string(),
        };
        b.push_str(&format!(
            ", nächster Meilenstein „{}“ nicht erreicht (Seed {}).",
            next.label, res.scenario.seed
        ));
        if let Some(d) = m.dawns.last() {
            let gold: i64 = d.gold.iter().sum();
            b.push_str(&format!(
                " Zu Tagesbeginn Tag {}: Hub {}, {} Kämpfer, Gold {gold}.",
                d.day, d.hub_level, d.fighters
            ));
        }
        if m.castle_fall_tick.is_some() {
            b.push_str(" Die Burg ist gefallen.");
        }
        return b;
    }
    String::new()
}

/// Sagt, ob die Konfiguration einem Modus des Stufen-Tests (`fullgame_test.go`) entspricht, sonst die Abweichung.
pub fn test_match(sc: &Scenario, seeds: &[String]) -> String {
    let mut diff: Vec<String> = Vec::new();
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
        diff.push("Seeds nicht nur k3c-dev".to_string());
    }
    if !diff.is_empty() {
        return format!("weicht vom Stufen-Test ab: {}", diff.join(", "));
    }
    let mode = if sc.helps.is_empty() { "ohne" } else { sc.helps.as_str() };
    format!("entspricht dem Stufen-Test {mode}")
}
