//! Bericht von `task balance` (Port von `engine/balance/summary.go`, B-157, BAL2.2): Pass/Fail je Ziel als JSON und Markdown,
//! Vergleich mit einer Baseline. Die Baseline ist selbst ein Summary-JSON (`testdata/balance/baseline.json`), ein eigenes
//! Format gibt es nicht. Texte der Tabelle: `summary_text.rs`.

use std::collections::HashMap;

use k3c_data::balance_targets::{Target, Targets};
use serde::{Deserialize, Serialize};

use crate::evaluate::{Evaluate, STATUS_NARROW, STATUS_OK, Verdict};
use crate::fehler::Fehler;
use crate::figures::Figures;
use crate::parallel::{ProgressFn, run_all, tick_progress};
use crate::progress::{GoalView, Progress, QualityView, progress_of};
use crate::run::{Matrix, Report, Scenario, Tick};
use crate::scope::{ROLE_GOAL, ROLE_GUARD, role};
use crate::seedmode::SeedModes;

/// Ein bewertetes Ziel; `pass` gilt für „ok“ und „knapp“.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Entry {
    #[serde(flatten)]
    pub verdict: Verdict,
    pub pass: bool,
}

/// Der Bericht: alle Ziele mit Urteil. `seeds` ist die Zahl der gespielten Seeds (kleiner als in der Datei bei `--seeds N`).
/// `varianten` und `kennzahlen` (WZ28.4) fehlen in älteren Berichten und in der Baseline; `read_summary` liest beide.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Summary {
    pub version: i64,
    pub seeds: i64,
    /// dev, all oder auswahl (B-634); fehlt in älteren Berichten.
    #[serde(rename = "seedMode", skip_serializing_if = "String::is_empty")]
    pub seed_mode: String,
    /// Die Seeds bei höchstens 5, sonst leer.
    #[serde(rename = "seedNames", skip_serializing_if = "Vec::is_empty")]
    pub seed_names: Vec<String>,
    /// Alle Ziele mit Rolle goal erreicht (B-648); ohne Rollen (alte Berichte) alle Ziele.
    pub pass: bool,
    /// Urteil über die Ziele (Rolle goal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<GoalView>,
    /// Korridore mit Rolle guard im Korridor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<QualityView>,
    /// Meilenstein-Leiter des Insel-Laufs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<Progress>,
    pub targets: Vec<Entry>,
    /// Wert-Varianten des Laufs (`datei.json:a.b.c=wert`).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub varianten: Vec<String>,
    /// `figures_of` des Laufs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kennzahlen: Option<Figures>,
}

/// Ein Ziel, dessen Bewertung gegenüber der Baseline anders ist. `before`/`after` sind Status; „neu“ und „entfällt“, wenn das
/// Ziel nur auf einer Seite steht. `tipped`: Pass wechselt (ok/knapp ↔ verletzt/ungültig).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub name: String,
    pub before: String,
    pub after: String,
    pub tipped: bool,
}

/// Läufe und Bericht der Ziele (Go: `Targets.RunTargets`, `RunTargetsProgress`, `Summarize`).
pub trait Summarize {
    /// Spielt alle Szenarien der Ziele über die ersten `n` Seeds der Datei (0 = alle bzw. dem Seed-Modus folgend).
    fn run_targets(&self, n: i64) -> Result<Report, Fehler>;
    /// Wie `run_targets`; `tick` (Go: `Targets.Tick`) und `progress` erfahren nach jedem Lauf fertige und alle Läufe.
    fn run_targets_progress(&self, n: i64, tick: Option<Tick>, progress: Option<ProgressFn>) -> Result<Report, Fehler>;
    /// Bewertet einen Bericht gegen die Ziele.
    fn summarize(&self, r: &Report) -> Summary;
}

impl Summarize for Targets {
    fn run_targets(&self, n: i64) -> Result<Report, Fehler> {
        self.run_targets_progress(n, None, None)
    }

    fn run_targets_progress(&self, n: i64, tick: Option<Tick>, progress: Option<ProgressFn>) -> Result<Report, Fehler> {
        let seeds = self.seed_list(n).to_vec();
        let mut matrix: Option<Matrix> = None;
        let mut all: Vec<Scenario> = Vec::new();
        let mut done: Vec<Scenario> = Vec::new();
        for g in &self.targets {
            let key = scenario_of(g);
            if done.contains(&key) {
                continue;
            }
            done.push(key);
            let m = Matrix {
                seeds: seeds.clone(),
                players: vec![g.players],
                bots: vec![g.bot.clone()],
                depths: vec![g.depth],
                days: g.days,
                grade: g.grade.clone(),
                world: g.world.clone(),
                helps: g.helps.clone(),
                ..Matrix::default()
            };
            all.extend(m.scenarios()?);
            matrix.get_or_insert(m);
        }
        let results = run_all(&all, 0, tick_progress(tick, progress)).map_err(Fehler::Abbruch)?;
        Ok(Report {
            matrix: matrix.unwrap_or_default(),
            results,
        })
    }

    fn summarize(&self, r: &Report) -> Summary {
        let mut s = Summary {
            version: 1,
            seeds: r.matrix.seeds.len() as i64,
            ..Summary::default()
        };
        if !self.seeds.is_empty() {
            s.seed_mode = self.seed_mode_of(&r.matrix.seeds).to_string();
        }
        if r.matrix.seeds.len() <= 5 {
            s.seed_names = r.matrix.seeds.clone();
        }
        let (mut goal, mut quality) = (GoalView::default(), QualityView::default());
        for mut v in self.evaluate(r) {
            let rl = role(&v.target).to_string();
            if rl != ROLE_GOAL && v.status == STATUS_NARROW {
                v.status = STATUS_OK.into(); // „knapp“ gibt es nur bei Zielen (B-648)
            }
            let pass = v.status == STATUS_OK || v.status == STATUS_NARROW;
            if rl == ROLE_GOAL {
                goal.total += 1;
                goal.hit += i64::from(pass);
            } else if rl == ROLE_GUARD {
                quality.total += 1;
                quality.passed += i64::from(pass);
            }
            s.targets.push(Entry { verdict: v, pass });
        }
        goal.reached = goal.hit == goal.total;
        s.pass = goal.reached;
        s.goal = Some(goal);
        if quality.total > 0 {
            s.quality = Some(quality);
        }
        if let Some(sc) = self.goal_scenario() {
            s.progress = progress_of(r, &sc, &r.matrix.seeds);
        }
        s
    }
}

/// Das Szenario eines Ziels ohne Seed.
fn scenario_of(g: &Target) -> Scenario {
    Scenario {
        players: g.players,
        bot: g.bot.clone(),
        depth: g.depth,
        days: g.days,
        grade: g.grade.clone(),
        world: g.world.clone(),
        helps: g.helps.clone(),
        ..Scenario::default()
    }
}

/// Die Szenario-Auswahl der Insel-Läufe (Go: `Targets.goalScenario`).
pub trait GoalScenario {
    /// Das Szenario des ersten Ziels mit Rolle goal und Welt (Insel-Lauf), ohne Tiefe.
    fn goal_scenario(&self) -> Option<Scenario>;
}

impl GoalScenario for Targets {
    fn goal_scenario(&self) -> Option<Scenario> {
        let g = self
            .targets
            .iter()
            .find(|g| !g.world.is_empty() && role(g) == ROLE_GOAL)?;
        Some(Scenario {
            depth: 0,
            ..scenario_of(g)
        })
    }
}

impl Summary {
    /// Schreibt den Bericht eingerückt mit Zeilenende (Go: `JSON`).
    pub fn json(&self) -> Result<String, Fehler> {
        let mut s = serde_json::to_string_pretty(self).map_err(|e| Fehler::Daten(format!("balance: {e}")))?;
        s.push('\n');
        Ok(s)
    }
}

/// Liest ein Summary-JSON (Baseline; Go: `ReadSummary`).
pub fn read_summary(raw: &str) -> Result<Summary, Fehler> {
    serde_json::from_str(raw).map_err(|e| Fehler::Daten(format!("baseline: {e}")))
}

impl Entry {
    /// Unterscheidet Ziele auch bei gleichem Namen (das Szenario gehört dazu; Go: `targetKey`).
    fn target_key(&self) -> String {
        let t = &self.verdict.target;
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            t.measure, t.name, t.players, t.bot, t.depth, t.days, t.grade, t.world, t.helps
        )
    }

    fn name(&self) -> &str {
        &self.verdict.target.name
    }

    fn status(&self) -> &str {
        &self.verdict.status
    }
}

/// Nennt die Ziele, deren Status sich gegenüber der Baseline geändert hat, in der Reihenfolge von `cur` (entfallene am
/// Ende). Wertänderungen ohne Statuswechsel sind keine Änderung (Go: `Compare`).
pub fn compare(base: &Summary, cur: &Summary) -> Vec<Change> {
    let mut before: HashMap<String, &Entry> = base.targets.iter().map(|e| (e.target_key(), e)).collect();
    let mut out = Vec::new();
    for e in &cur.targets {
        match before.remove(&e.target_key()) {
            None => out.push(Change {
                name: e.name().to_string(),
                before: "neu".into(),
                after: e.status().to_string(),
                tipped: !e.pass,
            }),
            Some(b) if b.status() != e.status() => out.push(Change {
                name: e.name().to_string(),
                before: b.status().to_string(),
                after: e.status().to_string(),
                tipped: b.pass != e.pass,
            }),
            Some(_) => {}
        }
    }
    for b in &base.targets {
        if before.contains_key(&b.target_key()) {
            out.push(Change {
                name: b.name().to_string(),
                before: b.status().to_string(),
                after: "entfällt".into(),
                tipped: b.pass,
            });
        }
    }
    out
}
