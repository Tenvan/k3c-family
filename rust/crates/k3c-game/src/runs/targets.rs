//! Ziel-Modus von `sim balance` (Port von `engine/simcli/targets.go`, RU40.1): `--targets` bewertet die Ziele aus
//! `data/balance-targets.json`, schreibt Bericht (JSON, Markdown, HTML), vergleicht mit der Baseline und schreibt auf Wunsch
//! Replays der Seeds verletzter Ziele. Dazu `--write-baseline` und `--insights` (vorhandenen Bericht neu auswerten).

use std::path::Path;
use std::time::{Instant, SystemTime};

use k3c_balance::baselines::utc_parts;
use k3c_balance::figures::figures_of;
use k3c_balance::insights_files::{read_baseline, reevaluate, write_reports};
use k3c_balance::parallel::ProgressFn;
use k3c_balance::run::Report;
use k3c_balance::scope::{Restrict, Scope};
use k3c_balance::seedmode::SeedModes;
use k3c_balance::summary::{Summarize, Summary};
use k3c_balance::targets::default_targets;
use k3c_balance::variant::with_variants;
use k3c_data::balance_targets::Targets;

use super::RunError;
use super::balance::{Env, fail, ints, io, split, write_replays};
use super::flags::Parsed;
use super::gotext::quote;
use super::sensitivity::VaryOpts;

/// Die Optionen des Ziel-Modus (Go: `targetOpts`).
pub struct TargetOpts {
    /// 0 = Seed-Modus der Datei (dev: nur der erste, sonst alle).
    pub seeds: i64,
    /// `--seed-list`: genau diese Seeds statt der Datei (B-634).
    pub seed_list: Vec<String>,
    /// Ordner für `balance-<Zeit>.*`.
    pub dir: String,
    pub baseline: String,
    pub write: bool,
    /// Ordner für Replays der Seeds verletzter Ziele.
    pub replays: String,
    pub variants: Vec<String>,
    pub scope: Scope,
    pub progress: bool,
}

impl TargetOpts {
    /// Liest Seeds und Szenario der Zielläufe aus den Schaltern (Go: `prepare`, `seedsFrom`, `scopeFrom`).
    pub fn from(p: &Parsed, vary: &VaryOpts) -> Result<TargetOpts, RunError> {
        let seeds = if p.is_set("seeds") { p.int("seeds") } else { 0 };
        let list = p.text("seed-list");
        if !list.is_empty() && seeds > 0 {
            return Err(RunError::Usage("--seeds und --seed-list schließen sich aus".into()));
        }
        let mut scope = Scope {
            bot: p.text("bot"),
            grade: p.text("grade"),
            world: p.text("world"),
            helps: p.text("helps"),
            ..Scope::default()
        };
        if p.is_set("depth") {
            scope.depth = Some(p.int("depth"));
        }
        if p.is_set("days") {
            scope.days = Some(p.int("days"));
        }
        if p.is_set("players") {
            let players = p.text("players");
            match ints(&players).ok().as_deref() {
                Some([n]) => scope.players = Some(*n),
                _ => {
                    return Err(RunError::Usage(format!(
                        "--players mit --targets braucht genau eine Zahl, war {}",
                        quote(&players)
                    )));
                }
            }
        }
        Ok(TargetOpts {
            seeds,
            seed_list: split(&list),
            dir: p.text("report-dir"),
            baseline: p.text("baseline"),
            write: p.flag("write-baseline"),
            replays: p.text("replay-dir"),
            variants: vary.variants.clone(),
            scope,
            progress: p.flag("progress"),
        })
    }

    /// Die Ziele mit dem Szenario aus den Schaltern und, mit `--seed-list`, genau diesen Seeds.
    pub fn targets(&self, v: &Env) -> Result<Targets, RunError> {
        let ts = default_targets().map_err(|e| fail(e, &v.stop))?;
        let ts = ts.restrict(&self.scope).map_err(|e| fail(e, &v.stop))?;
        if self.seed_list.is_empty() {
            return Ok(ts);
        }
        ts.with_seeds(&self.seed_list).map_err(|e| fail(e, &v.stop))
    }
}

/// Spielt die Ziele, schreibt den Bericht und vergleicht mit der Baseline. Verletzte Ziele sind kein Fehler: das
/// Pflicht-Gate gibt es nicht (BAL2).
pub fn run(o: &TargetOpts, v: &mut Env) -> Result<(), RunError> {
    let ts = o.targets(v)?;
    let start = Instant::now();
    v.logf("🚀 k3c-game sim: Balancing gestartet");
    let tick = v.tick();
    let (out, stop) = (&mut *v.out, v.stop.clone());
    let progress: Option<ProgressFn> = o
        .progress
        .then(move || Box::new(move |done: usize, total: usize| out.progress(done as i64, total as i64)) as ProgressFn);
    let report = with_variants(&o.variants, || ts.run_targets_progress(o.seeds, Some(tick), progress))
        .map_err(|e| fail(e, &stop))?;
    let runs = report.results.len();
    let mut sum = ts.summarize(&report);
    sum.varianten.clone_from(&o.variants);
    sum.kennzahlen = figures_of(&report);
    if o.write {
        return write_baseline(&o.baseline, &sum);
    }
    let base = read_baseline_file(&o.baseline)?;
    write_replays(&only_fail_seeds(report, &sum), &o.replays)?;
    write_summary(&o.dir, &sum, base.as_ref(), &o.baseline, v)?;
    v.logf(&format!(
        "✅ k3c-game sim: {} Läufe in {}s",
        runs,
        start.elapsed().as_secs()
    ));
    Ok(())
}

fn write_baseline(path: &str, sum: &Summary) -> Result<(), RunError> {
    if path.is_empty() {
        return Err(RunError::Usage("--write-baseline braucht --baseline DATEI".into()));
    }
    let text = sum.json().map_err(|e| RunError::Failed(e.to_string()))?;
    if let Some(dir) = Path::new(path).parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(dir).map_err(|e| io(&e))?;
    }
    std::fs::write(path, text).map_err(|e| io(&e))
}

/// `None`, wenn keine Datei angegeben ist oder sie fehlt (der Bericht weist darauf hin).
fn read_baseline_file(path: &str) -> Result<Option<Summary>, RunError> {
    if path.is_empty() {
        return Ok(None);
    }
    let base = read_baseline(Path::new(path)).map_err(|e| RunError::Failed(e.to_string()))?;
    Ok(base)
}

/// Die Zeit im Dateinamen: `JJJJMMTT-hhmmss` in UTC (Go: Ortszeit; std kennt keine Zeitzone).
pub fn stamp() -> String {
    let [y, mo, d, h, mi, s] = utc_parts(SystemTime::now());
    format!("{y:04}{mo:02}{d:02}-{h:02}{mi:02}{s:02}")
}

/// Schreibt `balance-<Zeit>.json`, `.md` und `.html` nach `dir` und meldet die Pfade.
fn write_summary(
    dir: &str,
    sum: &Summary,
    base: Option<&Summary>,
    base_name: &str,
    v: &mut Env,
) -> Result<(), RunError> {
    std::fs::create_dir_all(dir).map_err(|e| io(&e))?;
    let text = sum.json().map_err(|e| RunError::Failed(e.to_string()))?;
    let stem = Path::new(dir)
        .join(format!("balance-{}", stamp()))
        .to_string_lossy()
        .into_owned();
    std::fs::write(format!("{stem}.json"), text).map_err(|e| io(&e))?;
    write_reports(&stem, sum, base, base_name, ".").map_err(|e| fail(e, &v.stop))?;
    v.out
        .report(&[format!("{stem}.json"), format!("{stem}.md"), format!("{stem}.html")]);
    v.logf(&format!(
        "💾 k3c-game sim: Bericht {stem}.json, {stem}.md und {stem}.html"
    ));
    Ok(())
}

/// Wertet einen vorhandenen Bericht neu aus (`--insights`); Grafiken aus `public/` des aktuellen Ordners.
pub fn run_insights(path: &str, v: &mut Env) -> Result<(), RunError> {
    let page = reevaluate(path, ".").map_err(|e| fail(e, &v.stop))?;
    v.out.report(std::slice::from_ref(&page));
    v.logf(&format!("📄 k3c-game sim: Auswertung {page}"));
    Ok(())
}

/// Behält die Replays der Läufe, deren Seed bei einem verletzten Ziel genannt wird.
fn only_fail_seeds(mut r: Report, sum: &Summary) -> Report {
    let failing: Vec<&String> = sum.targets.iter().flat_map(|e| &e.verdict.fail_seeds).collect();
    for res in &mut r.results {
        if !failing.contains(&&res.scenario.seed) {
            res.replay = None;
        }
    }
    r
}
