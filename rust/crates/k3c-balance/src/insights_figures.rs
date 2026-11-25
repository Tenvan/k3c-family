//! Ampeln der Kennzahlen (Port von `engine/balance/insights_figures.go`, B-632) aus `Figures` und den Richtwerten in
//! data/balance-hints.json. Je Grad: normal immer, die anderen Grade nur bei `allGrades` und nur, wenn die Ampel nicht grün
//! ist (sonst wird die Leiste unübersichtlich).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::figures::{Figures, NightFigures, round1};
use crate::figures_island::IslandFigures;
use crate::hints::{Band, FIGURE_KEYS, FigureHint, Hints};
use crate::insights::{LIGHT_GREEN, LIGHT_GREY, grade_name};
use crate::ship_need::ShipFigures;
use crate::summary_text::num;

/// Kennzahlen mit Ampel, die Nächte für die Diagramme und die grünen Bänder je Diagramm.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
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

/// Die Kennzahlen mit Ampel zu einem Bericht; `None`: Bericht ohne Kennzahlen (Go: `figureInsights`).
pub fn figure_insights(f: Option<&Figures>, h: &Hints) -> FigureInsights {
    let mut out = FigureInsights::default();
    for k in ["idleShare", "goldBalance"] {
        if let Some(fh) = h.figures.get(k) {
            out.bands.insert(k.into(), fh.green);
        }
    }
    let Some(f) = f else {
        out.note = "Bericht ohne Kennzahlen (älter als WZ28.4): Kennzahl-Ampeln grau.".into();
        for k in FIGURE_KEYS {
            if let Some(fh) = h.figures.get(k) {
                out.lights.push(light_of_key(k, fh, None, "kein Messwert".into()));
            }
        }
        return out;
    };
    out.note = format!(
        "Kennzahlen aus {} gültigen Läufen; Richtwerte sind Startwerte (data/balance-hints.json).",
        f.runs
    );
    out.nights = f.nights.clone();
    out.island = f.island.clone();
    out.ship = f.ship.clone();
    out.lights
        .extend(tick_light(h, "firstWallTick", f.first_wall_tick_median));
    out.lights
        .extend(tick_light(h, "firstWorkshopTick", f.first_workshop_tick_median));
    for g in grades_of(&f.nights) {
        out.lights.extend(grade_lights(h, &g, &nights_of(&f.nights, &g)));
    }
    out
}

fn light_of_key(key: &str, fh: &FigureHint, value: Option<f64>, text: String) -> FigureLight {
    FigureLight {
        key: key.into(),
        name: fh.name.clone(),
        unit: fh.unit.clone(),
        value,
        light: LIGHT_GREY.into(),
        text,
        ..FigureLight::default()
    }
}

fn tick_light(h: &Hints, key: &str, v: Option<f64>) -> Option<FigureLight> {
    let fh = h.figures.get(key)?;
    let mut l = light_of_key(key, fh, v, "in keinem Lauf erreicht".into());
    if let Some(v) = v {
        l.light = fh.light(v).into();
        l.text = format!("Median; {}", fh.band_text());
    }
    Some(l)
}

fn grade_lights(h: &Hints, grade: &str, nights: &[NightFigures]) -> Vec<FigureLight> {
    let mut out = Vec::new();
    for key in FIGURE_KEYS {
        let Some(fh) = h.figures.get(key) else { continue };
        let Some(calc) = grade_figure(key, nights) else {
            continue;
        };
        if !grade.is_empty() && !fh.all_grades {
            continue;
        }
        let mut l = light_of_key(key, fh, None, "kein Messwert".into());
        l.grade = grade_name(grade).into();
        if let Some(v) = calc {
            let v = round1(v);
            l.value = Some(v);
            l.light = fh.light(v).into();
            l.text = fh.band_text();
        }
        if grade.is_empty() || l.light != LIGHT_GREEN {
            out.push(l);
        }
    }
    out
}

/// Die Kennzahl aus den Nächten eines Grads (Go: `gradeFigures`); äußeres `None`: keine Rechnung für den Schlüssel,
/// inneres `None`: kein Wert.
fn grade_figure(key: &str, nights: &[NightFigures]) -> Option<Option<f64>> {
    Some(match key {
        "idleShare" => mean_of(nights, 1, |n| n.idle_share),
        "goldBalance" => mean_of(nights, 2, gold_of),
        "woodBalance" => mean_of(nights, 1, |n| n.resource_in.wood - n.resource_out.wood),
        "stoneBalance" => mean_of(nights, 1, |n| n.resource_in.stone - n.resource_out.stone),
        "survivalDrop" => survival_drop(nights),
        _ => return None,
    })
}

impl FigureHint {
    /// „Richtwert grün -20 bis 60, gelb -60 bis 120“ (Go: `bandText`).
    pub fn band_text(&self) -> String {
        format!(
            "Richtwert grün {}, gelb {}",
            band_text(&self.green),
            band_text(&self.yellow)
        )
    }
}

fn band_text(b: &Band) -> String {
    match (b[0], b[1]) {
        (None, None) => "beliebig".into(),
        (None, Some(hi)) => format!("≤ {}", num(hi)),
        (Some(lo), None) => format!("≥ {}", num(lo)),
        (Some(lo), Some(hi)) => format!("{} bis {}", num(lo), num(hi)),
    }
}

/// Nach Läufen gewichtetes Mittel ab Nacht `from` (Go: `meanOf`).
fn mean_of(nights: &[NightFigures], from: i64, pick: impl Fn(&NightFigures) -> f64) -> Option<f64> {
    let (mut sum, mut runs) = (0.0, 0);
    for n in nights.iter().filter(|n| n.night >= from && n.runs > 0) {
        sum += pick(n) * n.runs as f64;
        runs += n.runs;
    }
    (runs > 0).then(|| sum / runs as f64)
}

/// Größter Abfall des Überlebens von einer Nacht zur nächsten in Prozentpunkten (vor Nacht 1: 100 %).
fn survival_drop(nights: &[NightFigures]) -> Option<f64> {
    if nights.is_empty() {
        return None;
    }
    let (mut prev, mut drop) = (100.0_f64, 0.0_f64);
    for n in nights {
        drop = drop.max(prev - n.survival);
        prev = n.survival;
    }
    Some(drop)
}

fn gold_of(n: &NightFigures) -> f64 {
    n.day_income - n.day_expense + n.night_income - n.night_expense
}

fn grades_of(nights: &[NightFigures]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for n in nights {
        if !out.contains(&n.grade) {
            out.push(n.grade.clone());
        }
    }
    out
}

/// Nächte eines Grads, nach Nacht sortiert.
fn nights_of(nights: &[NightFigures], grade: &str) -> Vec<NightFigures> {
    let mut out: Vec<NightFigures> = nights.iter().filter(|n| n.grade == grade).cloned().collect();
    out.sort_by_key(|n| n.night);
    out
}
