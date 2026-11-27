//! Auftrag eines Offline-Laufs (Port von `spec.go`): jedes Feld wird ein Flag von `k3c-game sim balance`.
//! Null bzw. leer heißt: Flag weglassen, die EXE nimmt alle Ziele aus `data/balance-targets.json`.
//!
//! Jeder Wert aus einer Eingabe geht nur nach [`Spec::check`] als eigenes Argument an die EXE (nie über eine Shell).

use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::balance::baselines::{baseline_file, valid_baseline_name};
use crate::balance::insights_files::BASELINE_PATH;
use crate::balance::names::{MIX_SEP, bot_names, helps_names, known_bot, worlds};
use crate::balance::report_types::is_zero;
use crate::balance::tune::{MAX_TUNE_PARAMS, MAX_TUNE_STEPS, TUNE_SEARCH, TUNE_SWEEP};
use crate::kitx::go_quoted;

/// Ein Offline-Lauf der Balancing-Seite. Zum Lesen von JSON wie Go (`null`, unbekannte Felder) siehe
/// `super::stamp::read_object`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Spec {
    #[serde(skip_serializing_if = "is_zero")]
    pub seeds: i64,
    /// Genau diese Seeds (`--seed-list`, B-634) statt der ersten Seeds bzw. des Seed-Modus der Datei.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub seed_list: Vec<String>,
    /// 0 ist Wald, daher `Option`: `None` = Tiefe des Ziels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
    #[serde(skip_serializing_if = "is_zero")]
    pub players: i64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub bot: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub grade: String,
    #[serde(skip_serializing_if = "is_zero")]
    pub days: i64,
    /// Welt-ID aus `data/worlds` (BS11, B-642); der Lauf spielt die ganze erste Insel statt einer Stufe.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub world: String,
    /// Dev-Hilfen wie im Stufen-Test (hilfen, voll, boss), nur mit `world`.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub helps: String,
    /// `datei.json:pfad.zum.wert=wert`
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vary: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,
    /// Name einer gesicherten Baseline (`reports/baseline/<name>/`, B-640) als Vergleich statt der Baseline des Repos.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub baseline: String,
    /// Automatisches Balancing (WZ28.6), Ergebnis `tuning-*.json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tune: Option<Tune>,
    /// Je feste Variante ein eigener Lauf gegen einen gemeinsamen Basis-Lauf (B-662, `Registry::start_offline`).
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub per_variant: bool,
    /// Der Lauf zeigt sich im Spiel (`--window`, RU40.3), Läufe nacheinander.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub head: bool,
    /// Zeitraffer 1 bis 64.
    #[serde(skip_serializing_if = "is_zero")]
    pub speed: i64,
}

/// `--tune`, `--param`, `--steps` und `--budget` der EXE; 0 heißt Vorgabe der EXE.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Tune {
    /// sweep oder search
    pub mode: String,
    /// `datei.json:pfad.zum.wert=min..max`, 1 oder 2
    pub params: Vec<String>,
    #[serde(skip_serializing_if = "is_zero")]
    pub steps: i64,
    #[serde(skip_serializing_if = "is_zero")]
    pub budget: i64,
}

const MAX_VARY: usize = 20;
const MAX_BUDGET: i64 = 1000;
/// Zeichen eines Labels.
pub(super) const MAX_LABEL: usize = 60;

/// Ein ungültiges Muster fände nie etwas: jede Prüfung lehnte dann ab.
macro_rules! patterns {
    ($($name:ident = $re:expr;)*) => {
        $(static $name: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new($re).ok());)*
    };
}

patterns! {
    GRADE_NAME = r"^[a-z0-9-]{1,20}$";
    SEED_NAME = r"^[A-Za-z0-9_-]{1,40}$";
    // Datei ohne Ordner, Pfad aus Namen und Indizes, Wert ohne Leerraum (nur ASCII-Leerraum wie Go `\s`).
    VARY_ARG = r"^[A-Za-z0-9_-]+\.json:[A-Za-z0-9_.\[\]-]+=[^\t\n\f\r =]{1,64}$";
    // wie VARY_ARG, der Wert ist ein Bereich min..max aus zwei Zahlen
    TUNE_PARAM = r"^[A-Za-z0-9_-]+\.json:[A-Za-z0-9_.\[\]-]+=-?[0-9]+(\.[0-9]+)?\.\.-?[0-9]+(\.[0-9]+)?$";
}

fn matches(re: &LazyLock<Option<Regex>>, text: &str) -> bool {
    re.as_ref().is_some_and(|re| re.is_match(text))
}

fn rejected(msg: &str) -> Result<(), String> {
    Err(format!("lauf abgelehnt: {msg}"))
}

/// Der erste zutreffende Grund.
fn first_bad(checks: &[(bool, &str)]) -> Result<(), String> {
    match checks.iter().find(|(bad, _)| *bad) {
        Some((_, msg)) => rejected(msg),
        None => Ok(()),
    }
}

fn duplicates(list: &[String]) -> bool {
    let mut sorted: Vec<&String> = list.iter().collect();
    sorted.sort();
    sorted.windows(2).any(|w| w[0] == w[1])
}

impl Spec {
    /// Prüft die Spec über Grenzen und Positivlisten; jeder Fehler ist eine Zeile mit Grund. `root` ist der Checkout
    /// (Welten unter `data/worlds`).
    pub fn check(&self, root: &Path) -> Result<(), String> {
        self.check_limits()?;
        for v in &self.vary {
            if !matches(&VARY_ARG, v) || v.contains("..") {
                return rejected(&format!("vary {}, Form datei.json:pfad.zum.wert=wert", go_quoted(v)));
            }
        }
        self.check_head()?;
        self.check_baseline()?;
        self.check_world(root)?;
        self.check_helps()?;
        self.check_per_variant()?;
        self.tune.as_ref().map_or(Ok(()), Tune::check)
    }

    fn check_limits(&self) -> Result<(), String> {
        let bot = format!("bot: {} oder je Spieler a{MIX_SEP}b", bot_names().join(", "));
        let vary = format!("höchstens {MAX_VARY} Varianten");
        let label = format!("label höchstens {MAX_LABEL} Zeichen, eine Zeile");
        first_bad(&[
            (!(0..=200).contains(&self.seeds), "seeds 1 bis 200"),
            (self.seed_list.len() > 200, "seedList höchstens 200 Seeds"),
            (
                self.seeds > 0 && !self.seed_list.is_empty(),
                "seeds oder seedList, nicht beides",
            ),
            (duplicates(&self.seed_list), "seedList ohne doppelte Seeds"),
            (
                self.seed_list.iter().any(|n| !matches(&SEED_NAME, n)),
                "seedList: Buchstaben, Ziffern, _ und -, höchstens 40 Zeichen",
            ),
            (self.depth.is_some_and(|d| !(0..=20).contains(&d)), "depth 0 bis 20"),
            (!(0..=4).contains(&self.players), "players 1 bis 4"),
            (!(0..=100).contains(&self.days), "days 1 bis 100"),
            (!self.bot.is_empty() && !known_bot(&self.bot), &bot),
            (
                !self.grade.is_empty() && !matches(&GRADE_NAME, &self.grade),
                "grade: Kleinbuchstaben, Ziffern, Bindestrich",
            ),
            (self.vary.len() > MAX_VARY, &vary),
            (
                self.label.chars().count() > MAX_LABEL || self.label.contains(['\r', '\n']),
                &label,
            ),
        ])
    }

    /// Fenster und Zeitraffer (RU40.3): speed nur mit head, 1 bis 64; das Fenster zeigt Matrix und Ziele, nicht das
    /// Abstimmen.
    fn check_head(&self) -> Result<(), String> {
        first_bad(&[
            (self.speed != 0 && !self.head, "speed gilt nur mit head"),
            (!(0..=64).contains(&self.speed), "speed 1 bis 64"),
            (
                self.head && self.tune.is_some(),
                "head geht nicht mit Abstimmung (nur Matrix und Ziele zeigt das Fenster)",
            ),
        ])
    }

    /// „je Variante“ braucht feste Varianten und keine Bereiche (B-662).
    fn check_per_variant(&self) -> Result<(), String> {
        if !self.per_variant {
            return Ok(());
        }
        first_bad(&[
            (self.tune.is_some(), "je Variante geht nicht mit Abstimmung"),
            (
                self.vary.is_empty(),
                "je Variante braucht mindestens eine feste Variante",
            ),
        ])
    }

    fn check_baseline(&self) -> Result<(), String> {
        if self.baseline.is_empty() {
            return Ok(());
        }
        valid_baseline_name(&self.baseline).or_else(|e| rejected(&e))
    }

    fn check_helps(&self) -> Result<(), String> {
        if self.helps.is_empty() {
            return Ok(());
        }
        if !helps_names().contains(&self.helps.as_str()) {
            return rejected(&format!("helps: {}", helps_names().join(", ")));
        }
        first_bad(&[(self.world.is_empty(), "helps braucht eine world")])
    }

    fn check_world(&self, root: &Path) -> Result<(), String> {
        if self.world.is_empty() {
            return Ok(());
        }
        // Ohne lesbare Welten ist die Liste leer.
        let known = worlds(root).unwrap_or_default();
        if known.contains(&self.world) {
            return Ok(());
        }
        rejected(&format!("world: {}", known.join(", ")))
    }

    /// Bezeichnung des Laufs: Label oder eine Kurzform der gesetzten Felder.
    pub fn title(&self) -> String {
        if !self.label.is_empty() {
            return self.label.clone();
        }
        let mut parts = vec!["offline·balance".to_owned()];
        for (name, n) in [("seeds", self.seeds), ("players", self.players), ("days", self.days)] {
            if n > 0 {
                parts.push(format!("{name} {n}"));
            }
        }
        if !self.seed_list.is_empty() {
            parts.push(format!("seeds {}", self.seed_list.join(",")));
        }
        if let Some(depth) = self.depth {
            parts.push(format!("depth {depth}"));
        }
        let texts = [&self.bot, &self.grade, &self.world, &self.helps];
        parts.extend(texts.into_iter().filter(|t| !t.is_empty()).cloned());
        if !self.vary.is_empty() {
            parts.push(format!("{} Varianten", self.vary.len()));
        }
        if !self.baseline.is_empty() {
            parts.push(format!("gegen {}", self.baseline));
        }
        if self.head {
            parts.push("mit Fenster".to_owned());
        }
        if let Some(tune) = &self.tune {
            parts.push(format!("tune {} {} Parameter", tune.mode, tune.params.len()));
        }
        parts.join(" · ")
    }

    /// Argumente der EXE; `dir` ist der Berichtsordner relativ zum Checkout. Die Baseline des Repos liefert den Trend
    /// der Auswertung (B-632); fehlt sie, rechnet die EXE ohne.
    pub fn args(&self, dir: &str) -> Vec<String> {
        let baseline = if self.baseline.is_empty() {
            BASELINE_PATH.to_owned()
        } else {
            baseline_file(&self.baseline)
        };
        let fixed = ["balance", "--targets", "--report-dir", dir, "--progress", "--baseline"];
        let mut args: Vec<String> = fixed.map(str::to_owned).to_vec();
        args.push(baseline);
        let mut flag = |name: &str, value: String| args.extend([name.to_owned(), value]);
        for (name, n) in [
            ("--seeds", self.seeds),
            ("--players", self.players),
            ("--days", self.days),
        ] {
            if n > 0 {
                flag(name, n.to_string());
            }
        }
        if !self.seed_list.is_empty() {
            flag("--seed-list", self.seed_list.join(","));
        }
        if let Some(depth) = self.depth {
            flag("--depth", depth.to_string());
        }
        for (name, text) in [
            ("--bot", &self.bot),
            ("--grade", &self.grade),
            ("--world", &self.world),
            ("--helps", &self.helps),
        ] {
            if !text.is_empty() {
                flag(name, text.clone());
            }
        }
        for v in &self.vary {
            flag("--vary", v.clone());
        }
        args.extend(self.tune.as_ref().map(Tune::args).unwrap_or_default());
        args
    }
}

impl Tune {
    /// Prüft das Abstimmen wie vary: Modus, 1 bis 2 Parameter, kein „..“ vor dem Bereich, Grenzen von steps und budget.
    fn check(&self) -> Result<(), String> {
        let steps = format!("tune.steps 2 bis {MAX_TUNE_STEPS}");
        let budget = format!("tune.budget 1 bis {MAX_BUDGET}");
        first_bad(&[
            (
                self.mode != TUNE_SWEEP && self.mode != TUNE_SEARCH,
                "tune.mode: sweep oder search",
            ),
            (
                self.params.is_empty() || self.params.len() > MAX_TUNE_PARAMS,
                "tune.params: 1 oder 2 Parameter",
            ),
            (self.steps != 0 && !(2..=MAX_TUNE_STEPS).contains(&self.steps), &steps),
            (!(0..=MAX_BUDGET).contains(&self.budget), &budget),
        ])?;
        for p in &self.params {
            let path = p.split_once('=').map_or(p.as_str(), |(path, _)| path);
            if !matches(&TUNE_PARAM, p) || path.contains("..") {
                let p = go_quoted(p);
                return rejected(&format!("tune.params {p}, Form datei.json:pfad.zum.wert=min..max"));
            }
        }
        Ok(())
    }

    /// Flags des Abstimmens.
    fn args(&self) -> Vec<String> {
        let mut out = vec!["--tune".to_owned(), self.mode.clone()];
        for p in &self.params {
            out.extend(["--param".to_owned(), p.clone()]);
        }
        for (name, n) in [("--steps", self.steps), ("--budget", self.budget)] {
            if n > 0 {
                out.extend([name.to_owned(), n.to_string()]);
            }
        }
        out
    }
}

#[cfg(test)]
#[path = "mix_tests.rs"]
mod mix_tests;
#[cfg(test)]
#[path = "spec_baseline_tests.rs"]
mod spec_baseline_tests;
