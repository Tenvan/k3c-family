//! `k3c-game sim balance` headless (Port von `engine/simcli/balance.go`, RU40.1): Szenario-Matrix, Ziel-Modus (`--targets`,
//! `targets.rs`), Abstimmung (`--tune`, `tune.rs`), Sensitivität und Kurven (`sensitivity.rs`), Replay abspielen (`--play`).
//! Dieselben Schalter, JSON-Zeilen, Berichtsdateien und Exit-Codes wie `k3c-sim balance`; die Logik liegt in `k3c-balance`.
//!
//! Abbruch: Der `Stop` wird zwischen zwei Läufen geprüft (Go: `env.tick`). Der Rückruf löst dazu eine Panik mit dem Text
//! `ABORT_TEXT` aus (`resume_unwind`, ohne Meldung); `run_all` fängt sie in den Arbeitern und gibt sie als `Fehler::Abbruch`
//! zurück, jede andere Stelle fängt `run` am Ende. Es entsteht kein Bericht.

use std::io::{Read, Write};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::path::Path;
use std::sync::Arc;

use k3c_balance::fehler::Fehler;
use k3c_balance::replay::{Playback, Replay, play, read_replay};
use k3c_balance::run::{Matrix, Report, Tick, run as run_matrix_in};
use k3c_balance::vary::with_data;

use super::RunError;
use super::emit::{Emitter, json_string};
use super::flags::{self, Parsed};
use super::gojson::compact;
use super::gotext::quote;
use super::input::Flags;
use super::input_balance::{balance_args, cut_input};
use super::stop::Stop;
use super::{balance_windowed, sensitivity, targets, tune};

pub use super::balance_specs::SPECS;

/// Text der Panik, mit der ein Rückruf einen Lauf nach Abbruch beendet.
pub const ABORT_TEXT: &str = "abgebrochen";

/// Die Umgebung eines Aufrufs: JSON-Zeilen, Meldungen für Menschen und das Abbruch-Zeichen.
pub struct Env<'a, 'b> {
    pub out: &'a mut Emitter<'b>,
    pub err: &'a mut dyn Write,
    pub stop: Stop,
}

impl Env<'_, '_> {
    /// Eine Meldung für Menschen nach stderr.
    pub fn logf(&mut self, msg: &str) {
        let _ = writeln!(self.err, "{msg}");
    }

    /// Der Rückruf nach jedem fertigen Lauf: beendet den Lauf, wenn der Abbruch verlangt ist (Go: `env.tick`).
    pub fn tick(&self) -> Tick {
        let stop = self.stop.clone();
        Tick(Arc::new(move || {
            if stop.is_set() {
                resume_unwind(Box::new(ABORT_TEXT));
            }
        }))
    }
}

/// Ein Fehler von `k3c-balance` als Fehler des Aufrufs: Abbruch bleibt Abbruch, ein Absturz im Lauf Exit 1.
pub fn fail(e: Fehler, stop: &Stop) -> RunError {
    match e {
        Fehler::Abbruch(c) if c.value == ABORT_TEXT || stop.is_set() => RunError::Aborted,
        Fehler::Abbruch(c) => RunError::Failed(format!("💥 Absturz in der Engine: {}", c.value)),
        other => RunError::Failed(other.to_string()),
    }
}

/// Ein Fehler beim Lesen oder Schreiben einer Datei (Exit 1).
pub fn io(e: &std::io::Error) -> RunError {
    RunError::Failed(e.to_string())
}

fn usage(msg: impl Into<String>) -> RunError {
    RunError::Usage(msg.into())
}

/// Wird `--stop-stdin` verlangt und liegt die Eingabe nicht auf stdin? Dann liest `main` stdin als Abbruch-Signal.
pub fn wants_stop(rest: &[String]) -> bool {
    let Ok((path, args)) = cut_input(rest) else {
        return false;
    };
    let Ok(p) = flags::parse(SPECS, &args) else {
        return false;
    };
    p.flag("stop-stdin") && path != "-"
}

/// Zerlegt `a, b,,c` an Kommas (Go: `split`).
pub fn split(s: &str) -> Vec<String> {
    s.split(',')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .map(ToString::to_string)
        .collect()
}

/// Ganze Zahlen mit Komma (Go: `ints`).
pub fn ints(s: &str) -> Result<Vec<i64>, RunError> {
    split(s)
        .iter()
        .map(|p| p.parse::<i64>().map_err(|_| usage(format!("keine Zahl: {}", quote(p)))))
        .collect()
}

/// Einstieg: führt `balance` mit den Argumenten hinter dem Unterbefehl aus.
pub fn run(args: &[String], stdin: &mut dyn Read, v: &mut Env) -> Result<(), RunError> {
    match catch_unwind(AssertUnwindSafe(|| run_inner(args, stdin, v))) {
        Ok(r) => r,
        Err(p) if p.downcast_ref::<&str>().is_some_and(|s| *s == ABORT_TEXT) => Err(RunError::Aborted),
        Err(p) => resume_unwind(p),
    }
}

fn run_inner(args: &[String], stdin: &mut dyn Read, v: &mut Env) -> Result<(), RunError> {
    let (input, _) = cut_input(args)?;
    let p = flags::parse(SPECS, &balance_args(args, stdin)?)?;
    if p.help {
        let _ = v.err.write_all(flags::defaults_text(SPECS).as_bytes());
        return Ok(());
    }
    if !p.rest.is_empty() {
        return Err(usage(format!("unerwartete Argumente: [{}]", p.rest.join(" "))));
    }
    let f = Flags {
        input,
        headless: p.flag("headless"),
        window: p.flag("window"),
        speed: p.is_set("speed").then(|| p.int("speed")),
        data: p.is_set("data").then(|| p.text("data")),
        stop_stdin: p.flag("stop-stdin"),
    };
    super::check_flags("balance", &f)?;
    let window = f.window.then(|| f.speed.unwrap_or(1));
    if f.data.is_none() {
        return dispatch(&p, v, window);
    }
    let src = super::data_source(&f)?;
    with_data(src, || Ok(dispatch(&p, v, window))).map_err(|e| fail(e, &v.stop))?
}

/// Wählt den Lauf wie Gos `cmdBalance`; `window` ist die Geschwindigkeit von `--window` (RU40.2).
fn dispatch(p: &Parsed, v: &mut Env, window: Option<i64>) -> Result<(), RunError> {
    let vary = sensitivity::VaryOpts::from(p);
    check_combo(p, &vary).map_err(usage)?;
    if let Some(speed) = window {
        return balance_windowed::run(p, &vary, v, speed);
    }
    if !p.text("insights").is_empty() {
        return targets::run_insights(&p.text("insights"), v);
    }
    if vary.vary || p.flag("curves") {
        return sensitivity::run(p, &vary, v);
    }
    if p.flag("targets") {
        let o = targets::TargetOpts::from(p, &vary)?;
        return match tune::TuneOpts::from(p) {
            Some(t) => tune::run(&o, &t, v),
            None => targets::run(&o, v),
        };
    }
    if !p.text("play").is_empty() {
        return play_file(&p.text("play"), &p.text("out"), p.flag("legacy"), v);
    }
    run_matrix(p, v)
}

/// Lehnt Schalter ab, die `--targets` bzw. `--tune` brauchen (Go: `checkFlags`).
fn check_combo(p: &Parsed, vary: &sensitivity::VaryOpts) -> Result<(), String> {
    let on = p.flag("targets");
    let params = !p.list("param").is_empty();
    let tuning = !p.text("tune").is_empty();
    let msg = if !vary.variants.is_empty() && !on {
        "--vary datei.json:pfad=wert braucht --targets"
    } else if !p.text("world").is_empty() && !on {
        "--world braucht --targets"
    } else if !p.text("helps").is_empty() && !on {
        "--helps braucht --targets"
    } else if (tuning || params) && !on {
        "--tune und --param brauchen --targets"
    } else if params && !tuning {
        "--param braucht --tune sweep|search"
    } else if p.flag("write-baseline") && p.text("baseline").is_empty() {
        "--write-baseline braucht --baseline DATEI"
    } else {
        return Ok(());
    };
    Err(msg.to_string())
}

/// Die Seeds der Matrix: `list` oder 1..n (Go: `seedNames`).
fn seed_names(n: i64, list: &str) -> Vec<String> {
    if !list.is_empty() {
        return split(list);
    }
    (1..=n).map(|i| i.to_string()).collect()
}

/// Spielt die Szenario-Matrix (ohne `--targets`) und schreibt den Bericht nach `--out` oder als Zeile `result`.
fn run_matrix(p: &Parsed, v: &mut Env) -> Result<(), RunError> {
    let m = Matrix {
        tick: Some(v.tick()),
        ..matrix_of(p)?
    };
    let report = run_matrix_in(&m).map_err(|e| fail(e, &v.stop))?;
    write_matrix(&report, p, v)
}

/// Die Matrix aus den Schaltern (Go: `cmdBalance`).
pub fn matrix_of(p: &Parsed) -> Result<Matrix, RunError> {
    Ok(Matrix {
        seeds: seed_names(p.int("seeds"), &p.text("seed-list")),
        players: ints(&p.text("players"))?,
        bots: split(&p.text("bots")),
        depths: ints(&p.text("depths"))?,
        days: p.int("days"),
        gold_threshold: p.int("gold-threshold"),
        legacy: p.flag("legacy"),
        ..Matrix::default()
    })
}

/// Schreibt Replays und Bericht der Matrix (headless wie im Fenster).
pub fn write_matrix(report: &Report, p: &Parsed, v: &mut Env) -> Result<(), RunError> {
    write_replays(report, &p.text("replay-dir"))?;
    let text = report.json().map_err(|e| fail(e, &v.stop))?;
    write(&p.text("out"), &text, v)
}

/// Schreibt nach `out` (Bytes wie bis RS2.2 `bin/k3c-balance`) oder, ohne Datei, als Zeile `{"event":"result","report":…}`.
pub fn write(out: &str, text: &str, v: &mut Env) -> Result<(), RunError> {
    if !out.is_empty() {
        return std::fs::write(out, text).map_err(|e| io(&e));
    }
    let report = compact(text).map_err(|_| RunError::Failed("bericht ist kein JSON".into()))?;
    v.out.result_report(&report);
    Ok(())
}

/// Schreibt je gültigem Lauf `<seed>-p<spieler>-<bot>-d<tiefe>.replay.json` nach `dir`.
pub fn write_replays(r: &Report, dir: &str) -> Result<(), RunError> {
    if dir.is_empty() {
        return Ok(());
    }
    std::fs::create_dir_all(dir).map_err(|e| io(&e))?;
    for res in &r.results {
        let Some(replay) = &res.replay else { continue };
        let s = &res.scenario;
        let name = format!("{}-p{}-{}-d{}.replay.json", s.seed, s.players, s.bot, s.depth);
        let text = replay.json().map_err(|e| RunError::Failed(e.to_string()))?;
        std::fs::write(Path::new(dir).join(name), text).map_err(|e| io(&e))?;
    }
    Ok(())
}

/// Spielt eine Replay-Datei ab und schreibt Ticks, Endzustand-Hash, Burgfall-Tick und Warnungen als JSON (`--play`).
fn play_file(path: &str, out: &str, legacy: bool, v: &mut Env) -> Result<(), RunError> {
    let r = read_replay_file(path, legacy)?;
    let pb = play(&r).map_err(|e| fail(e, &v.stop))?;
    write_playback(&pb, out, v)
}

/// Liest eine Aufnahme (`--play`).
pub fn read_replay_file(path: &str, legacy: bool) -> Result<Replay, RunError> {
    let raw = std::fs::read_to_string(path).map_err(|e| io(&e))?;
    let mut r: Replay = read_replay(&raw).map_err(|e| RunError::Failed(format!("{path}: {e}")))?;
    r.scenario.legacy = legacy;
    Ok(r)
}

/// Schreibt das Ergebnis einer Wiedergabe (headless wie im Fenster).
pub fn write_playback(pb: &Playback, out: &str, v: &mut Env) -> Result<(), RunError> {
    for w in &pb.warnings {
        v.logf(&format!("🚫 k3c-game sim: Warnung: {w}"));
    }
    let fall = pb.castle_fall_tick.map_or("null".to_string(), |t| t.to_string());
    let warnings = if pb.warnings.is_empty() {
        "null".to_string()
    } else {
        let list: Vec<String> = pb.warnings.iter().map(|w| format!("    {}", json_string(w))).collect();
        format!("[\n{}\n  ]", list.join(",\n"))
    };
    let text = format!(
        "{{\n  \"ticks\": {},\n  \"endHash\": {},\n  \"castleFallTick\": {fall},\n  \"warnings\": {warnings}\n}}\n",
        pb.ticks,
        json_string(&pb.end_hash)
    );
    write(out, &text, v)
}
