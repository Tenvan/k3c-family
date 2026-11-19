//! Port von `main` aus `native/k3c-game/main.go` (RU30.6): Schalter lesen, den Wegwerf-Ordner der Messung anlegen und
//! aufräumen, das Spiel laufen lassen und das Ende melden. `build`, das Fenster und das Fehlerfenster liegen in
//! `launch.rs` (`Launch` zeigt einen Fehler vor dem Spiel mit demselben `Fault` wie einen Fehler im Spiel, Go: `showFault`),
//! die Aufnahme in `shots_run.rs` (eigener Wegwerf-Ordner), `--assets` in `launch::run`.
//!
//! Ein Fehler endet mit `💥 <Meldung>` auf stderr und Exit-Code 1 (Go: `fatal`), das normale Ende meldet
//! `🛑 K3C natives Spiel beendet` auf stdout; `--assets` endet still (Go: `return` vor der Meldung).

use std::process::ExitCode;

use super::Args;
use super::launch;
use super::shots_run::{remove_temp, shot_saves};

/// Meldung nach dem normalen Ende.
pub const END_MESSAGE: &str = "🛑 K3C natives Spiel beendet";

/// Läuft mit den gelesenen Schaltern bis zum Ende. Die Messung (`--measure`) arbeitet in einem Wegwerf-Ordner neben den
/// Spielständen (Autosave verändert `--saves` nicht), der danach verschwindet, auch bei einem Fehler (Go: `removeTemp`
/// vor `fatal`). Aufnahme (`--shots`) und `--assets` kommen ohne Umweg an `launch::run`.
pub fn run(args: &Args) -> Result<(), String> {
    if args.assets || args.measure.seconds <= 0.0 {
        return launch::run(args);
    }
    let tmp = shot_saves(args)?;
    let mut a = args.clone();
    a.saves = tmp.display().to_string();
    let res = launch::run(&a);
    remove_temp(&tmp);
    res
}

/// Liest die Schalter und startet das Spiel; zweiter Wert: `--assets` (endet ohne `🛑`-Meldung).
pub fn start(rest: &[String]) -> (Result<(), String>, bool) {
    match launch::parse(rest) {
        Ok(a) => (run(&a), a.assets),
        Err(e) => (Err(e), false),
    }
}

/// Meldet das Ergebnis wie Go (`fatal` bzw. `🛑`-Zeile) und liefert den Exit-Code.
pub fn finish(res: Result<(), String>, quiet: bool) -> u8 {
    match res {
        Ok(()) => {
            if !quiet {
                println!("{END_MESSAGE}");
            }
            0
        }
        Err(e) => {
            eprintln!("💥 {e}");
            1
        }
    }
}

/// Der Aufruf `k3c-game [play] [Schalter]`: Exit-Code 0 oder 1.
pub fn play(rest: &[String]) -> ExitCode {
    let (res, quiet) = start(rest);
    ExitCode::from(finish(res, quiet))
}

#[cfg(test)]
#[path = "run_tests.rs"]
mod run_tests;
