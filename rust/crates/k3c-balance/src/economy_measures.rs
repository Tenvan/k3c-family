//! Kennzahlen der Wirtschafts-Korridore (Port von `engine/balance/economy_measures.go`, BAL6, B-347;
//! docs/rules/zielkorridore.md › BR1 › Korridore der Wirtschaft). Zeitgrenzen hängen an Tag und Phase: „vor Ende der
//! hellen Phase von Tag n“ = bis Dämmerungsbeginn von Tag n, „vor Tag n“ = vor dem Tagesbeginn von Tag n (Ereignis
//! `dawn`). Fehlt ein Tagesbeginn, weil das Szenario kürzer ist, zählt bei „vor Tag n“ jeder Zeitpunkt im Lauf, die übrigen
//! Bedingungen gelten dann als nicht erfüllt; die Ziele in data/balance-targets.json wählen die Tage passend.

use crate::metrics::{DawnMetrics, Metrics};

/// Holz am Tagesbeginn ≥ 100.
const WOOD_AT_DAWN_MIN: i64 = 100;
/// Ab Tag 3.
const WOOD_FROM_DAY: i64 = 3;
/// Kämpfer je Hub zu Beginn von Tag 3 …
const FIGHTERS_DAY_EARLY: i64 = 3;
/// … mindestens 4
const FIGHTERS_EARLY: i64 = 4;
/// und zu Beginn von Tag 6 …
const FIGHTERS_DAY_LATE: i64 = 6;
/// … mindestens 8.
const FIGHTERS_LATE: i64 = 8;

/// Die Art einer Kennzahl: Anteil der Läufe (0/1 je Lauf) oder Median der Werte (Go: `KindShare`, `KindMedian`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasureKind {
    Share,
    Median,
}

/// Die Schlüssel der Wirtschafts-Kennzahlen (Go: `economyMeasures`; `targets.rs` nimmt sie in seine Tabelle auf).
pub const ECONOMY_MEASURES: [&str; 6] = [
    "goldAtDawnPerPlayer",
    "firstTowerBeforeDusk2",
    "woodAtDawnFrom3",
    "hubLevel2BeforeDay5",
    "hubLevel3BeforeDay10",
    "fightersAtDawn",
];

/// Art und Werte der Wirtschafts-Kennzahl `key` für einen Lauf; `None` für einen fremden Schlüssel.
pub fn economy_measure(key: &str, m: &Metrics) -> Option<(MeasureKind, Vec<f64>)> {
    let share = |b: bool| Some((MeasureKind::Share, vec![b2f(b)]));
    match key {
        // Gold je Monarch zu jedem Tagesbeginn ab Tag 2 innerhalb des Szenarios (Tag 1 ist das Startgold).
        "goldAtDawnPerPlayer" => Some((MeasureKind::Median, gold_at_dawn(m))),
        // Erster Turm gebaut bis Dämmerungsbeginn von Tag 2.
        "firstTowerBeforeDusk2" => share(before_dusk(m, m.first_tower_tick, 2)),
        // Holz im Vorrat zu jedem Tagesbeginn ab Tag 3 innerhalb des Szenarios mindestens 100.
        "woodAtDawnFrom3" => Some((MeasureKind::Share, wood_at_dawn(m))),
        // Hub-Stufe 2 vor Tag 5, Hub-Stufe 3 vor Tag 10.
        "hubLevel2BeforeDay5" => share(before_dawn(m, m.hub_level2_tick, 5)),
        "hubLevel3BeforeDay10" => share(before_dawn(m, m.hub_level3_tick, 10)),
        // Kämpfer der Stufe zu Beginn von Tag 3 mindestens 4 und zu Beginn von Tag 6 mindestens 8.
        "fightersAtDawn" => {
            let (early, late) = (dawn_of(m, FIGHTERS_DAY_EARLY), dawn_of(m, FIGHTERS_DAY_LATE));
            share(
                matches!((early, late), (Some(e), Some(l)) if e.fighters >= FIGHTERS_EARLY && l.fighters >= FIGHTERS_LATE),
            )
        }
        _ => None,
    }
}

/// Go: `b2f`.
pub fn b2f(b: bool) -> f64 {
    if b { 1.0 } else { 0.0 }
}

fn gold_at_dawn(m: &Metrics) -> Vec<f64> {
    let last = last_day(m);
    m.dawns
        .iter()
        .filter(|d| d.day >= 2 && d.day <= last)
        .flat_map(|d| d.gold.iter().map(|&g| g as f64))
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

/// Der letzte Tag des Szenarios: Ein gültiger Lauf endet mit dem Tagesbeginn danach (letzter Eintrag in `dawns`); `days`
/// hat dafür schon eine Zeile mehr.
pub fn last_day(m: &Metrics) -> i64 {
    m.dawns.last().map_or(0, |d| d.day - 1)
}

/// Der Zeitpunkt `tick` liegt spätestens beim Dämmerungsbeginn von Tag `day`.
pub fn before_dusk(m: &Metrics, tick: Option<i64>, day: i64) -> bool {
    let dusk = usize::try_from(day - 1)
        .ok()
        .and_then(|i| m.days.get(i))
        .and_then(|d| d.dusk_tick);
    matches!((tick, dusk), (Some(t), Some(d)) if t <= d)
}

/// Der Zeitpunkt `tick` liegt vor dem Tagesbeginn von Tag `day`; endet der Lauf vorher, genügt es, dass es ihn im Lauf gab
/// (der Lauf endet mit dem Morgen nach dem letzten Tag, also vor Tag `day`).
pub fn before_dawn(m: &Metrics, tick: Option<i64>, day: i64) -> bool {
    let Some(t) = tick else { return false };
    dawn_of(m, day).is_none_or(|d| t < d.tick)
}

/// Der Stand zu Beginn von Tag `day`, `None` ohne diesen Tagesbeginn im Lauf.
pub fn dawn_of(m: &Metrics, day: i64) -> Option<&DawnMetrics> {
    m.dawns.iter().find(|d| d.day == day)
}
