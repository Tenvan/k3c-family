//! Kennzahlen im Bericht von `--targets` (WZ28.4, B-596): verdichtet über alle gültigen Läufe, je Grad und Nacht.
//! Beschreibung: `docs/rules/zielkorridore.md` › Kennzahlen im Bericht.

use std::path::Path;

use serde::{Deserialize, Serialize};

use super::figures_island::{IslandFigures, island_figures_of};
use super::report_types::{DayMetrics, Report, Stock};
use super::ship_figures::{ShipFigures, ship_figures_of};
use super::targets::b2f;

/// Kennzahlen eines Berichts. Mediane über die Läufe, in denen es den Zeitpunkt gab (`null`: in keinem).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Figures {
    /// Gültige Läufe.
    pub runs: i64,
    pub first_wall_tick_median: Option<f64>,
    pub first_workshop_tick_median: Option<f64>,
    /// Go legt die Liste leer an (`[]`); `null` entsteht nur in fremden Dateien.
    pub nights: Vec<NightFigures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub island: Option<IslandFigures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship: Option<ShipFigures>,
}

/// Mittelwerte je Lauf für Tag n samt Nacht n eines Grads (leer = normal). `survival`: Anteil der Läufe in %, in
/// denen die Burg in diesem Tag nicht fiel; `idle_share`: Anteil wartender Bauern-Ticks in %.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct NightFigures {
    pub grade: String,
    pub night: i64,
    pub runs: i64,
    pub survival: f64,
    pub day_income: f64,
    pub day_expense: f64,
    pub night_income: f64,
    pub night_expense: f64,
    pub resource_in: Resources,
    pub resource_out: Resources,
    pub idle_share: f64,
}

/// Ein Rohstoff-Fluss als Mittelwert je Lauf.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Resources {
    pub wood: f64,
    pub stone: f64,
    pub copper: f64,
    pub iron: f64,
    pub crystal: f64,
}

/// Verdichtet die gültigen Läufe; `None` ohne gültigen Lauf. Reihenfolge: Grad wie im Bericht, dann Nacht.
pub fn figures_of(root: &Path, r: &Report) -> Option<Figures> {
    let mut f = Figures::default();
    let (mut walls, mut shops) = (Vec::new(), Vec::new());
    let mut idle: Vec<(i64, i64)> = Vec::new(); // (wartend, alle) je Eintrag in f.nights
    for res in &r.results {
        let Some(m) = res.metrics.as_ref().filter(|_| res.valid) else {
            continue;
        };
        f.runs += 1;
        walls.extend(m.first_wall_tick.map(|t| t as f64));
        shops.extend(m.first_workshop_tick.map(|t| t as f64));
        let days = usize::try_from(res.scenario.days).unwrap_or(0);
        for (day, night) in m.days.iter().take(days).zip(1..) {
            let at = f.night(&res.scenario.grade, night);
            if at == idle.len() {
                idle.push((0, 0));
            }
            // `at` zeigt nach `night` und dem `push` immer auf einen Eintrag beider Listen.
            if let (Some(row), Some(sum)) = (f.nights.get_mut(at), idle.get_mut(at)) {
                row.add(day);
                sum.0 = sum.0.wrapping_add(day.peasant_idle_ticks);
                sum.1 = sum.1.wrapping_add(day.peasant_ticks);
            }
        }
    }
    if f.runs == 0 {
        return None;
    }
    f.first_wall_tick_median = median_or_nil(&walls);
    f.first_workshop_tick_median = median_or_nil(&shops);
    f.island = island_figures_of(root, r);
    f.ship = ship_figures_of(root, r);
    for (row, (waiting, all)) in f.nights.iter_mut().zip(idle) {
        row.finish(waiting, all);
    }
    Some(f)
}

pub(crate) fn median_or_nil(vals: &[f64]) -> Option<f64> {
    (!vals.is_empty()).then(|| super::evaluate::median(vals))
}

impl Figures {
    /// Index der Zeile (Grad, Nacht); legt sie bei Bedarf an.
    fn night(&mut self, grade: &str, n: i64) -> usize {
        if let Some(i) = self.nights.iter().position(|x| x.grade == grade && x.night == n) {
            return i;
        }
        self.nights.push(NightFigures {
            grade: grade.to_owned(),
            night: n,
            ..NightFigures::default()
        });
        self.nights.len() - 1
    }
}

impl NightFigures {
    /// Summiert einen Tag; `finish` teilt durch die Zahl der Läufe.
    fn add(&mut self, d: &DayMetrics) {
        self.runs += 1;
        self.survival += b2f(!d.castle_fallen);
        self.day_income += d.income.wrapping_sub(d.night_income) as f64;
        self.day_expense += d.expense.wrapping_sub(d.night_expense) as f64;
        self.night_income += d.night_income as f64;
        self.night_expense += d.night_expense as f64;
        self.resource_in.add(&d.stock_in);
        self.resource_out.add(&d.stock_out);
    }

    fn finish(&mut self, idle: i64, ticks: i64) {
        let k = self.runs as f64;
        self.survival = round1(100.0 * self.survival / k);
        for p in [
            &mut self.day_income,
            &mut self.day_expense,
            &mut self.night_income,
            &mut self.night_expense,
        ] {
            *p = round1(*p / k);
        }
        self.resource_in.div(k);
        self.resource_out.div(k);
        if ticks > 0 {
            self.idle_share = round1(100.0 * idle as f64 / ticks as f64);
        }
    }
}

impl Resources {
    fn add(&mut self, s: &Stock) {
        self.wood += s.wood as f64;
        self.stone += s.stone as f64;
        self.copper += s.copper as f64;
        self.iron += s.iron as f64;
        self.crystal += s.crystal as f64;
    }

    fn div(&mut self, k: f64) {
        for p in [
            &mut self.wood,
            &mut self.stone,
            &mut self.copper,
            &mut self.iron,
            &mut self.crystal,
        ] {
            *p = round1(*p / k);
        }
    }
}

/// Rundet auf eine Nachkommastelle (stabiles JSON); halbe Werte von der Null weg wie Go `math.Round`.
pub(crate) fn round1(x: f64) -> f64 {
    (x * 10.0).round() / 10.0
}
