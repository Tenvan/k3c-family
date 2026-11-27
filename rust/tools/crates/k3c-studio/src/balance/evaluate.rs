//! Bewertung eines Ziels (BAL2.1). Statuswerte:
//! - ok: Wert im Korridor, mehr als die Randbreite von jeder gesetzten Grenze entfernt
//! - knapp: Wert im Korridor, aber höchstens die Randbreite von einer Grenze entfernt (besteht, Ampel gelb)
//! - verletzt: Wert außerhalb des Korridors
//! - ungültig: ein Lauf des Szenarios brach ab oder es gibt keinen Messwert

use serde::{Deserialize, Serialize};

use super::report_types::{Report, RunResult, is_zero};
use super::targets::{KIND_SHARE, Target, Targets, measure};

pub const STATUS_OK: &str = "ok";
pub const STATUS_NARROW: &str = "knapp";
pub const STATUS_VIOLATED: &str = "verletzt";
pub const STATUS_INVALID: &str = "ungültig";

/// Das Urteil über ein Ziel. `value` ist bei einem Anteil in %, bei einem Median in der Einheit der Kennzahl.
/// `fail_seeds` nennt bei „verletzt“ die Seeds zum Nachspielen.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Verdict {
    #[serde(flatten)]
    pub target: Target,
    pub value: f64,
    pub status: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fail_seeds: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub invalid_seeds: Vec<String>,
    /// Zahl der gewerteten Werte (Läufe ohne Wert zählen nicht, B-657).
    #[serde(skip_serializing_if = "is_zero")]
    pub counted: i64,
}

impl Targets {
    /// Bewertet alle Ziele gegen die Läufe eines Berichts (nur Seeds aus der Datei, nur das Szenario des Ziels).
    pub fn evaluate(&self, r: &Report) -> Vec<Verdict> {
        self.targets.iter().map(|g| g.evaluate(self, &r.results)).collect()
    }
}

impl Target {
    /// Gehört der Lauf zum Szenario des Ziels (ohne Seed)?
    fn runs(&self, res: &RunResult) -> bool {
        let s = &res.scenario;
        (s.players, &s.bot, s.depth, s.days) == (self.players, &self.bot, self.depth, self.days)
            && (&s.grade, &s.world, &s.helps) == (&self.grade, &self.world, &self.helps)
    }

    /// Eine unbekannte Kennzahl (nur ohne Ladeprüfung möglich) ergibt „ungültig“; Go stürzt dort ab.
    fn evaluate(&self, t: &Targets, results: &[RunResult]) -> Verdict {
        let mut v = Verdict {
            target: self.clone(),
            ..Verdict::default()
        };
        let mut vals = Vec::new();
        let mut owner: Vec<&str> = Vec::new(); // Seed zu jedem Wert
        let kennzahl = measure(&self.measure);
        for res in results {
            let seed = &res.scenario.seed;
            if !self.runs(res) || !t.seeds.contains(seed) {
                continue;
            }
            let Some(metrics) = res.metrics.as_ref().filter(|_| res.valid) else {
                v.invalid_seeds.push(seed.clone());
                continue;
            };
            for x in kennzahl.map_or_else(Vec::new, |k| (k.values)(metrics)) {
                vals.push(x);
                owner.push(seed);
            }
        }
        if vals.is_empty() || !v.invalid_seeds.is_empty() {
            STATUS_INVALID.clone_into(&mut v.status);
            return v;
        }
        v.value = self.value(&vals);
        v.counted = i64::try_from(vals.len()).unwrap_or(i64::MAX);
        self.rate(v.value, t).clone_into(&mut v.status);
        if v.status == STATUS_VIOLATED {
            v.fail_seeds = self.fail_seeds(v.value, &vals, &owner);
        }
        v
    }

    /// Anteil in % bzw. Median der Werte.
    fn value(&self, vals: &[f64]) -> f64 {
        if self.kind == KIND_SHARE {
            let sum: f64 = vals.iter().sum();
            return 100.0 * sum / vals.len() as f64;
        }
        median(vals)
    }

    /// Die Grenzen, die für „knapp“ zählen: ohne die natürlichen Enden eines Anteils (0 % und 100 %).
    pub(crate) fn bounds(&self) -> (Option<f64>, Option<f64>) {
        if self.kind == KIND_SHARE {
            (self.lower.filter(|v| *v > 0.0), self.upper.filter(|v| *v < 100.0))
        } else {
            (self.lower, self.upper)
        }
    }

    /// Randbreite: Prozentpunkte bei Anteilen, bei Medianen der Bruchteil der Korridorbreite (eine Grenze: deren
    /// Betrag), sofern das Ziel keinen eigenen Wert hat.
    fn margin_of(&self, t: &Targets) -> f64 {
        if self.kind == KIND_SHARE {
            return self.margin.unwrap_or(t.margin.share_pp);
        }
        let width = match (self.lower, self.upper) {
            (Some(lo), Some(hi)) => hi - lo,
            (Some(lo), None) => lo.abs(),
            (None, hi) => hi.map_or(0.0, f64::abs),
        };
        self.margin.unwrap_or(t.margin.median_fraction) * width
    }

    fn rate(&self, v: f64, t: &Targets) -> &'static str {
        if self.lower.is_some_and(|lo| v < lo) || self.upper.is_some_and(|hi| v > hi) {
            return STATUS_VIOLATED;
        }
        let (lo, hi) = self.bounds();
        let m = self.margin_of(t);
        if lo.is_some_and(|lo| v - lo <= m) || hi.is_some_and(|hi| hi - v <= m) {
            return STATUS_NARROW;
        }
        STATUS_OK
    }

    fn fail_seeds(&self, value: f64, vals: &[f64], owner: &[&str]) -> Vec<String> {
        let mut seeds: Vec<String> = Vec::new();
        for (x, seed) in vals.iter().zip(owner) {
            let fails = if self.kind == KIND_SHARE {
                (*x == 0.0) == self.lower.is_some_and(|lo| value < lo)
            } else {
                self.lower.is_some_and(|lo| *x < lo) || self.upper.is_some_and(|hi| *x > hi)
            };
            if fails && !seeds.iter().any(|s| s == seed) {
                seeds.push((*seed).to_owned());
            }
        }
        seeds
    }
}

/// Median; bei gerader Anzahl der Mittelwert der beiden mittleren, ohne Werte 0 (Aufrufer prüfen vorher).
pub(crate) fn median(vals: &[f64]) -> f64 {
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    let n = s.len();
    match (s.get(n / 2), n % 2) {
        (Some(hi), 0) => (s[n / 2 - 1] + hi) / 2.0,
        (Some(mid), _) => *mid,
        (None, _) => 0.0,
    }
}
