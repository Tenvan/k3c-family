//! `sim balance --window` (RU40.2): Matrix, `--targets` und `--play` rechnen ihre Läufe nacheinander im Fenster
//! (`balance_window.rs`) und schreiben danach dieselben Berichte wie headless (`balance.rs`, `targets.rs`). Die anderen
//! Betriebsarten (`--tune`, `--vary`, `--curves`, `--insights`) rechnen sehr viele Läufe oder gar keinen: dafür gibt es
//! kein Fenster (Exit 2).

use std::time::Instant;

use k3c_balance::run::{Matrix, Report, RunResult, Scenario};
use k3c_balance::seedmode::SeedModes;
use k3c_balance::variant::with_variants;
use k3c_data::balance_targets::Targets;

use super::RunError;
use super::balance::{Env, fail, matrix_of, read_replay_file, write_matrix, write_playback};
use super::balance_run::Done;
use super::balance_window::{BalanceApp, Job, Outcome};
use super::flags::Parsed;
use super::sensitivity::VaryOpts;
use super::targets::{self, TargetOpts};
use super::tune;

/// Rechnet die Läufe im Fenster und liefert ihre Ergebnisse; Fenster zu vor dem Ende ist ein Abbruch, kein Bildschirm ein
/// Fehler (Exit 1).
fn drive(jobs: Vec<Job>, speed: i64, v: &mut Env) -> Result<Vec<Done>, RunError> {
    let mut app = BalanceApp::windowed(
        jobs,
        v.stop.clone(),
        speed,
        None,
        Some(crate::replay::run::art_config()),
    );
    let cfg = k3c_platform::Config {
        title: "k3c sim balance".to_string(),
        size: Some((1280, 720)),
        ..k3c_platform::Config::default()
    };
    k3c_platform::run_window(&mut app, cfg).map_err(RunError::Failed)?;
    match app.outcome() {
        Outcome::Finished(done) => Ok(done),
        Outcome::Aborted => Err(RunError::Aborted),
    }
}

fn results_of(done: Vec<Done>) -> Vec<RunResult> {
    done.into_iter()
        .filter_map(|d| match d {
            Done::Scenario(r) => Some(*r),
            Done::Replay(_) => None,
        })
        .collect()
}

/// Wählt den Lauf im Fenster wie `balance::dispatch` ohne Fenster.
pub fn run(p: &Parsed, vary: &VaryOpts, v: &mut Env, speed: i64) -> Result<(), RunError> {
    let other = vary.vary
        || p.flag("curves")
        || !p.text("insights").is_empty()
        || tune::TuneOpts::from(p).is_some()
        || !p.list("param").is_empty();
    if other {
        return Err(RunError::Usage(
            "--window gilt für die Matrix, --targets und --play (nicht für --tune, --vary, --curves, --insights)"
                .into(),
        ));
    }
    if p.flag("targets") {
        return run_targets(&TargetOpts::from(p, vary)?, v, speed);
    }
    if !p.text("play").is_empty() {
        return play_window(p, v, speed);
    }
    let m = matrix_of(p)?;
    let sc = m.scenarios().map_err(|e| fail(e, &v.stop))?;
    let jobs = sc.into_iter().map(|s| Job::Scenario(s, m.gold_threshold)).collect();
    let results = results_of(drive(jobs, speed, v)?);
    write_matrix(&Report { matrix: m, results }, p, v)
}

fn play_window(p: &Parsed, v: &mut Env, speed: i64) -> Result<(), RunError> {
    let r = read_replay_file(&p.text("play"), p.flag("legacy"))?;
    let done = drive(vec![Job::Replay(Box::new(r))], speed, v)?;
    match done.into_iter().next() {
        Some(Done::Replay(pb)) => {
            let pb = (*pb).map_err(|e| fail(e, &v.stop))?;
            write_playback(&pb, &p.text("out"), v)
        }
        _ => Err(RunError::Failed("keine Wiedergabe".into())),
    }
}

/// Die Szenarien der Ziele (Go: `RunTargets`): je Szenario-Schlüssel eine Matrix über die Seeds, ohne Doppelte.
fn scenarios_of(ts: &Targets, seeds: &[String]) -> Result<(Matrix, Vec<Scenario>), k3c_balance::fehler::Fehler> {
    let mut first: Option<Matrix> = None;
    let mut all = Vec::new();
    let mut done: Vec<Scenario> = Vec::new();
    for g in &ts.targets {
        let key = Scenario {
            players: g.players,
            bot: g.bot.clone(),
            depth: g.depth,
            days: g.days,
            grade: g.grade.clone(),
            world: g.world.clone(),
            helps: g.helps.clone(),
            ..Scenario::default()
        };
        if done.contains(&key) {
            continue;
        }
        done.push(key);
        let m = Matrix {
            seeds: seeds.to_vec(),
            players: vec![g.players],
            bots: vec![g.bot.clone()],
            depths: vec![g.depth],
            days: g.days,
            grade: g.grade.clone(),
            world: g.world.clone(),
            helps: g.helps.clone(),
            ..Matrix::default()
        };
        all.extend(m.scenarios()?);
        first.get_or_insert(m);
    }
    Ok((first.unwrap_or_default(), all))
}

fn run_targets(o: &TargetOpts, v: &mut Env, speed: i64) -> Result<(), RunError> {
    let ts = o.targets(v)?;
    let start = Instant::now();
    v.logf("🚀 k3c-game sim: Balancing gestartet");
    let report = with_variants(&o.variants, || {
        let (matrix, all) = scenarios_of(&ts, ts.seed_list(o.seeds))?;
        let jobs = all.into_iter().map(|s| Job::Scenario(s, 0)).collect();
        match drive(jobs, speed, v) {
            Ok(done) => Ok(Ok(Report {
                matrix,
                results: results_of(done),
            })),
            Err(e) => Ok(Err(e)),
        }
    })
    .map_err(|e| fail(e, &v.stop))??;
    targets::finish(o, &ts, report, start, v)
}
