//! Texte der Auswertung (Port von `engine/balance/insights_text.go`, B-632): regelbasiert aus Ampeln, Abständen,
//! Kennzahlen und Baseline, ohne KI.

use std::cmp::Ordering;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::evaluate::{STATUS_NARROW, STATUS_OK, STATUS_VIOLATED};
use crate::figures::round1;
use crate::hints::Lever;
use crate::insights::{BaselineView, Insights, LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW, TargetInsight};
use crate::insights_figures::FigureLight;
use crate::insights_plain::LeverPlan;
use crate::summary_text::num;

/// Ein Tuning-Hinweis zu einem Ziel (Go: `Hint`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Hint {
    pub target: String,
    pub light: String,
    pub text: String,
    pub why: String,
    pub levers: Vec<Lever>,
    pub plans: Vec<LeverPlan>,
}

const MAX_BULLETS: usize = 8;

impl TargetInsight {
    /// Name des Ziels mit Grad, wenn nicht normal (Go: `label`).
    pub fn label(&self) -> String {
        if self.grade == "normal" {
            self.name.clone()
        } else {
            format!("{} ({})", self.name, self.grade)
        }
    }

    /// Messwert mit Einheit (Go: `valueText`).
    pub fn value_text(&self) -> String {
        self.entry.value_text()
    }

    /// Korridor als Text (Go: `boundsText`).
    pub fn bounds_text(&self) -> String {
        self.entry.bounds_text()
    }
}

impl FigureLight {
    /// Name der Kennzahl mit Grad, wenn nicht normal (Go: `FigureLight.label`).
    pub fn label(&self) -> String {
        if self.grade.is_empty() || self.grade == "normal" {
            self.name.clone()
        } else {
            format!("{} ({})", self.name, self.grade)
        }
    }

    /// Messwert mit Einheit, „–“ ohne Messwert (Go: `FigureLight.valueText`).
    pub fn value_text(&self) -> String {
        match self.value {
            None => "–".into(),
            Some(v) => format!("{} {}", num(v), self.unit).trim().to_string(),
        }
    }
}

/// Verletzte Ziele, am weitesten daneben zuerst (Go: `worst`).
pub fn worst(inn: &Insights) -> Vec<&TargetInsight> {
    let mut out: Vec<&TargetInsight> = inn.targets.iter().filter(|t| t.light == LIGHT_RED).collect();
    out.sort_by(|a, b| {
        let (x, y) = (a.distance.unwrap_or(0.0), b.distance.unwrap_or(0.0));
        x.partial_cmp(&y).unwrap_or(Ordering::Equal)
    });
    out
}

/// Die Kurzauswertung, höchstens sieben Zeilen (Go: `summary`).
pub fn summary(inn: &Insights) -> Vec<String> {
    let count = |l: &str| inn.counts.get(l).copied().unwrap_or(0);
    let mut out: Vec<String> = Vec::new();
    let mut verdict = "";
    if inn.goal.is_some() {
        out.push(inn.goal_text.clone());
        if let Some(p) = inn.progress.as_ref().filter(|p| !p.blocker.is_empty()) {
            out.push(p.blocker.clone());
        }
    } else if inn.pass {
        verdict = ": der Lauf besteht";
    } else {
        verdict = ": der Lauf besteht nicht";
    }
    out.push(format!(
        "{} von {} Zielen verletzt, {} knapp, {} im Korridor{}{}.",
        count(LIGHT_RED),
        inn.targets.len(),
        count(LIGHT_YELLOW),
        count(LIGHT_GREEN),
        invalid_text(count(LIGHT_GREY)),
        verdict
    ));
    let s = castle_sentence(inn);
    if !s.is_empty() {
        out.push(s);
    }
    let w = worst(inn);
    if !w.is_empty() {
        let parts: Vec<String> = w
            .iter()
            .take(2)
            .map(|t| format!("{} mit {} statt {}", t.label(), t.value_text(), t.bounds_text()))
            .collect();
        out.push(format!("Am weitesten daneben: {}.", parts.join("; ")));
        if let Some(lv) = first_lever(&w) {
            out.push(format!(
                "Größter Hebel: {} ({}, je Schritt um {} ändern).",
                lv.label,
                lv.path,
                step_text(lv.step)
            ));
        }
    }
    out.push(figure_sentence(inn));
    if inn.baseline.present {
        out.push(baseline_sentence(&inn.baseline));
    }
    out.truncate(7);
    out
}

fn invalid_text(n: i64) -> String {
    if n == 0 {
        String::new()
    } else {
        format!(", {n} ungültig")
    }
}

/// Die Pflicht-Kennzahl castleHeld über alle Grade in einem Satz (Go: `castleSentence`).
fn castle_sentence(inn: &Insights) -> String {
    let all: Vec<&TargetInsight> = inn.targets.iter().filter(|t| t.measure == "castleHeld").collect();
    let bad: Vec<&&TargetInsight> = all
        .iter()
        .filter(|t| t.light == LIGHT_RED || t.light == LIGHT_GREY)
        .collect();
    if all.is_empty() {
        return String::new();
    }
    if bad.is_empty() {
        return format!("Die Burg hält in allen {} Graden im Korridor.", all.len());
    }
    let names: Vec<String> = bad
        .iter()
        .map(|t| format!("{} {} ({})", t.grade, t.value_text(), t.bounds_text()))
        .collect();
    format!("Die Burg hält nicht im Korridor: {}.", names.join(", "))
}

fn figure_sentence(inn: &Insights) -> String {
    let mut red: Vec<String> = Vec::new();
    let mut grey = 0;
    for l in &inn.figures.lights {
        if l.light == LIGHT_RED {
            red.push(format!("{} {}", l.label(), l.value_text()));
        } else if l.light == LIGHT_GREY {
            grey += 1;
        }
    }
    if !red.is_empty() {
        let s = format!(
            "Kennzahlen außerhalb der Richtwerte: {}",
            red[..red.len().min(3)].join(", ")
        );
        return format!("{}.", s.strip_suffix('.').unwrap_or(&s));
    }
    if grey == inn.figures.lights.len() && grey > 0 {
        return "Keine Kennzahlen im Bericht: Kennzahl-Ampeln grau.".into();
    }
    "Alle Kennzahlen liegen im oder nahe am Richtwert.".into()
}

fn baseline_sentence(b: &BaselineView) -> String {
    let tipped = b.changes.iter().filter(|c| c.tipped).count();
    if b.changes.is_empty() {
        return "Gegenüber der Baseline hat kein Ziel den Status gewechselt.".into();
    }
    format!(
        "Gegenüber der Baseline wechseln {} Ziele den Status, {} kippen.",
        b.changes.len(),
        tipped
    )
}

fn first_lever<'a>(ts: &[&'a TargetInsight]) -> Option<&'a Lever> {
    ts.iter().find_map(|t| t.levers.first())
}

/// Ein Schritt in Prozent, z. B. „10 %“ (Go: `stepText`).
pub fn step_text(step: f64) -> String {
    format!("{} %", num(round1(step * 100.0)))
}

/// Die Stärken: Ziele im Korridor, Kennzahlen im Richtwert, Verbesserungen (Go: `strengths`).
pub fn strengths(inn: &Insights) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut by_measure: HashMap<&str, Vec<&TargetInsight>> = HashMap::new();
    let mut order: Vec<&str> = Vec::new();
    for t in &inn.targets {
        let e = by_measure.entry(t.measure.as_str()).or_default();
        if e.is_empty() {
            order.push(t.measure.as_str());
        }
        e.push(t);
    }
    for m in order {
        out.extend(measure_strengths(&by_measure[m]));
    }
    for l in &inn.figures.lights {
        if l.light == LIGHT_GREEN && (l.grade.is_empty() || l.grade == "normal") {
            out.push(format!("{} {} im Richtwert.", l.label(), l.value_text()));
        }
    }
    for t in &inn.targets {
        let better = t
            .trend
            .as_ref()
            .is_some_and(|tr| tr.better && tr.before_status == STATUS_VIOLATED);
        if better && t.light != LIGHT_RED {
            out.push(format!("{} ist gegenüber der Baseline wieder im Korridor.", t.label()));
        }
    }
    cap_list(out)
}

/// Alle Grade grün: ein Satz, sonst je grünem Ziel einer (Go: `measureStrengths`).
fn measure_strengths(ts: &[&TargetInsight]) -> Vec<String> {
    let green: Vec<&&TargetInsight> = ts.iter().filter(|t| t.light == LIGHT_GREEN).collect();
    if green.is_empty() {
        return Vec::new();
    }
    if green.len() == ts.len() && ts.len() > 1 {
        return vec![format!("{}: in allen {} Graden im Korridor.", ts[0].name, ts.len())];
    }
    green
        .iter()
        .map(|t| format!("{}: {} im Korridor {}.", t.label(), t.value_text(), t.bounds_text()))
        .collect()
}

/// Die Schwächen: verletzte und ungültige Ziele, rote Kennzahlen, gekippte Ziele (Go: `weaknesses`).
pub fn weaknesses(inn: &Insights) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for t in worst(inn) {
        out.push(format!(
            "{}: {} statt {}, {} Seeds betroffen.",
            t.label(),
            t.value_text(),
            t.bounds_text(),
            t.fail_seeds.len()
        ));
    }
    for t in &inn.targets {
        if t.light == LIGHT_GREY {
            out.push(format!(
                "{}: ungültig, {} Seeds abgebrochen oder ohne Messwert.",
                t.label(),
                t.invalid_seeds.len()
            ));
        }
    }
    for l in &inn.figures.lights {
        if l.light == LIGHT_RED {
            out.push(format!("{} {} ({}).", l.label(), l.value_text(), l.text));
        }
    }
    for c in &inn.baseline.changes {
        if c.tipped && c.after != STATUS_OK && c.after != STATUS_NARROW {
            out.push(format!(
                "Gekippt gegenüber der Baseline: {} ({} → {}).",
                c.name, c.before, c.after
            ));
        }
    }
    cap_list(out)
}

fn cap_list(mut s: Vec<String>) -> Vec<String> {
    s.truncate(MAX_BULLETS);
    s
}
