//! Ampeln der Kennzahlen (B-632) aus den Figures und den Richtwerten in `data/balance-hints.json`. Je Grad: normal
//! immer, die anderen Grade nur bei `allGrades` und nur, wenn die Ampel nicht grün ist (sonst wird die Leiste
//! unübersichtlich).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::figures::{Figures, NightFigures, round1};
use super::figures_island::IslandFigures;
use super::hints::{Band, FIGURE_KEYS, FigureHint, Hints};
use super::insights::{LIGHT_GREEN, LIGHT_GREY, grade_name};
use super::ship_figures::ShipFigures;
use super::summary::num;

/// Kennzahlen mit Ampel, die Nächte für die Diagramme und die grünen Bänder je Diagramm.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FigureInsights {
    pub note: String,
    pub bands: BTreeMap<String, Band>,
    pub lights: Vec<FigureLight>,
    pub nights: Vec<NightFigures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub island: Option<IslandFigures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship: Option<ShipFigures>,
}

/// Eine Kennzahl mit Ampel; `value` `None`: kein Messwert (graue Ampel).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FigureLight {
    pub key: String,
    pub name: String,
    pub unit: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub grade: String,
    pub value: Option<f64>,
    pub light: String,
    pub text: String,
}

/// Berechnet eine Kennzahl aus den Nächten eines Grads; `None`: kein Wert.
fn grade_figure(key: &str, nights: &[&NightFigures]) -> Option<Option<f64>> {
    Some(match key {
        "idleShare" => mean_of(nights, 1, |n| n.idle_share),
        "goldBalance" => mean_of(nights, 2, gold_of),
        "woodBalance" => mean_of(nights, 1, |n| n.resource_in.wood - n.resource_out.wood),
        "stoneBalance" => mean_of(nights, 1, |n| n.resource_in.stone - n.resource_out.stone),
        "survivalDrop" => survival_drop(nights),
        _ => return None,
    })
}

pub(crate) fn figure_insights(figures: Option<&Figures>, hints: &Hints) -> FigureInsights {
    let mut out = FigureInsights::default();
    for key in ["idleShare", "goldBalance"] {
        if let Some(h) = hints.figures.get(key) {
            out.bands.insert(key.to_owned(), h.green);
        }
    }
    let Some(f) = figures else {
        "Bericht ohne Kennzahlen (älter als WZ28.4): Kennzahl-Ampeln grau.".clone_into(&mut out.note);
        out.lights = FIGURE_KEYS.iter().filter_map(|key| grey_light(hints, key)).collect();
        return out;
    };
    out.note = format!(
        "Kennzahlen aus {} gültigen Läufen; Richtwerte sind Startwerte (data/balance-hints.json).",
        f.runs
    );
    out.nights.clone_from(&f.nights);
    out.island.clone_from(&f.island);
    out.ship.clone_from(&f.ship);
    let ticks = [
        ("firstWallTick", f.first_wall_tick_median),
        ("firstWorkshopTick", f.first_workshop_tick_median),
    ];
    out.lights = ticks
        .into_iter()
        .filter_map(|(key, value)| tick_light(hints, key, value))
        .collect();
    for grade in grades_of(&f.nights) {
        out.lights
            .extend(grade_lights(hints, grade, &nights_of(&f.nights, grade)));
    }
    out
}

fn light_of(hint: &FigureHint, key: &str) -> FigureLight {
    FigureLight {
        key: key.to_owned(),
        name: hint.name.clone(),
        unit: hint.unit.clone(),
        light: LIGHT_GREY.to_owned(),
        text: "kein Messwert".to_owned(),
        ..FigureLight::default()
    }
}

fn grey_light(hints: &Hints, key: &str) -> Option<FigureLight> {
    hints.figures.get(key).map(|h| light_of(h, key))
}

fn tick_light(hints: &Hints, key: &str, value: Option<f64>) -> Option<FigureLight> {
    let hint = hints.figures.get(key)?;
    let mut l = light_of(hint, key);
    l.value = value;
    "in keinem Lauf erreicht".clone_into(&mut l.text);
    if let Some(v) = value {
        hint.light(v).clone_into(&mut l.light);
        l.text = format!("Median; {}", hint.band_text());
    }
    Some(l)
}

fn grade_lights(hints: &Hints, grade: &str, nights: &[&NightFigures]) -> Vec<FigureLight> {
    let mut out = Vec::new();
    for key in FIGURE_KEYS {
        let Some(hint) = hints.figures.get(key) else { continue };
        if !grade.is_empty() && !hint.all_grades {
            continue;
        }
        let Some(value) = grade_figure(key, nights) else {
            continue;
        };
        let mut l = light_of(hint, key);
        grade_name(grade).clone_into(&mut l.grade);
        if let Some(v) = value {
            let v = round1(v);
            l.value = Some(v);
            hint.light(v).clone_into(&mut l.light);
            l.text = hint.band_text();
        }
        if grade.is_empty() || l.light != LIGHT_GREEN {
            out.push(l);
        }
    }
    out
}

impl FigureHint {
    /// „Richtwert grün -20 bis 60, gelb -60 bis 120“.
    fn band_text(&self) -> String {
        format!("Richtwert grün {}, gelb {}", self.green.text(), self.yellow.text())
    }
}

impl Band {
    fn text(self) -> String {
        match self.0 {
            [None, None] => "beliebig".to_owned(),
            [None, Some(hi)] => format!("≤ {}", num(hi)),
            [Some(lo), None] => format!("≥ {}", num(lo)),
            [Some(lo), Some(hi)] => format!("{} bis {}", num(lo), num(hi)),
        }
    }
}

/// Nach Läufen gewichtetes Mittel ab Nacht `from`.
fn mean_of(nights: &[&NightFigures], from: i64, pick: fn(&NightFigures) -> f64) -> Option<f64> {
    let (mut sum, mut runs) = (0.0, 0);
    for n in nights.iter().filter(|n| n.night >= from && n.runs > 0) {
        sum += pick(n) * n.runs as f64;
        runs += n.runs;
    }
    (runs != 0).then(|| sum / runs as f64)
}

/// Größter Abfall des Überlebens von einer Nacht zur nächsten in Prozentpunkten (vor Nacht 1: 100 %).
fn survival_drop(nights: &[&NightFigures]) -> Option<f64> {
    if nights.is_empty() {
        return None;
    }
    let (mut prev, mut drop) = (100.0, 0.0_f64);
    for n in nights {
        drop = drop.max(prev - n.survival);
        prev = n.survival;
    }
    Some(drop)
}

fn gold_of(n: &NightFigures) -> f64 {
    n.day_income - n.day_expense + n.night_income - n.night_expense
}

fn grades_of(nights: &[NightFigures]) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    for n in nights {
        if !out.contains(&n.grade.as_str()) {
            out.push(&n.grade);
        }
    }
    out
}

/// Nächte eines Grads, nach Nacht sortiert.
fn nights_of<'a>(nights: &'a [NightFigures], grade: &str) -> Vec<&'a NightFigures> {
    let mut out: Vec<&NightFigures> = nights.iter().filter(|n| n.grade == grade).collect();
    out.sort_by_key(|n| n.night);
    out
}
