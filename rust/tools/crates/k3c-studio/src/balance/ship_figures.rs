//! Schiff (Bedarf gegen Bestand, BS3.3) im Bericht: Median des Tags je Abschnitt über die Läufe, in denen er bezahlbar
//! war (`reached` in %), und die Ampel gegen den Korridor „Schiff fertig“ (zielkorridore.md § 6): zwischen 2 Tagen vor
//! und 1 Tag nach dem Sieg über den Endboss.

use std::fmt::Write as _;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::figures::{median_or_nil, round1};
use super::insights::{LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW};
use super::insights_markdown::light_icon;
use super::report_types::{Metrics, Report, load_ship, nil_empty};
use super::summary::go_fixed;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ShipFigures {
    #[serde(serialize_with = "nil_empty")]
    pub stages: Vec<ShipStageFigures>,
    /// Median des Endboss-Tags, `null`: kein Lauf besiegte ihn.
    pub endboss_day: Option<f64>,
    pub light: String,
    pub text: String,
}

/// Ein Abschnitt.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ShipStageFigures {
    pub id: String,
    pub name: String,
    pub reached: f64,
    pub day_median: Option<f64>,
}

const SHIP_BEFORE: f64 = 2.0; // Tage vor dem Endboss-Sieg, ab denen „Schiff fertig“ grün ist
const SHIP_AFTER: f64 = 1.0; // Tage danach

/// Verdichtet die Abschnitte der Läufe; `None`, wenn kein gültiger Lauf Schiff-Werte hat.
pub(crate) fn ship_figures_of(root: &Path, r: &Report) -> Option<ShipFigures> {
    let stages = load_ship(root).ok().filter(|s| !s.is_empty())?;
    let runs: Vec<&Metrics> = r
        .results
        .iter()
        .filter(|res| res.valid)
        .filter_map(|res| res.metrics.as_ref())
        .filter(|m| m.island.ship.len() == stages.len())
        .collect();
    if runs.is_empty() {
        return None;
    }
    let mut f = ShipFigures::default();
    for (i, st) in stages.iter().enumerate() {
        let days: Vec<f64> = runs
            .iter()
            .filter_map(|m| m.island.ship.get(i)?.affordable_day)
            .map(|d| d as f64)
            .collect();
        f.stages.push(ShipStageFigures {
            id: st.id.clone(),
            name: st.name.clone(),
            reached: round1(100.0 * days.len() as f64 / runs.len() as f64),
            day_median: median_or_nil(&days),
        });
    }
    let end: Vec<f64> = runs
        .iter()
        .filter_map(|m| m.island.endboss_day.map(|d| d as f64))
        .collect();
    f.endboss_day = median_or_nil(&end);
    let (light, text) = ship_light(&f);
    light.clone_into(&mut f.light);
    f.text = text;
    Some(f)
}

/// Bewertet den letzten Abschnitt (Schiff komplett bezahlbar) gegen den Korridor um den Endboss-Tag.
fn ship_light(f: &ShipFigures) -> (&'static str, String) {
    let parts: Vec<String> = f
        .stages
        .iter()
        .map(|s| format!("{} bezahlbar {}", s.name, day_label(s.day_median)))
        .collect();
    let mut text = format!("Schiff (Bedarf): {}", parts.join(", "));
    let Some(end) = f.endboss_day else {
        return (LIGHT_GREY, text + " (Ziel offen: Endboss nicht besiegt)");
    };
    let (lo, hi) = (end - SHIP_BEFORE, end + SHIP_AFTER);
    let _ = write!(text, " (Ziel T{}–T{})", day_num(lo), day_num(hi));
    let Some(d) = f.stages.last().and_then(|s| s.day_median) else {
        return (
            LIGHT_RED,
            text + "; Hinweis: Schiff in keinem Lauf bezahlbar, Einnahmen oder Schiffskosten in data/ship.json prüfen",
        );
    };
    if d >= lo && d <= hi {
        return (LIGHT_GREEN, text);
    }
    let near = (d < lo && lo - d <= 2.0) || (d > hi && d - hi <= 2.0);
    (if near { LIGHT_YELLOW } else { LIGHT_RED }, text + ship_hint(d < lo))
}

fn ship_hint(early: bool) -> &'static str {
    if early {
        "; Hinweis: Schiff zu früh bezahlbar, Schiffskosten in data/ship.json erhöhen"
    } else {
        "; Hinweis: Schiff zu spät bezahlbar, Schiffskosten in data/ship.json senken oder Einnahmen erhöhen"
    }
}

fn day_label(d: Option<f64>) -> String {
    d.map_or_else(|| "nie".to_owned(), |d| format!("T{}", day_num(d)))
}

fn day_num(d: f64) -> String {
    let text = go_fixed(d, Some(1));
    text.strip_suffix(".0").map(str::to_owned).unwrap_or(text)
}

impl ShipFigures {
    /// Schreibt die Zeile „Schiff (Bedarf): …“ mit Ampel.
    pub fn markdown(&self, b: &mut String) {
        let _ = write!(b, "\n{} {}\n", light_icon(&self.light), self.text);
    }
}
