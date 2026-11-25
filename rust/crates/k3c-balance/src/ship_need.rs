//! Schiff: Bedarf gegen Bestand (Port von `engine/balance/ship_need.go`, BS3.3, B-638 Stufe 1). Ohne Schiff-Mechanik (BS5)
//! rechnet der Bericht aus dem Verlauf des Laufs, an welchem Tag die Bots jeden Abschnitt hätten bezahlen können: Alles Gold,
//! das die Monarchen bis dahin bekommen haben (Startgold plus Einnahmen), und alles Material, das bis dahin in den
//! Insel-Vorrat floss, deckt die kumulierten Kosten der Abschnitte bis hierher. Das ist eine Obergrenze („hätten können“), denn
//! Gold und Material gehen im Lauf auch in andere Bauten; mit dem Bau (BS5, BS9) kommt der echte Tag.

use k3c_data::buildings::Cost;
use k3c_data::hub::Stock;
use serde::{Deserialize, Serialize};

use crate::curves::median_or_nil;
use crate::fehler::Fehler;
use crate::figures::round1;
use crate::insights::{LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW};
use crate::insights_markdown::light_icon;
use crate::island_world::shared;
use crate::metrics::Metrics;
use crate::run::Report;

/// Ein Abschnitt aus data/ship.json (Go: `ShipStage`).
#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
#[serde(default)]
pub struct ShipStage {
    pub id: String,
    pub name: String,
    pub cost: Cost,
}

/// Der früheste Tag, an dem ein Abschnitt (samt den Abschnitten davor) bezahlbar gewesen wäre; `None`: nie im Lauf (Go: `ShipNeed`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ShipNeed {
    pub id: String,
    pub affordable_day: Option<i64>,
}

/// Liest die Abschnitte aus data/ship.json (BS3.2; Go: `LoadShip`).
pub fn load_ship() -> Result<Vec<ShipStage>, Fehler> {
    #[derive(Deserialize, Default)]
    #[serde(default)]
    struct File {
        stages: Vec<ShipStage>,
    }
    let wrap = |e: String| Fehler::Daten(format!("ship.json: {e}"));
    let raw = shared()?.source.read("ship.json").map_err(|e| wrap(e.to_string()))?;
    let f: File = serde_json::from_str(&raw).map_err(|e| wrap(e.to_string()))?;
    Ok(f.stages)
}

/// Rechnet je Abschnitt den Tag, ab dem die Summe aller Zuflüsse die kumulierten Kosten deckt. Tag d zählt mit allem, was bis
/// zum Ende von Tag d zugeflossen ist (Go: `shipNeeds`).
pub fn ship_needs(stages: &[ShipStage], m: &Metrics) -> Vec<ShipNeed> {
    let mut cum = Cost::default();
    stages
        .iter()
        .map(|st| {
            cum = add_ship_cost(&cum, &st.cost);
            ShipNeed {
                id: st.id.clone(),
                affordable_day: affordable_day(&cum, m),
            }
        })
        .collect()
}

fn add_ship_cost(a: &Cost, b: &Cost) -> Cost {
    Cost {
        gold: a.gold + b.gold,
        wood: a.wood + b.wood,
        stone: a.stone + b.stone,
        copper: a.copper + b.copper,
        iron: a.iron + b.iron,
        crystal: a.crystal + b.crystal,
    }
}

fn affordable_day(need: &Cost, m: &Metrics) -> Option<i64> {
    let mut gold: i64 = m.dawns.first().map_or(0, |d| d.gold.iter().sum());
    let mut inflow = Stock::default();
    for d in &m.days {
        gold += d.income;
        inflow.wood += d.stock_in.wood;
        inflow.stone += d.stock_in.stone;
        inflow.copper += d.stock_in.copper;
        inflow.iron += d.stock_in.iron;
        inflow.crystal += d.stock_in.crystal;
        if gold >= need.gold
            && inflow.wood >= need.wood
            && inflow.stone >= need.stone
            && inflow.copper >= need.copper
            && inflow.iron >= need.iron
            && inflow.crystal >= need.crystal
        {
            return Some(d.day);
        }
    }
    None
}

/// Die Abschnitte im Bericht: Median des Tags über die Läufe, in denen der Abschnitt bezahlbar war (`reached` = Anteil der
/// Läufe in %), und die Ampel gegen den Korridor „Schiff fertig“ (zielkorridore.md § 6): zwischen 2 Tagen vor und 1 Tag nach
/// dem Sieg über den Endboss (Go: `ShipFigures`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ShipFigures {
    pub stages: Vec<ShipStageFigures>,
    /// Median des Endboss-Tags, `None`: kein Lauf besiegte ihn.
    pub endboss_day: Option<f64>,
    pub light: String,
    pub text: String,
}

/// Ein Abschnitt (Go: `ShipStageFigures`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ShipStageFigures {
    pub id: String,
    pub name: String,
    pub reached: f64,
    pub day_median: Option<f64>,
}

/// Tage vor dem Endboss-Sieg, ab denen „Schiff fertig“ grün ist.
const SHIP_BEFORE: f64 = 2.0;
/// Tage danach.
const SHIP_AFTER: f64 = 1.0;

/// Verdichtet die Abschnitte der Läufe; `None`, wenn kein gültiger Lauf Schiff-Werte hat (Go: `shipFiguresOf`).
pub fn ship_figures_of(r: &Report) -> Option<ShipFigures> {
    let stages = load_ship().ok().filter(|s| !s.is_empty())?;
    let runs: Vec<&Metrics> = r
        .results
        .iter()
        .filter(|res| res.valid)
        .filter_map(|res| res.metrics.as_ref())
        .filter(|m| m.ship.len() == stages.len())
        .collect();
    if runs.is_empty() {
        return None;
    }
    let mut f = ShipFigures::default();
    for (i, st) in stages.iter().enumerate() {
        let days: Vec<f64> = runs
            .iter()
            .filter_map(|m| m.ship[i].affordable_day)
            .map(|d| d as f64)
            .collect();
        f.stages.push(ShipStageFigures {
            id: st.id.clone(),
            name: st.name.clone(),
            reached: round1(100.0 * days.len() as f64 / runs.len() as f64),
            day_median: median_or_nil(&days),
        });
    }
    let end: Vec<f64> = runs.iter().filter_map(|m| m.endboss_day).map(|d| d as f64).collect();
    f.endboss_day = median_or_nil(&end);
    (f.light, f.text) = ship_light(&f);
    Some(f)
}

/// Bewertet den letzten Abschnitt (Schiff komplett bezahlbar) gegen den Korridor um den Endboss-Tag.
fn ship_light(f: &ShipFigures) -> (String, String) {
    let Some(last) = f.stages.last() else {
        return (LIGHT_GREY.into(), String::new());
    };
    let parts: Vec<String> = f
        .stages
        .iter()
        .map(|s| format!("{} bezahlbar {}", s.name, day_label(s.day_median)))
        .collect();
    let mut text = format!("Schiff (Bedarf): {}", parts.join(", "));
    let Some(end) = f.endboss_day else {
        return (LIGHT_GREY.into(), text + " (Ziel offen: Endboss nicht besiegt)");
    };
    let (lo, hi) = (end - SHIP_BEFORE, end + SHIP_AFTER);
    text += &format!(" (Ziel T{}–T{})", day_num(lo), day_num(hi));
    let Some(d) = last.day_median else {
        let hint = "; Hinweis: Schiff in keinem Lauf bezahlbar, Einnahmen oder Schiffskosten in data/ship.json prüfen";
        return (LIGHT_RED.into(), text + hint);
    };
    if d >= lo && d <= hi {
        return (LIGHT_GREEN.into(), text);
    }
    let near = (d < lo && lo - d <= 2.0) || (d > hi && d - hi <= 2.0);
    let light = if near { LIGHT_YELLOW } else { LIGHT_RED };
    (light.into(), text + ship_hint(d < lo))
}

fn ship_hint(early: bool) -> &'static str {
    if early {
        "; Hinweis: Schiff zu früh bezahlbar, Schiffskosten in data/ship.json erhöhen"
    } else {
        "; Hinweis: Schiff zu spät bezahlbar, Schiffskosten in data/ship.json senken oder Einnahmen erhöhen"
    }
}

fn day_label(d: Option<f64>) -> String {
    d.map_or_else(|| "nie".to_string(), |d| format!("T{}", day_num(d)))
}

/// `%.1f` ohne angehängtes „.0“ (Go: `dayNum`).
fn day_num(d: f64) -> String {
    let s = format!("{d:.1}");
    s.strip_suffix(".0").map_or(s.clone(), str::to_string)
}

impl ShipFigures {
    /// Schreibt die Zeile „Schiff (Bedarf): …“ mit Ampel (Go: `markdown`).
    pub fn markdown(&self, b: &mut String) {
        b.push_str(&format!("\n{} {}\n", light_icon(&self.light), self.text));
    }
}
