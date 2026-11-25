//! Automatisches Balancing (Port von `engine/balance/tune.go`, WZ28.6, B-596): Sweep (volles Raster) oder Suche über ein bis
//! zwei Werte aus data/; jeder Punkt spielt alle Ziele mit den Werten als Varianten im Speicher. Ergebnis: alle Punkte mit
//! Score, der beste Punkt, eine Begründung und ein Diff zur Übernahme von Hand. data/ wird nie geschrieben.

use k3c_data::balance_targets::Targets;
use serde::{Deserialize, Serialize};

use crate::evaluate::STATUS_INVALID;
use crate::fehler::Fehler;
use crate::summary::{Entry, Summarize, Summary};
use crate::summary_text::num;
use crate::targets::KIND_SHARE;
use crate::tune_diff::proposal_diff;
use crate::tuneparam::{MAX_TUNE_PARAMS, TuneParam, parse_tune_param};
use crate::variant::with_variants;

/// Modus: volles Raster.
pub const TUNE_SWEEP: &str = "sweep";
/// Modus: Suche (ein Parameter Ternärsuche, zwei Parameter Raster mit Verfeinerung).
pub const TUNE_SEARCH: &str = crate::tuneparam::TUNE_SEARCH;

/// Vorgaben und Grenzen der CLI-Flags `--steps` und `--budget`.
pub const DEFAULT_TUNE_STEPS: i64 = 5;
pub const MAX_TUNE_STEPS: i64 = 21;
pub const DEFAULT_TUNE_BUDGET: i64 = 40;

/// Fortschritt: fertige und höchstens alle Auswertungen.
pub type TuneProgress = Box<dyn FnMut(usize, usize) + Send>;

/// Beschreibt einen Lauf. `seeds`: die ersten n Seeds der Ziele (0 = alle). `variants` gelten für jeden Punkt, die Parameter
/// (`datei.json:a.b.c=min..max`) darüber; `steps` und `budget` 0 = Vorgabe.
#[derive(Default)]
pub struct TuneOptions {
    pub mode: String,
    pub params: Vec<String>,
    pub steps: i64,
    pub budget: i64,
    pub seeds: i64,
    pub variants: Vec<String>,
    pub progress: Option<TuneProgress>,
}

/// Ein ausgewerteter Punkt. `score`: Summe der normierten Abstände außerhalb der Korridore (0 = alle getroffen); `failed`
/// nennt die Ziele ohne Pass.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TunePoint {
    pub values: Vec<f64>,
    pub score: f64,
    pub pass: bool,
    pub failed: Vec<String>,
}

/// Der beste Punkt (kleinster Score, bei Gleichstand der erste).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TuneBest {
    pub values: Vec<f64>,
    pub score: f64,
    pub pass: bool,
}

/// Der Inhalt von `reports/…/tuning-<Zeit>.json`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Tuning {
    pub version: i64,
    pub mode: String,
    pub params: Vec<TuneParam>,
    pub points: Vec<TunePoint>,
    pub best: TuneBest,
    pub reason: String,
    pub diff: String,
}

impl Tuning {
    /// Schreibt das Ergebnis eingerückt mit Zeilenende (Go: `JSON`).
    pub fn json(&self) -> Result<String, Fehler> {
        let mut s = serde_json::to_string_pretty(self).map_err(|e| Fehler::Daten(format!("balance: {e}")))?;
        s.push('\n');
        Ok(s)
    }
}

/// Stimmt die Parameter gegen die Ziele ab (Go: `Targets.Tune`). Ungültige Optionen oder Parameter sind Fehler, verfehlte
/// Ziele nicht.
pub trait Tune {
    fn tune(&self, o: TuneOptions) -> Result<Tuning, Fehler>;
}

impl Tune for Targets {
    fn tune(&self, o: TuneOptions) -> Result<Tuning, Fehler> {
        let mut tu = Tuner::new(self, o)?;
        if tu.mode == TUNE_SWEEP {
            tu.evaluate_grid(&tu.axes(None, 0.0))?;
        } else {
            tu.search()?;
        }
        tu.result()
    }
}

/// Wertet Punkte aus, merkt sie (keine doppelte Auswertung) und hält Budget und Abbruch bei Score 0.
struct Tuner<'a> {
    t: &'a Targets,
    mode: String,
    steps: usize,
    budget: usize,
    seeds: i64,
    variants: Vec<String>,
    progress: Option<TuneProgress>,
    params: Vec<TuneParam>,
    points: Vec<TunePoint>,
    sums: Vec<Summary>,
    total: usize,
}

fn invalid(why: String) -> Fehler {
    Fehler::Daten(format!("tune: {why}"))
}

impl<'a> Tuner<'a> {
    fn new(t: &'a Targets, o: TuneOptions) -> Result<Self, Fehler> {
        let steps = if o.steps == 0 { DEFAULT_TUNE_STEPS } else { o.steps };
        let budget = if o.budget == 0 { DEFAULT_TUNE_BUDGET } else { o.budget };
        if o.mode != TUNE_SWEEP && o.mode != TUNE_SEARCH {
            return Err(invalid(format!(
                "modus {:?}, erwartet {TUNE_SWEEP} oder {TUNE_SEARCH}",
                o.mode
            )));
        }
        if o.params.is_empty() || o.params.len() > MAX_TUNE_PARAMS {
            return Err(invalid(format!(
                "1 bis {MAX_TUNE_PARAMS} Parameter, waren {}",
                o.params.len()
            )));
        }
        if !(2..=MAX_TUNE_STEPS).contains(&steps) {
            return Err(invalid(format!("steps 2 bis {MAX_TUNE_STEPS}, war {steps}")));
        }
        if budget < 1 {
            return Err(invalid(format!("budget ab 1, war {budget}")));
        }
        let params = o
            .params
            .iter()
            .map(|s| parse_tune_param(s))
            .collect::<Result<Vec<_>, _>>()?;
        let (steps, budget) = (steps as usize, budget as usize);
        let total = if o.mode == TUNE_SWEEP {
            budget.min(steps.pow(params.len() as u32))
        } else {
            budget
        };
        Ok(Tuner {
            t,
            mode: o.mode,
            steps,
            budget,
            seeds: o.seeds,
            variants: o.variants,
            progress: o.progress,
            params,
            points: Vec::new(),
            sums: Vec::new(),
            total,
        })
    }

    /// Budget erschöpft oder ein Punkt trifft alle Korridore.
    fn done(&self) -> bool {
        self.points.len() >= self.budget || self.points.iter().any(|p| p.score == 0.0)
    }

    /// Wertet einen Punkt aus (gerundet wie in der Datei) und liefert seinen Score; ein bekannter Punkt kostet nichts.
    fn eval(&mut self, values: &[f64]) -> Result<f64, Fehler> {
        let vals: Vec<f64> = self
            .params
            .iter()
            .zip(values)
            .map(|(p, &v)| p.norm(v.max(p.min).min(p.max)))
            .collect();
        if let Some(p) = self.points.iter().find(|p| p.values == vals) {
            return Ok(p.score);
        }
        if self.points.len() >= self.budget {
            return Ok(f64::INFINITY);
        }
        let mut specs = self.variants.clone();
        specs.extend(self.params.iter().zip(&vals).map(|(p, &v)| p.variant(v)));
        let rep = with_variants(&specs, || self.t.run_targets(self.seeds))?;
        let sum = self.t.summarize(&rep);
        let failed = sum
            .targets
            .iter()
            .filter(|e| !e.pass)
            .map(|e| e.verdict.target.name.clone())
            .collect();
        let pt = TunePoint {
            values: vals,
            score: score(&sum),
            pass: sum.pass,
            failed,
        };
        let s = pt.score;
        self.points.push(pt);
        self.sums.push(sum);
        if let Some(f) = &mut self.progress {
            f(self.points.len(), self.total);
        }
        Ok(s)
    }

    /// Rasterwerte je Parameter; mit `center` das Raster center ± cell (3 Werte je Achse), sonst min..max in `steps`.
    fn axes(&self, center: Option<&[f64]>, cell: f64) -> Vec<Vec<f64>> {
        let mut out = Vec::new();
        for (i, p) in self.params.iter().enumerate() {
            out.push(match center {
                None => grid(p.min, p.max, self.steps),
                Some(c) => {
                    let d = cell * (p.max - p.min);
                    vec![c[i] - d, c[i], c[i] + d]
                }
            });
        }
        out
    }

    /// Wertet alle Kombinationen der Achsen aus, bis das Budget erschöpft ist (Score 0 bricht nur bei search ab).
    fn evaluate_grid(&mut self, axes: &[Vec<f64>]) -> Result<(), Fehler> {
        let mut combos: Vec<Vec<f64>> = vec![Vec::new()];
        for axis in axes {
            combos = combos
                .iter()
                .flat_map(|c| {
                    axis.iter().map(move |&v| {
                        let mut n = c.clone();
                        n.push(v);
                        n
                    })
                })
                .collect();
        }
        for c in combos {
            if self.points.len() >= self.budget || (self.mode == TUNE_SEARCH && self.done()) {
                return Ok(());
            }
            self.eval(&c)?;
        }
        Ok(())
    }

    /// Ein Parameter Ternärsuche auf dem Score, zwei Parameter Raster und zwei Verfeinerungsrunden um den besten Punkt
    /// (halbe Zellbreite je Runde). Abbruch bei Score 0 oder Budget.
    fn search(&mut self) -> Result<(), Fehler> {
        if self.params.len() == 1 {
            return self.ternary();
        }
        let axes = self.axes(None, 0.0);
        self.evaluate_grid(&axes)?;
        let mut cell = 1.0 / (self.steps - 1) as f64;
        for _ in 0..2 {
            cell /= 2.0;
            let center = self.best().values.clone();
            let axes = self.axes(Some(&center), cell);
            self.evaluate_grid(&axes)?;
        }
        Ok(())
    }

    /// Sucht das Minimum des Scores über [min, max]: erst beide Enden, dann je Runde die Drittelpunkte.
    fn ternary(&mut self) -> Result<(), Fehler> {
        let p = self.params[0].clone();
        let (mut lo, mut hi) = (p.min, p.max);
        for v in [lo, hi] {
            self.eval(&[v])?;
        }
        while !self.done() && hi - lo > (p.max - p.min) * 1e-3 && !(p.int && hi - lo < 1.0) {
            let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
            let s1 = self.eval(&[m1])?;
            let s2 = self.eval(&[m2])?;
            if s1 <= s2 {
                hi = m2;
            } else {
                lo = m1;
            }
        }
        Ok(())
    }

    fn best(&self) -> &TunePoint {
        let mut best = &self.points[0];
        for p in &self.points {
            if p.score < best.score {
                best = p;
            }
        }
        best
    }

    fn result(&self) -> Result<Tuning, Fehler> {
        if self.points.is_empty() {
            return Err(invalid("kein Punkt ausgewertet".into()));
        }
        let b = self.best();
        Ok(Tuning {
            version: 1,
            mode: self.mode.clone(),
            params: self.params.clone(),
            points: self.points.clone(),
            best: TuneBest {
                values: b.values.clone(),
                score: b.score,
                pass: b.pass,
            },
            reason: self.reason(b),
            diff: proposal_diff(&self.params, &b.values)?,
        })
    }

    /// Erklärt den besten Punkt in 1–3 Sätzen: Werte, getroffene und verfehlte Korridore mit Abstand.
    fn reason(&self, b: &TunePoint) -> String {
        let at = self
            .points
            .iter()
            .position(|p| p.values == b.values)
            .unwrap_or_default();
        let sum = &self.sums[at];
        let set: Vec<String> = self
            .params
            .iter()
            .zip(&b.values)
            .map(|(p, &v)| format!("{}:{} = {} (bisher {})", p.file, p.path, p.format(v), p.format(p.base)))
            .collect();
        let head = format!(
            "Bester von {} Punkten ({}): {}.",
            self.points.len(),
            self.mode,
            set.join(", ")
        );
        let missed: Vec<String> = sum
            .targets
            .iter()
            .filter(|e| !e.pass)
            .map(|e| format!("{} {} statt {}", e.verdict.target.name, e.value_text(), e.bounds_text()))
            .collect();
        let hit = sum.targets.len() - missed.len();
        if missed.is_empty() {
            return format!("{head} Alle {hit} Korridore getroffen (Score 0).");
        }
        format!(
            "{head} {hit} von {} Korridoren getroffen, Score {}. Verfehlt: {}.",
            sum.targets.len(),
            num(b.score),
            missed.join("; ")
        )
    }
}

/// Summe der normierten Abstände außerhalb der Korridore. Anteile durch 100 Prozentpunkte, Mediane durch die Korridorbreite
/// (bei einer Grenze ihr Betrag, mindestens 1); ein ungültiges Ziel zählt 1. Gerundet auf 4 Stellen.
fn score(s: &Summary) -> f64 {
    let mut total = 0.0;
    for e in &s.targets {
        let (t, v) = (&e.verdict.target, e.verdict.value);
        if e.verdict.status == STATUS_INVALID {
            total += 1.0;
        } else if t.lower.is_some_and(|lo| v < lo) {
            total += (t.lower.unwrap_or_default() - v) / scale(e);
        } else if t.upper.is_some_and(|hi| v > hi) {
            total += (v - t.upper.unwrap_or_default()) / scale(e);
        }
    }
    (total * 1e4).round() / 1e4
}

fn scale(e: &Entry) -> f64 {
    let t = &e.verdict.target;
    if t.kind == KIND_SHARE {
        return 100.0;
    }
    if let (Some(lo), Some(hi)) = (t.lower, t.upper)
        && hi > lo
    {
        return hi - lo;
    }
    t.lower.or(t.upper).unwrap_or_default().abs().max(1.0)
}

/// `steps` Werte gleichmäßig von `lo` bis `hi`.
fn grid(lo: f64, hi: f64, steps: usize) -> Vec<f64> {
    (0..steps)
        .map(|i| lo + (hi - lo) * i as f64 / (steps - 1) as f64)
        .collect()
}
