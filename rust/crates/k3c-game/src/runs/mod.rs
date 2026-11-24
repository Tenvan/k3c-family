//! Läufe im Game-Client (RU33): `k3c-game sim <run|level|…>` führt Sim- und Level-Läufe aus, headless und mit stdout als
//! JSON-Zeilen. Port von `engine/simcli/simcli.go`; die Schnittstelle (Unterbefehle, Schalter, Zeilen, Exit-Codes) steht in
//! `docs/standards/k3c-game-laeufe.md`. Meldungen für Menschen gehen nach stderr, stdout trägt nur JSON-Zeilen.

use std::io::{Read, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;
use std::process::ExitCode;

use k3c_data::Source;

pub mod emit;
pub mod enginetools_level;
pub mod enginetools_run;
mod gotext;
pub mod input;
pub mod stop;

#[cfg(test)]
mod enginetools_tests;
#[cfg(test)]
mod input_tests;
#[cfg(test)]
mod mod_tests;

use emit::Emitter;
use gotext::quote;
use input::{Flags, parse_flags, read_level_input, read_run_input, read_source};
use stop::Stop;

/// Exit-Codes der Schnittstelle (wie `k3c-sim`).
pub const EXIT_OK: u8 = 0;
/// Lauf gescheitert oder Panik der Engine (Zeile `error`).
pub const EXIT_FAILED: u8 = 1;
/// Aufruf falsch (Unterbefehl, Schalter, JSON).
pub const EXIT_USAGE: u8 = 2;
/// Abgebrochen (Zeile `aborted`).
pub const EXIT_ABORTED: u8 = 130;

/// Fehler eines Aufrufs.
#[derive(Debug, Clone, PartialEq)]
pub enum RunError {
    /// Aufruffehler, Exit 2.
    Usage(String),
    /// Lauf gescheitert, Exit 1.
    Failed(String),
    /// Abbruch über `--stop-stdin`, Exit 130.
    Aborted,
}

fn usage(msg: impl Into<String>) -> RunError {
    RunError::Usage(msg.into())
}

/// Einstieg der EXE: `args` stehen hinter `sim`.
pub fn main(args: &[String]) -> ExitCode {
    // `--stop-stdin` liest stdin bis zum Ende; die Eingabe kommt dann aus einer Datei (sonst Exit 2 in `exec`)
    let wants_stop = args
        .split_first()
        .and_then(|(_, rest)| parse_flags(rest).ok())
        .is_some_and(|f| f.stop_stdin && f.input != "-");
    let code = if wants_stop {
        let stop = Stop::on_end_of(std::io::stdin());
        exec(
            args,
            &mut std::io::empty(),
            &mut std::io::stdout(),
            &mut std::io::stderr(),
            &stop,
        )
    } else {
        exec(
            args,
            &mut std::io::stdin().lock(),
            &mut std::io::stdout(),
            &mut std::io::stderr(),
            &Stop::never(),
        )
    };
    ExitCode::from(code)
}

/// Führt einen Unterbefehl mit eigenen Ein- und Ausgaben aus und liefert den Exit-Code. Eine Panik wird zum Fehler
/// (Exit 1), nie zum Absturz.
pub fn exec(args: &[String], stdin: &mut dyn Read, out: &mut dyn Write, err: &mut dyn Write, stop: &Stop) -> u8 {
    let mut emitter = Emitter::new(out);
    let result = catch_unwind(AssertUnwindSafe(|| dispatch(args, stdin, &mut emitter, stop)))
        .unwrap_or_else(|p| Err(crashed(&p, err)));
    match result {
        Ok(()) => EXIT_OK,
        Err(RunError::Aborted) => {
            emitter.aborted();
            let _ = writeln!(err, "🛑 k3c-game sim: abgebrochen");
            EXIT_ABORTED
        }
        Err(RunError::Usage(m)) => {
            let _ = writeln!(err, "❌ k3c-game sim: {m}");
            emitter.error(&m);
            EXIT_USAGE
        }
        Err(RunError::Failed(m)) => {
            let _ = writeln!(err, "❌ k3c-game sim: {m}");
            emitter.error(&m);
            EXIT_FAILED
        }
    }
}

/// Macht den Wert einer Panik zum Fehler (Exit 1) und schreibt ihn nach stderr.
fn crashed(p: &Box<dyn std::any::Any + Send>, err: &mut dyn Write) -> RunError {
    let text = p
        .downcast_ref::<&str>()
        .map(|s| (*s).to_string())
        .or_else(|| p.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "unbekannte Panik".to_string());
    let _ = writeln!(err, "💥 k3c-game sim: Panik: {text}");
    RunError::Failed(format!("💥 Absturz in der Engine: {text}"))
}

fn dispatch(args: &[String], stdin: &mut dyn Read, out: &mut Emitter, stop: &Stop) -> Result<(), RunError> {
    let Some((cmd, rest)) = args.split_first() else {
        return Err(usage("Unterbefehl fehlt: run, balance, level, world oder hub"));
    };
    match cmd.as_str() {
        "run" | "level" => {
            let flags = parse_flags(rest)?;
            check_flags(cmd, &flags)?;
            let src = data_source(&flags)?;
            let raw = read_source(&flags.input, stdin)?;
            let text = if cmd == "run" {
                let r = read_run_input(&raw)?;
                enginetools_run::run(&src, &r.biome, &r.seed, r.ticks, &r.inputs, stop)?
            } else {
                let r = read_level_input(&raw)?;
                enginetools_level::level(&src, &r.biome, &r.seed)?
            };
            out.result_text(&text);
            Ok(())
        }
        "balance" => Err(usage("sim balance folgt mit RU40")),
        "world" | "hub" => {
            check_flags(cmd, &parse_flags(rest)?)?;
            Err(usage(format!("sim {cmd} folgt mit RU33.2")))
        }
        other => Err(usage(format!(
            "unbekannter Unterbefehl {} (bekannt: run, balance, level, world, hub)",
            quote(other)
        ))),
    }
}

/// Prüft die Schalter gegeneinander (Exit 2 bei Widerspruch).
fn check_flags(cmd: &str, f: &Flags) -> Result<(), RunError> {
    if f.headless && f.window {
        return Err(usage("--headless und --window schließen sich aus"));
    }
    if f.window && cmd != "run" && cmd != "balance" {
        return Err(usage("--window gibt es nur bei run und balance"));
    }
    if f.window {
        return Err(usage("sim run --window folgt mit RU33.3"));
    }
    if f.speed.is_some() {
        return Err(usage("--speed gibt es nur mit --window"));
    }
    if f.stop_stdin && f.input == "-" {
        return Err(usage(
            "--stop-stdin braucht --input DATEI (stdin ist sonst die Eingabe)",
        ));
    }
    Ok(())
}

/// Der Daten-Ordner: `--data`, sonst `K3C_DATA_DIR`, Ordner neben der EXE oder die eingebetteten Dateien.
pub fn data_source(f: &Flags) -> Result<Source, RunError> {
    match &f.data {
        Some(dir) if Path::new(dir).is_dir() => Ok(Source::Dir(dir.into())),
        Some(dir) => Err(usage(format!("--data: Ordner {dir} nicht gefunden"))),
        None => Ok(Source::from_env()),
    }
}
