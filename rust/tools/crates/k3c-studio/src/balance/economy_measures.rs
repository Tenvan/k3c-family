//! Kennzahlen der Wirtschafts-Korridore (BAL6, B-347; `docs/rules/zielkorridore.md` › BR1). Zeitgrenzen hängen an Tag
//! und Phase: „vor Ende der hellen Phase von Tag n“ = bis Dämmerungsbeginn von Tag n, „vor Tag n“ = vor dem
//! Tagesbeginn von Tag n. Fehlt ein Tagesbeginn, weil das Szenario kürzer ist, zählt bei „vor Tag n“ jeder Zeitpunkt
//! im Lauf, die übrigen Bedingungen gelten dann als nicht erfüllt.

use super::report_types::{DawnMetrics, Metrics};
use super::targets::{KIND_MEDIAN, KIND_SHARE, Measure, b2f};

// Schwellen aus der Tabelle der Korridore.
const WOOD_AT_DAWN_MIN: i64 = 100; // Holz am Tagesbeginn
const WOOD_FROM_DAY: i64 = 3;
const FIGHTERS_DAY_EARLY: i64 = 3; // Kämpfer je Hub zu Beginn dieses Tages …
const FIGHTERS_EARLY: i64 = 4; // … mindestens
const FIGHTERS_DAY_LATE: i64 = 6;
const FIGHTERS_LATE: i64 = 8;

/// Die Kennzahlen der Wirtschaft; [`super::targets::measure`] nimmt sie zu den übrigen.
pub(crate) const ECONOMY_MEASURES: [Measure; 6] = [
    // Gold je Monarch zu jedem Tagesbeginn ab Tag 2 innerhalb des Szenarios (Tag 1 ist das Startgold).
    Measure::new("goldAtDawnPerPlayer", KIND_MEDIAN, gold_at_dawn),
    // Erster Turm gebaut bis Dämmerungsbeginn von Tag 2.
    Measure::new("firstTowerBeforeDusk2", KIND_SHARE, first_tower_before_dusk2),
    // Holz im Vorrat zu jedem Tagesbeginn ab Tag 3 mindestens 100.
    Measure::new("woodAtDawnFrom3", KIND_SHARE, wood_at_dawn),
    Measure::new("hubLevel2BeforeDay5", KIND_SHARE, hub_level2_before_day5),
    Measure::new("hubLevel3BeforeDay10", KIND_SHARE, hub_level3_before_day10),
    // Kämpfer der Stufe zu Beginn von Tag 3 mindestens 4 und zu Beginn von Tag 6 mindestens 8.
    Measure::new("fightersAtDawn", KIND_SHARE, fighters_at_dawn),
];

fn first_tower_before_dusk2(m: &Metrics) -> Vec<f64> {
    vec![b2f(before_dusk(m, m.first_tower_tick, 2))]
}

fn hub_level2_before_day5(m: &Metrics) -> Vec<f64> {
    vec![b2f(before_dawn(m, m.hub_level2_tick, 5))]
}

fn hub_level3_before_day10(m: &Metrics) -> Vec<f64> {
    vec![b2f(before_dawn(m, m.hub_level3_tick, 10))]
}

fn fighters_at_dawn(m: &Metrics) -> Vec<f64> {
    let early = dawn_of(m, FIGHTERS_DAY_EARLY).is_some_and(|d| d.fighters >= FIGHTERS_EARLY);
    let late = dawn_of(m, FIGHTERS_DAY_LATE).is_some_and(|d| d.fighters >= FIGHTERS_LATE);
    vec![b2f(early && late)]
}

fn gold_at_dawn(m: &Metrics) -> Vec<f64> {
    let last = last_day(m);
    m.dawns
        .iter()
        .filter(|d| d.day >= 2 && d.day <= last)
        .flat_map(|d| d.gold.iter().map(|g| *g as f64))
        .collect()
}

fn wood_at_dawn(m: &Metrics) -> Vec<f64> {
    let last = last_day(m);
    let mut seen = false;
    for d in m.dawns.iter().filter(|d| d.day >= WOOD_FROM_DAY && d.day <= last) {
        seen = true;
        if d.stock.wood < WOOD_AT_DAWN_MIN {
            return vec![0.0];
        }
    }
    vec![b2f(seen)]
}

/// Letzter Tag des Szenarios: ein gültiger Lauf endet mit dem Tagesbeginn danach (letzter Eintrag in `dawns`).
fn last_day(m: &Metrics) -> i64 {
    m.dawns.last().map_or(0, |d| d.day.wrapping_sub(1))
}

/// Liegt `tick` spätestens beim Dämmerungsbeginn von Tag `day` (ab 1)?
fn before_dusk(m: &Metrics, tick: Option<i64>, day: usize) -> bool {
    let dusk = day.checked_sub(1).and_then(|i| m.days.get(i)).and_then(|d| d.dusk_tick);
    matches!((tick, dusk), (Some(t), Some(dusk)) if t <= dusk)
}

/// Liegt `tick` vor dem Tagesbeginn von Tag `day`? Endet der Lauf vorher, genügt, dass es ihn im Lauf gab.
fn before_dawn(m: &Metrics, tick: Option<i64>, day: i64) -> bool {
    tick.is_some_and(|t| dawn_of(m, day).is_none_or(|d| t < d.tick))
}

/// Stand zu Beginn von Tag `day`, `None` ohne diesen Tagesbeginn im Lauf.
pub(crate) fn dawn_of(m: &Metrics, day: i64) -> Option<&DawnMetrics> {
    m.dawns.iter().find(|d| d.day == day)
}
