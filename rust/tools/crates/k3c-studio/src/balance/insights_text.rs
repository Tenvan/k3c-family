//! Texte der Auswertung (B-632): regelbasiert aus Ampeln, Abständen, Kennzahlen und Baseline, ohne KI.

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use super::evaluate::{STATUS_NARROW, STATUS_OK, STATUS_VIOLATED};
use super::figures::round1;
use super::hints::Lever;
use super::insights::{BaselineView, Insights, LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW, TargetInsight};
use super::insights_figures::FigureLight;
use super::insights_plain::LeverPlan;
use super::summary::num;

/// Ein Tuning-Hinweis zu einem Ziel.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
    /// Name des Ziels mit Grad, wenn nicht normal.
    pub(crate) fn label(&self) -> String {
        if self.grade == "normal" {
            return self.name.clone();
        }
        format!("{} ({})", self.name, self.grade)
    }

    pub(crate) fn value_text(&self) -> String {
        self.entry.value_text()
    }

    pub(crate) fn bounds_text(&self) -> String {
        self.entry.bounds_text()
    }
}

impl FigureLight {
    pub(crate) fn label(&self) -> String {
        if self.grade.is_empty() || self.grade == "normal" {
            return self.name.clone();
        }
        format!("{} ({})", self.name, self.grade)
    }

    pub(crate) fn value_text(&self) -> String {
        self.value.map_or_else(
            || "–".to_owned(),
            |v| format!("{} {}", num(v), self.unit).trim().to_owned(),
        )
    }
}

/// Verletzte Ziele, am weitesten daneben zuerst.
pub(crate) fn worst(insights: &Insights) -> Vec<&TargetInsight> {
    let mut out: Vec<&TargetInsight> = insights.targets.iter().filter(|t| t.light == LIGHT_RED).collect();
    // rote Ziele haben immer einen Abstand; wie Go zählen -0 und 0 als gleich
    out.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(Ordering::Equal));
    out
}

fn count(insights: &Insights, light: &str) -> i64 {
    insights.counts.get(light).copied().unwrap_or_default()
}

/// Die Kurzauswertung, höchstens sieben Sätze.
pub(crate) fn summary(insights: &Insights) -> Vec<String> {
    let mut out = Vec::new();
    let mut verdict = "";
    if insights.goal.is_some() {
        out.push(insights.goal_text.clone());
        if let Some(p) = insights.progress.as_ref().filter(|p| !p.blocker.is_empty()) {
            out.push(p.blocker.clone());
        }
    } else if insights.pass {
        verdict = ": der Lauf besteht";
    } else {
        verdict = ": der Lauf besteht nicht";
    }
    out.push(format!(
        "{} von {} Zielen verletzt, {} knapp, {} im Korridor{}{verdict}.",
        count(insights, LIGHT_RED),
        insights.targets.len(),
        count(insights, LIGHT_YELLOW),
        count(insights, LIGHT_GREEN),
        invalid_text(count(insights, LIGHT_GREY))
    ));
    out.extend(castle_sentence(insights));
    worst_sentences(&worst(insights), &mut out);
    out.push(figure_sentence(insights));
    if insights.baseline.present {
        out.push(baseline_sentence(&insights.baseline));
    }
    out.truncate(7);
    out
}

fn worst_sentences(worst: &[&TargetInsight], out: &mut Vec<String>) {
    if worst.is_empty() {
        return;
    }
    let parts: Vec<String> = worst
        .iter()
        .take(2)
        .map(|t| format!("{} mit {} statt {}", t.label(), t.value_text(), t.bounds_text()))
        .collect();
    out.push(format!("Am weitesten daneben: {}.", parts.join("; ")));
    if let Some(lever) = worst.iter().find_map(|t| t.levers.first()) {
        out.push(format!(
            "Größter Hebel: {} ({}, je Schritt um {} ändern).",
            lever.label,
            lever.path,
            step_text(lever.step)
        ));
    }
}

fn invalid_text(n: i64) -> String {
    if n == 0 {
        return String::new();
    }
    format!(", {n} ungültig")
}

/// Die Pflicht-Kennzahl `castleHeld` über alle Grade in einem Satz.
fn castle_sentence(insights: &Insights) -> Option<String> {
    let all: Vec<&TargetInsight> = insights.targets.iter().filter(|t| t.measure == "castleHeld").collect();
    if all.is_empty() {
        return None;
    }
    let names: Vec<String> = all
        .iter()
        .filter(|t| t.light == LIGHT_RED || t.light == LIGHT_GREY)
        .map(|t| format!("{} {} ({})", t.grade, t.value_text(), t.bounds_text()))
        .collect();
    if names.is_empty() {
        return Some(format!("Die Burg hält in allen {} Graden im Korridor.", all.len()));
    }
    Some(format!("Die Burg hält nicht im Korridor: {}.", names.join(", ")))
}

fn figure_sentence(insights: &Insights) -> String {
    let lights = &insights.figures.lights;
    let red: Vec<String> = lights
        .iter()
        .filter(|l| l.light == LIGHT_RED)
        .take(3)
        .map(|l| format!("{} {}", l.label(), l.value_text()))
        .collect();
    let grey = lights.iter().filter(|l| l.light == LIGHT_GREY).count();
    if !red.is_empty() {
        let text = format!("Kennzahlen außerhalb der Richtwerte: {}", red.join(", "));
        return format!("{}.", text.strip_suffix('.').unwrap_or(&text));
    }
    if grey > 0 && grey == lights.len() {
        return "Keine Kennzahlen im Bericht: Kennzahl-Ampeln grau.".to_owned();
    }
    "Alle Kennzahlen liegen im oder nahe am Richtwert.".to_owned()
}

fn baseline_sentence(baseline: &BaselineView) -> String {
    if baseline.changes.is_empty() {
        return "Gegenüber der Baseline hat kein Ziel den Status gewechselt.".to_owned();
    }
    let tipped = baseline.changes.iter().filter(|c| c.tipped).count();
    format!(
        "Gegenüber der Baseline wechseln {} Ziele den Status, {tipped} kippen.",
        baseline.changes.len()
    )
}

/// Schrittweite eines Hebels in Prozent.
pub(crate) fn step_text(step: f64) -> String {
    format!("{} %", num(round1(step * 100.0)))
}

pub(crate) fn strengths(insights: &Insights) -> Vec<String> {
    let mut order: Vec<&str> = Vec::new();
    for t in &insights.targets {
        if !order.contains(&t.measure.as_str()) {
            order.push(&t.measure);
        }
    }
    let mut out = Vec::new();
    for measure in order {
        let same: Vec<&TargetInsight> = insights.targets.iter().filter(|t| t.measure == measure).collect();
        out.extend(measure_strengths(&same));
    }
    for l in &insights.figures.lights {
        if l.light == LIGHT_GREEN && (l.grade.is_empty() || l.grade == "normal") {
            out.push(format!("{} {} im Richtwert.", l.label(), l.value_text()));
        }
    }
    for t in &insights.targets {
        let back = t
            .trend
            .as_ref()
            .is_some_and(|tr| tr.better && tr.before_status == STATUS_VIOLATED);
        if back && t.light != LIGHT_RED {
            out.push(format!("{} ist gegenüber der Baseline wieder im Korridor.", t.label()));
        }
    }
    out.truncate(MAX_BULLETS);
    out
}

/// Alle Grade grün: ein Satz, sonst je grünem Ziel einer.
fn measure_strengths(targets: &[&TargetInsight]) -> Vec<String> {
    let green: Vec<&&TargetInsight> = targets.iter().filter(|t| t.light == LIGHT_GREEN).collect();
    if green.len() == targets.len() && targets.len() > 1 {
        return vec![format!(
            "{}: in allen {} Graden im Korridor.",
            targets[0].name,
            targets.len()
        )];
    }
    green
        .iter()
        .map(|t| format!("{}: {} im Korridor {}.", t.label(), t.value_text(), t.bounds_text()))
        .collect()
}

pub(crate) fn weaknesses(insights: &Insights) -> Vec<String> {
    let mut out: Vec<String> = worst(insights)
        .iter()
        .map(|t| {
            format!(
                "{}: {} statt {}, {} Seeds betroffen.",
                t.label(),
                t.value_text(),
                t.bounds_text(),
                t.fail_seeds.len()
            )
        })
        .collect();
    for t in insights.targets.iter().filter(|t| t.light == LIGHT_GREY) {
        out.push(format!(
            "{}: ungültig, {} Seeds abgebrochen oder ohne Messwert.",
            t.label(),
            t.invalid_seeds.len()
        ));
    }
    for l in insights.figures.lights.iter().filter(|l| l.light == LIGHT_RED) {
        out.push(format!("{} {} ({}).", l.label(), l.value_text(), l.text));
    }
    for c in &insights.baseline.changes {
        if c.tipped && c.after != STATUS_OK && c.after != STATUS_NARROW {
            out.push(format!(
                "Gekippt gegenüber der Baseline: {} ({} → {}).",
                c.name, c.before, c.after
            ));
        }
    }
    out.truncate(MAX_BULLETS);
    out
}
