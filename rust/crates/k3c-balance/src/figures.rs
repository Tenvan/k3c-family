//! Kennzahlen im Bericht von `--targets` (Port von `engine/balance/figures.go`, WZ28.4, B-596): verdichtet über alle
//! gültigen Läufe des Berichts, je Grad und Nacht. Beschreibung: docs/rules/zielkorridore.md › Kennzahlen im Bericht;
//! Korridore gibt es dafür (noch) nicht.

use serde::{Deserialize, Serialize};

use crate::economy_measures::b2f;
use crate::figures_island::{IslandFigures, island_figures_of};
use crate::metrics::DayMetrics;
use crate::pending::{ShipFigures, median_or_nil, ship_figures_of};
use crate::run::Report;

/// Die Kennzahlen eines Berichts (Go: `Figures`). Mediane über die Läufe, in denen es den Zeitpunkt gab (null: in keinem).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Figures {
    /// Gültige Läufe.
    pub runs: i64,
    #[serde(rename = "firstWallTickMedian")]
    pub first_wall_tick_median: Option<f64>,
    #[serde(rename = "firstWorkshopTickMedian")]
    pub first_workshop_tick_median: Option<f64>,
    pub nights: Vec<NightFigures>,
    /// Nur bei Läufen mit Welt (BS11).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub island: Option<IslandFigures>,
    /// Bedarf gegen Bestand (BS3.3), nur bei Läufen mit Welt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship: Option<ShipFigures>,
}

/// Mittelwerte je Lauf für Tag n samt Nacht n eines Grads (leer = normal; Go: `NightFigures`). `survival` ist der Anteil
/// der Läufe in %, in denen die Burg in diesem Tag nicht fiel; `idle_share` der Anteil wartender Bauern-Ticks in %.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
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

/// Ein Rohstoff-Fluss als Mittelwert je Lauf (Go: `Resources`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Resources {
    pub wood: f64,
    pub stone: f64,
    pub copper: f64,
    pub iron: f64,
    pub crystal: f64,
}

/// Verdichtet die gültigen Läufe; `None` ohne gültigen Lauf. Reihenfolge: Grad wie im Bericht, dann Nacht (Go: `FiguresOf`).
pub fn figures_of(r: &Report) -> Option<Figures> {
    let mut f = Figures::default();
    let (mut walls, mut shops): (Vec<f64>, Vec<f64>) = (Vec::new(), Vec::new());
    let (mut idle, mut ticks): (Vec<i64>, Vec<i64>) = (Vec::new(), Vec::new()); // je Eintrag in f.nights
    for res in &r.results {
        let Some(m) = res.metrics.as_ref().filter(|_| res.valid) else {
            continue;
        };
        f.runs += 1;
        append_tick(&mut walls, m.first_wall_tick);
        append_tick(&mut shops, m.first_workshop_tick);
        let n = m.days.len().min(usize::try_from(res.scenario.days).unwrap_or(0));
        for (i, d) in m.days.iter().take(n).enumerate() {
            let at = f.night(&res.scenario.grade, i as i64 + 1);
            if at == idle.len() {
                idle.push(0);
                ticks.push(0);
            }
            f.nights[at].add(d);
            idle[at] += d.peasant_idle_ticks;
            ticks[at] += d.peasant_ticks;
        }
    }
    if f.runs == 0 {
        return None;
    }
    f.first_wall_tick_median = median_or_nil(&walls);
    f.first_workshop_tick_median = median_or_nil(&shops);
    f.island = island_figures_of(r);
    f.ship = ship_figures_of(r);
    for (i, n) in f.nights.iter_mut().enumerate() {
        n.finish(idle[i], ticks[i]);
    }
    Some(f)
}

impl Figures {
    /// Der Index der Zeile (Grad, Nacht); sie wird bei Bedarf angelegt (Go: `Figures.night`).
    fn night(&mut self, grade: &str, n: i64) -> usize {
        if let Some(i) = self.nights.iter().position(|x| x.grade == grade && x.night == n) {
            return i;
        }
        self.nights.push(NightFigures {
            grade: grade.to_string(),
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
        self.day_income += (d.income - d.night_income) as f64;
        self.day_expense += (d.expense - d.night_expense) as f64;
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
    fn add(&mut self, s: &k3c_data::hub::Stock) {
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

fn append_tick(v: &mut Vec<f64>, tick: Option<i64>) {
    if let Some(t) = tick {
        v.push(t as f64);
    }
}

/// Rundet auf eine Nachkommastelle (stabiles JSON; Go: `round1`).
pub fn round1(x: f64) -> f64 {
    (x * 10.0).round() / 10.0
}
