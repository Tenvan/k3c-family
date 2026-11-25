//! Auswertung eines Balancing-Berichts ohne KI (Port von `engine/balance/insights.go`, B-632): Ampel je Ziel und Kennzahl,
//! Stärken, Schwächen, Tuning-Hinweise, Kurzauswertung und Vorschlag für den nächsten Lauf. Eingang ist ein Summary (auch ein
//! altes ohne Kennzahlen) samt optionaler Baseline; Richtwerte und Hebel aus data/balance-hints.json (`hints.rs`). Die Texte
//! und Vorschläge liefern `insights_levers`, `insights_next`, `insights_markdown` und bis RU31.5 `pending.rs`.

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::delta::{DeltaTarget, GroupDelta, TrendFacts, build_delta, free_levers};
use crate::evaluate::{STATUS_INVALID, STATUS_NARROW, STATUS_OK, STATUS_VIOLATED, bounds};
use crate::figures::round1;
use crate::hints::{Hints, Lever};
use crate::insights_figures::{FigureInsights, figure_insights};
use crate::insights_levers::{LeverGroup, lever_groups};
use crate::insights_next::{NextRun, hints, next_run};
use crate::insights_plain::{LeverPlan, gap_text, plain, plans, trend_plain};
use crate::insights_progress::goal_text;
use crate::insights_text::{Hint, strengths, summary, weaknesses};
use crate::progress::{GoalView, Progress, QualityView};
use crate::summary::{Change, Entry, Summary, compare};
use crate::targets::KIND_SHARE;

/// Ampel: ok.
pub const LIGHT_GREEN: &str = "green";
/// Ampel: knapp.
pub const LIGHT_YELLOW: &str = "yellow";
/// Ampel: verletzt.
pub const LIGHT_RED: &str = "red";
/// Ampel: ungültig bzw. ohne Messwert.
pub const LIGHT_GREY: &str = "grey";

/// Die Auswertung; das JSON speist den HTML-Bericht (report/*.js) und die Workbench (ReportView).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Insights {
    pub title: String,
    pub run: String,
    pub generated: String,
    pub seeds: i64,
    pub pass: bool,
    pub counts: BTreeMap<String, i64>,
    pub summary: Vec<String>,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub targets: Vec<TargetInsight>,
    pub figures: FigureInsights,
    pub hints: Vec<Hint>,
    /// Stellschrauben nach Wirkung zusammengefasst (`insights_levers.go`).
    #[serde(rename = "leverGroups")]
    pub lever_groups: Vec<LeverGroup>,
    pub next: NextRun,
    pub baseline: BaselineView,
    /// Vergleich je Werte-Gruppe (`delta.rs`), leer ohne Baseline.
    pub delta: Vec<GroupDelta>,
    pub varianten: Vec<String>,
    /// B-648: Urteil über das Ziel, Qualität der Korridore, Spielfortschritt und der Lauf, mit dem verglichen wurde.
    pub goal: Option<GoalView>,
    pub quality: Option<QualityView>,
    pub progress: Option<Progress>,
    pub previous: String,
    #[serde(rename = "goalText")]
    pub goal_text: String,
}

/// Ein Ziel mit Ampel. `distance`: Abstand zur nächsten Grenze in % der Korridorbreite, positiv im Korridor, negativ
/// außerhalb; `None` ohne Messwert.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TargetInsight {
    pub name: String,
    pub group: String,
    pub measure: String,
    pub kind: String,
    pub unit: String,
    pub grade: String,
    pub scenario: String,
    pub value: f64,
    pub lower: Option<f64>,
    pub upper: Option<f64>,
    pub status: String,
    pub light: String,
    pub distance: Option<f64>,
    pub trend: Option<Trend>,
    #[serde(rename = "failSeeds")]
    pub fail_seeds: Vec<String>,
    #[serde(rename = "invalidSeeds")]
    pub invalid_seeds: Vec<String>,
    pub rule: String,
    pub levers: Vec<Lever>,
    /// Gespielte (bei `counted > 0` gewertete) Läufe.
    pub runs: i64,
    pub plain: String,
    pub gap: String,
    #[serde(rename = "trendText")]
    pub trend_text: String,
    pub plans: Vec<LeverPlan>,
    #[serde(skip)]
    pub entry: Entry,
}

/// Der Vergleich mit der Baseline: `better`, wenn der Abstand zum Korridor größer (bzw. die Verletzung kleiner) wurde.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Trend {
    pub before: f64,
    pub delta: f64,
    pub better: bool,
    #[serde(rename = "beforeStatus")]
    pub before_status: String,
}

/// Der Vergleich mit der Baseline für die Anzeige.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BaselineView {
    pub present: bool,
    pub note: String,
    pub changes: Vec<Change>,
}

/// Beschreibt den Lauf für Kopf und Befehle: Ordner (simtest-run-N), Zeit und Name der Baseline.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Meta {
    pub run: String,
    pub generated: String,
    pub baseline: String,
    /// Eingefrorene Werte-Gruppen: keine Hebel, Delta nur beim Kippen.
    pub frozen: Vec<String>,
}

impl DeltaTarget for TargetInsight {
    fn name(&self) -> &str {
        &self.name
    }
    fn group(&self) -> &str {
        &self.group
    }
    fn light(&self) -> &str {
        &self.light
    }
    fn status(&self) -> &str {
        &self.status
    }
    fn trend(&self) -> Option<TrendFacts<'_>> {
        self.trend.as_ref().map(|t| TrendFacts {
            delta: t.delta,
            better: t.better,
            before_status: &t.before_status,
        })
    }
}

/// Wertet einen Bericht aus; `base == None`: ohne Trend und Vergleich (Go: `BuildInsights`).
pub fn build_insights(s: &Summary, base: Option<&Summary>, h: &Hints, m: &Meta) -> Insights {
    let counts = [LIGHT_GREEN, LIGHT_YELLOW, LIGHT_RED, LIGHT_GREY]
        .iter()
        .map(|l| ((*l).to_string(), 0))
        .collect();
    let mut inn = Insights {
        title: m.run.clone(),
        run: m.run.clone(),
        generated: m.generated.clone(),
        seeds: s.seeds,
        pass: s.pass,
        varianten: s.varianten.clone(),
        counts,
        ..Insights::default()
    };
    let before: HashMap<String, &Entry> = base
        .map(|b| b.targets.iter().map(|e| (e.target_key(), e)).collect())
        .unwrap_or_default();
    for e in &s.targets {
        let mut t = target_insight(e, h);
        if let Some(b) = before.get(&e.target_key()) {
            t.trend = trend(b, e);
        }
        t.levers = free_levers(&t.levers, &m.frozen);
        t.runs = if e.verdict.counted > 0 {
            e.verdict.counted
        } else {
            s.seeds
        }; // nur gewertete Läufe (B-657)
        t.plain = plain(&t);
        t.gap = gap_text(&t);
        t.trend_text = trend_plain(&t);
        t.plans = plans(&t);
        *inn.counts.entry(t.light.clone()).or_insert(0) += 1;
        inn.targets.push(t);
    }
    inn.figures = figure_insights(s.kennzahlen.as_ref(), h);
    inn.baseline = baseline_view(s, base, &m.baseline);
    if base.is_some() {
        inn.delta = build_delta(&inn.targets, &m.frozen);
    }
    inn.strengths = strengths(&inn);
    inn.weaknesses = weaknesses(&inn);
    inn.goal = s.goal.clone();
    inn.quality = s.quality.clone();
    inn.progress = s.progress.clone();
    inn.goal_text = goal_text(&inn);
    inn.hints = hints(&inn);
    inn.lever_groups = lever_groups(&inn);
    inn.next = next_run(&inn, s);
    inn.summary = summary(&inn);
    inn
}

pub(crate) fn target_insight(e: &Entry, h: &Hints) -> TargetInsight {
    let (v, g) = (&e.verdict, &e.verdict.target);
    let mut t = TargetInsight {
        name: g.name.clone(),
        group: g.group.clone(),
        measure: g.measure.clone(),
        kind: g.kind.clone(),
        unit: unit_of(&g.kind).into(),
        grade: grade_name(&g.grade).into(),
        scenario: format!(
            "{} Spieler · {} · Tiefe {} · {} Tage",
            g.players, g.bot, g.depth, g.days
        ),
        value: v.value,
        lower: g.lower,
        upper: g.upper,
        status: v.status.clone(),
        light: light_of(&v.status).into(),
        fail_seeds: v.fail_seeds.clone(),
        invalid_seeds: v.invalid_seeds.clone(),
        rule: g.rule.clone(),
        entry: e.clone(),
        ..TargetInsight::default()
    };
    if v.status != STATUS_INVALID {
        t.distance = Some(distance(e));
        if v.status != STATUS_OK {
            t.levers = h.levers_for(e).to_vec();
        }
    }
    t
}

/// Die Ampel zu einem Status (Go: `lightOf`).
pub fn light_of(status: &str) -> &'static str {
    match status {
        STATUS_OK => LIGHT_GREEN,
        STATUS_NARROW => LIGHT_YELLOW,
        STATUS_VIOLATED => LIGHT_RED,
        _ => LIGHT_GREY,
    }
}

fn unit_of(kind: &str) -> &'static str {
    if kind == KIND_SHARE { "%" } else { "" }
}

/// Der Grad; leer heißt normal (Go: `gradeName`).
pub fn grade_name(g: &str) -> &str {
    if g.is_empty() { "normal" } else { g }
}

/// Abstand zur nächsten Grenze in % der Korridorbreite (eine Grenze: deren Betrag, mindestens 1). Außerhalb negativ,
/// gemessen an der verletzten Grenze; im Korridor an der nächsten nicht natürlichen Grenze (`bounds`).
pub fn distance(e: &Entry) -> f64 {
    let (g, v) = (&e.verdict.target, e.verdict.value);
    let width = match (g.lower, g.upper) {
        (Some(lo), Some(hi)) if hi - lo < 1.0 => hi.abs().max(1.0), // Korridor ohne Breite (100–100)
        (Some(lo), Some(hi)) => hi - lo,
        (Some(b), None) | (None, Some(b)) => b.abs().max(1.0),
        (None, None) => 1.0,
    };
    if let Some(lo) = g.lower
        && v < lo
    {
        return round1(100.0 * (v - lo) / width);
    }
    if let Some(hi) = g.upper
        && v > hi
    {
        return round1(100.0 * (hi - v) / width);
    }
    let (lo, hi) = bounds(g);
    let best = [lo, hi]
        .into_iter()
        .flatten()
        .map(|b| (v - b).abs())
        .fold(f64::INFINITY, f64::min);
    if best.is_infinite() {
        return 100.0;
    }
    round1(100.0 * best / width)
}

fn trend(b: &Entry, e: &Entry) -> Option<Trend> {
    if b.verdict.status == STATUS_INVALID || e.verdict.status == STATUS_INVALID {
        return None;
    }
    Some(Trend {
        before: b.verdict.value,
        delta: round1(e.verdict.value - b.verdict.value),
        better: distance(e) >= distance(b),
        before_status: b.verdict.status.clone(),
    })
}

fn baseline_view(s: &Summary, base: Option<&Summary>, name: &str) -> BaselineView {
    let Some(base) = base else {
        return BaselineView {
            note: "Keine Baseline: kein Trend.".into(),
            ..BaselineView::default()
        };
    };
    let mut v = BaselineView {
        present: true,
        note: format!("Baseline: {name}"),
        changes: compare(base, s),
    };
    if base.seeds != s.seeds {
        v.note += &format!(" ({} Seeds, dieser Lauf {})", base.seeds, s.seeds);
    }
    v
}
