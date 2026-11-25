//! Bewertung der Ziele gegen die Läufe eines Berichts (Port von `engine/balance/evaluate.go`, BAL2.1).

use k3c_data::balance_targets::{Target, Targets};
use serde::{Deserialize, Serialize};

use crate::pending::measure_values;
use crate::run::{Report, RunResult};

/// Wert im Korridor, mehr als die Randbreite von jeder gesetzten Grenze entfernt.
pub const STATUS_OK: &str = "ok";
/// Wert im Korridor, aber höchstens die Randbreite von einer Grenze entfernt (besteht, Ampel gelb).
pub const STATUS_NARROW: &str = "knapp";
/// Wert außerhalb des Korridors.
pub const STATUS_VIOLATED: &str = "verletzt";
/// Ein Lauf des Szenarios brach ab oder es gibt keinen Messwert; der Wert ist dann nicht belastbar.
pub const STATUS_INVALID: &str = "ungültig";

/// Die Art „Anteil“ eines Ziels (Go: `KindShare`).
const KIND_SHARE: &str = "share";

/// Das Urteil über ein Ziel (Go: `Verdict`). `value` ist bei einem Anteil in %, bei einem Median in der Einheit der
/// Kennzahl. `fail_seeds` nennt bei „verletzt“ die Seeds zum Nachspielen (Replay): bei einem Anteil unter der Untergrenze
/// die Seeds ohne erfüllte Bedingung, über der Obergrenze die mit erfüllter, bei einem Median die Seeds mit Wert außerhalb.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Verdict {
    #[serde(flatten)]
    pub target: Target,
    pub value: f64,
    pub status: String,
    #[serde(rename = "failSeeds", skip_serializing_if = "Vec::is_empty")]
    pub fail_seeds: Vec<String>,
    #[serde(rename = "invalidSeeds", skip_serializing_if = "Vec::is_empty")]
    pub invalid_seeds: Vec<String>,
    /// Zahl der gewerteten Werte (Läufe ohne Wert, z. B. castleHeld ohne Fenster, zählen nicht; B-657).
    #[serde(skip_serializing_if = "is_zero")]
    pub counted: i64,
}

fn is_zero(v: &i64) -> bool {
    *v == 0
}

/// Bewertet die Ziele einer Datei (Go: `Targets.Evaluate`).
pub trait Evaluate {
    /// Bewertet alle Ziele gegen die Läufe eines Berichts (nur Seeds aus der Datei, nur das Szenario des Ziels).
    fn evaluate(&self, r: &Report) -> Vec<Verdict>;
}

impl Evaluate for Targets {
    fn evaluate(&self, r: &Report) -> Vec<Verdict> {
        self.targets
            .iter()
            .map(|g| evaluate_target(g, self, &r.results))
            .collect()
    }
}

/// Der Lauf gehört zum Szenario des Ziels (ohne Seed; Go: `Target.runs`).
fn runs(g: &Target, res: &RunResult) -> bool {
    let s = &res.scenario;
    s.players == g.players
        && s.bot == g.bot
        && s.depth == g.depth
        && s.days == g.days
        && s.grade == g.grade
        && s.world == g.world
        && s.helps == g.helps
}

fn evaluate_target(g: &Target, t: &Targets, results: &[RunResult]) -> Verdict {
    let mut v = Verdict {
        target: g.clone(),
        ..Verdict::default()
    };
    let mut vals: Vec<f64> = Vec::new();
    let mut owner: Vec<&str> = Vec::new(); // Seed zu jedem Wert
    for res in results {
        if !runs(g, res) || !t.seeds.contains(&res.scenario.seed) {
            continue;
        }
        let Some(m) = res.metrics.as_ref().filter(|_| res.valid) else {
            v.invalid_seeds.push(res.scenario.seed.clone());
            continue;
        };
        for x in measure_values(&g.measure, m) {
            vals.push(x);
            owner.push(&res.scenario.seed);
        }
    }
    if vals.is_empty() || !v.invalid_seeds.is_empty() {
        v.status = STATUS_INVALID.into();
        return v;
    }
    v.value = value(g, &vals);
    v.counted = vals.len() as i64;
    v.status = rate(g, v.value, t).into();
    if v.status == STATUS_VIOLATED {
        v.fail_seeds = fail_seeds(g, v.value, &vals, &owner);
    }
    v
}

/// Anteil in % bzw. Median der Werte.
fn value(g: &Target, vals: &[f64]) -> f64 {
    if g.kind == KIND_SHARE {
        let sum: f64 = vals.iter().sum();
        return 100.0 * sum / vals.len() as f64;
    }
    median(vals)
}

/// Median der Werte (nicht leer); bei gerader Anzahl der Mittelwert der beiden mittleren (Go: `median`).
pub fn median(vals: &[f64]) -> f64 {
    let mut s = vals.to_vec();
    s.sort_unstable_by(f64::total_cmp);
    let n = s.len();
    if n.is_multiple_of(2) {
        (s[n / 2 - 1] + s[n / 2]) / 2.0
    } else {
        s[n / 2]
    }
}

/// Die Grenzen, die für „knapp“ zählen: Die natürlichen Enden eines Anteils (0 % und 100 %) gehören nicht dazu, sonst
/// wäre jedes Ergebnis mit 100 % „knapp“.
fn bounds(g: &Target) -> (Option<f64>, Option<f64>) {
    let (mut lo, mut hi) = (g.lower, g.upper);
    if g.kind == KIND_SHARE {
        lo = lo.filter(|&l| l > 0.0);
        hi = hi.filter(|&h| h < 100.0);
    }
    (lo, hi)
}

/// Die Randbreite: Prozentpunkte bei Anteilen, bei Medianen der Bruchteil der Korridorbreite (bei einer einzigen Grenze
/// deren Betrag), sofern das Ziel keinen eigenen Wert hat.
fn margin(g: &Target, t: &Targets) -> f64 {
    if g.kind == KIND_SHARE {
        return g.margin.unwrap_or(t.margin.share_pp);
    }
    let width = match (g.lower, g.upper) {
        (Some(l), Some(u)) => u - l,
        (Some(l), None) => l.abs(),
        (None, u) => u.unwrap_or(0.0).abs(), // Go: Zeiger auf nil wäre ein Absturz, das Laden verlangt eine Grenze
    };
    g.margin.unwrap_or(t.margin.median_fraction) * width
}

fn rate(g: &Target, v: f64, t: &Targets) -> &'static str {
    if g.lower.is_some_and(|l| v < l) || g.upper.is_some_and(|u| v > u) {
        return STATUS_VIOLATED;
    }
    let (lo, hi) = bounds(g);
    let m = margin(g, t);
    if lo.is_some_and(|l| v - l <= m) || hi.is_some_and(|h| h - v <= m) {
        return STATUS_NARROW;
    }
    STATUS_OK
}

fn fail_seeds(g: &Target, value: f64, vals: &[f64], owner: &[&str]) -> Vec<String> {
    let mut seeds: Vec<String> = Vec::new();
    for (i, &x) in vals.iter().enumerate() {
        let fails = if g.kind == KIND_SHARE {
            (x == 0.0) == g.lower.is_some_and(|l| value < l)
        } else {
            g.lower.is_some_and(|l| x < l) || g.upper.is_some_and(|u| x > u)
        };
        if fails && !seeds.iter().any(|s| s == owner[i]) {
            seeds.push(owner[i].to_string());
        }
    }
    seeds
}
