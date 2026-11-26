//! Auswertung eines Balancing-Berichts ohne KI (B-632): Ampel je Ziel und Kennzahl, Stärken, Schwächen,
//! Tuning-Hinweise, Kurzauswertung und Vorschlag für den nächsten Lauf. Eingang ist ein Summary (auch ein altes ohne
//! Kennzahlen) samt optionaler Baseline; Richtwerte und Hebel aus `data/balance-hints.json` ([`super::hints`]).

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::delta::{GroupDelta, build_delta, free_levers};
use super::evaluate::{STATUS_INVALID, STATUS_NARROW, STATUS_OK, STATUS_VIOLATED};
use super::figures::round1;
use super::hints::{Hints, Lever};
use super::insights_figures::{FigureInsights, figure_insights};
use super::insights_levers::{LeverGroup, lever_groups};
use super::insights_next::{NextRun, hints, next_run};
use super::insights_plain::LeverPlan;
use super::insights_progress::goal_text;
use super::insights_text::{Hint, strengths, summary, weaknesses};
use super::progress::{GoalView, Progress, QualityView};
use super::report_types::nil_empty;
use super::summary::{Change, Entry, Summary, compare};
use super::targets::KIND_SHARE;

/// Ampeln: grün ok, gelb knapp, rot verletzt, grau ungültig bzw. ohne Messwert.
pub const LIGHT_GREEN: &str = "green";
pub const LIGHT_YELLOW: &str = "yellow";
pub const LIGHT_RED: &str = "red";
pub const LIGHT_GREY: &str = "grey";

/// Die Auswertung; das JSON speist den HTML-Bericht und die Oberfläche.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Insights {
    pub title: String,
    pub run: String,
    pub generated: String,
    pub seeds: i64,
    pub pass: bool,
    pub counts: BTreeMap<String, i64>,
    pub summary: Vec<String>,
    #[serde(serialize_with = "nil_empty")]
    pub strengths: Vec<String>,
    #[serde(serialize_with = "nil_empty")]
    pub weaknesses: Vec<String>,
    pub targets: Vec<TargetInsight>,
    pub figures: FigureInsights,
    pub hints: Vec<Hint>,
    /// Stellschrauben nach Wirkung zusammengefasst.
    pub lever_groups: Vec<LeverGroup>,
    pub next: NextRun,
    pub baseline: BaselineView,
    /// Vergleich je Werte-Gruppe, leer ohne Baseline.
    pub delta: Vec<GroupDelta>,
    pub varianten: Vec<String>,
    pub goal: Option<GoalView>,
    pub quality: Option<QualityView>,
    pub progress: Option<Progress>,
    /// Lauf, mit dem verglichen wurde.
    pub previous: String,
    pub goal_text: String,
}

/// Ein Ziel mit Ampel. `distance`: Abstand zur nächsten Grenze in % der Korridorbreite, positiv im Korridor.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
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
    #[serde(serialize_with = "nil_empty")]
    pub fail_seeds: Vec<String>,
    #[serde(serialize_with = "nil_empty")]
    pub invalid_seeds: Vec<String>,
    pub rule: String,
    pub levers: Vec<Lever>,
    /// Gespielte Läufe.
    pub runs: i64,
    /// Messwert, Sollwert und Lage in Sätzen.
    pub plain: String,
    pub gap: String,
    pub trend_text: String,
    pub plans: Vec<LeverPlan>,
    /// Das bewertete Ziel, nicht im JSON.
    #[serde(skip)]
    pub entry: Entry,
}

/// Vergleich mit der Baseline: `better`, wenn der Abstand zum Korridor größer (die Verletzung kleiner) wurde.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Trend {
    pub before: f64,
    pub delta: f64,
    pub better: bool,
    pub before_status: String,
}

/// Vergleich mit der Baseline für die Anzeige.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BaselineView {
    pub present: bool,
    pub note: String,
    pub changes: Vec<Change>,
}

/// Der Lauf für Kopf und Befehle.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Meta {
    pub run: String,
    pub generated: String,
    pub baseline: String,
    /// Eingefrorene Werte-Gruppen: keine Hebel, Delta nur beim Kippen.
    pub frozen: Vec<String>,
}

/// Wertet einen Bericht aus; `base` `None`: ohne Trend und Vergleich. `root` liefert die Daten der Hebel.
pub fn build_insights(root: &Path, s: &Summary, base: Option<&Summary>, h: &Hints, m: &Meta) -> Insights {
    let mut out = Insights {
        title: m.run.clone(),
        run: m.run.clone(),
        generated: m.generated.clone(),
        seeds: s.seeds,
        pass: s.pass,
        varianten: s.varianten.clone(),
        counts: [LIGHT_GREEN, LIGHT_YELLOW, LIGHT_RED, LIGHT_GREY]
            .map(|light| (light.to_owned(), 0))
            .into(),
        ..Insights::default()
    };
    add_targets(&mut out, root, s, base, h, &m.frozen);
    out.figures = figure_insights(s.kennzahlen.as_ref(), h);
    out.baseline = baseline_view(s, base, &m.baseline);
    if base.is_some() {
        out.delta = build_delta(&out.targets, &m.frozen);
    }
    out.strengths = strengths(&out);
    out.weaknesses = weaknesses(&out);
    out.goal = s.goal;
    out.quality = s.quality;
    out.progress.clone_from(&s.progress);
    out.goal_text = goal_text(&out);
    out.hints = hints(&out);
    out.lever_groups = lever_groups(&out);
    out.next = next_run(&out, s);
    out.summary = summary(&out);
    out
}

fn add_targets(out: &mut Insights, root: &Path, s: &Summary, base: Option<&Summary>, h: &Hints, frozen: &[String]) {
    let before: HashMap<String, &Entry> = base
        .iter()
        .flat_map(|b| &b.targets)
        .map(|e| (e.target_key(), e))
        .collect();
    for e in &s.targets {
        let mut t = target_insight(e, h);
        if let Some(b) = before.get(&e.target_key()) {
            t.trend = trend(b, e);
        }
        t.levers = free_levers(&t.levers, frozen);
        t.runs = s.seeds;
        if e.verdict.counted > 0 {
            t.runs = e.verdict.counted; // nur gewertete Läufe (B-657)
        }
        t.plain = t.plain();
        t.gap = t.gap_text();
        t.trend_text = t.trend_plain();
        t.plans = t.plans(root);
        *out.counts.entry(t.light.clone()).or_default() += 1;
        out.targets.push(t);
    }
}

fn target_insight(e: &Entry, h: &Hints) -> TargetInsight {
    let (v, g) = (&e.verdict, &e.verdict.target);
    let invalid = v.status == STATUS_INVALID;
    TargetInsight {
        name: g.name.clone(),
        group: g.group.clone(),
        measure: g.measure.clone(),
        kind: g.kind.clone(),
        unit: unit_of(&g.kind).to_owned(),
        grade: grade_name(&g.grade).to_owned(),
        scenario: format!(
            "{} Spieler · {} · Tiefe {} · {} Tage",
            g.players, g.bot, g.depth, g.days
        ),
        value: v.value,
        lower: g.lower,
        upper: g.upper,
        status: v.status.clone(),
        light: light_of(&v.status).to_owned(),
        distance: (!invalid).then(|| distance(e)),
        fail_seeds: v.fail_seeds.clone(),
        invalid_seeds: v.invalid_seeds.clone(),
        rule: g.rule.clone(),
        levers: if invalid || v.status == STATUS_OK {
            Vec::new()
        } else {
            h.levers_for(e)
        },
        entry: e.clone(),
        ..TargetInsight::default()
    }
}

fn light_of(status: &str) -> &'static str {
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

fn grade_name(grade: &str) -> &str {
    if grade.is_empty() { "normal" } else { grade }
}

/// Abstand zur nächsten Grenze in % der Korridorbreite (eine Grenze: deren Betrag, mindestens 1). Außerhalb negativ,
/// gemessen an der verletzten Grenze; im Korridor an der nächsten nicht natürlichen Grenze.
fn distance(e: &Entry) -> f64 {
    let (v, g) = (e.verdict.value, &e.verdict.target);
    let width = match (g.lower, g.upper) {
        // ein Korridor ohne Breite (100–100) misst am Betrag der Grenze statt durch null zu teilen
        (Some(lo), Some(hi)) if hi - lo < 1.0 => hi.abs().max(1.0),
        (Some(lo), Some(hi)) => hi - lo,
        (Some(bound), None) | (None, Some(bound)) => bound.abs().max(1.0),
        (None, None) => 1.0,
    };
    if let Some(lo) = g.lower.filter(|lo| v < *lo) {
        return round1(100.0 * (v - lo) / width);
    }
    if let Some(hi) = g.upper.filter(|hi| v > *hi) {
        return round1(100.0 * (hi - v) / width);
    }
    let (lo, hi) = g.bounds();
    let best = [lo, hi]
        .into_iter()
        .flatten()
        .fold(f64::INFINITY, |best, b| best.min((v - b).abs()));
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
            note: "Keine Baseline: kein Trend.".to_owned(),
            ..BaselineView::default()
        };
    };
    let mut note = format!("Baseline: {name}");
    if base.seeds != s.seeds {
        note.push_str(&format!(" ({} Seeds, dieser Lauf {})", base.seeds, s.seeds));
    }
    BaselineView {
        present: true,
        note,
        changes: compare(base, s),
    }
}
